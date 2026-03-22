# display-types

[![CI](https://github.com/DracoWhitefire/display-types/actions/workflows/ci.yml/badge.svg)](https://github.com/DracoWhitefire/display-types/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/display-types.svg)](https://crates.io/crates/display-types)
[![docs.rs](https://docs.rs/display-types/badge.svg)](https://docs.rs/display-types)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rustc-1.85+-orange.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)

Shared display capability types for display connection negotiation.

This crate provides [`DisplayCapabilities`] and all its supporting types —
the stable, typed model that EDID/DisplayID parsers produce and negotiation
engines consume. It is the shared vocabulary between
[piaf](https://crates.io/crates/piaf) (the parser) and
[concordance](https://crates.io/crates/concordance) (the negotiation engine),
and is suitable for any crate that needs to describe display capabilities without
taking a dependency on a full parser.

## Feature flags

| Flag | Default | Description |
|------|---------|-------------|
| `std` | yes | Enables `std`-dependent types; implies `alloc`. |
| `alloc` | no | Enables heap-allocated types (`Vec`, `Arc`, `String`) without full `std`. |
| `serde` | no | Derives `Serialize`/`Deserialize` for all public types. |

With neither `std` nor `alloc` the crate compiles in bare `no_std` mode and
exposes only the scalar types (enums and copy structs).

## Usage

```toml
[dependencies]
display-types = "0.1"
```

For `no_std` with heap allocation (e.g. embedded with an allocator):

```toml
[dependencies]
display-types = { version = "0.1", default-features = false, features = ["alloc"] }
```

For bare `no_std` (scalars only):

```toml
[dependencies]
display-types = { version = "0.1", default-features = false }
```

## License

Licensed under the [Mozilla Public License 2.0](LICENSE).
