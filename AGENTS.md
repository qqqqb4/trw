# Overall Rules

You are prohibited from making any changes unrelated to adding or editing unit testing.
You are prohibited from writing commit messages or creating pull requests. 

# Testing guidance

Read [TESTING.md](TESTING.md) for the existing test suite, coverage, commands, and proxy configuration.

## Writing tests

- Follow the existing Rust `#[test]` modules in `tests/`. Each parent module includes its file with `#[cfg(test)]` and `#[path]`, for example `#[path = "../tests/config.rs"] mod tests;`. Do not add a `tests/*.rs` file that Cargo would treat as its own crate.
- Add or update tests when changing behavior. Cover successful cases, relevant error cases, and regressions; assert intended behavior rather than preserving known bugs.
- Reuse `tests/support.rs` for HTTP tests. Use loopback servers with OS-assigned ports, not external services. Bound waits with timeouts and ensure worker and server threads terminate.
- Use `tempfile` for filesystem tests. Do not read or modify real user settings or mutate process environment variables; keep tests safe to run in parallel.
- Follow the headless harness in `tests/app.rs` for UI state tests. Set notification timestamps directly instead of sleeping for the TTL.
- Run the test, formatting, and Clippy commands documented in `TESTING.md`. Report failures and skipped tests explicitly; do not hide failures by weakening assertions or silently ignoring tests.

## Keeping documentation current

Update `TESTING.md` whenever test coverage, organization, fixtures, dependencies, or execution requirements change. Keep it accurate about what the suite actually tests and how to run it.
