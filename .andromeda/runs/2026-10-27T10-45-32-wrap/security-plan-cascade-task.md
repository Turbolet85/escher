# Task — security-plan · 2026-10-27-upstream-sync-engine-debts

## The session
An upstream sync: upstream/main read at `ebacd68bd` (merge base `23b9b26a9` unmoved), eleven upstream commits taken by no-commit cherry-pick on the founder's word, sixteen left out, nothing sent upstream; three one-hunk conflict resolutions.
Engine changes for every document, remote pages included: `:open` matches a `details` or `dialog` carrying `open`; `::before`/`::after` vote in flow classification; a text input focused before its editor exists enables IME at construction; the windowed shell routes redraw requests through its event loop.
blitz-vibey-script: the default script fetcher narrows — a `file:` URL loads only for a `file:` document (`document_may_load`, a new `FetchError` variant), checked by `fetch()`, module `import` and `<script src>`; CSS properties become accessors on `CSSStyleDeclaration`; the runtime and element bindings split into `runtime/` and `dom/element/` modules (moved, none removed); the uncaught-JS-error `tracing::error!` moved whole into `report_js_error` in `runtime/events.rs`.
No dependency moved: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest byte-identical; audit green (advisories ok, licenses ok), no ignore added.
No new port, socket, listener, env read in source, print or log site; one new test-only `TMPDIR` read in blitz-vibey-script `tests/dom.rs`; escher-driver and escher-telemetry unchanged.
Measured: workspace tally 209 · 1125 · 0 · 12, blitz-vibey-script `tests/dom.rs` 27 → 30 tests, fork CI green 17/17 on `e795a088c`.

## Reached files
- .github/workflows/publish-browser.yml · merge surface
- apps/browser/src/main.rs · merge surface
- apps/browser/src/tab.rs · merge surface
- wpt/WPT_COMMIT · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-vibey-script/src/document.rs · merge surface
- packages/blitz-vibey-script/src/fetch.rs · merge surface
- packages/blitz-vibey-script/src/lib.rs · merge surface
- packages/blitz-vibey-script/src/runtime.rs · merge surface
- packages/blitz-vibey-script/src/dom/element.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/attributes.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/form_controls.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/geometry.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/markup.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/scrolling.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/selectors.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/css.rs · merge surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/fetch.rs · merge surface
- packages/blitz-vibey-script/src/runtime/modules.rs · merge surface
- packages/blitz-vibey-script/src/runtime/timers.rs · merge surface
- packages/blitz-vibey-script/src/runtime/window.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/{css,dom_exception,fetch,fonts,interfaces,node_constants,style,stylesheets,window}.js · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/all.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface

