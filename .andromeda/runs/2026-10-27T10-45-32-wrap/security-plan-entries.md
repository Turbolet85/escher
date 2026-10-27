## 2026-10-27-upstream-sync-engine-debts — the script runtime's module split: citations follow the moved code
**Section:** §API Security (api-security.md: JS-facing fetch) · §Input Validation (input-validation.md: seven JS API arguments rows) · §Threat Model Summary (threat-model-summary.md: the script-execution entry point) · §Error Handling (body: `eval`/`eval_module`, module resolve or fetch failures) · §Logging & Monitoring (body: the uncaught-JS-error `tracing::error!`)
**Change:**
- 17 citations into blitz-vibey-script's `runtime.rs` and `dom/element.rs`, whose cited code left those files, re-pointed by hand to its new home: the GET/HEAD-only fetch to `runtime/bootstrap/fetch.js` over the native bridge in `runtime/fetch.rs`; the `Response` status RangeError and the `fetch()` method TypeError to `runtime/bootstrap/fetch.js`; the selector `SyntaxError` to `dom/element/selectors.rs`; the three scroll-option TypeErrors to `dom/element/scrolling.rs`; the `width`/`height` reflection to `dom/element/attributes.rs`; the timer delays and ids to `runtime/timers.rs`; `CSS.registerProperty` to `runtime/css.rs`; `<body onload>` to `runtime/events.rs` and `fetch()`'s URL resolution to `runtime/fetch.rs`; `eval` to its new lines in `runtime.rs` and `eval_module` to `runtime/modules.rs`; module resolve or fetch failures to `runtime/modules.rs`; the uncaught-error `tracing::error!` to `runtime/events.rs`, in `report_js_error`.
- No claim text changed by the re-point.
**Why:** upstream's `e6f4615ec`, taken in this sync, split the runtime into `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` with nine bootstrap scripts and the element bindings into `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs` — a move, `runtime.rs` keeping 151 lines, nothing removed; each claim still holds, at a path the old citation no longer named.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the per-sync lock record of the eleven-commit take
**Section:** §Dependency Security (dependency-security.md: a new sync record)
**Change:**
- A new sync record: the upstream sync read at `ebacd68bd` took eleven commits, `d63ed2c8c` · `6587e7e79` · `e6f4615ec` · `a7ea9631b` · `b5b163450` · `7fc9ee3af` · `2963b65a2` · `cee6b8731` · `53e9d81c6` · `6fee0db25` · `efdc5472a`, by no-commit cherry-picks of upstream's text (`e795a088c`), sixteen left out with their measured reasons, the merge base staying `23b9b26a9`; `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest byte-identical to the tree before it, so no package enters, leaves or moves; the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency; the audit leg reads `advisories ok` and `licenses ok`, every standing ignore encountered and none added.
**Why:** each upstream sync keeps one lock record. The take is the founder's own word on the take-shape fork (2026-10-27, relayed verbatim by the overseer, the operator's dialog answer recording it as his) — ratified, not PROVISIONAL.
**Kept:** the body's Pinning line stands unedited: this cherry-pick entered no package, so its claim still reads true.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — citations re-pointed by the take's line shifts
**Section:** §Error Handling (error-handling.md: shell and runtime setup) · §Input Validation (input-validation.md) · §Secret Management (secret-management.md: the publish signing material) · §Threat Model Summary (threat-model-summary.md) · the body's §Data Protection, §Dependency Security (Pinning, Supply chain integrity), §Secret Management, §Error Handling and §Logging & Monitoring
**Change:**
- 52 `file:line` citations re-pointed by the measured line map of the files the take moved — 22 in the body, 5 in error-handling.md, 18 in input-validation.md, 3 in secret-management.md, 4 in threat-model-summary.md — into blitz-dom `layout/construct.rs`, `mutator.rs` and `node/element.rs`, blitz-shell `lib.rs` and `window.rs`, blitz-vibey-script `document.rs`, `fetch.rs` and `runtime.rs`, `apps/browser/src/tab.rs`, `tests/blitz-tests/tests/all.rs` and `.github/workflows/publish-browser.yml`.
- Three ranges stretched over code grown inside them (`execute_scripts` whole; the script-skip records, now a forbidden load among them; `FetchError`'s `Display`, now with its forbidden arm), and three citations of publish's `dx bundle` line re-pointed by hand to where it now stands inside a retry loop.
- No claim text changed by the re-point.
**Why:** the take's eleven commits shifted the cited lines in those files; each claim still holds and cites where the code now is.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the script fetch's `file:` refusal by document, a narrowing
**Section:** §Input Validation (body: Script fetch; input-validation.md: a new Script fetch row) · §Threat Model Summary (threat-model-summary.md: the script-execution trust boundary) · §Error Handling (body: the `FetchError` line)
**Change:**
- Script fetch, body: was «invalid `data:` URLs or non-UTF-8 script bytes are `FetchError::InvalidData`»; now it adds that a `file:` URL loads only for a `file:` document, `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError` — a narrowing: the default fetcher takes only `file:` and `data:`, and same-origin and CORS checks are absent.
- input-validation.md, a new Script fetch row (a `file:` URL, by the loading document): `document_may_load(document_url, url)` lets a `file:` URL load only for a `file:` document, in every document, remote pages included; `fetch()`, a module `import` and `<script src>` each check it, the refusal a `FetchError` variant re-exported from the crate root; witnessed by `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`.
- The trust boundary gains: `document_may_load` refuses a `file:` URL for any other document, remote pages included, with a `FetchError`, checked by JS `fetch()`, module `import` and `<script src>` alike — a narrowing of the boundary, not a widening.
- `FetchError`'s `Display`: was «scheme, IO and invalid-data messages»; now «scheme, IO, invalid-data and forbidden-load messages».
**Why:** upstream's `cee6b8731`, taken on the founder's own word (2026-10-27, relayed verbatim by the overseer), which ratified the `file:` refusal among this take's engine behaviour changes for every document. It is recorded as a narrowing, not a boundary widening: a refusal added on an existing input surface, no scheme, carrier or crossing added.
**Kept:** the body's Threat Model script-execution line stands unedited: the default fetcher still takes only `file:` and `data:`, so its claim holds; the document check is stated in its detail row.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the `open` attribute read by presence, `:open` matching
**Section:** §Input Validation (input-validation.md: a new Markup attributes row)
**Change:**
- A new Markup attributes row, `open` (`details` · `dialog`): read by its presence on a `details` or a `dialog`, in every document, remote pages included — `ElementData::flush_open_state` sets `ElementState::OPEN` from it, called at element creation from `ElementData::new` and from `DocumentMutator::set_attribute` / `remove_attribute` on an `open` write; `NonTSPseudoClass::Open` reads that state, so `:open` matches such an element (was: never).
- In `set_attribute` the `open` arm stands after the range re-sanitize arm as a second independent `if`; no attribute text is carried out and no print or log site stands on the path.
- No test asserts `:open`; `details_element` and the a11y leg read green.
**Why:** upstream's take, an engine change for every document, ratified for this take by the founder's own word (2026-10-27, relayed verbatim by the overseer) under the playbook pattern for an engine behaviour change for every document; no master stated `:open` before, so the read is a new fact in its owner's row.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — CSS property accessors on `CSSStyleDeclaration`
**Section:** §Input Validation (input-validation.md: a new JS API arguments row)
**Change:**
- A new JS API arguments row, CSS property accessors on `CSSStyleDeclaration`: script reads and writes CSS properties through named accessors that `installStyleProperties` defines, one per name `css_property_names` takes from Stylo's property list; witnessed by `style_property_accessors`.
**Why:** upstream's `efdc5472a`, taken on the founder's own word (2026-10-27, relayed verbatim by the overseer); recorded as a realization on the registered script-to-DOM crossing, not a boundary widening, per the 2026-10-06 precedent; no master stated it before.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — dioxus-cli in the upstream-only publish workflow
**Section:** §Dependency Security (body: CI tooling)
**Change:**
- CI tooling: was «dioxus-cli is pinned to 0.7.8»; now the upstream-only publish workflow installs dioxus-cli `0.8.0-alpha.3`, or on a platform flagged `use_git_cli` from git at upstream's pinned rev, behind its `DioxusLabs/blitz` repository guard; wpt cli stays pinned to 0.0.14; the citation widened to the whole install step.
**Why:** the take's publish-workflow commits (`a7ea9631b` · `b5b163450`, upstream-only and repository-guarded) changed the install, so the old pin read false.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — `TMPDIR` read at a second test-only site
**Section:** §Secret Management (body: environment values read)
**Change:**
- `TMPDIR`, through `std::env::temp_dir()`: was read in escher-driver's unix-only client unit test alone; now at two test-only sites — that test, its listener's directory `eab-{pid}` as before, and blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files`, which makes the directory of files its `file:`-load tests read; test code only, not secret.
**Why:** upstream's `cee6b8731` added the helper with the `file:`-load tests; the environment values list, the one a secret source is measured against, names every read, and this one is test code only, not secret; no env read was added in source.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
