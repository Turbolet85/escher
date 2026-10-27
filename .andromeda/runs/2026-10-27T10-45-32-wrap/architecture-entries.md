## 2026-10-27-upstream-sync-engine-debts — citations into the taken files re-pointed where the take's added lines moved them
**Section:** §Conventions (conventions.md: Formatting and lints, Known gaps, Feature gating, Unsafe and borrowing, Tests) · §Cross-cutting Patterns (cross-cutting-patterns.md: Logging and timing, Service wiring, Invalidation and state integrity) · §Design Philosophy (design-philosophy.md) · §Established Decisions (body; established-decisions.md: [Resource loading], [Unsupported features], [Scrolling and input], [Layout], [DOM semantics], [HTML parsing], [Shell and windowing], [JavaScript]) · §Existing Scopes (existing-scopes.md: blitz-dom layout, dioxus-native-dom) · §Occupied Resources (body; occupied-resources.md: Process-wide state and threads) · §Standard Contracts (standard-contracts.md: Document core, Mutation, query and CSSOM, Nodes and widgets, Paint and shell, JavaScript) · §Infrastructure Patterns (body) · §Project Intent (body) · §Inherited Defaults (body)
**Change:**
- Each citation whose cited code the take's added text moved inside its own file takes its new lines: blitz-dom `layout/construct.rs`, `mutator.rs` and `node/element.rs`; blitz-shell `lib.rs` and `window.rs`; dioxus-native-dom `dioxus_document.rs`; `apps/browser/src/tab.rs`; blitz-vibey-script `document.rs` and `fetch.rs`; `.github/workflows/publish-browser.yml`.
- Three cited blocks grew and are stretched: the browser's straight-line chrome-write block, was `tab.rs:126-148`, now `:126-154`; Paint and shell's shell-provider citation, was blitz-shell `lib.rs:87-219`, now `:87-226`; the `ScriptDocument` API, was `document.rs:55-283`, now `:55-292`. The body's `dx bundle` citations, was `publish-browser.yml:152-153`, now `:152-157`, the command inside a retry loop.
- No claim text changed by these re-points.
**Why:** the eleven taken commits added text inside the cited files and moved the cited lines (the report's `moved-files`, `publish` and `browser-tab` counts); the citation sweep re-pointed each row whose claim holds, a stretched block holding.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — blitz-vibey-script's runtime and element bindings in child modules
**Section:** §Existing Scopes (existing-scopes.md: a new scopes bullet) · §Standard Contracts (standard-contracts.md: JavaScript) · the citations of §Conventions (conventions.md: Known gaps, Visibility and naming, Unsafe and borrowing) · §Cross-cutting Patterns (body: Error handling; cross-cutting-patterns.md: Logging and timing, Event processing) · §Established Decisions (established-decisions.md: [Document defaults], [JavaScript]) · §Design Philosophy (design-philosophy.md) · §Occupied Resources (body: URL schemes and identities; occupied-resources.md: Process-wide state and threads) · §Infrastructure Patterns (body: JavaScript runtime) · §Project Intent (body: Embedding targets)
**Change:**
- Existing Scopes, new: blitz-vibey-script's `runtime` module keeps its parts in the child modules `bootstrap`, `css`, `events`, `fetch`, `modules`, `timers` and `window`, with nine JS bootstrap scripts under `runtime/bootstrap/` — `css`, `dom_exception`, `fetch`, `fonts`, `interfaces`, `node_constants`, `style`, `stylesheets`, `window` — concatenated into `BOOTSTRAP_JS`, geometry kept a separate `GEOMETRY_JS`; `dom::element`'s bindings are its children `attributes`, `form_controls`, `geometry`, `markup`, `scrolling` and `selectors`.
- JavaScript: was «a JS bootstrap string defines history, fetch, DOMException, CSSOM classes and interface objects»; now a JS bootstrap, `BOOTSTRAP_JS`, a `concat!` of nine `include_str!` scripts under `runtime/bootstrap/`, defines them.
- Every citation of code that left `runtime.rs` or `dom/element.rs` names the child module or bootstrap script that now holds it; a citation of code still in `runtime.rs` takes its new lines. Claim text otherwise unchanged.
**Why:** upstream's `e6f4615ec`, a move — `runtime.rs` keeps 151 lines, nothing removed, `ScriptRuntime`'s public methods unchanged in name — taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4). The citation sweep read each moved row's claim false at its old path; each was re-pointed by hand at this pass.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — `document_may_load`: a `file:` script load only for a `file:` document
**Section:** §Conventions (conventions.md: Visibility and naming) · §Standard Contracts (body: JavaScript; standard-contracts.md: JavaScript, a new `values · contract-javascript` block) · §Infrastructure Patterns (body: JavaScript runtime)
**Change:**
- Body, JavaScript: adds that the public `document_may_load` lets a `file:` URL load only for a `file:` document, its refusal a `FetchError` variant, checked by `fetch()`, a module `import` and `<script src>`.
- JavaScript contract: `FetchError`, was `UnsupportedScheme`, `Io`, `InvalidData`; now also `Forbidden` (a `file:` load for a non-`file:` document); `__blitz_fetch_sync`'s other failures, a forbidden load among them, are a TypeError.
- New bullet: `document_may_load(document_url: Option<&Url>, url: &Url) -> bool`, public and re-exported from the crate root beside `FetchError`: a `file:` URL may be loaded only by a document whose own URL is `file:`, any other URL by any document; `fetch()` (through `__blitz_fetch_sync`), a module `import` and a `<script src>` each ask it before the fetcher is called and read a refusal as `FetchError::Forbidden`, and `external_script_urls` leaves out a script it refuses — the default fetcher still taking only `file:` and `data:`, same-origin and CORS checks still absent.
- Visibility and naming: the crate root's re-exports, was `ScriptDocument`, `DefaultScriptFetcher`, `FetchError`, `ScriptFetcher`; now those and `document_may_load`.
- Infrastructure Patterns: the ES module loader's fetch is gated by `document_may_load`.
**Why:** upstream's `cee6b8731`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4), which ratifies it for this take as an engine behaviour change for every document; recorded as a narrowing, not a boundary widening (the report's Decisions).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — `::before` / `::after` in flow classification, and its one-mode test
**Section:** §Conventions (conventions.md: Tests) · §Established Decisions (established-decisions.md: [Layout]) · §Existing Scopes (existing-scopes.md: blitz-tests)
**Change:**
- [Layout], new: every box-generating `::before` / `::after` votes in flow classification (`classify_flow_children`): an inline-block whose only child is a pseudo exports its line baseline and bottom-aligns, and a grid row grows by the baseline shim of pseudo-only items — an upstream engine change for every document, its test file not asserting both layout modes.
- Existing Scopes, new: blitz-tests' `pseudo_element_flow_classification`, 3 tests, an auto-discovered target, its `mod` line also in the unbuilt `all.rs`; the file does not run both layout modes.
- Tests: the both-layout-modes convention, was stated with no exception; now it names upstream's `pseudo_element_flow_classification`, which does not assert both modes, `incremental_oracle` green beside it.
**Why:** upstream's `2963b65a2`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); its test file asserts one layout mode as upstream wrote it (the report's Coverage).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — IME enabled at a focused input's editor construction
**Section:** §Established Decisions (established-decisions.md: [Layout]) · §Cross-cutting Patterns (cross-cutting-patterns.md: a new `values · crosscut-service-wiring` block)
**Change:**
- [Layout], new: a text input already focused when its editor is constructed enables IME through `shell_provider.set_ime_enabled(true)` in `create_text_editor`, held by dioxus-native-dom's `input_focused_on_mount_enables_ime` — an upstream engine change for every document.
- Service wiring, new block: a text input that holds focus before its editor exists has the shell provider's `set_ime_enabled(true)` called when `create_text_editor` constructs that editor.
**Why:** upstream's `7fc9ee3af`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); dioxus-native-dom's lib gains the one unit test (81 → 82, the report's Counts).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the windowed shell's redraw requests through its event loop
**Section:** §Cross-cutting Patterns (cross-cutting-patterns.md: the new `values · crosscut-service-wiring` block) · §Established Decisions (established-decisions.md: a new `values · dec-shell-and-windowing` block) · §Standard Contracts (standard-contracts.md: a new `values · contract-paint-and-shell` block)
**Change:**
- Paint and shell, new: `BlitzShellProvider` holds its window, the shell's `BlitzShellProxy` and its document's id, and its `request_redraw` sends `BlitzShellEvent::RequestRedraw { doc_id }` through that proxy, so the windowed shell's redraw requests travel through its event loop.
- [Shell and windowing], new: that route, held by the windowed `seven_guis_native` smoke boot and `focus_redraw` — an upstream engine change for every document.
- Service wiring: the new block points the route a windowed shell's redraw request takes to §Standard Contracts → Paint and shell.
**Why:** upstream's `6587e7e79`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); witnessed by the windowed smoke boot and `focus_redraw` (the report's Coverage).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the merge surfaces read at `ebacd68bd`, their owner Epoch 3's sync
**Section:** §Established Decisions (established-decisions.md: [Scrolling and input] — range input, keyboard activation, the engine-debts fixes, the clock, the select model)
**Change:**
- Range input: was, per «2026-10-24-upstream-sync-polish-and-ship — `multi_doc`, the merge surfaces and their owners, the `inline` paths and the coverage exclusion's clause», upstream's hunks taken at `f9cbb2756` reaching `node/element.rs`, `node/mod.rs`, `layout/construct.rs` and `mutator.rs`, the one conflict `construct.rs`'s import list; now at `ebacd68bd` the taken commits reach `node/element.rs`, `layout/construct.rs` and `mutator.rs`, the one conflict `mutator.rs`'s `set_attribute`, resolved by placing upstream's `open` arm after the range re-sanitize arm (`min` · `max` · `step` → `resanitize_range_input`) as a second independent `if`.
- Keyboard activation: at `ebacd68bd` the taken commits reach `node/element.rs` (`ElementData::flush_open_state` and its call from `ElementData::new`) with no conflict and leave `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched.
- The engine-debts fixes' residual line: was «(target: next), re-read at the next upstream sync»; now re-read row by row with its owner quoted at the 2026-10-27 upstream sync, its owner Epoch 3's «Upstream sync ahead of the UI graph».
- The clock and the select model: was «the same `.andromeda/residuals.md` line (target: next)»; now the same line, whose owner is that Epoch 3 entry; the select model's blitz-vibey-script surface, was `runtime.rs`, now `runtime/events.rs`.
**Why:** the take's resolutions as measured (the report's Symbols / APIs). The owner move is the operator's own word on the phase's P5 card (the operator, 2026-10-27, inputs#I5), which re-reads residual line 36 at this sync.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — six upstream behaviour changes for every document, ratified for this take
**Section:** §Established Decisions (established-decisions.md: [Scrolling and input], after the direct engine changes)
**Change:**
- New: six behaviour changes for every document are upstream's and are not counted among escher's direct engine changes: `:open` ([DOM semantics]); the `::before` / `::after` flow vote and IME at a focused input's editor construction ([Layout]); the windowed shell's redraw route ([Shell and windowing]); the `file:` script-load refusal (§Standard Contracts → JavaScript); and the CSS property accessors on `CSSStyleDeclaration`. Each stands ratified for its take under the playbook pattern for an engine behaviour change for every document; the `file:` refusal is a narrowing and the accessors a realization on the registered script-to-DOM crossing.
**Why:** the founder's own word «Одиннадцать коммитов (Рекомендую)» (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4; the operator's P4 dialog answer, inputs#I3) ratifies the pattern for this take. The narrowing and the realization are the plan's reading, per the 2026-10-06 precedent (the report's Decisions). Not PROVISIONAL: his own word.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — `:open` and `ElementData::flush_open_state`
**Section:** §Established Decisions (established-decisions.md: [DOM semantics]) · §Standard Contracts (standard-contracts.md: Nodes and widgets)
**Change:**
- Nodes and widgets, new: `ElementData::flush_open_state(&mut self)`, public: it sets `ElementState::OPEN` on a `details` or `dialog` carrying an `open` attribute and clears it on any other; `ElementData::new` calls it, and so does the mutator on an `open` write — in `set_attribute` its arm a second independent `if` after the range re-sanitize arm — and Stylo's `NonTSPseudoClass::Open` reads that state, so `:open` matches such an element in every document.
- [DOM semantics], new: `:open` matches a `details` or `dialog` that carries `open` — an upstream engine change for every document with no test of its own, `details_element` and the a11y leg green and `summary` activation unchanged.
**Why:** upstream's `53e9d81c6`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); `NonTSPseudoClass::Open` read `false` before it, and no master stated `:open` (the report's Counts). A new public symbol, registered; the commit adds no test (the report's Coverage).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the browser's tab lenses across a webview props replacement
**Section:** §Established Decisions (established-decisions.md: a new `values · dec-browser-app` block)
**Change:**
- New: a tab's lenses survive a replacement of its webview props, held by `tab_lens_survives_webview_props_replacement` — upstream's, in the upstream-only app.
**Why:** upstream's `d63ed2c8c`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); the browser is upstream-only, so no escher surface moves.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — `TMPDIR` at a second test-only site, and its test directory
**Section:** §Occupied Resources (body: Environment variables; occupied-resources.md: Environment variables, a new Filesystem bullet)
**Change:**
- Body: `TMPDIR`, was read «in one unit test only»; now at two test-only sites (escher-driver's client unit test and blitz-vibey-script's `tests/dom.rs`).
- Environment variables: `std::env::temp_dir()`, was read in escher-driver's unix-only client unit test alone; now at two test-only sites, test code only — that test, to place its test-only listener's directory, and blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files`, to place the `file:` load tests' directory.
- Filesystem, new: `temp_dir_with_files` makes one directory outside `target/tmp/`, `blitz-vibey-script-{name}-{pid}` under the system temporary directory, and writes the `file:` load tests' files into it — `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` load from it — test code only.
**Why:** upstream's `cee6b8731` brought the helper and its two tests, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4): a read and a write only a test makes, not secret, registered as escher-driver's `eab-{pid}` directory is (the report's Symbols / APIs and Counts).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the upstream take read at `ebacd68bd`: twenty-four taken, sixteen left out
**Section:** §Project Intent (body: Upstream sync; project-intent.md: Upstream sync)
**Change:**
- Body: was, per «2026-10-24-upstream-sync-polish-and-ship — the upstream take read at `f9cbb2756`: stack, conventions and project intent», upstream `main` last read at `f9cbb2756`, thirteen of its commits taken; now read at `ebacd68bd`, twenty-four of the forty commits past the merge base taken by no-commit cherry-picks, the merge base `23b9b26a9` unmoved, sixteen left out.
- project-intent: the take adds eleven together (`e795a088c`), each named by its subject: `d63ed2c8c` · `6587e7e79` · `e6f4615ec` · `a7ea9631b` · `b5b163450` · `7fc9ee3af` · `2963b65a2` · `cee6b8731` · `53e9d81c6` · `6fee0db25` · `efdc5472a`; was «thirteen of its commits taken», now twenty-four; sixteen left out, each with its measured reason, among them git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3, which wait for escher's next version; Stylo "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3" unchanged, no git dependency; the measurement now this chunk's `evidence/lock.md`.
**Why:** the take is the founder's own word «Одиннадцать коммитов (Рекомендую)» (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); every manifest, the lock and `deny.toml` byte-identical to the base (the report's Dependencies).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the publish workflow's dioxus-cli
**Section:** §Stack and Technologies (stack-and-technologies.md: the publish workflow)
**Change:**
- Publish: was dioxus-cli "0.7.8"; now `0.8.0-alpha.3` (or, on a platform flagged `use_git_cli`, a git install at a pinned rev), the workflow upstream-only and repository-guarded; its bundle-step citation, was `publish-browser.yml:152-153`, now `:152-166`.
**Why:** upstream's `a7ea9631b` · `b5b163450`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); the citation sweep read the stated pin false.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the stand's typography in the Headless stand contract, F2 ratified
**Section:** §Standard Contracts (standard-contracts.md: Headless stand, the `values · contract-headless-stand` block)
**Change:**
- The OPEN gap marker owned by F2 — the stand's typography stated here rather than on design-system's product surface — drops.
- Standing, now ratified: the stand's typography is DejaVu Sans, its one bundled font, stated in this contract and not on design-system's product surface — `font_ctx()` registers it for every generic family with system fonts off, on native; every stand boot holds it through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included; seven_guis' native `woff` feature decodes it.
**Why:** F2 is ratified by the founder's own word «Подтвердить (Рекомендую)» (the founder, 2026-10-27, by phone, relayed by the operator, inputs#I7 on inputs#I6), superseding the delegate's PROVISIONAL answer by rule; the operator confirmed the discharge at this wrap's P2 halt (the operator, 2026-10-27, inputs#I8).
**Kept:** the PROVISIONAL entry «2026-10-27-readme-and-records — the stand's typography in the Headless stand contract (PROVISIONAL)» keeps its words and is not superseded: its typography claim, DejaVu Sans stated in the Headless stand contract, still stands in the body.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — CSS properties as `CSSStyleDeclaration` accessors
**Section:** §Standard Contracts (standard-contracts.md: a new `values · contract-javascript` block)
**Change:**
- New: CSS properties are accessors on `CSSStyleDeclaration`: the bootstrap's `installStyleProperties` defines, once per style prototype, an accessor under each name `__blitz_css_property_names` returns and under its camel-case form — `css_property_names`, Stylo's non-custom property names that blitz-dom supports (`css_property_is_supported`) — on the registered script-to-DOM crossing, no carrier added.
**Why:** upstream's `efdc5472a`, taken on the founder's own word (the founder, 2026-10-27, relayed verbatim by the overseer, inputs#I4); a realization on the registered crossing, not a boundary widening (the report's Decisions); no master stated it before.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
