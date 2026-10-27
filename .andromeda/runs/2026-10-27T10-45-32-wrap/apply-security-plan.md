# Apply — security-plan

## Verdicts
- fanout-security-plan.md 1 · applied · .andromeda/security-plan.md:101#1
- fanout-security-plan.md 2 · applied · .andromeda/security-plan.md:302#1
- fanout-security-plan.md 3 · applied · .andromeda/security-plan.md:247#1
- fanout-security-plan.md 4 · applied · .andromeda/security-plan.md:328#1, .andromeda/security-plan.md:336#1, .andromeda/security-plan.md:386#1
- fanout-security-plan--cascade-g1.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:18#1
- fanout-security-plan.md 7 · applied · .andromeda/security-plan-detail/threat-model-summary.md:17#1, .andromeda/security-plan-detail/threat-model-summary.md:17#2
- fanout-security-plan--cascade-g2.md 1 · applied · .andromeda/security-plan-detail/input-validation.md:98+#1
- fanout-security-plan--cascade-g2.md 2 · applied · .andromeda/security-plan-detail/input-validation.md:180+#1
- fanout-security-plan--cascade-g2.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:317+#1
- fanout-security-plan.md 6 · applied · .andromeda/security-plan-detail/input-validation.md:296#1, .andromeda/security-plan-detail/input-validation.md:299#1, .andromeda/security-plan-detail/input-validation.md:302#1, .andromeda/security-plan-detail/input-validation.md:305#1, .andromeda/security-plan-detail/input-validation.md:308#1, .andromeda/security-plan-detail/input-validation.md:311#1, .andromeda/security-plan-detail/input-validation.md:314#1
- fanout-security-plan--cascade-g3.md 1 · applied · .andromeda/security-plan-detail/dependency-security.md:22+#1
- fanout-security-plan.md 5 · applied · .andromeda/security-plan-detail/api-security.md:46#1

## Edits
- line: .andromeda/security-plan.md:101
  old: «Script fetch — invalid `data:` URLs or non-UTF-8 script bytes are `FetchError::InvalidData`.»
  new: «Script fetch — invalid `data:` URLs or non-UTF-8 script bytes are `FetchError::InvalidData`; a `file:` URL loads only for a `file:` document, `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError` — a narrowing: the default fetcher takes only `file:` and `data:`, and same-origin and CORS checks are absent.»
