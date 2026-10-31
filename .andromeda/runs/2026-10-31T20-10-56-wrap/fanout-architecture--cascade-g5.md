proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 178
    section: "## Standard Contracts"
    change: "In the sentence quoting `covered`'s meaning, after the quoted meaning and before «the remedy, the eight causes and their names unchanged», add: and, since 2026-10-31-select-popup, the clause «an option of a closed select lies under it: click the select first, then the option»."
    sidecar: "contract-driver-session: `covered`'s meaning gains the clause on an option of a closed select; name, remedy and order unchanged (2026-10-31-select-popup)."
    rationale: "Report Changes, Driver: `refusal.rs` `covered`'s meaning gains that clause, name, remedy and order unchanged; Counts `covered-text` names standard-contracts:178 as changed."
    basis: packages/escher-driver/src/refusal.rs:76-81
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 179
    section: "## Standard Contracts"
    change: "After the clause stating what `type`'s help says, add: ; the fixed help of `click`, `type` and `press` restated at 2026-10-31-select-popup, `click`'s saying a click on a select opens its list, each still a fixed literal of the verb table."
    sidecar: "contract-driver-session: the fixed help of `click`, `type` and `press` restated for the select list (2026-10-31-select-popup)."
    rationale: "Report Changes, Driver: `schema.rs` restates `click`'s, `type`'s and `press`'s fixed help, the program's own `'static` text per Schema / config; the New text lists `click`'s help from «a click on a select opens its list»."
    basis: packages/escher-driver/src/schema.rs:222-226
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 189
    section: "## Standard Contracts"
    change: "Replace «on every target but a `multiple` select, which it focuses with no click (below)» with «on every target but a select, single or `multiple`, which it focuses with no click (below)»; the rest of the line stands."
    sidecar: "contract-driver-session: `type` focuses any select with no click, a single select too (2026-10-31-select-popup)."
    rationale: "Report Changes, Driver: `type` on any select focuses with no click; Counts `type-focus` names standard-contracts:189 as changed."
    basis: packages/escher-driver/src/execute.rs:236-238
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 190
    section: "## Standard Contracts"
    change: "Rewrite the opening to read: `type` on a select, single or `multiple`: `Target` carries `select`, and `type` does not click such a target, a click landing on a `multiple` select's row at the centre and opening a single select's list: the crate-private `focus_select` focuses it with no click and clears the typeahead, without which a second `type` joins the first's text; the rest of the line stands."
    sidecar: "contract-driver-session: `Target.multiple` is now `Target.select` and `focus_multiple_select` is now `focus_select`, reaching every select (2026-10-31-select-popup)."
    rationale: "Report Retired names `Target.multiple` and `focus_multiple_select`; Changes, Driver: `Target.multiple` → `Target.select`, `focus_multiple_select` → `focus_select`; Pointer: `handle_click`'s `select` arm opens a single select's list; Counts `focus-select` and `type-focus` name standard-contracts:190 as changed."
    basis: packages/escher-driver/src/execute.rs:558-560
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 190
    section: "## Standard Contracts"
    change: "Add after this line, in the entity's indented continuation shape: ` `focus_select` also closes any list, and `type` on a single select fires no DOM focus events, a text opening on a space typing nothing there.`"
    sidecar: "contract-driver-session: `focus_select` closes any list; `type` on a single select fires no DOM focus events (2026-10-31-select-popup)."
    rationale: "Report Changes, Driver: `focus_select` also closes any list; `type` on any select focuses with no click, so it fires no DOM focus events on a single select either, and a text opening on a space types nothing there. New fact F20."
    basis: packages/escher-driver/src/execute.rs:558-560
    fact: F20
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 212
    section: "## Standard Contracts"
    change: "In the `overlap` reading replace «a closed select's options excepted (below)» with «a closed select's options and an open list's rows and groups excepted (below)»."
    sidecar: "contract-driver-session: the lint's `overlap` exclusion also holds for an open list's rows and groups (2026-10-31-select-popup)."
    rationale: "Report Changes, Driver: `Entry.in_select_box` is true also for an open list's row or group; Counts `closed-option` and `lint-exclude` place the moved reading at standard-contracts:212 and :213."
    basis: packages/escher-driver/src/lint.rs:339-342
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 212
    section: "## Standard Contracts"
    change: "After the sentence defining a node's clip region add: `clip_region` takes an open row's region from `visible_region` alone, with no `contain: paint` cut."
    sidecar: "contract-driver-session: an open row's clip region is `visible_region` alone, with no `contain: paint` cut (2026-10-31-select-popup)."
    rationale: "Report Changes, Driver: `clip_region` takes an open row's region from `visible_region` alone, no `contain: paint` cut. New fact F21."
    basis: packages/escher-driver/src/lint.rs:405-408
    fact: F21
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 213
    section: "## Standard Contracts"
    change: "Rewrite the opening to read: The `overlap` reading leaves out a closed select's options and an open list's rows and groups: `Entry.in_select_box` reads a closed select's option through `closed_select_of`, by the element and never the role, so CRUD's `role=\"option\"` rows are not read as a select's, and is true also for an open list's row or group, and `overlaps` skips such an entry on either side; a `multiple` select's rows are linted as usual."
    sidecar: "contract-driver-session: `Entry.in_closed_select` is now `in_select_box`, true also for an open list's row or group (2026-10-31-select-popup)."
    rationale: "Report Retired names `Entry.in_closed_select`; Changes, Driver: `Entry.in_closed_select` → `in_select_box`, true also for an open list's row or group; Counts `closed-option` and `lint-exclude` name standard-contracts:213 as changed."
    basis: packages/escher-driver/src/lint.rs:339-342
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 220
    section: "## Standard Contracts"
    change: "Before «a `multiple` select's rows read their own client rects» add: an open list's rows read their own bounds in the list and `overlaps` skips them too, so they read no `overlap` on the stand, `an_open_lists_rows_read_no_overlap_on_the_stand` and `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads` pinning it;"
    sidecar: "contract-driver-session: an open list's rows read their own bounds and no `overlap`, the closed reading and `at_boot` standing (2026-10-31-select-popup)."
    rationale: "Report Changes, Schema / config: a row's own `bounds` cross in the snapshot; Driver: `in_select_box`; Outcome (lint): `at_boot` `[0, 0, 0, 0]`, a row no `overlap`, the planted pair reading, met by those two tests; Counts `closed-option` and `lint-exclude` name standard-contracts:220 as changed."
    basis: tests/blitz-tests/tests/stand_lint.rs:493-519
note: "F13, F18 and F19 stay with the scrolling and bridge entities they name (other groups or the new-detail pass); the report gives only the added clause of `covered`'s meaning and a truncated first line of `click`'s help, so the orchestrator's validator should read their final wording, and that of `press`'s quoted Ctrl help at :192, against `refusal.rs` and `schema.rs`."
