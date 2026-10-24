proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 42
    section: '## 1. Test Scope Summary'
    change: 'blitz-paint row, `password_caret_click`: after "an engine check on a parsed page with no `stand_` prefix, so not selected by `run stand`" add "— and, since 2026-10-24-stand-requirement-sweep, run by the `stand-repeat` leg through its named `--test` list, one of the ten 0.2.0 witnesses outside the prefix, the file keeping its name (ratified by the founder, 2026-10-24, «Список в скрипте (Рекомендую)») —"'
    sidecar: 'test-scope-summary: password_caret_click also runs in stand-repeat by its named list (2026-10-24-stand-requirement-sweep)'
    rationale: 'Report Changes, Schema / config: ci-leg.sh stand-repeat (54-58) adds `--test password_caret_click` among ten named targets after the `stand_*.rs` glob; Count 1 lists `test-scope-summary.md:42` as moved, the not-by-`run stand` half still true (Harness / gate surface: `run stand` keeps the prefix rule).'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:66-69'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 96
    section: '## 1. Test Scope Summary'
    change: '`accessibility_states` 5: "run by the workspace `test` leg and per package and not by the `a11y` leg" → "run by the workspace `test` leg, per package and, since 2026-10-24-stand-requirement-sweep, the `stand-repeat` leg through its named `--test` list, and not by the `a11y` leg"'
    sidecar: 'test-scope-summary: accessibility_states gains stand-repeat among its runners'
    rationale: 'Report Count 1 moves `test-scope-summary.md:96` (accessibility_states) — `--test accessibility_states` is one of stand-repeat''s ten; the a11y leg''s six files are unchanged (Expected amendments, a11y-plan bullet).'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:97-99'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 99
    section: '## 1. Test Scope Summary'
    change: 'the clause closing `accessibility_merged_states` · `accessibility_active_descendant_unread` · `inert_subtree`: "each its own auto-discovered target, run by the workspace `test` leg and per package and not by the `a11y` leg''s six" → "each its own auto-discovered target, run by the workspace `test` leg and per package — `accessibility_active_descendant_unread` and `inert_subtree`, since 2026-10-24-stand-requirement-sweep, also by the `stand-repeat` leg through its named `--test` list, `accessibility_merged_states` not — and not by the `a11y` leg''s six"'
    sidecar: 'test-scope-summary: inert_subtree and accessibility_active_descendant_unread gain stand-repeat; accessibility_merged_states unchanged'
    rationale: 'Report Count 1 moves `:99` (the inert_subtree and active-descendant clause); the ten named in ci-leg.sh 54-58 hold those two and not accessibility_merged_states, so the shared clause splits.'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:66-69'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 108
    section: '## 1. Test Scope Summary'
    change: '`stand_act_spans`: "35 calls per mode — one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone —" → "36 calls per mode (35 until 2026-10-24-stand-requirement-sweep) — one `snapshot` with `part` 1 and, since that chunk, a paged part 2 among them, each line `verb="snapshot"` alone —"'
    sidecar: 'test-scope-summary: stand_act_spans 35 → 36 calls per mode, a paged part read in the sink capture'
    rationale: 'Report Count 6 moves `test-scope-summary.md:108` ("35 calls per mode — one `snapshot` with `part` 1 among them"); Symbols: the call count 35 → 36 (stand_act_spans.rs:727), the part-read line shape unchanged.'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:151-156'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 108
    section: '## 1. Test Scope Summary'
    change: 'after the call count add: "since 2026-10-24-stand-requirement-sweep `child_records_each_command_at_info` holds the shared paged page through `fixtures::hold("paged", fixtures::many_items, …)` and reads `snapshot_part(2)` as `Outcome::Part` with `part` 2 and `parts` ≥ 2, `Needles::read` taking the page''s 400 `item-NNN` ids and its `#items > button` texts as needles, so no id or name of the paged page reaches a line (tests/blitz-tests/tests/stand_act_spans.rs:685-693; tests/blitz-tests/tests/stand_act_spans.rs:243)"'
    sidecar: 'test-scope-summary: stand_act_spans reads a paged part 2 of fixtures::many_items, its ids and names among the needles'
    rationale: 'Task F9; report Symbols, stand_act_spans.rs 685-693 and 243.'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:53-56'
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: '## 1. Test Scope Summary'
    change: '"since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` 2, a stand check `run stand` picks up by its prefix" → "since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` 3 (2 until 2026-10-24-stand-requirement-sweep, which added a second record and its leg check), a stand check `run stand` picks up by its prefix"; "`WITNESSES`, the requirement-to-check record: 16 rows" stays, byte-unchanged, with `WITNESSES_020` beside it'
    sidecar: 'test-scope-summary: stand_requirement_sweep 2 → 3 tests, a second record WITNESSES_020 beside WITNESSES'
    rationale: 'Report Count 3 moves `test-scope-summary.md:117` ("`stand_requirement_sweep` 2", "`WITNESSES` … 16 rows"); Symbols: 2 → 3 tests, `WITNESSES` and `every_requirement_names_its_witness` byte-unchanged.'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:116-121'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: '## 1. Test Scope Summary'
    change: 'after the `WITNESSES` clause add: "and, since 2026-10-24-stand-requirement-sweep, `WITNESSES_020`, the 0.2.0 record: nine rows v020-01…v020-09 in order, 46 witnesses — v020-01 and v020-02 holding `Unit` witnesses alone (six crate unit tests in `packages/blitz-dom`, `packages/escher-driver`, `packages/dioxus-native-dom` and `examples/seven_guis/src/tasks/timer.rs`), v020-08 one `Unit` (`node/select.rs`), every other row a non-`Unit` witness (tests/blitz-tests/tests/stand_requirement_sweep.rs:538-818)"'
    sidecar: 'test-scope-summary: WITNESSES_020 — nine 0.2.0 rows, 46 witnesses'
    rationale: 'Task F2; report Symbols, `WITNESSES_020` (538-818).'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:28-30'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: '## 1. Test Scope Summary'
    change: 'beside `WITNESSES_020` add: "a `Subject::Unit` witness names a crate''s unit test, run by the workspace `test` leg and outside `stand-repeat` by construction (tests/blitz-tests/tests/stand_requirement_sweep.rs:227-229)"'
    sidecar: 'test-scope-summary: Subject::Unit — a crate unit test, outside stand-repeat by construction'
    rationale: 'Task F3; report Symbols, `Subject::Unit` (227-229).'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:26-27'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: '## 1. Test Scope Summary'
    change: 'beside it add: "`every_0_2_0_requirement_names_its_witness_inside_stand_repeat` asserts the nine rows in order, each named test found as `fn {test}(` in its file; v020-01 and v020-02 `Unit` alone and every other row a non-`Unit` witness; every non-`Unit` blitz-tests witness a `stand_*` file or one of `STAND_REPEAT_NAMED` (10), every `examples/seven_guis/tests/` witness one of `STAND_REPEAT_SEVEN_GUIS` (8); and `.github/scripts/ci-leg.sh`, read through `include_str!`, holding the `tests/blitz-tests/tests/stand_*.rs` glob and `--test {name}` for all 18 (tests/blitz-tests/tests/stand_requirement_sweep.rs:820-844; tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069)"'
    sidecar: 'test-scope-summary: the 0.2.0 leg check — every witness inside stand-repeat, read from ci-leg.sh'
    rationale: 'Task F4; report Symbols, `STAND_REPEAT_NAMED` (820-832), `STAND_REPEAT_SEVEN_GUIS` (834-844), the test (994-1069).'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:31-38'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 120
    section: '## 1. Test Scope Summary'
    change: '"since 2026-10-19-visual-lint, `stand_lint` 6, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb" → "since 2026-10-19-visual-lint, `stand_lint` 7 (6 until 2026-10-24-stand-requirement-sweep), declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, in its lint-verb test, on the `lint` verb"'
    sidecar: 'test-scope-summary: stand_lint 6 → 7 tests'
    rationale: 'Report Count 4 moves `test-scope-summary.md:120` ("`stand_lint` 6"); Symbols: a seventh test added at stand_lint.rs 226-274, ahead of the lint-verb test, so the ordinal "its sixth" no longer names it.'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:124-128'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: '## 1. Test Scope Summary'
    change: '"a clipped or off-screen node holding text also reading `truncated` is asserted by no check" → "a clipped or off-screen node holding text also reading `truncated` is asserted, since 2026-10-24-stand-requirement-sweep, by `stand_lint`''s `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` (tests/blitz-tests/tests/stand_lint.rs:226-274)"'
    sidecar: 'test-scope-summary: the clipped/off-screen co-reading of truncated now asserted by stand_lint'
    rationale: 'Report Count 4 moves `test-scope-summary.md:121` ("is asserted by no check"); Outcome: the new test met in both layout modes.'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:124-128'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: '## 1. Test Scope Summary'
    change: 'after the co-reading add: "on its in-file page `reading_page` — `fx-cut-text`, a nowrap line past its 100 × 40 `overflow: hidden` box; `fx-padded-text`, a 60 px paragraph whose `padding-right: 100px` alone passes its box; `fx-off-text`, wholly right of the 800 px viewport (tests/blitz-tests/tests/stand_lint.rs:110-141)"'
    sidecar: 'test-scope-summary: stand_lint reading_page — fx-cut-text, fx-padded-text, fx-off-text'
    rationale: 'Task F5; report Symbols, `reading_page()` (110-141).'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:40-42'
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: '## 1. Test Scope Summary'
    change: 'beside it add: "in both layout modes it reads `fx-cut-text` clipped and truncated, `fx-padded-text` clipped, and `fx-off-text` truncated and off-screen — `fx-cut-text`''s line seen in part, `fx-off-text`''s not at all — and the `lint` verb answers the same"'
    sidecar: 'test-scope-summary: the co-reading test findings, both modes, the lint verb agreeing'
    rationale: 'Task F6; report Symbols (stand_lint.rs 226-274) and Deviations, step 2 (the two arms told apart).'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:43-45'
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 122
    section: '## 1. Test Scope Summary'
    change: '`select_interaction` 7: "(run by the workspace `test` leg, by name and by `run all`, not by `run stand`)" → "(run by the workspace `test` leg, by name, by `run all` and, since 2026-10-24-stand-requirement-sweep, by the `stand-repeat` leg through its named `--test` list, not by `run stand`)"'
    sidecar: 'test-scope-summary: select_interaction gains stand-repeat among its runners'
    rationale: 'Report Count 1 moves `test-scope-summary.md:122` (select_interaction "not by `run stand`"); `--test select_interaction` is one of the ten in ci-leg.sh 54-58, and `run stand` keeps its prefix rule.'
    basis: 'escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:97-99'
note: 'test-scope-summary.md:22 (text_extent 3 → 6, its limits asserted, its runner clause — report Counts 1 and 5) is in no group-2 picked entity, so F7 · F8 go to the new-detail pass; citations into reached files — :108 stand_act_spans.rs:555-567, :117 stand_requirement_sweep.rs:90-216 · 241-536 · 957-992, :120-121 stand_lint.rs:26-108 · 173-392 · 394-416, :123 stand_snapshot_parts.rs:59-310, :90 fixtures/mod.rs:17-130 — sit after lines the chunk added or moved, and the report states no new numbers for them.'
