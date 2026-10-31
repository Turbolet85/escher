# arch · Conventions — category read

## Carried
- (.andromeda/architecture-detail/conventions.md:8) The popup's code in blitz-dom, blitz-paint, dioxus-native-dom and escher-driver must pass both clippy lines and fmt, the blitz-dom line building it with its own feature list, through the one leg runner.
  > `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings` — then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` — each run as its `.github/scripts/ci-leg.sh` leg in CI and on the dev host
- (.andromeda/architecture.md:102) Every doc comment the chunk adds or restates, the schema help's neighbours included, must build clean under rustdoc's deny.
  > rustdoc warnings are errors workflow-wide, over every library crate
- (.andromeda/architecture-detail/conventions.md:8) Any `expect` or `unwrap` the overlay needs must carry a local allow beside its justification.
  > `clippy::expect_used` / `clippy::unwrap_used` allowed locally next to a justification
- (.andromeda/architecture-detail/conventions.md:21) Any new module or file for the overlay in paint, hit testing or keys must open with `//!` docs.
  > modules and files open with `//!` docs
- (.andromeda/architecture-detail/conventions.md:21) Any public item the chunk adds must carry `///`.
  > public items carry `///`
- (.andromeda/architecture-detail/conventions.md:21) The open list's behaviour must cite the HTML specification's select rules above or next to its implementation.
  > spec URLs cited above or next to implementations
- (.andromeda/architecture-detail/conventions.md:21) Each constant the chunk adds, the list's engine-default colours and sizes among them, must state its rationale in its doc comment.
  > doc comments state rationale for constants
- (.andromeda/architecture-detail/conventions.md:27) Any gap the popup leaves open in the code must be marked in this form.
  > marked with `TODO`/`FIXME`
- (.andromeda/architecture-detail/conventions.md:33) The popup's open state and its internals must stay crate-private, with nothing widened beyond what another crate reads.
  > crate internals are `pub(crate)` / `pub(super)`
- (.andromeda/architecture-detail/conventions.md:33) The list's paint must live in a module of its own concern, not folded into an existing one.
  > paint rendering split one module per concern
- (.andromeda/architecture-detail/conventions.md:39) Any popup code that reads an optional capability, the tree's open state under `accessibility` among it, must sit behind that capability's cargo feature.
  > optional functionality gated by cargo features
- (.andromeda/architecture-detail/conventions.md:39) Any tracing the popup adds in an engine crate must be gated per call site with this fallback.
  > in the engine and upstream crates tracing is gated per call site with `#[cfg(feature = "tracing")]`, with a `#[cfg(not(feature = "tracing"))] let _ = …;` fallback
- (.andromeda/architecture.md:116) The overlay's walks over a select's options and any `unsafe` it needs must follow these forms.
  > `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments; borrow conflicts worked around by taking a child list or layout out and restoring it; let-chains in `if` conditions
- (.andromeda/architecture.md:119) Any change of DOM state the popup's open, close or choice writes must go through the mutator.
  > mutations go through `DocumentMutator`, which flushes deferred work on `Drop` and requests a redraw only for in-document mutations
- (.andromeda/architecture.md:119) If the open state is held as an attribute, its write must take the Stylo snapshot first, so the sheet restyles the select.
  > a Stylo snapshot is taken before attribute mutation of in-document nodes
- (.andromeda/architecture.md:119) The list's key and pointer functions, which produce the select's `input` and `change`, must take the callback.
  > event-producing functions take a `dispatch_event` callback
- (.andromeda/architecture.md:106) The list's rules added to blitz-dom's user-agent sheet must keep that file's licence header in place.
  > blitz-dom's `assets/default.css` carries an MPL 2.0 header
- (.andromeda/architecture-detail/conventions.md:52) Unit tests of the select rules and the overlay must sit at the end of their source file in this form.
  > unit tests in `#[cfg(test)] mod tests` at the end of the source file
- (.andromeda/architecture-detail/conventions.md:52) A unit-test module the chunk moves or adds in a file of its own must follow this form and move the stated count of eight.
  > eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent
- (.andromeda/architecture-detail/conventions.md:52) The restated `covered` and open-list schema text must be tested in the schema's existing test file and module path.
  > escher-driver's `mcp::tests` and `schema::tests` in `mcp_tests.rs` and `schema_tests.rs`
- (.andromeda/architecture-detail/conventions.md:53) Each new popup check in blitz-tests must cover one behaviour and open with this doc.
  > each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug
- (.andromeda/architecture-detail/conventions.md:53) A document-level popup test must build its document in this form.
  > documents built with `HtmlDocument::from_html` and a `DocumentConfig` with a `ColorScheme::Light` viewport and `HtmlProvider`, then `resolve(0.0)`
- (.andromeda/architecture-detail/conventions.md:53) Popup layout and geometry scenarios, the options' real boxes among them, must run in both layout modes.
  > scenarios run in both layout modes `for incremental in [false, true]`
- (.andromeda/architecture-detail/conventions.md:53) New test names must follow this form.
  > descriptive snake_case test names
