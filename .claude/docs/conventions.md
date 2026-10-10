# Project Conventions

_Extracted from `.andromeda/architecture.md` §Conventions (citations live there). Detail that does not fit CLAUDE.md's budget._

## File & directory naming
- Crates under `packages/`, apps under `apps/`, example crates under `examples/{name}/`, root examples as `examples/*.rs`; integration tests one file per behaviour in `tests/blitz-tests/tests/`, each its own test target — upstream's single-binary `tests/all.rs` stays in the manifest at `test = false`, never built, pinned by `BlitzTestsTargetsTest` — with the stand checks' shared tables and helpers stated once in `tests/common/mod.rs` (no test target).
- Modules and files open with a `//!` doc; examples open with a `//!` comment saying what they demonstrate.
- Paint rendering is split one module per concern (`blitz-paint/src/render/*`).

## Variable & identifier naming
- Standard Rust casing. No-op implementations are prefixed `Dummy` (`DummyNetProvider`, …); internal JS natives are prefixed `__blitz_`.
- Consuming builder methods named `with_*` / `without_*` (`with_virtual_time`, `without_timer_thread`).
- Stylo and Parley types are aliased under `stylo` / `parley` modules.
- `#[non_exhaustive]` on extensible public structs (`NavigationOptions`, `Request`).

## Manifests
- Dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`; dependency groups separated by comment headers.
- In-repo crates are declared with `default-features = false` — except `seven_guis`, which keeps its default renderer feature (dioxus-native refuses to compile with none); package metadata inherited with `*.workspace = true`.
- App and example crates, blitz-test-harness, escher-telemetry and escher-driver set `publish = false`; docs.rs builds with all features and `doc_cfg` under `docsrs`.

## Visibility
- Crate internals are `pub(crate)` / `pub(super)`; the slotmap key is used only at the storage boundary — public APIs use `NodeId`.

## Feature gating
- Optional capabilities are Cargo features forwarded across crates (each dioxus-native feature forwards to the same-named blitz feature; `accessibility` also to dioxus-native-dom's, which gates its own `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules). The workspace takes both Dioxus crates with `default-features = false`, so a feature is on only where a crate names it or depends on a crate that names it — escher-driver names dioxus-native-dom's `accessibility`, which reaches both seven_guis binaries through that edge.
- In the engine and upstream crates `tracing` is gated per call site with a `#[cfg(not(feature = "tracing"))] let _ = …;` fallback; escher-telemetry (no `[features]`) emits its startup and panic events ungated, and escher-driver takes `tracing` with no feature for its one command span, built by hand with `tracing::info_span!` (no `#[instrument]`).
- Desktop-only code gated by the repeated cfg list windows / macos / linux / dragonfly / freebsd / netbsd / openbsd.

## Error handling
- Fallible APIs return typed results (`Result<_, CssomError>`, `Result<_, ParseError>`, `Result<Resource, String>`); error types are enums with `Display` and `From` per wrapped error.
- Mutex locks recover from poisoning with `unwrap_or_else(|err| err.into_inner())`.
- `clippy::expect_used` / `clippy::unwrap_used` allowed only locally, next to a justification. Examples may fail with `unwrap`/`expect`/`panic!`.

## Unsafe and borrowing
- `unsafe` blocks carry an `allow unsafe_code` or `// SAFETY:` comment.
- Borrow conflicts are worked around by taking a child list or layout out and restoring it; let-chains in `if` conditions are used.

## DOM API patterns
- Mutations go through `DocumentMutator`, which flushes deferred work on `Drop`; take a Stylo snapshot before attribute mutation of in-document nodes.
- Event-producing functions take a `dispatch_event` callback.
- Repeated regexes live in `LazyLock<Regex>` or are compiled per worker.

## Known gaps and markers
- Gaps marked `TODO` / `FIXME`; ordering-sensitive blocks marked `WARNING: DO NOT REORDER`; sections delimited with `// === … ===`.
- Spec URLs are cited above or next to implementations; constants carry a rationale doc comment.

## Lints
- `clippy::collapsible_if` allowed crate-wide in browser, blitz-dom, blitz-html, blitz-vibey-script; other allows are local, ideally with a reason.

## Logging
See `.claude/rules/observability.md` for enforcement; this doc carries the conventions.

## Testing
See `.claude/rules/testing.md` (authoring) and `.claude/rules/verification-harness.md` (the `Harness` driver).

## Apps and examples
- Binaries set `windows_subsystem = "windows"` outside tests; Android entry `#[unsafe(no_mangle)] pub fn android_main`.
- Dioxus examples: `fn main() { dioxus_native::launch(app); }`, `fn app() -> Element`, `#[component]`, state via `use_signal` / `use_store` / `use_memo`; example CSS in `const CSS: &str` raw strings.

## Cross-references
- Architecture: `.andromeda/architecture.md` · directory layout: §Infrastructure Patterns · gotchas: `.claude/docs/gotchas.md` · per-crate notes: `.claude/docs/services/`.