## Touches
- Threat Model Summary, the script-execution boundary (body line 13, its detail's trust-boundary line): the default fetcher still takes only `file:` and `data:`, and now a `file:` URL only for a `file:` document, checked by `fetch()`, module `import` and `<script src>`, recorded as a narrowing, not a widening; same-origin and CORS still absent, `isTrusted` unchanged.
- Input Validation, the Script fetch rule: beside `FetchError::InvalidData`, a non-`file:` document's `file:` load is now refused with a `FetchError` before anything is read.
- Input Validation, the Markup attributes rule (attributes read from parsed content, remote pages included, `disabled` and `inert` by presence): `open` on a `details` or `dialog` is now read by presence into `ElementState::OPEN`, so `:open` matches.
- Input Validation, CSS declarations and JS API arguments: script can now read and write CSS properties as named accessors on `CSSStyleDeclaration`, the names taken from Stylo's property list, on the registered script-to-DOM crossing.
- Error Handling, the `FetchError` typed-error line (citation `packages/blitz-vibey-script/src/fetch.rs:17-30`): a variant joins for the refusal (fetch.rs lines 10-12 and 23 added, re-exported from `lib.rs:43`), so the cited span and the list of messages move.
- Error Handling, the `eval`/`eval_module` line (citation `runtime.rs:1458-1514`) and the module resolve or fetch TypeError line (citation `runtime.rs:1225-1240`): the code left `runtime.rs` (151 lines kept) for the `runtime/` modules; the citations re-point, the claims unchanged unless a reading moved.
- Error Handling, script URL resolve or fetch failures recorded and the script skipped (citation `packages/blitz-vibey-script/src/document.rs:157-177`): `document.rs` gained lines at 16, 163-169, 210-211 and 222, the `<script src>` refusal among them.
- Logging & Monitoring, the JS console and uncaught-JS-error line (citations `runtime.rs:29-51`, `runtime.rs:1104-1105`, `document.rs:265-266`): the `js_console` bridge and the uncaught-error site now stand in `runtime/events.rs` (`report_js_error`, the `tracing::error!` at line 21, still `#[cfg(feature = "tracing")]`, engine target `blitz_vibey_script`); the document.rs line shifts.
- Error Handling, results discarded and the clipboard (citations `packages/blitz-shell/src/lib.rs:125-129`, `:133`, `:159`, `:175-177`, `:193-195`) and Data Protection's clipboard and file-dialog citations (`lib.rs:162-196`, `:198-225`): blitz-shell `lib.rs` gained lines at 90, 93-98 and 104-106, so each shifts.
- Error Handling, the unsupported winit variant citation `packages/blitz-shell/src/window.rs:850-854`: window.rs gained lines 193-194.
- Error Handling, the unresolvable iframe `src` citation `packages/blitz-dom/src/mutator.rs:1405-1409`: mutator.rs gained lines 368-371 and 520-523.
- Error Handling, panic paths in layout (citations `packages/blitz-dom/src/layout/construct.rs:876`, `:1040`): construct.rs gained lines 313-317, 860 and 867-872.
- Error Handling, error pages skip history recording and favicon probing (citation `apps/browser/src/tab.rs:140-149`): tab.rs gained lines 130-136, 177-182 and 294-343.
- Dependency Security, CI tooling (dioxus-cli pin cited at `.github/workflows/publish-browser.yml:126`, the bare `apt-get` line `:148-150`, `--locked` at `:157`) and Supply chain integrity (signed artifacts and signing material `:37-41`, `:106-108`, `:110-120`, `:179-181`, `:183-185`; lockfile verification `:157`): publish-browser.yml gained lines at 126 and 153-166; the upstream-only and repository-guard claims hold.
- Logging & Monitoring, CI publish logging (citations `publish-browser.yml:33`, `:37`, `:157`): the same file's added lines.
- Dependency Security, the per-sync lock record (body Pinning bullet on each upstream sync; detail lines 9 and 20-22): this sync entered no package and moved no lock edge — every manifest, the lock and `deny.toml` byte-identical.
- Dependency Security, Audit tool: re-read green on this sync, every standing ignore encountered, none added; the inline `log = "0.4"` in blitz-vibey-script's manifest unchanged.
- Secret Management, environment values read, the `TMPDIR` entry (body line 302, stated as escher-driver's one unit test): test-only `TMPDIR` reads move from 1 to 2 sites.
- Secret Management, the never-in-code claim: no env read added to non-test source.
- Logging & Monitoring and the logging-redaction bootstrap phase: no print or log site added; the moved uncaught-error event is printed by escher's sink with its message redacted, `host_log` and `mcp_log` green under both builds, the sink's allowlist and scrub sets unchanged.
- Input Validation and Threat Model, the remote-content boundary: `:open`, the pseudo flow vote, IME enabled at an editor's construction and the shell's redraw route are engine changes for every document, remote pages included, ratified by the founder for this take.
- Citations into blitz-vibey-script `src/dom/element.rs`: the element bindings moved into `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs`; the `scroll_into_view` caller now in `dom/element/scrolling.rs` 230-268, calling the same engine method.
- Test inventory: blitz-vibey-script `tests/dom.rs` 27 → 30 tests (lines added at 455-488 and 859-950), witnesses `remote_document_cannot_load_file_urls`, `file_document_can_load_sibling_file_urls` and `style_property_accessors`; blitz-tests gains the target `pseudo_element_flow_classification`.
- Any reading of the upstream sync state (upstream/main `ebacd68bd`, merge base `23b9b26a9`, nothing sent upstream under the founder's standing hold).

## New facts
- F1 · input-validation · Script fetch by document: `document_may_load(document_url, url)` lets a `file:` URL load only for a `file:` document; `fetch()`, module `import` and `<script src>` each check it before fetching, the refusal a `FetchError`; witnessed by `tests/dom.rs` `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`.
- F2 · error-handling · `FetchError` gains a variant for a `file:` load refused for a non-`file:` document, re-exported from blitz-vibey-script's `lib.rs`.
- F3 · input-validation · The `open` attribute on a `details` or `dialog`, in every document, remote pages included, is read by presence through `ElementData::flush_open_state` at element creation and on a mutator `set_attribute` or `remove_attribute` of `open`, setting `ElementState::OPEN` so `:open` matches; no attribute text carried out and no print or log site added.
- F4 · input-validation · Script access to CSS properties as named `CSSStyleDeclaration` accessors (`runtime/css.rs` `css_property_names`, `bootstrap/style.js` `installStyleProperties`), names taken from Stylo's property list, a realization on the registered script-to-DOM crossing, not a widening; witness `tests/dom.rs` `style_property_accessors`.
- F5 · secret-management · A second test-only `TMPDIR` read: `std::env::temp_dir()` in blitz-vibey-script `tests/dom.rs` `temp_dir_with_files` (lines 860-869, the call at 863), test code only, not secret, reached by no binary.
- F6 · dependency-security · This sync's lock record: upstream/main `ebacd68bd`, merge base `23b9b26a9`, eleven commits cherry-picked and sixteen left out with reasons; `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest byte-identical, no package entered, left or moved; audit green with no ignore added.

## Untouched
- The driver session socket, its state directory, wire, bounds and idle expiry: escher-driver is unchanged.
- The MCP stdio server, its framing, bounds and request-id echo: escher-driver is unchanged.
- The recording directory, kept checks, `Check::from_bytes`, replay and the two committed Counter checks: unchanged.
- The command schema, `validate`, refusal causes and session errors, the `command` and `tool_call` spans: unchanged.
- escher-telemetry's sink, its target allowlist, scrub sets and panic hook: unchanged.
- blitz-net's `file:` URL reads with no path restriction, `data:` decoding, the absent response-size cap and request timeout: blitz-net is not reached.
- Data Protection's sqlite history (`Connection::open`) and API Security's external browser open (`webbrowser::open`): token matches of `open`, not `:open`.
- The `CARGO_TARGET_TMPDIR` and `CARGO_BIN_EXE_escher-session` env entries: a different variable sharing the token.
- The deny.toml ignores (paste, zip, the zbus family) and memmap2's unsound advisory kept out: no ignore added or dropped.
- Pinning: no git dependency, taffy and parley registry versions, workspace inheritance: manifests byte-identical.
- The stable element id, the password and file-input masking, `MASKED_VALUE` and `MASKED_STAND_IN`: the snapshot readers are not reached.
- The Authentication & Authorization absences and the URL `password` accessor in `dom/hyperlink.rs`: neither reached.
- The cold-agent pipe, agent-run.sh, apt-install.sh and coverage_floor.py inputs: no script changed.
- The WPT runner's local file reads and the wpt-post-results repository guard: the runner and that workflow are not reached.
- Unsafe code in stylo_taffy and blitz-dom layout `mod.rs` and `table.rs`: not reached.