- (.andromeda/architecture-detail/conventions.md:53) Assertion helpers the popup's tests add must report the caller's line.
  > `#[track_caller]` assertion helpers
- (.andromeda/architecture-detail/conventions.md:53) A popup check that clicks an option may repeat the pointer-event builder in its own file, outside the stand's shared modules.
  > a pointer-event builder is repeated per file
- (.andromeda/architecture-detail/conventions.md:54) Any stand table the popup moves or row it adds that `stand_contrast`, `stand_keyboard` or `stand_lint` share must be stated once in this module.
  > while the seven_guis stand checks state the tables and helpers they share once, in the module `tests/blitz-tests/tests/common/mod.rs`, which a check reads by declaring `mod common;` — a module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior
- (.andromeda/architecture-detail/conventions.md:54) `stand_act_select` and any new session-holding popup check must share through this module, and a new `stand_act_*` check must move the stated count of fifteen.
  > the checks that hold a `Session` — the session checks, the settle check `stand_settle`, the fifteen driver-action checks `stand_act_*`, since 2026-10-10-mcp-surface `stand_mcp_tools`, since 2026-10-12-headless-screenshot, `stand_screenshot_repeat` and, since 2026-10-14-stand-requirement-sweep, the driver sweep `stand_requirement_sweep`, and the served-page checks `stand_cells_served` and `stand_load_page` — keep what they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`, read by `mod session_common;`
- (.andromeda/architecture-detail/conventions.md:54) Each popup test must first assert its fixture holds the list open, or closed, before asserting what follows.
  > tests first assert the fixture produces the condition under test
- (.andromeda/architecture-detail/conventions.md:54) A new popup check must be one file directly under `tests/`, with no `[[test]]` table added to the manifest.
  > blitz-tests keeps one test target per file directly under `tests/` — its `[package]` sets no `autotests` — and the one file there that is declared and never built is upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, held at `test = false`
- (.andromeda/architecture-detail/conventions.md:61) A popup fixture page read by more than one check must live in this module, and one with a single reader must stay in its check's file.
  > A third such module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; what it holds is test-plan §3's (Crate-local test helpers), and a page with one reader stays in its check's file
- (.andromeda/architecture-detail/conventions.md:62) Any row the popup adds to the sweep or the MCP listing test must keep this split, its messages naming the row and index only.
  > escher-driver's MCP listing test and blitz-tests' 0.2.0 requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private `#[track_caller]` helpers whose messages name the row and index only
- (.andromeda/architecture-detail/conventions.md:68) Any CSS the chunk adds to the flight booker's page for the stand's popup must follow this form, the list's defaults staying in the engine's sheet.
  > example CSS held in `const CSS: &str` raw strings emitted in a `style` element

## Bindings
- **Crate-private internals ↔ §Standard Contracts, Nodes and widgets**: the visibility rule (.andromeda/architecture-detail/conventions.md:33) meets the select's rules living in the crate-private `node::select` with no public type (.andromeda/architecture.md:151) and the choice's crate-private writers (.andromeda/architecture.md:135); the popup's open state must fit both.
- **`dispatch_event` callback ↔ §Standard Contracts, Events**: a callback-taking event function (.andromeda/architecture.md:119) must keep `change` after `input` on a select's choice and the queued Click order (.andromeda/architecture.md:141).
- **Paint module per concern ↔ §Standard Contracts, Paint and shell**: the list's module (.andromeda/architecture-detail/conventions.md:33) paints through `paint_scene` for every document and renderer (.andromeda/architecture.md:157).
- **Spec URLs ↔ §Design Philosophy**: the citation form (.andromeda/architecture-detail/conventions.md:21) serves "Behave like a browser, and cite the spec" (.andromeda/architecture.md:9).
- **Clippy's blitz-dom line ↔ §Established Decisions, Default features**: its feature list (.andromeda/architecture-detail/conventions.md:8) must build whatever the popup gates by feature, against blitz-dom's default set (.andromeda/architecture.md:42).
- **Feature gating ↔ §Cross-cutting Patterns, Config management**: the cargo-feature rule (.andromeda/architecture-detail/conventions.md:39) meets `accessibility` gating the snapshot modules (.andromeda/architecture.md:265), where the options' real boxes are read.
- **Popup checks ↔ §Established Decisions, the select model**: the tests' forms (.andromeda/architecture-detail/conventions.md:53) hold the chunk's restatement of "no popup is painted" and of `covered` on a closed option (.andromeda/architecture.md:62).
- **Fixtures module ↔ test-plan §3**: the module's holding (.andromeda/architecture-detail/conventions.md:61) is named there as test-plan §3's, Crate-local test helpers, so a popup fixture binds on both sides.
- **MPL header ↔ §Infrastructure Patterns, Build system**: the sheet under its header (.andromeda/architecture.md:106) is the default user-agent stylesheet embedded at compile time (.andromeda/architecture.md:224), so a list rule reaches every document that takes the default sheet.