- line: .andromeda/security-plan.md:247
  old: «dioxus-cli is pinned to 0.7.8; wpt cli to 0.0.14 (.github/workflows/ci.yml:384; .github/workflows/publish-browser.yml:126;»
  new: «the upstream-only publish workflow installs dioxus-cli `0.8.0-alpha.3`, or on a platform flagged `use_git_cli` from git at upstream's pinned rev, behind its `DioxusLabs/blitz` repository guard; wpt cli is pinned to 0.0.14 (.github/workflows/ci.yml:384; .github/workflows/publish-browser.yml:122-130;»
- line: .andromeda/security-plan.md:302
  old: «in escher-driver's unix-only client unit test `client::imp::tests`, which places its test-only listener's directory `eab-{pid}` there and removes it at the end: a `UnixListener` that accepts one connection and never answers, served by no binary and reached by no caller, so no crossing; test code only, not secret (packages/escher-driver/src/client.rs:263-308;»
  new: «at two test-only sites: escher-driver's unix-only client unit test `client::imp::tests`, which places its test-only listener's directory `eab-{pid}` there and removes it at the end — a `UnixListener` that accepts one connection and never answers, served by no binary and reached by no caller, so no crossing — and blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files`, which makes the directory of files its `file:`-load tests read; test code only, not secret (packages/escher-driver/src/client.rs:263-308; packages/blitz-vibey-script/tests/dom.rs:860-869;»
- line: .andromeda/security-plan.md:328
  old: «(packages/blitz-vibey-script/src/runtime.rs:1458-1514)»
  new: «(packages/blitz-vibey-script/src/runtime.rs:127-138; packages/blitz-vibey-script/src/runtime/modules.rs:94-131)»
- line: .andromeda/security-plan.md:336
  old: «with scheme, IO and invalid-data messages, and `std::error::Error` (packages/blitz-vibey-script/src/fetch.rs:17-30); module resolve or fetch failures become JS TypeErrors (packages/blitz-vibey-script/src/runtime.rs:1225-1240)»
  new: «with scheme, IO, invalid-data and forbidden-load messages, and `std::error::Error` (packages/blitz-vibey-script/src/fetch.rs:17-30); module resolve or fetch failures become JS TypeErrors (packages/blitz-vibey-script/src/runtime/modules.rs:65-86)»
- line: .andromeda/security-plan.md:386
  old: «(packages/blitz-vibey-script/src/runtime.rs:1104-1105;»
  new: «(packages/blitz-vibey-script/src/runtime/events.rs:20-21;»
- line: .andromeda/security-plan-detail/threat-model-summary.md:17
  old: «(packages/blitz-vibey-script/src/runtime.rs:1526-1567)»
  new: «(packages/blitz-vibey-script/src/runtime/events.rs:100-141)»
- line: .andromeda/security-plan-detail/threat-model-summary.md:17
  old: «(packages/blitz-vibey-script/src/runtime.rs:2357-2361)»
  new: «(packages/blitz-vibey-script/src/runtime/fetch.rs:32-36)»
- line: .andromeda/security-plan-detail/threat-model-summary.md:18
  old: «any other scheme returns `UnsupportedScheme` (packages/blitz-vibey-script/src/fetch.rs:48-70).»
  new: «any other scheme returns `UnsupportedScheme` (packages/blitz-vibey-script/src/fetch.rs:48-70). A `file:` URL loads only for a `file:` document: `document_may_load` refuses it for any other document, remote pages included, with a `FetchError`, checked by JS `fetch()`, module `import` and `<script src>` alike — a narrowing of the boundary, not a widening (packages/blitz-vibey-script/src/fetch.rs:40-46; packages/blitz-vibey-script/src/runtime/fetch.rs:40-44; packages/blitz-vibey-script/src/document.rs:163-169).»
- after: .andromeda/security-plan-detail/input-validation.md:98
  anchor: «  - **How (stack-specific):** Invalid `data:` URLs or non-UTF-8 script bytes produce `FetchError::InvalidData` (packages/blitz-vibey-script/src/fetch.rs:60-67)»
  add: «- **Boundary:** Script fetch»
  add: «  - **What to validate:** A `file:` URL, by the loading document»
  add: «  - **How (stack-specific):** `document_may_load(document_url, url)` lets a `file:` URL load only for a `file:` document, in every document, remote pages included (packages/blitz-vibey-script/src/fetch.rs:40-46); `fetch()` (packages/blitz-vibey-script/src/runtime/fetch.rs:40-44), a module `import` (packages/blitz-vibey-script/src/runtime/modules.rs:59-91) and `<script src>` (packages/blitz-vibey-script/src/document.rs:163-169) each check it, the refusal a `FetchError` variant (packages/blitz-vibey-script/src/fetch.rs:10-12) re-exported from the crate root (packages/blitz-vibey-script/src/lib.rs:43); the default fetcher takes only `file:` and `data:`, and same-origin and CORS checks are absent — a narrowing, not a boundary widening; witnessed by `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` (packages/blitz-vibey-script/tests/dom.rs:904-928; packages/blitz-vibey-script/tests/dom.rs:930-950) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:180
  anchor: « the range-input row) and the Enter and Space activation arms act only on what `ElementData::is_disabled` does not read disabled (packages/blitz-dom/src/events/keyboard.rs:202-233; the keyboard-activation row); the snapshot's `NodeState::enabled` follows the same presence reading as focusability, so a `disabled="false"` and a bare `disabled` both read `Some(false)` and neither is focusable (packages/dioxus-native-dom/src/snapshot.rs:165-168; packages/dioxus-native-dom/src/snapshot.rs:583-630; tests/blitz-tests/tests/stand_snapshot_state.rs:1-11) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md and, for focusability by presence, as measured at escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** `open` (`details` · `dialog`)»
  add: «  - **How (stack-specific):** Read by its presence on a `details` or a `dialog`, in every document, remote pages included: `ElementData::flush_open_state` sets `ElementState::OPEN` from it (packages/blitz-dom/src/node/element.rs:543-549), called at element creation from `ElementData::new` (packages/blitz-dom/src/node/element.rs:488-489) and from `DocumentMutator::set_attribute` / `remove_attribute` on an `open` write (packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523); `NonTSPseudoClass::Open` reads that state, so `:open` matches such an element (packages/blitz-dom/src/stylo.rs:510); in `set_attribute` the `open` arm stands after the range re-sanitize arm as a second independent `if`; no attribute text is carried out and no print or log site stands on the path; no test asserts `:open`, while `details_element` and the a11y leg read green — an engine change for every document, as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:317
  anchor: «  - **How (stack-specific):** `setProperty` ignores invalid declarations; `cssText` assignment re-serializes and drops invalid declarations (packages/blitz-vibey-script/src/dom/style.rs:120-123; packages/blitz-vibey-script/src/dom/style.rs:142-150)»
  add: «- **Boundary:** JS API arguments»
  add: «  - **What to validate:** CSS property accessors on `CSSStyleDeclaration`»
  add: «  - **How (stack-specific):** Script reads and writes CSS properties through named accessors that `installStyleProperties` defines on `CSSStyleDeclaration` (packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48), one per name `css_property_names` takes from Stylo's property list (packages/blitz-vibey-script/src/runtime/css.rs:8-18) — a realization on the registered script-to-DOM crossing, not a boundary widening; witnessed by `style_property_accessors` (packages/blitz-vibey-script/tests/dom.rs:455-487) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- line: .andromeda/security-plan-detail/input-validation.md:296
  old: «(packages/blitz-vibey-script/src/runtime.rs:120-123)»
  new: «(packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:38-39)»
- line: .andromeda/security-plan-detail/input-validation.md:299
  old: «(packages/blitz-vibey-script/src/runtime.rs:164-168)»
  new: «(packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:81-83)»
- line: .andromeda/security-plan-detail/input-validation.md:302
  old: «packages/blitz-vibey-script/src/dom/element.rs:1450-1515)»
  new: «packages/blitz-vibey-script/src/dom/element/selectors.rs:9-17)»
- line: .andromeda/security-plan-detail/input-validation.md:305
  old: «(packages/blitz-vibey-script/src/dom/element.rs:1181-1198; packages/blitz-vibey-script/src/dom/element.rs:1328-1350); a single non-object `scrollTo` argument throws TypeError (packages/blitz-vibey-script/src/dom/element.rs:1236-1241)»
  new: «(packages/blitz-vibey-script/src/dom/element/scrolling.rs:35-52; packages/blitz-vibey-script/src/dom/element/scrolling.rs:206-228); a single non-object `scrollTo` argument throws TypeError (packages/blitz-vibey-script/src/dom/element/scrolling.rs:90-95)»
- line: .andromeda/security-plan-detail/input-validation.md:308
  old: «(packages/blitz-vibey-script/src/dom/element.rs:683-699; packages/blitz-vibey-script/src/dom/element.rs:737-748)»
  new: «(packages/blitz-vibey-script/src/dom/element/attributes.rs:289-305; packages/blitz-vibey-script/src/dom/element/attributes.rs:343-354)»
- line: .andromeda/security-plan-detail/input-validation.md:311
  old: «(packages/blitz-vibey-script/src/runtime.rs:2069-2077; packages/blitz-vibey-script/src/runtime.rs:2396-2398)»
  new: «(packages/blitz-vibey-script/src/runtime/timers.rs:51-59; packages/blitz-vibey-script/src/runtime/timers.rs:135-137)»
- line: .andromeda/security-plan-detail/input-validation.md:314
  old: «(packages/blitz-vibey-script/src/runtime.rs:2263-2313)»
  new: «(packages/blitz-vibey-script/src/runtime/css.rs:81-131)»
- after: .andromeda/security-plan-detail/dependency-security.md:22
  anchor: «- The upstream sync read at `f9cbb2756` took ten commits, `28f848d3b` · `9eeebb302` · `e4ca5b9b5` · `46d7cf993` · `fb297a38c` · `50bf2f235` · `f8723f495` · `21edc92d9` · `f102663b8` · `f9cbb2756`, by no-commit cherry-picks of upstream's text (`34f6f051b`), the merge base staying `23b9b26a9`: no package enters or leaves `Cargo.lock` (987 packages), and `Cargo.toml` and `deny.toml` are byte-identical to the tree before it; cargo re-points twelve edges to versions already locked — eleven `windows-sys`, and `cssparser-macros 0.7.1`'s `syn` to 2.0.119; `mimalloc` 0.1.52 · `libmimalloc-sys` 0.1.49, already locked, become `apps/browser`'s default global allocator and a direct dependency of the WPT runner (`mimalloc = "0.1.48"`, one added lock edge), reviewed by hand — MIT, repository `purpleprotocol/mimalloc_rust`, `libmimalloc-sys` building vendored C through `cc` (`links = "mimalloc"`), neither a proc-macro, both reaching only upstream-only binaries; `tokio` moves to 1.54.1 in the lock alone (`cargo update -p tokio`), past RUSTSEC-2026-0386 (Audit tool); the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", no git dependency; git Stylo 0.23-pre (`06ecec886`, with `3954ecb85` and `1c5265a8b`), the six Taffy-main commits (`b8f4ba6fd` and `9babd796a`, a git Taffy, with `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) and Dioxus 0.8.0-alpha.3 (`286874cc1`) are not taken; the audit leg reads `advisories ok` and `licenses ok` with `deny.toml` unchanged — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/lock.md»
  add: «- The upstream sync read at `ebacd68bd` took eleven commits, `d63ed2c8c` · `6587e7e79` · `e6f4615ec` · `a7ea9631b` · `b5b163450` · `7fc9ee3af` · `2963b65a2` · `cee6b8731` · `53e9d81c6` · `6fee0db25` · `efdc5472a`, by no-commit cherry-picks of upstream's text (`e795a088c`), sixteen left out with their measured reasons, the merge base staying `23b9b26a9`: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical to the tree before it, so no package enters, leaves or moves; the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency; the audit leg reads `advisories ok` and `licenses ok`, every standing ignore encountered and none added — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md»
- line: .andromeda/security-plan-detail/api-security.md:46
  old: «(packages/blitz-vibey-script/src/runtime.rs:155-180)»
  new: «(packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:81-83; packages/blitz-vibey-script/src/runtime/fetch.rs:14-66)»

## Patterns
[[pattern]]
id = "sec-01"
fixed = "dioxus-cli is pinned to 0.7.8"
