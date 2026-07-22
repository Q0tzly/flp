# Project Instructions

## Overview

`flp` is a minimal Rust CLI tool that prints the absolute path of a given file or directory, or the current working directory when no argument is provided.

## Build & Test

```sh
cargo check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
```

## Code Style

- Standard `rustfmt` formatting (`cargo fmt`)
- `clippy` nursery warnings are enabled in CI; clean code preferred
- Error handling uses `Result<Command, AppError>`; side effects are isolated in `main()`
- `Display` is implemented for error types rather than ad-hoc `message()` methods
- Tests target pure functions (`resolve_path`, `run`) rather than `main()`

## Release Checklist

1. Update `CHANGELOG.md` if present
2. Bump version in `Cargo.toml` following semver
3. Run full test suite: `cargo test`
4. Commit version bump with the functional change (not a separate commit)
5. Tag: `git tag vX.Y.Z`
6. Push tag: `git push origin vX.Y.Z`
7. Publish to crates.io: `cargo publish`

## Contact

Repository: https://github.com/Q0tzly/flp
