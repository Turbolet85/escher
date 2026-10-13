proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (dioxus-native-dom)"
    change: "dioxus-native-dom: 67 unit tests in nine files (61 in eight until 2026-10-13-upstream-sync-polish-and-ship, whose upstream merge added six: four in dioxus_document.rs's test module — `removed_nodes_are_dropped`, `suspended_nodes_are_dropped_and_recreated`, `todo_like_stress`, `mounted_data_of_removed_element_reports_error` (packages/dioxus-native-dom/src/dioxus_document.rs:496-777) — and two in a new `mutation_writer::tests` — `falsy_boolean_attributes_are_removed`, `boolean_attributes_follow_their_value_after_mount` (packages/dioxus-native-dom/src/mutation_writer.rs:477-578); 60 until 2026-10-12-self-description …), the rest of the inventory as it reads"
    sidecar: "test-plan §1 dioxus-native-dom: 61 → 67 unit tests, eight → nine files — upstream's four dioxus_document drop tests and two mutation_writer falsy-attribute tests arrived with the 434e0661 merge (2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The report's Counts / qualifiers moved states dioxus-native-dom unit tests 61 → 67 at test-plan.md:24 (pattern test-tallies), and its Coverage of new surfaces names the tests: unit mutation_writer +2, dioxus_document +4, element_id 1 changed; New text lists mutation_writer.rs's `mod tests` as wholly added (477-578), so the files holding tests grow from eight to nine."
    basis: "escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/evidence/counts.md; New text rows packages/dioxus-native-dom/src/dioxus_document.rs:496-777, packages/dioxus-native-dom/src/mutation_writer.rs:477-578"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (dioxus-native and stylo_taffy, observed absent)"
    change: "the parenthetical list of dioxus-native-dom files matching the test search adds mutation_writer.rs (since 2026-10-13-upstream-sync-polish-and-ship, upstream's `mutation_writer::tests`); stylo_taffy still holds no test — the merge's `button_input_display` and `effective_display` arrived without one"
    sidecar: "test-plan §1 observed-absent row: mutation_writer.rs joins the dioxus-native-dom files holding tests (2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "Second statement of which dioxus-native-dom files hold tests (the eight-file claim retired above); New text lists `mod tests` at mutation_writer.rs:477-578 as added, and lists stylo_taffy's convert.rs:204-211 and wrapper.rs:63-72 with no test row."
    basis: "New text rows packages/dioxus-native-dom/src/mutation_writer.rs:477-578, packages/stylo_taffy/src/convert.rs:204-211, packages/stylo_taffy/src/wrapper.rs:63-72"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (dioxus-native-dom)"
    change: "the `element_id` unit tests build the same way from a plain `fn() -> Element` app and flip a `GlobalSignal` inside `vdom.in_runtime` to re-render; since 2026-10-13-upstream-sync-polish-and-ship a re-render drops a removed node and its descendants and clears their `element_id_mapping` / `node_id_mapping` entries (upstream #1185, `remove_and_drop_node`), so `a_removed_element_reads_no_id` asserts that the removed `p` no longer resolves (`get_node` is `None`), that `element_id` of it is `None` and that `element_ids()` lists no pair with it — the hand `DocumentMutator::remove_node_if_unparented` call and the detached-parent assertion are gone (packages/dioxus-native-dom/src/element_id.rs:396-423)"
    sidecar: "test-plan §3 dioxus-native-dom helpers: a re-render now drops a removed node (#1185); the element-id test no longer stales a detached node by remove_node_if_unparented (2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The report's Spec claims disproved states test-plan.md:99 names `DocumentMutator::remove_node_if_unparented` in the description of a test that no longer calls it, and that a re-render now drops the node (measured by `removed_nodes_are_dropped ... ok` and `a_removed_element_reads_no_id ... ok`); Symbols / APIs (Ours, changed) states the test's new assertions at element_id.rs:396-423."
    basis: "packages/dioxus-native-dom/src/element_id.rs:396-423 (report, Symbols / APIs); packages/dioxus-native-dom/src/mutation_writer.rs:110-124"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (tests/blitz-tests)"
    change: "after the four checks that arrived with the 2026-10-10 upstream merge, add five that arrived with the 2026-10-13 one (434e0661), each its own target: a restyled owner's anonymous-block text following it (`anonymous_block_restyle`, 10), a button and a button-like input centring their content after `default.css`'s button-input rule became `inline-block` (`button_alignment`, 16), removing `checked` unchecking a checkbox and a radio (`checked_attribute`, 2), a pointer leaving the document, a sub-document host or the window — and a finger lifted or cancelled — clearing hover with leave events (`pointer_leave`, 7, through a hand-written `ShellProvider`), and `DocumentConfig.ua_stylesheets` used as given, the default config taking the default sheet and a later sheet overriding an earlier (`ua_stylesheet_order`, 3); the package gaining the dev-dependency `cursor-icon` for them (tests/blitz-tests/tests/anonymous_block_restyle.rs:10-48; tests/blitz-tests/tests/button_alignment.rs:20-24; tests/blitz-tests/tests/checked_attribute.rs:53-61; tests/blitz-tests/tests/pointer_leave.rs:28-52; tests/blitz-tests/tests/ua_stylesheet_order.rs:20-43)"
    sidecar: "test-plan §1 blitz-tests coverage: five upstream checks of the 434e0661 merge — anonymous_block_restyle 10, button_alignment 16, checked_attribute 2, pointer_leave 7, ua_stylesheet_order 3 — and the cursor-icon dev-dependency (2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The report's Files and Crates / modules state five new integration targets under tests/blitz-tests (+38 tests in the five files, Counts), Dependencies states the new dev-dependency `cursor-icon`, and Coverage of new surfaces maps the new paths to them (UserAgentStylesheet::parse → ua_stylesheet_order 3; PointerLeave → pointer_leave 7; button-input layout → button_alignment 16); the per-file counts 10 and 2 are the test heads New text lists (38 − 3 − 7 − 16 = 12)."
    basis: "New text rows for the five new files under tests/blitz-tests/tests/"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (tests/blitz-tests, Dioxus integration)"
    change: "the clearing of a falsy Dioxus `disabled` and of each boolean attribute `dioxus_falsy_boolean_attrs` names (27, the bridge's own list until 2026-10-13-upstream-sync-polish-and-ship) on both attribute paths and in both layout modes; since that chunk the bridge clears by upstream's `is_bool_attr`, 28 names (`inert` added), our `BOOLEAN_ATTRIBUTES` retired — the check file unchanged and green, and whether any check clears a falsy `inert` not stated"
    sidecar: "test-plan §1 Dioxus integration: the bridge's falsy list is upstream's is_bool_attr, 27 → 28 names (inert); the integration check's own list not re-read (2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The report's Counts states the falsy-attribute list 27 → 28 (`inert` added), pattern falsy-attrs standing at test-plan.md:26 and :330; Symbols / APIs states `BOOLEAN_ATTRIBUTES` removed and `is_bool_attr` (28 names) taking its place; the Files list does not name tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs, and Coverage of new surfaces credits the a11y effect (a falsy `inert` cleared) without naming a test that reads it — the new 28th name has no stated check at integration tier."
    basis: "packages/dioxus-native-dom/src/mutation_writer.rs:439-475 (report, Symbols / APIs)"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "118 `.rs` files, 117 test targets and the unbuilt `all.rs` (118 and 117 since 2026-10-13-upstream-sync-polish-and-ship's upstream merge added `anonymous_block_restyle.rs`, `button_alignment.rs`, `checked_attribute.rs`, `pointer_leave.rs` and `ua_stylesheet_order.rs`, each its own auto-discovered target with no manifest line, upstream's `all.rs` gaining their five `mod` lines and staying unbuilt; 113 and 112 since 2026-10-12-headless-screenshot …), and at that chunk's run 0 `Running tests/all.rs` lines and no `all` executable among 128"
    sidecar: "test-plan §2 Directory pattern: blitz-tests 113 → 118 files, 112 → 117 targets, 128 workspace executables with no all (2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The report's Counts states `tests/blitz-tests/tests/*.rs` files 113 → 118, so 117 targets beside the unbuilt `all.rs`, and workspace test executables 123 → 128; Crates / modules states no `autotests = false`, `[[test]] all` still `test = false`, `BlitzTestsTargetsTest` ok, and upstream's `all.rs` gaining five `mod` lines; Outcome reads `executables 128`, `all-target 0` and `Running tests/all.rs` 0."
    basis: "escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/evidence/counts.md; New text rows tests/blitz-tests/tests/all.rs (10 · 14-15 · 47 · 69)"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (CI workflows and leg script)"
    change: "the CI-scripts leg runs 79 tests over six test files — test_ci_workflows.py's 25, test_wpt_diff_to_pr.py's 7, test_agent_run.py's 14, test_cold_agent.py's 27, test_wpt_area_changes.py's 5 and test_blitz_tests_targets.py's 1 — re-read at 2026-10-13-upstream-sync-polish-and-ship (`Ran 79 tests`, `OK`; +1, upstream #1183) and green at CI#38422391931 on a6c24d24; 78 as measured at CI#38013740580 on 9462a7e4"
    sidecar: "test-plan §4 CI-scripts leg: 78 → 79 tests, test_wpt_area_changes.py 4 → 5 (upstream #1183, 2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The report's Counts states the CI-scripts leg 78 → 79 tests at test-plan.md:8 and :145 (pattern test-tallies); Harness / gate surface states upstream's wpt_area_changes.py now sorts WPT areas by path segment and its test file gains one test; Outcome records `Ran 79 tests`, `OK`."
    basis: "New text rows .github/scripts/test_wpt_area_changes.py:61-68, .github/scripts/wpt_area_changes.py:74"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (CI Python scripts)"
    change: "79 tests in six test files (§4) — 78 until 2026-10-13-upstream-sync-polish-and-ship, whose upstream merge added one to test_wpt_area_changes.py"
    sidecar: "test-plan §1 CI Python scripts: 78 → 79 tests (2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The same CI-scripts count restated in §1; the report's Counts names test-plan.md:8 beside :145."
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (CI Python script)"
    change: "upstream's second test file, test_wpt_area_changes.py, holds 5 tests over `wpt_area_changes.py` — the four of `CompareTest` (areas use the URL path alone, variants stay distinct in area totals, changed areas are counted, no change yields none) and, since 2026-10-13-upstream-sync-polish-and-ship (upstream #1183), `test_nested_areas_sort_before_sibling_with_same_prefix`: areas sort by path segment (.github/scripts/test_wpt_area_changes.py:61-68)"
    sidecar: "test-plan §4 CI Python script: test_wpt_area_changes.py 4 → 5 tests, the path-segment sort (#1183; 2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "Third statement of test_wpt_area_changes.py's count (4 in the §4 leg tally retired above); the report's Harness / gate surface states the script sorts areas by path segment and its test file gains one test; New text lists the new test's head at 61-68."
    basis: "New text row .github/scripts/test_wpt_area_changes.py:61-68"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Local baseline (`cargo test --workspace`, CI's test leg)"
    change: "append: re-counted at 2026-10-13-upstream-sync-polish-and-ship (the upstream merge of 434e0661, 16 commits): 179 result lines, 893 passed · 0 failed · 12 ignored (+44 passed: five new result lines, every one upstream's — blitz-tests' `anonymous_block_restyle` 10, `button_alignment` 16, `checked_attribute` 2, `pointer_leave` 7 and `ua_stylesheet_order` 3 — and dioxus-native-dom +6 in its lib line, to 67; executables 123 → 128, the unbuilt `all` none of them), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/evidence/counts.md), and CI#38422391931's linux job reading the same 179"
    sidecar: "test-plan §9 workspace leg: 174 · 849 · 0 · 12 → 179 · 893 · 0 · 12 (+5 upstream targets, +6 dioxus-native-dom unit tests; 2026-10-13-upstream-sync-polish-and-ship)"
    rationale: "The report's Counts states the workspace leg 174 result lines · 849 passed · 0 failed · 12 ignored → 179 · 893 · 0 · 12, +5 targets and +44 tests (38 in the five new files, 6 in dioxus-native-dom), stated at test-plan.md:330 (pattern test-tallies); Cross-project states the CI run's linux `test result:` lines at 179."
    basis: "escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/evidence/counts.md"
note: "D-tests-framework and D-tests-obs-harness hold (runner unchanged — five new targets each their own, `all` unbuilt; no script, harness, status or log format changed); not proposed: §3 agent-run Proof could take this chunk's re-read (`run stand` 139 · 0 · 7 over 36, `run all` 487 · 0 · 12, first re-read since 396), and upstream's public `synthetic_form_event` has no test in this repo (report: tests ✗, unused by escher)."
