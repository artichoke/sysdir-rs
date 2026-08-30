# Contributing to sysdir-rs

Welcome to [Artichoke]. Thanks for taking the time to contribute.

sysdir-rs provides Rust bindings to the Darwin `sysdir(3)` APIs. This crate
enumerates filesystem paths for standard system directories on macOS, iOS, tvOS,
and watchOS.

If sysdir-rs does not resolve a system directory correctly, or if its platform
behavior is surprising, please [file an issue].

Maintenance of this repository is Codex-first. Prefer asking Codex to prepare
routine code, documentation, CI, and dependency changes. Contributors should
focus on issue selection, review, release decisions, and validating that the
resulting diff and CI status match the intended change.

## Setup

sysdir-rs includes Rust, C header, and text sources. Developing on sysdir-rs
requires configuring several dependencies.

sysdir-rs uses [mise] to manage the local development toolchain declared in
[`mise.toml`](mise.toml), including Node.js, Rust, and auxiliary Rust tools. For
Rust, `mise` uses [rustup] under the hood. Nightly-only Rust workflows in this
repository continue to use `rustup` directly.

### Rust Toolchain

sysdir-rs depends on Rust and several compiler plugins for linting and
formatting. sysdir-rs is guaranteed to build on the Rust version declared as the
minimum supported Rust version in [`Cargo.toml`](Cargo.toml).

#### Installation

Install and activate [mise], then install the toolchains declared in
[`mise.toml`](mise.toml):

```sh
mise install
```

`mise.toml` configures the latest stable Rust toolchain with the `minimal`
profile plus the `clippy` and `rustfmt` components. `mise` installs that
toolchain via [rustup].

Documentation checks use nightly Rust. Install nightly with
`rustup toolchain install nightly` if you run those workflows locally.

To update your stable Rust compiler to the latest version, run:

```sh
rustup update stable
```

### Rust Crates

sysdir-rs has no third-party Rust crate dependencies. Once you have the Rust
toolchain installed, you can build the crate by running:

```sh
cargo build
```

sysdir-rs uses direct tool invocations instead of a repository task runner. The
most common development commands are:

```sh
cargo build --workspace
cargo test --workspace
cargo fmt
cargo clippy --workspace --all-features --all-targets
pnpm run fmt
RUSTDOCFLAGS="-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs" \
  cargo +nightly doc --workspace
```

The repository uses direct tool invocations instead of a language-specific task
runner.

### Node.js

Node.js is an optional dependency that is used for formatting text sources with
[prettier].

Node.js is only required for formatting if modifying the following filetypes:

- `md`
- `yaml`
- `yml`

Install Node.js with `mise`:

```sh
mise install
```

Install the repository-local Node.js dependencies with:

```sh
pnpm install
```

## Linting

To lint and format Rust sources run:

```sh
cargo clippy --workspace --all-features --all-targets
cargo fmt
```

To format text sources run:

```sh
pnpm exec prettier --check '**/*'
```

## Testing

A PR must have new or existing tests for it to be merged. The [Rust book chapter
on testing] is a good place to start.

To run tests:

```sh
cargo test
```

`cargo test` accepts a filter argument that will limit test execution to tests
that substring match. For example, to run all of the `NEXT_ROOT` tests:

```sh
cargo test next_root
```

Tests are run for every PR. All builds must pass before merging a PR.

## Codex Maintenance Workflow

Prefer asking Codex to prepare changes on a branch, including any docs and CI
updates needed for the patch. Review the resulting diff as authored code:

- Confirm the change is scoped to the issue or maintenance task.
- Confirm generated or mechanical changes are intentional.
- Confirm CI passes before merging.
- Ask Codex to follow up on review comments or failed checks.

## Bindings

sysdir-rs binds directly to `libSystem` with vendored bindings in
[`src/sys.rs`](src/sys.rs). Treat that file as checked-in source; changes to the
generated bindings should be rare and reviewed carefully.

## Publishing

Maintainers publish releases through crates.io trusted publishing. See
[`docs/publishing.md`](docs/publishing.md) for the trust configuration, release
procedure, and failure-recovery guidance.

## Updating Dependencies

### Rust Crates

sysdir-rs does not currently depend on third-party Rust crates. If a dependency
is added in the future, version specifiers in `Cargo.toml` are NPM caret-style
by default. A version specifier of `4.1.2` means `4.1.2 <= version < 5.0.0`.

Regular dependency bumps are handled by [@dependabot].

[mise]: https://mise.jdx.dev/
[rustup]: https://rustup.rs/
[prettier]: https://prettier.io/
[rust book chapter on testing]:
  https://doc.rust-lang.org/book/ch11-00-testing.html
[@dependabot]: https://dependabot.com/
[artichoke]: https://github.com/artichoke
[file an issue]: https://github.com/artichoke/sysdir-rs/issues/new
