# sysdir-rs

[![GitHub Actions](https://github.com/artichoke/sysdir-rs/workflows/CI/badge.svg)](https://github.com/artichoke/sysdir-rs/actions)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/sysdir.svg)](https://crates.io/crates/sysdir)
[![API](https://docs.rs/sysdir/badge.svg)](https://docs.rs/sysdir)

Enumeration of the filesystem paths for the various standard system directories
where apps, resources, etc. get installed.

This crate exposes Rust bindings to the `sysdir(3)` library functions provided
by `libSystem.dylib` on macOS, iOS, tvOS, and watchOS.

The `sysdir` API first appeared in OS X 10.12, iOS 10, watchOS 3 and tvOS 10
replacing the deprecated `NSSystemDirectories(3)` API.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sysdir = "1.3.3"
```

Then resolve well-known directories like this:

```rust
use core::ffi::{c_char, CStr};

use sysdir::*;

let mut path = [0; PATH_MAX as usize];

let dir = sysdir_search_path_directory_t::SYSDIR_DIRECTORY_USER;
let domain_mask = SYSDIR_DOMAIN_MASK_LOCAL;

unsafe {
    let mut state = sysdir_start_search_path_enumeration(dir, domain_mask);
    loop {
        let path = path.as_mut_ptr().cast::<c_char>();
        state = sysdir_get_next_search_path_enumeration(state, path);
        if state == 0 {
            break;
        }
        let path = CStr::from_ptr(path);
        let bytes = path.to_bytes();
        assert!(bytes.ends_with(b"/Users"));
    }
}
```

`sysdir-rs` exposes raw `sysdir(3)` search-path strings from Darwin. Those
values are not always normalized filesystem paths:

- user-domain results may contain a literal `~` instead of an expanded home
  directory
- if `NEXT_ROOT` is set and honored by the process, many local, network, and
  system-domain results are prefixed by that directory
- returned values are not guaranteed to be UTF-8 if `NEXT_ROOT` contains
  non-UTF-8 bytes

If you intend to use returned values with filesystem APIs, expand `~`, account
for `NEXT_ROOT`, and validate UTF-8 before opening or creating files.

You can test this crate works on your platform by running the example:

```shell
cargo run --example enumerate_system_dirs
```

## Development

`sysdir-rs` uses [mise][mise] to manage its local development toolchain. Install
`mise`, then install the tools declared in [`mise.toml`](mise.toml):

```sh
mise install
```

`mise.toml` installs Node.js, stable Rust with `clippy` and `rustfmt`,
`cargo-deny`, and `zizmor`.

Install the repository-local Node.js dependencies with:

```sh
npm ci
```

Build API documentation with nightly Rust:

```sh
rustup run --install nightly cargo doc --workspace
```

Maintenance of this repository is Codex-first. Prefer asking Codex to make
routine code, documentation, CI, and dependency updates, then review the
resulting diff and CI status before merging. See
[`CONTRIBUTING.md`](CONTRIBUTING.md) for contribution expectations.

## Implementation

sysdir-rs binds directly to `libSystem` with vendored bindings. This crate has
no dependencies other than `libSystem`.

[mise]: https://mise.jdx.dev/

Note that this crate is completely empty on non-Apple platforms.

## `no_std`

sysdir-rs is `no_std` and only requires `core`.

## Minimum Supported Rust Version

This crate requires at least Rust 1.85.0. This version can be bumped in minor
releases.

## License

`sysdir-rs` is distributed under the terms of either the
[MIT License](LICENSE-MIT) or the
[Apache License (Version 2.0)](LICENSE-APACHE).
