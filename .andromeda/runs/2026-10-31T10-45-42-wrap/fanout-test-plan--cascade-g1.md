proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 21
    section: "## 1. Test Scope Summary"
    change: "The bullet's count reads 38 `#[test]` functions (36 until 2026-10-31-engine-debts), and its close reads: since 2026-10-13-range-input-interaction, 3 in form.rs's test module, `a_range_input_submits_its_current_value` and, since 2026-10-31-engine-debts, `a_multiple_select_submits_its_set_less_its_disabled_rows` and `a_single_select_submits_its_choice_through_a_group` (packages/blitz-dom/src/form.rs:558-617), which pin a named `select`'s entries, every chosen option that is not disabled, by its value, where until that chunk they had no test of their own. The clause saying they have no test of their own leaves."
    sidecar: "test-scope-summary: blitz-dom document/mutator/net/form 36 to 38 tests; form.rs 1 to 3; a named select's entries now have tests of their own (2026-10-31-engine-debts)."
    rationale: "Report Changes, Crates / modules: form.rs's test module already stood and gained two tests; Outcome (F6, F7) names form.rs +2; New text lists both tests. The added lines of document.rs and mutator.rs fall before their test modules' stated ranges, so the arithmetic moves by form.rs alone."
    basis: packages/blitz-dom/src/form.rs:558-617
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 17
    section: "## 1. Test Scope Summary"
    change: "The broad line's close reads: a named `select`'s entries are pinned by two form.rs tests, a `multiple` set less its disabled rows and a single choice through a group."
    sidecar: "test-plan §1: the blitz-dom broad line no longer says a named select's entries have no test of their own."
    rationale: "The same claim as test-scope-summary#2.3, in the body; report Outcome (F6, F7) names form.rs +2."
    basis: packages/blitz-dom/src/form.rs:558-617
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "`the single-choice select model` reads `the select model`, `the four node::select unit tests` reads `the node::select unit tests`, and the clause `its stated limits (...) asserted by no check` is replaced by: since 2026-10-31-engine-debts it models a `multiple` select and an `optgroup`'s options, asserted there and through the driver by `stand_act_select`; no popup is painted and Enter and Space open nothing on a single select, by `enter_and_space_open_nothing_on_a_single_select` (tests/blitz-tests/tests/select_interaction.rs:355-380); a closed single select's option stays refused `covered`, under a group too (tests/blitz-tests/tests/stand_act_select.rs:448-457); `size` is unread and a group's label is named in the tree and unpainted; an option with no author key reading in `unkeyed_actionable` stands as stated, that chunk naming no check of it."
    sidecar: "test-scope-summary: the select model's multiple and optgroup limits moved and are asserted; the closed option's covered refusal and the no-popup reading stated with their checks."
    rationale: "Report Symbols / APIs F6, F7, F8 and Counts `multiple-select`, `optgroup` name test-scope-summary.md:24 changed; Outcome (F6, F7, F8) met by `select_interaction` 18 and `stand_act_select` 10; New text lists the two tests. The report names no check of the `unkeyed_actionable` limit, so that one is not claimed asserted."
    basis: tests/blitz-tests/tests/select_interaction.rs:355-380
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "`the single-choice select model` reads `the select model, single and multiple, with optgroup`, and `while the select's are asserted by no check, nor is the process-wide Parallel traversal lock` reads: the select's `multiple`, `optgroup` and no-popup readings are asserted by `select_interaction` and `stand_act_select`, while the process-wide `Parallel` traversal lock is asserted by no check."
    sidecar: "test-plan §1: the blitz-dom covered-behaviours line no longer names a single-choice model or says the select's limits are asserted by no check."
    rationale: "The same claim as test-scope-summary#2.4, in the body; the report's Expected amendments carry test-plan §1's `the select's limits now asserted`, and its Outcome (F6, F7, F8) names the checks."
    basis: tests/blitz-tests/tests/select_interaction.rs:355-380
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (hit testing) — a non-invertible transform:** blitz-tests' `hit_non_invertible_transform` 2, since 2026-10-31-engine-debts, both layout modes: a box under a transform with determinant 0 or not finite takes no hit, `Node::hit_inner` answering `None` for the whole subtree, and an invertible twin is hit where it shows (tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71; packages/blitz-dom/src/node/node.rs:1472-1477)"
    sidecar: "test-scope-summary: new record, the non-invertible transform's hit rule and its check."
    rationale: "Report Symbols / APIs F2 and New targets: `hit_non_invertible_transform` 2, `non-invertible`'s control unfired, so no master states it; Outcome (F2) met. It sits beside the entity's hit-testing row."
    basis: tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (select) — `multiple` and groups:** blitz-tests' `select_interaction`, since 2026-10-31-engine-debts, both layout modes, pins a `multiple` select's rows having boxes of their own, a 17-row key table, typed text, a 10-row click table, a disabled select, removal of `multiple` settling to one option, a seeded set stepped from its last chosen row, and groups on both select kinds (tests/blitz-tests/tests/select_interaction.rs:447-476; tests/blitz-tests/tests/select_interaction.rs:478-556; tests/blitz-tests/tests/select_interaction.rs:558-588; tests/blitz-tests/tests/select_interaction.rs:601-689; tests/blitz-tests/tests/select_interaction.rs:725-743; tests/blitz-tests/tests/select_interaction.rs:781-894)"
    sidecar: "test-scope-summary: new record, what select_interaction pins of a multiple select and of groups."
    rationale: "Report Outcome (F6, F7) names `select_interaction` 18; New text lists each test and table, each running `for incremental in [false, true]`. It sits beside the entity's select row."
    basis: tests/blitz-tests/tests/select_interaction.rs:447-476
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (select) — the chosen-row pair:** the engine default `#c8c8c8` on `#000000`, in `default.css`, Chromium's inactive selected pair, is measured at 12.56:1 by `select_interaction`'s `a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair`, since 2026-10-31-engine-debts (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:691-712)"
    sidecar: "test-scope-summary: new record, the chosen-row pair and its measured ratio."
    rationale: "Report Schema / config states the pair and its 12.56:1, measured by `select_interaction`; Coverage of new surfaces repeats it. It sits beside the entity's select row."
    basis: tests/blitz-tests/tests/select_interaction.rs:691-712
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (select) — what no driver check reaches:** the driver cannot build a non-adjacent `multiple` set, `press` holding no Ctrl, and Ctrl+A is not modelled, so no driver check reaches either (packages/escher-driver/src/schema.rs:261-263) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md; owner: the `press` residual line of that chunk's P5 card"
    sidecar: "test-scope-summary: new record, the multiple select's sets no driver check reaches."
    rationale: "Report Symbols / APIs: Ctrl+A is not modelled, and `press`'s help says a set of rows that are not adjacent needs Ctrl, which this verb does not carry; `press-ctrl` lands the limit as a residual line, P5 card row 4. It sits beside the entity's stated select limits."
    basis: packages/escher-driver/src/schema.rs:261-263
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 32
    section: "## 1. Test Scope Summary"
    change: "After `classify_flow_children;` the bullet adds: since 2026-10-31-engine-debts its three tests run `for incremental in [false, true]`, upstream's both-modes exception ended (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:44-54; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:61-71; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:88-106);"
    sidecar: "test-scope-summary: pseudo_element_flow_classification's three tests run in both layout modes; upstream's both-modes exception retired."
    rationale: "Report Retired: `upstream's both-modes exception`, the three tests now running `for incremental in [false, true]`; Counts `flow-class` names test-scope-summary.md:32 changed; Outcome (CARRY 2) met."
    basis: tests/blitz-tests/tests/pseudo_element_flow_classification.rs:44-54
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 33
    section: "## 1. Test Scope Summary"
    change: "`node/select.rs holds 4 inline table tests of the select's rules` reads `node/select.rs holds inline table tests of the select's rules (4 until 2026-10-31-engine-debts)`, `keeping one chosen option` reads `keeping one chosen option on a single select, a multiple select's other options left alone`, and after `text stripped and collapsed` the list adds: since that chunk, a step moving from the row it is given, a span listing the enabled rows between two either way round, and the option list running through a group in tree order (packages/blitz-dom/src/node/select.rs:438-477; packages/blitz-dom/src/node/select.rs:560-589)."
    sidecar: "test-scope-summary: node/select.rs' stated tables now hold steps, spans and groups; the one-chosen-option rule narrowed to a single select."
    rationale: "Report New text lists `a_step_moves_from_the_row_it_is_given`, `a_span_lists_the_enabled_rows_between_two_either_way_round` and `the_list_of_options_runs_through_a_group_in_tree_order`; Symbols / APIs F6 says `set_option_selectedness` leaves a `multiple` select's other options alone. The report states no total for this file, so none is written."
    basis: packages/blitz-dom/src/node/select.rs:438-477
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 34
    section: "## 1. Test Scope Summary"
    change: "`it has no test of its own — details_element and the a11y leg read green beside it` reads: blitz-tests' `open_state` 4 is its own test since 2026-10-31-engine-debts; until then it had none, `details_element` and the a11y leg reading green beside it."
    sidecar: "test-scope-summary: the `:open` gap closed by open_state."
    rationale: "Report Counts `open-state` names test-scope-summary.md:34 changed; Outcome (CARRY 2) met, `open_state` 4."
    basis: tests/blitz-tests/tests/open_state.rs:54-143
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "The line's close `:open matching a details or dialog carrying open has no test of its own` reads: `:open` matching a `details` or `dialog` carrying `open` is read by blitz-tests' `open_state`."
    sidecar: "test-plan §1: the `:open` gap closed by open_state in the blitz-dom covered-behaviours line."
    rationale: "Report Counts `open-state` names test-plan.md:19 changed; Outcome (CARRY 2) met, `open_state` 4."
    basis: tests/blitz-tests/tests/open_state.rs:54-143
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 34
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (node) — `:open`, its check:** blitz-tests' `open_state` 4, since 2026-10-31-engine-debts, both layout modes, pins `:open` read from the attribute at construction, moved by a mutator write and removal, and flipped by a summary's click and Enter (tests/blitz-tests/tests/open_state.rs:54-143)"
    sidecar: "test-scope-summary: new record, what open_state pins."
    rationale: "Report New targets name `open_state` 4; New text lists its four tests, each running `for incremental in [false, true]`. It sits beside the entity it closes."
    basis: tests/blitz-tests/tests/open_state.rs:54-143
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 71
    section: "## 1. Test Scope Summary"
    change: "`tests/dom.rs holds 30 #[test] functions` reads `tests/dom.rs holds 31 #[test] functions (30 until 2026-10-31-engine-debts)`, and its `as measured at` names escher-0.3.0/chunks/2026-10-31-engine-debts/report.md."
    sidecar: "test-scope-summary: blitz-vibey-script tests/dom.rs 30 to 31."
    rationale: "Report Counts `unit-counts`: blitz-vibey-script `dom` 30 to 31, test-scope-summary.md:71 changed; Outcome gate `-p blitz-vibey-script --locked --test dom` (31)."
    basis: packages/blitz-vibey-script/tests/dom.rs:372-400
note: "F19, F20 (unit-test-strategy) and F21 (integration-test-strategy) are other categories' facts, though #2.3, #3.3 name the same tests; F1, F4, F5, F6, F11 to F16 have no picked carrier here, their rows at test-scope-summary.md:101-153 not in this group; blitz-dom's +6 reads form.rs +2 and three named select tests, the sixth unnamed by the report, so #3.3 states no total for node/select.rs."
