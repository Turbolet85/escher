
## 2026-10-13-upstream-sync-polish-and-ship — the merge's tests: five upstream targets, the bridge's six unit tests, the CI-scripts and workspace tallies
**Section:** §1 Test Scope Summary (CI Python scripts · dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests) · §2 Test Strategy → Directory pattern · §3 Test Harness Contract → Crate-local test helpers · §4 Unit Test Strategy (CI Python script · CI workflows and leg script) · §9 CI Integration → Local baseline
**Change:**
- dioxus-native-dom: 61 unit tests in eight files → 67 in nine — upstream's four drop tests in `dioxus_document.rs`'s module and two in a new `mutation_writer::tests`; mutation_writer.rs joins the files holding tests; stylo_taffy still holds none.
- tests/blitz-tests: five upstream checks, each its own target, with the dev-dependency `cursor-icon`: `anonymous_block_restyle` 10, `button_alignment` 16, `checked_attribute` 2, `pointer_leave` 7, `ua_stylesheet_order` 3.
- Directory pattern: 113 files and 112 targets → 118 and 117, `all.rs` gaining five `mod` lines and staying unbuilt; no `all` executable among 128.
- Dioxus integration: the falsy check still names its 27; the bridge clears by upstream's 28-name `is_bool_attr` (`inert` added). Whether a check reads a falsy `inert` is not measured.
- §3 helpers: was "stale a detached node's id by dropping it through `DocumentMutator::remove_node_if_unparented` — a re-render alone only detaches it"; now a re-render drops the node (#1185), and `a_removed_element_reads_no_id` asserts it no longer resolves, reads no id and is in no pair of `element_ids()`.
- CI scripts: 78 → 79 tests, test_wpt_area_changes.py 4 → 5 (nested areas sort by path segment, #1183); §1, §4 and the leg tally agree.
- §9 Local baseline: 174 · 849 · 0 · 12 → 179 result lines · 893 passed · 0 failed · 12 ignored; executables 123 → 128, `all` none of them.
**Why:** every figure re-read on the merged tree at this chunk's runs, the old figures kept as dated history. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's).
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/
