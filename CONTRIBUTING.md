# Contributing to display-types

Thanks for your interest in contributing. This document covers the basics.

## Getting started

See [`doc/setup.md`](doc/setup.md) for build and test instructions.

## Issues and pull requests

**Open an issue first** if you're unsure whether something is a bug or if you want to
discuss a change before implementing it. For small, self-contained fixes a PR on its own
is fine.

- Bug reports: describe the EDID field or data block in question and what the type
  fails to represent or express correctly.
- Feature requests: a brief description of what you need and why is enough to start a
  conversation.
- PRs: keep them focused. One logical change per PR makes review faster and keeps
  history readable.

## Coding standards

- Run `cargo fmt` and `cargo clippy -- -D warnings` before pushing.
- Public items need rustdoc comments (`cargo rustdoc -- -D missing_docs` must pass).
- `#![forbid(unsafe_code)]` is enforced; no unsafe code.
- Keep `no_std` compatibility. All scalar types (enums and copy structs) must compile
  without `alloc` or `std`. Heap-allocated types are gated behind `#[cfg(any(feature =
  "alloc", feature = "std"))]`.
- All public structs and enums must be `#[non_exhaustive]`.

## Commit and PR expectations

- Write commit messages in the imperative mood ("Add support for …", not "Added …").
- Keep commits logically atomic. A PR that touches three unrelated things should be
  three commits (or three PRs).
- Tests are expected for new logic. New types without associated logic (plain data
  structs) don't need unit tests, but any function that maps or derives values from
  inputs does.
- CI must be green before a PR can merge: fmt, clippy, docs, all test and build
  targets, and coverage must not drop more than 0.1% below the baseline (stored in
  `.coverage-baseline`). New logic without tests will likely trip this.

## Publishing and downstream dependencies

display-types is consumed by [piaf](https://github.com/DracoWhitefire/piaf) and
[concordance](https://github.com/DracoWhitefire/concordance) as versioned crate
dependencies. Changes that add or modify public API must be merged, tagged, and
published to crates.io before downstream crates can adopt them. The workflow is:

1. Merge and tag a release in this repo (`vX.Y.Z`).
2. The publish CI workflow uploads the new version to crates.io.
3. Update the version constraint in the downstream crate's `Cargo.toml`.

## Review process

PRs are reviewed on a best-effort basis. Expect feedback within a few days; if you
haven't heard back in a week feel free to ping the thread. Reviews aim to be
constructive — if something needs to change, the reviewer will explain why. Approval
from the maintainer is required to merge.

## Code of Conduct

This project follows the [Contributor Covenant 3.0](CODE_OF_CONDUCT.md). Please read
it before participating.
