proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "blitz-tests holds 132 `.rs` files, 131 test targets and the unbuilt `all.rs` — since 2026-10-18-engine-debts added `accessibility_merged_states.rs`, `inert_subtree.rs`, `accessibility_active_descendant_unread.rs`, `transformed_client_rect.rs` and `stand_act_transformed.rs`, each its own auto-discovered target with no manifest line (127 and 126 until then)."
    sidecar: "test-plan §2 Directory pattern: blitz-tests 127 → 132 files, 126 → 131 targets (five new checks of 2026-10-18-engine-debts)."
    rationale: "Report Changes → Counts / qualifiers moved: blitz-tests files 127 → 132 and targets 126 → 131, five new files, `files-127` stale at test-plan.md:66; Files lists the five new checks."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:110-111"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "the `stand_act_*` count reads fourteen, not thirteen — `stand_act_transformed` the fourteenth since 2026-10-18-engine-debts — in the list of `mod session_common;` readers."
    sidecar: "test-plan §2 Directory pattern: the thirteen `stand_act_*` → fourteen (`stand_act_transformed`)."
    rationale: "Report Counts / qualifiers moved: `stand_act_*` files 13 → 14, `stand-act-13` stale at test-plan.md:66 (and :21, :100, :101, :143, :181, session-lifecycle.md:11)."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:118-121"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests (session checks))"
    change: "`mod session_common;` is declared by the fourteen driver-action checks `stand_act_*` (`stand_act_transformed` since 2026-10-18-engine-debts) — 22 readers (21 until 2026-10-18-engine-debts)."
    sidecar: "test-plan §3 session checks: `session_common` readers 21 → 22, the `stand_act_*` files thirteen → fourteen (`stand_act_transformed`)."
    rationale: "Report Counts / qualifiers moved: `session_common` readers 21 → 22 (`sc-readers` stale at test-plan.md:101) and `stand-act-13` stale at :101; Harness / gate surface: `stand_act_transformed` is a `session_common` reader."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:116-117"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "`tests/session_common/mod.rs` is read through `mod session_common;` by 22 checks (21 until 2026-10-18-engine-debts added `stand_act_transformed.rs`)."
    sidecar: "test-plan §2 Directory pattern: `session_common` readers 21 → 22."
    rationale: "Report Counts / qualifiers moved: `session_common` readers 21 → 22 (grep -l '^mod session_common;' → 22); the sweep named only :101 — :66 states the same count in its own words."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:116-117"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: "the executor's verbs and refusals are covered by the fourteen `stand_act_*` integration files (`stand_act_transformed` the fourteenth since 2026-10-18-engine-debts, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot)."
    sidecar: "test-plan §1 escher-driver: the thirteen `stand_act_*` files → fourteen."
    rationale: "Report Counts / qualifiers moved: `stand-act-13` stale at test-plan.md:21."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:118-121"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests (stand checks))"
    change: "`mod common;` is declared by six of the fourteen driver-action checks (fourteen since 2026-10-18-engine-debts added `stand_act_transformed`)."
    sidecar: "test-plan §3 stand checks: «six of the thirteen driver-action checks» → fourteen."
    rationale: "Report Counts / qualifiers moved: `stand-act-13` stale at test-plan.md:100."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:118-121"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (escher-driver)"
    change: "the fourteen `stand_act_*` files drive the executor in process through a `Session` (§5) — fourteen since 2026-10-18-engine-debts added `stand_act_transformed`."
    sidecar: "test-plan §4 escher-driver: the thirteen `stand_act_*` files → fourteen."
    rationale: "Report Counts / qualifiers moved: `stand-act-13` stale at test-plan.md:143."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:118-121"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session ↔ held instance"
    change: "the closing parenthesis reads «the fourteen `stand_act_*` files, §1», and the row gains `stand_act_transformed` (since 2026-10-18-engine-debts): a `click` on a translated button is accepted and lands, and its snapshot `bounds` read where it shows, both layout modes (tests/blitz-tests/tests/stand_act_transformed.rs:87-127)."
    sidecar: "test-plan §5 Session ↔ held instance: the `stand_act_*` files thirteen → fourteen, `stand_act_transformed` added."
    rationale: "Report Counts / qualifiers moved: `stand-act-13` stale at test-plan.md:181; Outcome: (arch · F8) met — `stand_act_transformed` 2."
    basis: "tests/blitz-tests/tests/stand_act_transformed.rs:87-127"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Session lifecycle (session-proof)"
    change: "the driver-action files read fourteen, `stand_act_transformed` 2 after `stand_act_screenshot` 6, the total 50 (48 until 2026-10-18-engine-debts added `stand_act_transformed`)."
    sidecar: "test-plan §3 → Session lifecycle (session-proof): fourteen driver-action files, `stand_act_transformed` 2, 48 → 50."
    rationale: "Report Counts / qualifiers moved: `stand-act-13` stale at registries/contracts/test-plan/session-lifecycle.md:11, the key file of test-plan; Outcome: `stand_act_transformed` 2 passed."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Legs)"
    change: "the stand-repeat leg runs one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 41 files (40 until 2026-10-18-engine-debts added `stand_act_transformed`)."
    sidecar: "test-plan §9 Legs: stand-repeat's `stand_*.rs` files 40 → 41."
    rationale: "Report Counts / qualifiers moved: `stand_*.rs` files 40 → 41, `stand-40` stale at test-plan.md:315; Harness / gate surface: the new `stand_*` file picked up by `run stand` and `stand-repeat` by name."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:112-115"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Flakiness budget"
    change: "the `stand-repeat` leg runs the 41 `stand_*.rs` files (40 until 2026-10-18-engine-debts) and seven_guis' seven process checks three times with no retry."
    sidecar: "test-plan §10 Flakiness budget: «the 40 `stand_*.rs` files» → 41."
    rationale: "Report Counts / qualifiers moved: `stand-40` stale at test-plan.md:344; Outcome: `stand-repeat: 3 of 3 rounds green`."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:112-115"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → dioxus-native-dom"
    change: "72 unit tests in nine files (69 until 2026-10-18-engine-debts, which added three snapshot tests — `an_aria_disabled_button_still_reads_enabled`, `a_switch_reads_its_aria_checked_as_checked_and_not_pressed` and `a_toggle_button_still_reads_its_aria_pressed_as_pressed` (packages/dioxus-native-dom/src/snapshot.rs:785-838)), the snapshot builder's tests twenty-one."
    sidecar: "test-plan §1 dioxus-native-dom: 69 → 72 unit tests, three snapshot tests of the `checked` / `pressed` sources and `aria-disabled`."
    rationale: "Report Counts / qualifiers moved: dioxus-native-dom lib tests 69 → 72, `dnd-69` stale at test-plan.md:24; New text lists the three tests at snapshot.rs:785-838; Outcome: dioxus-native-dom lib 72 passed."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:785-838"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → dioxus-native-dom"
    change: "the present-`disabled` snapshot test's gloss reads «focusability read by presence on a form control (`button` · `input` · `select` · `textarea`, `ElementData::is_disabled`) since 2026-10-18-engine-debts», not «in every document» on any element."
    sidecar: "test-plan §1 dioxus-native-dom: `disabled` focusability by presence narrowed to form controls (F7)."
    rationale: "Report Symbols / APIs F7: `ElementData::is_disabled` — `can_be_disabled()` and a `disabled` attribute by presence; `disabled-any` stale at test-plan.md:24 @c3192."
    basis: "packages/blitz-dom/src/node/element.rs:490-494"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "in `stand_keyboard`'s reading, a `disabled` attribute skips a form control by Tab and not a `<div disabled tabindex=0>`, which takes focus (`a_disabled_attribute_skips_a_form_control_and_not_a_div`, renamed at 2026-10-18-engine-debts from `a_disabled_attribute_of_any_value_is_skipped`) — the known limit retired."
    sidecar: "test-plan §1 tests/blitz-tests: `stand_keyboard`'s `disabled` reading restated for F7, the `<div disabled tabindex=0>` limit retired."
    rationale: "Report Symbols / APIs F7 (a `<div disabled tabindex=0>` takes focus) and Renamed tests; `disabled-any` stale at test-plan.md:26 @c39076."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:135-136"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "`accessibility_states` 5: the fourth test `aria_checked_reads_its_three_values_on_a_checkable_role_alone` (renamed at 2026-10-18-engine-debts from `aria_checked_sets_none_of_the_three`) — `aria-checked`'s `true` · `false` · `mixed` written as the toggle on the five checkable roles alone, and a native checkbox or radio reading neither `aria-checked` nor `aria-pressed` — and a fifth, `aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere` (tests/blitz-tests/tests/accessibility_states.rs:245-316)."
    sidecar: "test-plan §1 tests/blitz-tests: `accessibility_states` 4 → 5, `aria-checked` read on the checkable roles (F2)."
    rationale: "Report Counts / qualifiers moved: `accessibility_states` 4 → 5, fourth test rewritten, a fifth added (`acc-states` stale at test-plan.md:26 @c11104, `aria-checked` stale @c11412); Deviations 5: the native exclusion."
    basis: "tests/blitz-tests/tests/accessibility_states.rs:245-316"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "the «observed absent: a test naming the builder's merged `aria-expanded`, `aria-disabled` or `aria-level` reads» clause gives way to: since 2026-10-18-engine-debts `accessibility_merged_states` 3 — `aria-expanded` reads `true` and `false` and nothing else, `aria-disabled` sets disabled for `true` alone, `aria-level` reads its number less one and the heading defaults, each table's row count asserted, both layout modes (tests/blitz-tests/tests/accessibility_merged_states.rs:88-157)."
    sidecar: "test-plan §1 tests/blitz-tests: the merged ARIA reads observed present — `accessibility_merged_states` 3 (the CARRY)."
    rationale: "Report Counts / qualifiers moved: the CARRY, «observed absent» → present (`accessibility_merged_states`, 3 tests), `merged-aria` stale at test-plan.md:26 @c13325-13528; Outcome (tests · CARRY) met."
    basis: "tests/blitz-tests/tests/accessibility_merged_states.rs:88-157"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "the `dioxus_falsy_boolean_attrs` clause reads `stand_keyboard` pinning that Tab skips an inert subtree (`an_inert_subtree_is_skipped_by_tab`, renamed at 2026-10-18-engine-debts from `the_engine_honours_no_inert`), and the row gains `inert_subtree` 4 — Tab and Shift+Tab skip an inert subtree, a hit passes through it, the tree holds no node for it, and removing `inert` restores focus, hit and tree, the focus-fixup limit (a node made inert keeps focus) pinned there, both layout modes (tests/blitz-tests/tests/inert_subtree.rs:68-194)."
    sidecar: "test-plan §1 tests/blitz-tests: `inert` honoured (F1) — `inert_subtree` 4 added, `stand_keyboard`'s inert test renamed."
    rationale: "Report Symbols / APIs F1, Renamed tests, Outcome (a11y · F1) `inert_subtree` 4 passed; `inert` stale at test-plan.md:26 @c3218."
    basis: "tests/blitz-tests/tests/inert_subtree.rs:68-194"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "`stand_keyboard`'s reading «the engine honours no `inert`, Tab reaching buttons under `inert: true` and `inert: false`» reads: Tab skips an inert subtree (`an_inert_subtree_is_skipped_by_tab`, since 2026-10-18-engine-debts)."
    sidecar: "test-plan §1 tests/blitz-tests: `stand_keyboard`'s «engine honours no `inert`» retired (F1)."
    rationale: "Report Symbols / APIs F1 and Renamed tests; `inert` stale at test-plan.md:26 @c39390."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:135"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "`scrolled_box_absolute_position` 1 reads `Node::absolute_position` of a scrolled box where it stands — a box at (40, 50) reads (40, 50) at every scroll offset, only its ancestors' scroll subtracted (`a_scrolled_box_reads_its_absolute_position_where_it_stands`, renamed and restated at 2026-10-18-engine-debts) — no longer measured and pinned, not fixed."
    sidecar: "test-plan §1 tests/blitz-tests: `Node::absolute_position` fixed (F4), `scrolled_box_absolute_position` restated."
    rationale: "Report Symbols / APIs F4; Outcome (arch · F4) met — (40, 50) at every scroll offset; `abs-position` stale at test-plan.md:26 @c31963-32661."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:52-56"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (Inspection helpers)"
    change: "for a box that is itself scrolled, `layout_rect`, `layout_rect_of` and `center_of` — built on `Node::absolute_position` — read where the box stands since 2026-10-18-engine-debts (a box at (40, 50) reads (40, 50) at every scroll offset), as `get_client_bounding_rect` and the snapshot's `bounds` do; `scrolled_box_absolute_position` its check, the residuals ownership retired."
    sidecar: "test-plan §3 Inspection helpers: a scrolled box's `layout_rect` · `center_of` no longer less its own scroll (F4)."
    rationale: "Report Symbols / APIs F4: every caller moves with it, the harness's `layout_rect_of` included; `abs-position` stale at test-plan.md:92."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:52-56"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "in `hit_clipped_at_scrolling_box` 13, a `contain: paint` box stops the hit at its padding box (`a_contain_paint_box_stops_the_hit_at_its_padding_box`, renamed at 2026-10-18-engine-debts from `a_contain_paint_box_with_visible_overflow_is_still_hit_through`) — hit and paint agree, no longer «measured, not fixed»."
    sidecar: "test-plan §1 tests/blitz-tests: the hit at `contain: paint` fixed (F5), its test renamed."
    rationale: "Report Symbols / APIs F5, Renamed tests, Outcome (design · F5) `hit_clipped_at_scrolling_box` 13 passed; `contain-paint` stale at test-plan.md:26 @c31866."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:137-138"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "the clause «what stays measured and not fixed being the `contain: paint` hit and `Node::absolute_position` of a scrolled box» reads: both fixed in the engine at 2026-10-18-engine-debts, pinned by `hit_clipped_at_scrolling_box` and `scrolled_box_absolute_position`."
    sidecar: "test-plan §1 tests/blitz-tests: the two «measured, not fixed» readings retired (F4, F5)."
    rationale: "Report Symbols / APIs F4 and F5; `contain-paint` stale at test-plan.md:26 @c32442 and `abs-position` stale in the same span."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:152-157"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session ↔ held instance"
    change: "the «two readings stay as measured» sentence reads: `covered`'s fixed meaning states, since 2026-10-18-engine-debts, two readings — a box clipped on one axis stops a hit at its whole padding box, and a box with no width or no height is hit at its centre on the element that holds it — while the `contain: paint` hit is fixed in the engine (F5)."
    sidecar: "test-plan §5 Session ↔ held instance: `covered`'s limits — `contain: paint` dropped, the no-area reading added."
    rationale: "Report Symbols / APIs → The driver texts: `Cause::Covered`'s meaning drops the `contain: paint` sentence and gains the no-area one (F9 A); `contain-paint` and `covered-limits` stale at test-plan.md:181 @c3933/@c4046/@c4200."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:86-93"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "the `hit_clipped_at_scrolling_box` fixture condition reads `BaseDocument::get_client_bounding_rect` mapping the transformed box through its transform since 2026-10-18-engine-debts (the CSSOM View divergence retired), and the row gains `transformed_client_rect` 4 — a translated box read moved by its translation, a scaled box scaled about its centre, a child of a translated parent moving with it while its offset rect does not, and a target inside a translated scrolling box reading outside its `visible_region`, the measured `off-screen` limit (tests/blitz-tests/tests/transformed_client_rect.rs:79-220) — and `stand_act_transformed` 2, the fourteenth `stand_act_*` file, a stand check `run stand` picks up (tests/blitz-tests/tests/stand_act_transformed.rs:87-127)."
    sidecar: "test-plan §1 tests/blitz-tests: the client rect under transforms (F8) — `transformed_client_rect` 4 and `stand_act_transformed` 2 added, the layout-position divergence retired."
    rationale: "Report Symbols / APIs F8, Deviations 7 (the `visible_region` misreading pinned), Outcome (arch · F8) met; `client-rect-tf` stale at test-plan.md:26 @c31023-31133."
    basis: "tests/blitz-tests/tests/transformed_client_rect.rs:79-220"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-paint"
    change: "`password_caret_click` 2 (1 until 2026-10-18-engine-debts): a click places the caret by the bullets — `click_password_input` lays out one U+2022 per character, the two fillings one index inside 1–7 (`a_click_at_one_offset_places_the_caret_by_the_bullets`, renamed from `a_click_at_one_offset_places_the_caret_by_the_real_characters`) — and `a_double_click_selects_the_whole_value` (tests/blitz-tests/tests/password_caret_click.rs:106-121); the real-characters limit and its residuals ownership retired."
    sidecar: "test-plan §1 blitz-paint: the password click caret by the bullets (F6), `password_caret_click` 1 → 2."
    rationale: "Report Symbols / APIs F6, Renamed tests, Counts (`password_caret_click` 1 → 2, `pw-caret` stale at test-plan.md:18), Deviations 11, Outcome (security · F6) met."
    basis: "tests/blitz-tests/tests/password_caret_click.rs:106-121"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-paint"
    change: "observed absent: a test of the password click's shift-click branch (`extend_selection_to_byte`) or of a triple click — `password_caret_click` reads a single click and a double click only."
    sidecar: "test-plan §1 blitz-paint: the password click's shift-click and triple-click branches recorded untested."
    rationale: "Report Symbols / APIs F6: a single click `move_to_byte`, a shift-click `extend_selection_to_byte`, two or more clicks `select_all` (Deviations 1: a triple click selects all); Renamed tests and Coverage of new surfaces name only the single-click and double-click tests — a new path with no test at the integration tier."
    basis: "packages/blitz-dom/src/events/pointer.rs:586-652"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "observed absent: a test of F7's consequence that a `<summary disabled>` or `<a href disabled>` takes Enter / Space activation and a click, and that a disabled ancestor `div` no longer stops `handle_click`'s walk — `stand_keyboard`'s restated test reads a form control and a `div` only."
    sidecar: "test-plan §1 tests/blitz-tests: F7's activation consequences on `summary` and `a` recorded untested."
    rationale: "Report Symbols / APIs F7 (Consequences) and Deviations 9; the report's tests for F7 are `stand_keyboard` 14 with `a_disabled_attribute_skips_a_form_control_and_not_a_div` alone renamed — no named check reads the `summary` / `a` activation or the ancestor walk."
    basis: "escher-0.2.0/chunks/2026-10-18-engine-debts/report.md:69-75"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "the row gains, since 2026-10-18-engine-debts, `accessibility_active_descendant_unread` 1 — a focused listbox carrying `aria-activedescendant` reads focus and no active descendant, both layout modes: the unread attribute pinned (F3 B, no engine change) (tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:39-81)."
    sidecar: "test-plan §1 tests/blitz-tests: `accessibility_active_descendant_unread` 1 added (F3 B pin)."
    rationale: "Report Symbols / APIs F3 B: `aria-activedescendant` stays unread, pinned by `accessibility_active_descendant_unread`; Outcome gate 1 passed; Files lists it among the five new checks."
    basis: "tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:39-81"
note: "D-tests-framework and D-tests-obs-harness hold — the chunk's commands are cargo test / ci-leg.sh as §2–§4 state, and no harness, agent-run verb, status shape or log format moved (Harness / gate surface: none). The §9 per-OS tallies were not read by the report (CI#39096326308 17/17 only), so no tally proposal is made. The native-checkedness limit (the tree states no native checked state) has no test-plan row; its home is the residuals line the report proposes."
