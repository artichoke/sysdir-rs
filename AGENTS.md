# Repository Map

This file is a map for agents working in this repository. It points to the
source-of-truth docs, configuration, and code landmarks; it should not duplicate
the policy held by those files.

## Start Here

- `README.md`: crate purpose, supported platform behavior, and public examples.
- `CONTRIBUTING.md`: local development setup and command expectations.
- `Cargo.toml`: crate metadata, MSRV, dependency ranges, and docs.rs metadata.
- `docs/guardrails/README.md`: index for Rust, OSS, unsafe, platform, testing,
  API, FFI, and performance guardrails.
- `docs/dependencies.md`: dependency and supply-chain posture.
- `docs/automations/README.md`: recurring maintenance map.
- `.github/labels.yaml`: PR label vocabulary for this repository.

## Change Map

- Public API, semver, MSRV, target support, or publishing:
  `docs/guardrails/api-stability-semver-and-msrv.md`,
  `docs/guardrails/working-in-public-and-publishing-oss-crates.md`,
  `Cargo.toml`, `README.md`, and `src/lib.rs`.
- Rust implementation quality, lints, generated docs, or error handling:
  `docs/guardrails/high-quality-rust-code.md`, `CONTRIBUTING.md`, `src/lib.rs`,
  and `.github/workflows/ci.yaml`.
- Platform or FFI behavior: `docs/guardrails/platform-specific-code.md`,
  `docs/guardrails/ffi-bindings-and-foreign-runtime-integration.md`,
  `docs/guardrails/unsafe-code.md`, `src/lib.rs`, `src/sys.rs`, `cext/sysdir.h`,
  and `sysdir.3`.
- Binding refreshes: `docs/automations/bindings-freshness.md`, `src/sys.rs`,
  `cext/sysdir.h`, and `sysdir.3`.
- Tests, target matrix, or off-target behavior:
  `docs/guardrails/testing-compatibility-and-conformance.md`,
  `tests/next_root.rs`, `examples/enumerate_system_dirs.rs`, and
  `.github/workflows/ci.yaml`.
- Dependency, audit, or runner maintenance: `docs/dependencies.md`,
  `docs/automations/dependency-sweep.md`,
  `docs/automations/github-actions-runner-images.md`, `.github/dependabot.yml`,
  `.github/workflows/audit.yaml`, and `.github/workflows/repo-labels.yaml`.
- Markdown, YAML, JSON, or generated formatting changes: `package.json`,
  `.prettierrc.yaml`, and `pnpm-lock.yaml`.

## Code Map

- `src/lib.rs`: crate-level docs, target gates, public API, and platform
  behavior.
- `src/sys.rs`: generated bindings to the platform `sysdir(3)` surface.
- `cext/sysdir.h`: bindgen input header.
- `sysdir.3`: source manual page used to understand upstream behavior.
- `tests/next_root.rs`: coverage for `NEXT_ROOT` path behavior.
- `examples/enumerate_system_dirs.rs`: public usage example and smoke test for
  enumeration.

## Pull Request Map

- Use labels from `.github/labels.yaml`; lopopolo-owned repositories require at
  least one `A-*` label.
- For automation-generated work, use `C-automation` and add the `codex` label.
  Keep `codex` as the last label definition in `.github/labels.yaml`.
- Do not add a Codex tag to PR titles or descriptions.
