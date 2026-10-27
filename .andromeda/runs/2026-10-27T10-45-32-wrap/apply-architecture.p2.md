# Apply — architecture · part 2 of 2

## Verdicts
- fanout-architecture--cascade-g4.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:78+#1
- fanout-architecture.md 3 · folded into fanout-architecture--cascade-g4.md 1
- fanout-architecture--cascade-g4.md 2 · folded into fanout-architecture--cascade-g4.md 1
- fanout-architecture--cascade-g5.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:99+#1
- fanout-architecture--cascade-g5.md 2 · refused · a hand removal · .andromeda/architecture-detail/standard-contracts.md:128–128
- fanout-architecture--cascade-g5.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:213+#1
- fanout-architecture--cascade-g5.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:213+#2
- fanout-architecture.md 11 · applied · .andromeda/architecture-detail/standard-contracts.md:210#1, .andromeda/architecture-detail/standard-contracts.md:210#2, .andromeda/architecture-detail/standard-contracts.md:210#3, .andromeda/architecture-detail/standard-contracts.md:210#4, .andromeda/architecture-detail/standard-contracts.md:210#5, .andromeda/architecture-detail/standard-contracts.md:210#6, .andromeda/architecture-detail/standard-contracts.md:211#1
- fanout-architecture--cascade-g6.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:78#1
- fanout-architecture--cascade-g6.md 3 · applied · .andromeda/architecture-detail/occupied-resources.md:39+#1
- fanout-architecture--cascade-g6.md 4 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:23+#1
- fanout-architecture.md 7 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:15#1, .andromeda/architecture-detail/cross-cutting-patterns.md:27#1, .andromeda/architecture-detail/cross-cutting-patterns.md:27#2, .andromeda/architecture-detail/cross-cutting-patterns.md:27#3
- fanout-architecture--cascade-g6.md 5 · applied · .andromeda/architecture-detail/project-intent.md:9#1, .andromeda/architecture-detail/project-intent.md:9#2, .andromeda/architecture-detail/project-intent.md:9#3, .andromeda/architecture-detail/project-intent.md:9#4
- fanout-architecture--cascade-g7.md 1 · applied · .andromeda/architecture-detail/existing-scopes.md:143+#1
- fanout-architecture--cascade-g7.md 2 · applied · .andromeda/architecture-detail/existing-scopes.md:143+#2
- fanout-architecture.md 10 · applied · .andromeda/architecture-detail/stack-and-technologies.md:73#1

## Edits
- after: .andromeda/architecture-detail/standard-contracts.md:78
  anchor: «- `ElementData::classes: ThinVec<Atom>`, public: the `class` attribute's atoms, filled by `ElementData::new` and `Clone` through the crate-private `parse_classes`, kept in sync by `DocumentMutator::set_attribute`'s `class` arm and by `remove_attribute`, and read by Stylo's `has_class` and `each_class`; `ElementData` is named outside blitz-dom by type only, never by a struct literal (packages/blitz-dom/src/node/element.rs:45-47; packages/blitz-dom/src/node/element.rs:56-61; packages/blitz-dom/src/mutator.rs:379-382; packages/blitz-dom/src/mutator.rs:503-506; packages/blitz-dom/src/stylo.rs:586-590).»
  add: «- `ElementData::flush_open_state(&mut self)`, public: it sets `ElementState::OPEN` on a `details` or `dialog` carrying an `open` attribute and clears it on any other; `ElementData::new` calls it, and so do `DocumentMutator::set_attribute` and `clear_attribute` on an `open` write — in `set_attribute` its arm a second independent `if` after the range re-sanitize arm (`min` · `max` · `step` → `resanitize_range_input`) — and Stylo's `NonTSPseudoClass::Open` reads that state, so `:open` matches such an element in every document (packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/node/element.rs:488-489; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510).»
- after: .andromeda/architecture-detail/standard-contracts.md:99
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-paint-and-shell -->»
  add: «»
  add: «- `BlitzShellProvider` holds its window, the shell's `BlitzShellProxy` and its document's id, and its `request_redraw` sends `BlitzShellEvent::RequestRedraw { doc_id }` through that proxy, so the windowed shell's redraw requests travel through its event loop (packages/blitz-shell/src/lib.rs:90; packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/lib.rs:104-106; packages/blitz-shell/src/window.rs:193-194).»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/standard-contracts.md:210
  old: «`UnsupportedScheme`, `Io`, `InvalidData` (packages/blitz-vibey-script/src/fetch.rs:7-38)»
  new: «`UnsupportedScheme`, `Io`, `InvalidData` and `Forbidden` (a `file:` load for a non-`file:` document) (packages/blitz-vibey-script/src/fetch.rs:7-38)»
- line: .andromeda/architecture-detail/standard-contracts.md:210
  old: «packages/blitz-vibey-script/src/runtime.rs:2328-2337»
  new: «packages/blitz-vibey-script/src/runtime/window.rs:210-223»
- line: .andromeda/architecture-detail/standard-contracts.md:210
  old: «other failures TypeError (packages/blitz-vibey-script/src/runtime.rs:2339-2388)»
  new: «other failures — a forbidden load among them — TypeError (packages/blitz-vibey-script/src/runtime/fetch.rs:9-66)»
