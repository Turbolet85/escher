proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "After «a closed select's options under it, each with its select's client rect as its `bounds`» add: «; a select whose list is open is no closed select, `closed_select_of` answering `None` for it, and the snapshot's builder reads `select_list_bounds(node)` before the client rect, so an open row's `bounds` is its painted row's in the snapshot and the diff, `differs` naming `bounds`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»; the closed case stands as written."
    sidecar: "screen-reader-support:11 — an open list's row reads its painted row's bounds; a closed select's options still read the select's rect."
    rationale: "The report's Bridge bullet has `closed_select_of` answer `None` for an open list and the builder read `select_list_bounds(node)` first (`snapshot.rs:152-155`); its `closed-option` row lists screen-reader-support:11 as changed, and Schema / config names a row's own `bounds` in the snapshot and the diff."
    basis: packages/dioxus-native-dom/src/snapshot.rs:152-155
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "New row following line 11: «- A single select's tree node takes `expanded` from a closed bool of engine state, no author attribute, its focus staying on the select while the list is open; `the_tree_reads_the_list_open_on_the_select_its_focus_staying_there` and `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match` pin that reading (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "screen-reader-support — new row, the tree's reading of an open list and its two pins (F15)."
    rationale: "Report's Tree bullet and Coverage row `tree expanded on a single select`: a closed bool from engine state, no author attribute, tests in `select_popup`; the two test names are listed under New text, by line."
    basis: tests/blitz-tests/tests/select_popup.rs:565-614
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "New row following line 11: «- An open list's highlighted row is in no snapshot and no tree, a stated limit, so an agent or an assistive technology reading either cannot see which row is current (docs/capabilities.md:208-210) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "screen-reader-support — new row, the highlight unread by the snapshot and the tree (F16)."
    rationale: "Report's Coverage row `tree expanded on a single select`: the highlight unread by the tree, stated; inputs#I7 lists the highlight absent from the snapshot and the tree as a limit owed a residual line; New text, by line lists docs/capabilities.md:208-210 «The highlight is unread»."
    basis: docs/capabilities.md:208-210
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "New row following line 11: «- What an assistive technology reads of an open list stays unmeasured, recorded as `unrunnable-here`, and the windowed list was read and not run; SC 2.1.1, 2.4.3 and 1.4.3 and the tree are asserted in the harness alone — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "screen-reader-support — new row, an open list's assistive-technology reading unmeasured, the windowed list not run (F19)."
    rationale: "Report's Coverage row `select list, engine overlay`: an assistive technology's reading `unrunnable-here`, the windowed list read and not run; Outcome's (a11y) line: met in the harness, what an assistive technology reads stays unmeasured."
    fact: F19
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 202
    section: "## 7. Screen Reader Support"
    change: "Replace «named in the tree and unpainted» with «named in the tree and painted as a header on an open list»; the rest of the line stands."
    sidecar: "a11y-plan §7 — a group's label is painted as a header on an open list; the unpainted limit retired."
    rationale: "The report's Retired bullet names «a group's label is unpainted», and its `group-label` row lists a11y-plan:202 as changed."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 33
    section: "## 7. Screen Reader Support"
    change: "New row following line 33: «- No `expanded` node key exists: `NodeState` gained no field and `NODE_FIELDS` stays thirteen, so a list's open state reaches the tree and not the snapshot, on the operator's word (inputs#I6) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "screen-reader-support — new row, no `expanded` node key, the open state in the tree alone (F17)."
    rationale: "Report's Bridge bullet: `NodeState` gains no field, `NODE_FIELDS` stays thirteen; Decisions & corrections and inputs#I6: the operator's word, no `expanded` node key."
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 33
    section: "## 7. Screen Reader Support"
    change: "New row following line 33: «- An option's label and a group's `label` leave an open list only as painted pixels in a `screenshot` and the snapshot's existing `name`, with no log site; a shot of a row crops to it and leaves the list open (`a_shot_of_a_row_crops_to_it_and_leaves_the_list_open`) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "screen-reader-support — new row, how an open list's labels leave: pixels and the existing `name`, no log site (F18)."
    rationale: "Report's Coverage row `select list, engine overlay`: no log site, a label leaves only as a painted pixel and the snapshot's existing `name`; Schema / config names the new pixel content; Outcome's (design) line names the shot's test."
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Replace «the list's fifteen words» with «the list's seventeen words, `select_popup` and `select_popup_painted` joining at 2026-10-31-select-popup»; the rest of the line stands."
    sidecar: "ci-integration:9 — stand-repeat's named `--test` words fifteen to seventeen."
    rationale: "The report's `repeat-names` row moves stand-repeat's named `--test` words fifteen to seventeen and lists a11y ci-integration:9 as changed; Harness / gate surface: the list gains two words, asserted by `test_ci_workflows.py:367`."
    basis: .github/scripts/ci-leg.sh:59-60
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Replace «14 files and 112 tests (`stand_keyboard` 21), over the tier-0 witnesses alone, 9 files and 86 tests (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)» with «14 files and 114 tests (`stand_keyboard` 23), over the tier-0 witnesses alone, 9 files and 88 tests (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»."
    sidecar: "ci-integration:10 — the a11y leg 112 to 114 tests, `stand_keyboard` 21 to 23, the tier-0 witnesses 86 to 88."
    rationale: "The report's `kbd-count` row moves `stand_keyboard` 21 to 23 and its `a11y-tally` row moves the leg to 114, kbd-count listing a11y ci-integration:10 as changed. The report states no 88: it is this line's own sum, 114 less the five tree-reading checks' 26 (6, 13, 3, 3, 1 on line 11), the +2 falling on `stand_keyboard`, one of the nine."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "Replace «`stand_keyboard` 21 and `stand_contrast` 7 (52)» with «`stand_keyboard` 23 and `stand_contrast` 7 (54)», and «14 result lines · 112 passed · 0 failed · 0 ignored, locally and on the \"Accessibility (a11y) tests\" job of CI#41192433140 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)» with «14 result lines · 114 passed · 0 failed · 0 ignored, locally and on the \"Accessibility (a11y) tests\" job of CI#41235503227 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»; the other per-file counts stand."
    sidecar: "ci-integration:11 — the a11y log 14 · 114 · 0 · 0 on CI#41235503227, `stand_keyboard` 23."
    rationale: "The report's `a11y-tally`, `kbd-count` and `ci-tally` rows each list a11y ci-integration:11 as changed; the gate reads `lines 14 passed 114 failed 0` and CI#41235503227 reads a11y 14 · 114 · 0 · 0. The six's sum 54 is 52 plus `stand_keyboard`'s +2, and 54 + 60 = 114."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 12
    section: "## 9. CI Integration"
    change: "New continuation following line 12: « `select_popup`, `select_popup_painted` and `stand_act_popup` run in the workspace test leg and three rounds in `stand-repeat`, the first two by its named `--test` list and `stand_act_popup` by its `stand_*` glob, and none joins the a11y leg, which stays fourteen files, though `stand_keyboard` carries the list's walk there (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`, `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`) (.github/scripts/ci-leg.sh:59-60; tests/blitz-tests/tests/stand_keyboard.rs:1082-1134 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "ci-integration — the list's three checks and their legs, none in the a11y leg (F27)."
    rationale: "Report's Harness / gate surface: the list gains two words, `stand_act_popup` joins `run stand` and `stand-repeat` by its prefix, the `a11y` leg stays fourteen files; New text, by line lists the two `stand_keyboard` walks at 1082-1134."
    basis: .github/scripts/ci-leg.sh:59-60
    fact: F27
note: Neither scoped detector fired (the list's §7 reading is covered and now stated, and no a11y or obs schema moved). F18 sits on the boundary widening that halts P2 on inputs#I4. ci-integration:13's `STATE_ROWS` 18 is left as the dated cold-agent-bar record, though the report's `state-rows` row does not name that line, so the orchestrator may want to confirm it. The 88 at ci-integration:10 is computed, not reported.
