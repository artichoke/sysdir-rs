# Bindings Freshness Automation

The bindings freshness automation is a scheduled repository maintenance role for
checking whether the checked-in `sysdir(3)` bindings or bundled `sysdir(3)` man
page have drifted from the current macOS SDK.

The automation must read `docs/automations/README.md` and this document before
running. Human feedback on any output it produces, including inbox follow-ups,
pull request comments, review decisions, failed validation, or missed generated
content changes, should be reviewed and used to update this document before
repeating the same class of issue. Automation-authored pull request comments
must start with the stable prefix `Codex automation note:`. Treat comments with
that prefix as automation state, not human feedback for this learning loop.

## Schedule

Run once per week against this repository on macOS. SDK and man page changes are
infrequent, but a weekly check keeps drift visible after Xcode and runner image
updates.

## Scope

Review generated or SDK-derived repository content:

- `src/sys.rs`, the checked-in Rust bindings for `sysdir(3)`;
- `sysdir.3`, the checked-in rendered man page;
- `cext/sysdir.h`, the bindgen input header;
- `mise.toml`, when local generation tooling needs to change.

Do not make unrelated code, documentation, CI, dependency, or release changes in
this automation.

## Generation Checks

Use the repository toolchain from `mise.toml` when available. The bindgen CLI is
provided by `cargo:bindgen-cli`.

Generate fresh bindgen output from the current SDK with:

```sh
bindgen --use-core \
  --allowlist-function 'sysdir.*' \
  --allowlist-type 'sysdir.*' \
  --allowlist-var 'PATH_MAX' \
  --rustified-enum 'sysdir.*' \
  cext/sysdir.h
```

Compare the generated API surface with `src/sys.rs`. The checked-in file
contains repository-local review edits such as the license header,
`#[non_exhaustive]`, and the opaque enumeration state helper. Do not blindly
overwrite those edits. If bindgen discovers new constants, types, function
signatures, or generated attributes, update `src/sys.rs` deliberately and
preserve the local review edits unless they are no longer correct.

Regenerate the bundled man page with:

```sh
man sysdir | col -bx
```

Compare that output with `sysdir.3`. If the SDK man page changed, update the
checked-in copy.

If generation cannot run because the host is not macOS, Xcode command-line tools
are unavailable, `sysdir(3)` is missing, or bindgen is not installed, open an
inbox item that explains the blocker instead of guessing whether the bindings
are fresh.

## Changes

If generated content is current, do not create a branch, commit, push, or pull
request. Open an inbox item with the SDK or runner image checked, the commands
run, and the result.

If `src/sys.rs` or `sysdir.3` changes, create one pull request for the generated
content update. Pull requests from this automation must include the `A-ffi`,
`A-release`, `C-automation`, and `codex` labels.

Any merged generated-content change should lead to a new minor release of
`sysdir`. If the automation opens a generated-content pull request, include a
release follow-up in the inbox summary. After that pull request merges, open a
separate release-prep pull request that bumps the crate minor version, updates
`html_root_url`, updates README installation examples, and prepares the tag for
the publish workflow. Do not duplicate an existing open release-prep pull
request.

Do not enable auto-merge for generated-content changes unless the diff is
obviously mechanical, validation passes, and the generated API surface is
backward compatible.

## Validation and Summary

For generated-content changes, run:

```sh
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-features --all-targets
RUSTDOCFLAGS="-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs" \
  cargo +nightly doc --workspace
npx prettier --check '**/*'
```

If a release-prep pull request is opened, also run `cargo package --allow-dirty`
before finalizing the pull request.

Open an inbox item after every run summarizing:

- SDK, runner image, and Xcode versions inspected;
- bindgen command and man page command results;
- whether `src/sys.rs` changed, and what API surface changed;
- whether `sysdir.3` changed;
- validation run and any skipped checks;
- pull request and auto-merge status, if a pull request was opened;
- release-prep status or required human follow-up.
