# Agent Instructions

You are working in `artichoke/sysdir-rs`, a Rust crate that exposes bindings to
`sysdir(3)` on Apple platforms and compiles as an empty crate elsewhere.

Users rely on Apple platform path semantics, off-target build behavior,
generated binding provenance, MSRV, and the public crate API. Treat those as
compatibility surfaces.

## Operating Loop

1. Classify the change before editing.
2. Read [ARCHITECTURE.md](ARCHITECTURE.md) for the crate codemap and durable
   invariants.
3. Use the matching workflow section below to choose the guardrails and runbooks
   to consult.
4. Keep the diff narrow. Do not mix behavior, dependency posture, release
   metadata, formatting, binding refreshes, and automation cleanup unless the
   task requires it.
5. Add or update focused tests for behavior changes, especially changes that
   affect platform gates, path semantics, or generated bindings.
6. Run checks that match the risk of the change; use
   [CONTRIBUTING.md](CONTRIBUTING.md) for local command expectations. If a
   relevant check is skipped, explain why in the PR.
7. Update README, crate docs, guardrails, or runbooks when public behavior,
   compatibility claims, target support, MSRV, dependency policy, or release
   process changes.

## Platform Behavior And Bindings

Use this workflow for Apple target behavior, off-target behavior, path
semantics, framework linkage, FFI signatures, and `sysdir(3)` binding changes.

Consult:

- [Platform-specific code](docs/guardrails/platform-specific-code.md), for
  target-gating and platform contract expectations.
- [FFI and foreign runtime integration](docs/guardrails/ffi-bindings-and-foreign-runtime-integration.md),
  for binding and ABI expectations.
- [Unsafe code](docs/guardrails/unsafe-code.md), for unsafe boundary review.
- [Testing and conformance](docs/guardrails/testing-compatibility-and-conformance.md),
  for target-matrix and regression coverage.

Preserve documented `sysdir(3)` semantics unless the task explicitly asks for a
breaking compatibility change.

## Binding Refreshes

Use this workflow for regenerating or reviewing generated bindings.

Consult:

- [Bindings freshness automation](docs/automations/bindings-freshness.md), for
  the refresh procedure.
- [FFI and foreign runtime integration](docs/guardrails/ffi-bindings-and-foreign-runtime-integration.md),
  for generated binding review expectations.
- [Platform-specific code](docs/guardrails/platform-specific-code.md), for
  target behavior after the refresh.

Keep generated binding diffs separate from unrelated cleanup.

## Public API, MSRV, And Releases

Use this workflow for API shape, docs.rs metadata, crate metadata, MSRV, semver,
publishing, changelog, and release-readiness changes.

Consult:

- [API stability, semver, and MSRV](docs/guardrails/api-stability-semver-and-msrv.md),
  for public contract and compatibility impact.
- [Working in public and publishing](docs/guardrails/working-in-public-and-publishing-oss-crates.md),
  for OSS release and communication expectations.

Call out compatibility and target-support impact in the PR.

## Implementation Quality

Use this workflow for refactors, lint posture, error handling, documentation
quality, crate attributes, and maintainability changes that do not intentionally
change behavior.

Consult:

- [High-quality Rust code](docs/guardrails/high-quality-rust-code.md), for lint,
  documentation, and maintainability expectations.
- [Testing and conformance](docs/guardrails/testing-compatibility-and-conformance.md),
  if the refactor touches behavior-sensitive paths.

Prefer mechanical refactors that preserve behavior and are easy to review.

## Dependencies, CI, And Automation

Use this workflow for dependency ranges, audits, Dependabot, GitHub Actions,
runner image updates, labels, and recurring maintenance.

Consult:

- [Dependency posture](docs/dependencies.md), for supply-chain expectations.
- [Dependency sweep automation](docs/automations/dependency-sweep.md), for
  dependency update procedure.
- [GitHub Actions runner images](docs/automations/github-actions-runner-images.md),
  for runner maintenance.
- [Working in public and publishing](docs/guardrails/working-in-public-and-publishing-oss-crates.md),
  if the change affects release or user-facing maintenance policy.

Keep mechanical dependency and automation updates separate from behavior
changes.

## Documentation-Only Changes

Use this workflow for README, crate docs, guardrails, runbooks, and PR/process
documentation.

Consult:

- [High-quality Rust code](docs/guardrails/high-quality-rust-code.md), for
  documentation quality expectations.
- [Working in public and publishing](docs/guardrails/working-in-public-and-publishing-oss-crates.md),
  for public-facing OSS communication.
- The guardrail for the topic being documented when docs describe API, FFI,
  platform, dependency, or release behavior.

Docs-only PRs may skip Rust tests when the PR explains why. Still run the repo
formatter.

## Pull Requests

- State the change class and compatibility impact.
- Use labels from `.github/labels.yaml`; include at least one `A-*` label.
- For automation-generated work, use `C-automation` and the `codex` label.
- Do not add a Codex tag to the title or description.
