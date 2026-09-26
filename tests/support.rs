//! Loopback-only HTTP fixture shared by provider and worker tests.
//! All waits are bounded; no environment variables or user settings are changed.

use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::network::FromUIMessage;

pub const TEST_TIMEOUT: Duration = Duration::from_secs(5);

pub fn translation_request(text: &str) -> FromUIMessage {
    FromUIMessage {
        input_lang: "en".to_owned(),
        target_lang: "ru".to_owned(),
        text: text.to_owned(),
    }
}

#[derive(Debug)]
pub struct RecordedRequest {
    pub request_line: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

impl RecordedRequest {
    pub fn form(&self) -> BTreeMap<String, String> {
        form_urlencoded::parse(&self.body).into_owned().collect()
    }
}

pub struct MockServer {
    pub address: String,
    requests: Receiver<RecordedRequest>,
    stop: Sender<()>,
    task: Option<JoinHandle<()>>,
}

impl MockServer {
    pub fn new(responses: impl IntoIterator<Item = (u16, &'static str)>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock server");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let address = listener.local_addr().expect("mock address").to_string();
        let responses: Vec<_> = responses.into_iter().collect();
        let (request_tx, requests) = mpsc::channel();
        let (stop, stop_rx) = mpsc::channel();
        let task = thread::spawn(move || {
            for (status, body) in responses {
                let Some(mut stream) = accept_request(&listener, &stop_rx) else {
                    return;
                };
                stream
                    .set_read_timeout(Some(TEST_TIMEOUT))
                    .expect("read timeout");
                stream
                    .set_write_timeout(Some(TEST_TIMEOUT))
                    .expect("write timeout");
                let request = read_request(&mut stream).expect("read HTTP request");
                // Close each connection so the fixture also works with pooled clients.
                write!(stream,
                    "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len(),
                ).expect("write mock response");
                request_tx.send(request).expect("record HTTP request");
            }
        });
        Self {
            address,
            requests,
            stop,
            task: Some(task),
        }
    }

    pub fn request(&self) -> RecordedRequest {
        self.requests
            .recv_timeout(TEST_TIMEOUT)
            .expect("mock server did not receive a request")
    }

    pub fn finish(mut self) {
        self.task
            .take()
            .expect("server task")
            .join()
            .expect("mock server panicked");
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(task) = self.task.take() {
            let result = task.join();
            // Do not panic twice if a test assertion is already unwinding.
            if !thread::panicking() {
                result.expect("mock server panicked");
            }
        }
    }
}

fn accept_request(listener: &TcpListener, stop: &Receiver<()>) -> Option<TcpStream> {
    let deadline = Instant::now() + TEST_TIMEOUT;
    loop {
        match listener.accept() {
            Ok((stream, _)) => return Some(stream),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                assert!(
                    Instant::now() < deadline,
                    "timed out waiting for HTTP request"
                );
                match stop.recv_timeout(Duration::from_millis(5)) {
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    _ => return None,
                }
            }
            Err(error) => panic!("accept mock request: {error}"),
        }
    }
}

fn read_request(stream: &mut TcpStream) -> io::Result<RecordedRequest> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut headers = BTreeMap::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "incomplete headers",
            ));
        }
        if line == "\r\n" {
            break;
        }
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid header"))?;
        headers.insert(name.to_ascii_lowercase(), value.trim().to_owned());
    }
    // send_form uses Content-Length, not chunked transfer encoding.
    let length = headers
        .get("content-length")
        .and_then(|length| length.parse::<usize>().ok())
        .filter(|length| *length <= 1024 * 1024)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid content length"))?;
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    Ok(RecordedRequest {
        request_line: request_line.trim_end().to_owned(),
        headers,
        body,
    })
}
