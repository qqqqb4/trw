# Testing guidance

Read [TESTING.md](TESTING.md) for the existing test suite, coverage, commands, and proxy configuration.

## Writing tests

- Follow the existing Rust `#[test]` modules in `src/<module>/tests.rs` and `src/providers/libretranslate/tests.rs`. Register new test modules with `#[cfg(test)] mod tests;` in their parent module.
- Add or update tests when changing behavior. Cover successful cases, relevant error cases, and regressions; assert intended behavior rather than preserving known bugs.
- Reuse `src/test_support.rs` for HTTP tests. Use loopback servers with OS-assigned ports, not external services. Bound waits with timeouts and ensure worker and server threads terminate.
- Use `tempfile` for filesystem tests. Do not read or modify real user settings or mutate process environment variables; keep tests safe to run in parallel.
- Follow the headless harness in `src/app/tests.rs` for UI state tests. Set notification timestamps directly instead of sleeping for the TTL.
- Run the test, formatting, and Clippy commands documented in `TESTING.md`. Report failures and skipped tests explicitly; do not hide failures by weakening assertions or silently ignoring tests.

## Keeping documentation current

Update `TESTING.md` whenever test coverage, organization, fixtures, dependencies, or execution requirements change. Keep it accurate about what the suite actually tests and how to run it.