- line: .andromeda/architecture-detail/standard-contracts.md:210
  old: «packages/blitz-vibey-script/src/runtime.rs:1862-1871»
  new: «packages/blitz-vibey-script/src/runtime/events.rs:362-371»
- line: .andromeda/architecture-detail/standard-contracts.md:210
  old: «(packages/blitz-vibey-script/src/runtime.rs:32-33;»
  new: «(packages/blitz-vibey-script/src/runtime/bootstrap.rs:12;»
- line: .andromeda/architecture-detail/standard-contracts.md:210
  old: «a JS bootstrap string defines history, fetch, DOMException, CSSOM classes and interface objects (packages/blitz-vibey-script/src/runtime.rs:35-1095)»
  new: «a JS bootstrap, `BOOTSTRAP_JS`, a `concat!` of nine `include_str!` scripts under `runtime/bootstrap/`, defines history, fetch, DOMException, CSSOM classes and interface objects (packages/blitz-vibey-script/src/runtime/bootstrap.rs:14-35)»
- line: .andromeda/architecture-detail/standard-contracts.md:211
  old: «packages/blitz-vibey-script/src/dom/element.rs:819-829»
  new: «packages/blitz-vibey-script/src/dom/element/markup.rs:8-22»
- after: .andromeda/architecture-detail/standard-contracts.md:213
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-javascript -->»
  add: «»
  add: «- `document_may_load(document_url: Option<&Url>, url: &Url) -> bool`, public and re-exported from the crate root beside `FetchError`: a `file:` URL may be loaded only by a document whose own URL is `file:`, any other URL by any document; `fetch()` (through `__blitz_fetch_sync`), a module `import` and a `<script src>` each ask it before the fetcher is called and read a refusal as `FetchError::Forbidden`, and `external_script_urls` leaves out a script it refuses — so the default fetcher still takes only `file:` and `data:`, a `file:` URL only for a `file:` document, and same-origin and CORS checks are still absent (packages/blitz-vibey-script/src/fetch.rs:40-46; packages/blitz-vibey-script/src/lib.rs:43; packages/blitz-vibey-script/src/runtime/fetch.rs:40-44; packages/blitz-vibey-script/src/runtime/modules.rs:77-81; packages/blitz-vibey-script/src/document.rs:163-169; packages/blitz-vibey-script/src/document.rs:222; packages/blitz-vibey-script/tests/dom.rs:904-928; packages/blitz-vibey-script/tests/dom.rs:930-950).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/standard-contracts.md:213
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-javascript -->»
  add: «»
  add: «- CSS properties are accessors on `CSSStyleDeclaration`: the bootstrap's `installStyleProperties` defines, once per style prototype, an accessor under each name `__blitz_css_property_names` returns and under its camel-case form — `css_property_names`, Stylo's non-custom property names that blitz-dom supports (`css_property_is_supported`) — on the registered script-to-DOM crossing, no carrier added (packages/blitz-vibey-script/src/runtime/css.rs:8-18; packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48; packages/blitz-vibey-script/tests/dom.rs:455-487).»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/occupied-resources.md:78
  old: «in escher-driver's unix-only client unit test, to place its test-only listener's directory, test code only (packages/escher-driver/src/client.rs:263-308; since 2026-10-14-quality-gates)»
  new: «at two test-only sites, test code only — escher-driver's unix-only client unit test, to place its test-only listener's directory (packages/escher-driver/src/client.rs:263-308; since 2026-10-14-quality-gates), and blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files`, to place the `file:` load tests' directory (packages/blitz-vibey-script/tests/dom.rs:860-869)»
- after: .andromeda/architecture-detail/occupied-resources.md:39
  anchor: «- The recorded live runs' committed copies also stand under `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-*` (`live-run.md`, `live-verdict.json`, `live-events.jsonl`, `live-check.json`, `live-transcript.jsonl`, the transcript with its host paths masked)»
  add: «- blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files` makes one directory outside `target/tmp/`, `blitz-vibey-script-{name}-{pid}` under the system temporary directory (`std::env::temp_dir()`), and writes the `file:` load tests' files into it — `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` load from it — test code only (packages/blitz-vibey-script/tests/dom.rs:860-869; packages/blitz-vibey-script/tests/dom.rs:904-928; packages/blitz-vibey-script/tests/dom.rs:930-950)»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:23
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · crosscut-service-wiring -->»
  add: «»
  add: «- A text input that holds focus before its editor exists has the shell provider's `set_ime_enabled(true)` called when `create_text_editor` constructs that editor (packages/blitz-dom/src/layout/construct.rs:860; packages/blitz-dom/src/layout/construct.rs:867-872); the route a windowed shell's redraw request takes, through its event loop, is §Standard Contracts → Paint and shell's»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:15
  old: «packages/blitz-vibey-script/src/runtime.rs:1104-1105»
  new: «packages/blitz-vibey-script/src/runtime/events.rs:20-21»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:27
  old: «packages/blitz-vibey-script/src/runtime.rs:1639;»
  new: «packages/blitz-vibey-script/src/runtime/timers.rs:35;»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:27
  old: «packages/blitz-vibey-script/src/runtime.rs:1690-1692»
  new: «packages/blitz-vibey-script/src/runtime/events.rs:190-192»
- line: .andromeda/architecture-detail/cross-cutting-patterns.md:27
  old: «packages/blitz-vibey-script/src/dom/element.rs:1023-1037»
  new: «packages/blitz-vibey-script/src/dom/element/geometry.rs:13-27»
- line: .andromeda/architecture-detail/project-intent.md:9
  old: «upstream `main` last read at `f9cbb2756`, thirteen of its commits taken»
  new: «upstream `main` last read at `ebacd68bd`, twenty-four of its commits taken»
- line: .andromeda/architecture-detail/project-intent.md:9
  old: «`f9cbb2756` (#1264, the flake's toolchain) — git Stylo 0.23-pre»
  new: «`f9cbb2756` (#1264, the flake's toolchain), then eleven together (`e795a088c`): `d63ed2c8c` (browser tab lenses kept across a webview props replacement) · `6587e7e79` (the windowed shell's redraw requests through its event loop) · `e6f4615ec` (blitz-vibey-script's runtime and element bindings in child modules) · `a7ea9631b` · `b5b163450` (publish-browser) · `7fc9ee3af` (IME enabled at a focused input's editor construction) · `2963b65a2` (`::before`/`::after` voting in flow classification) · `cee6b8731` (a `file:` script load only for a `file:` document) · `53e9d81c6` (`:open`) · `6fee0db25` (the WPT pin) · `efdc5472a` (CSS properties as `CSSStyleDeclaration` accessors); sixteen left out, each with its measured reason, among them git Stylo 0.23-pre»
- line: .andromeda/architecture-detail/project-intent.md:9
  old: «(`286874cc1`) wait for escher's next version»
  new: «(`286874cc1`), which wait for escher's next version»
- line: .andromeda/architecture-detail/project-intent.md:9
  old: «(as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)»
  new: «(as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md)»
- after: .andromeda/architecture-detail/existing-scopes.md:143
  anchor: «- blitz-tests' `stand_requirement_sweep` reads whether a 0.2.0 witness lies inside `stand-repeat` through a private target predicate of its own, `inside_stand_repeat`, which holds the two path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS` (tests/blitz-tests/tests/stand_requirement_sweep.rs:1097 — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md).»
  add: «- blitz-vibey-script's `runtime` module keeps its parts in the child modules `bootstrap`, `css`, `events`, `fetch`, `modules`, `timers` and `window` (`runtime/*.rs`), with nine JS bootstrap scripts under `runtime/bootstrap/` — `css`, `dom_exception`, `fetch`, `fonts`, `interfaces`, `node_constants`, `style`, `stylesheets`, `window` — concatenated into `BOOTSTRAP_JS`, geometry kept a separate `GEOMETRY_JS`; `dom::element`'s bindings are its children `attributes`, `form_controls`, `geometry`, `markup`, `scrolling` and `selectors` (`dom/element/*.rs`) (packages/blitz-vibey-script/src/runtime.rs:21-27; packages/blitz-vibey-script/src/runtime/bootstrap.rs:12-36; packages/blitz-vibey-script/src/dom/element.rs:9-14).»
- after: .andromeda/architecture-detail/existing-scopes.md:143
  anchor: «- blitz-tests' `stand_requirement_sweep` reads whether a 0.2.0 witness lies inside `stand-repeat` through a private target predicate of its own, `inside_stand_repeat`, which holds the two path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS` (tests/blitz-tests/tests/stand_requirement_sweep.rs:1097 — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md).»
  add: «- blitz-tests' `pseudo_element_flow_classification`, 3 tests, an auto-discovered target, its `mod` line also in the unbuilt `all.rs`: a `::before`/`::after` pseudo-element that generates its own box takes part in its container's flow classification as a child element does — an inline-block whose only child is a pseudo exporting its line baseline, top- and bottom-aligned, and a grid row growing by the baseline shim of pseudo-only items; the file does not run both layout modes (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78; tests/blitz-tests/tests/all.rs:54).»
- line: .andromeda/architecture-detail/stack-and-technologies.md:73
  old: «publish with Rust "1.96.1" and dioxus-cli "0.7.8" (»
  new: «publish with Rust "1.96.1" and dioxus-cli `0.8.0-alpha.3` (or, on a platform flagged `use_git_cli`, a git install at a pinned rev) — the workflow upstream-only and repository-guarded (»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = "a JS bootstrap string defines"

[[pattern]]
id = "arch2-02"
regex = '`UnsupportedScheme`, `Io`, `InvalidData` \('

[[pattern]]
id = "arch2-03"
regex = 'temp_dir\(\)` — on unix that variable, else `/tmp` — in escher'

[[pattern]]
id = "arch2-04"
fixed = "last read at `f9cbb2756`"

[[pattern]]
id = "arch2-05"
fixed = "thirteen of its commits taken"

[[pattern]]
id = "arch2-06"
fixed = 'dioxus-cli "0.7.8"'
