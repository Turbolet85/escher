proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 43
    section: "## 6. Log Coverage"
    change: "Add under **examples**, after the Cells row:   - `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, under the state directory `cc-popup`, reads a hosted shot of an open list equal to the in-process one, an answer and no log line (examples/seven_guis/tests/cli_commands.rs:419-464; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: add cli_commands' popup test, a hosted shot of an open list equal to the in-process one (select-popup)"
    rationale: "Report Coverage of new surfaces, the list's pixels: tests `cli_commands` hosted shot equal to in-process; Symbols / APIs, Test-only state directories: `cc-popup`"
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 53
    section: "## 6. Log Coverage"
    change: "Add under **escher-driver**:   - no site added: the driver's restated fixed texts, `click`'s, `type`'s and `press`'s help and `covered`'s meaning, are the program's own `'static` text, holding no call text and adding no log site or span field, the `command` span keeping its eight fields (`stand_act_spans`) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: add the restated driver help and covered texts as fixed text adding no log site or span field (select-popup)"
    rationale: "Report Schema / config: three fixed help texts and `covered`'s meaning restated, the program's own `'static` text; Coverage, driver help and covered text: span fields eight, unmoved; no call text"
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 71
    section: "## 6. Log Coverage"
    change: "Add after the focussed-node row:   - the `Focussed node` line's count is unsettled at 2026-10-31-select-popup: 0 lines through the sink on the workspace build with no base reading, so neither moved nor unmoved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/counts.md)"
    sidecar: "log-coverage: record the Focussed node line's count as unsettled, 0 lines through the sink with no base reading (select-popup)"
    rationale: "Report Decisions & corrections: the `Focussed node` count is not settled, 0 lines through the sink on the workspace build, no base reading, so neither moved nor unmoved (`evidence/counts.md`)"
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 75
    section: "## 6. Log Coverage"
    change: "Extend the reader row's named readers with dioxus-native-dom's new public `is_select`, re-exported and read by `execute.rs`'s `locate` alone, instrumentation n/a and adding no log site or print like the rest (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: the silent reader row gains dioxus-native-dom's is_select (select-popup)"
    rationale: "Report Counts, `focus-select`: log-coverage:75 names the readers, its silence holds; add `is_select`. Symbols / APIs, Driver: `is_select` read by `execute.rs` `locate` alone"
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 89
    section: "## 6. Log Coverage"
    change: "Add under **blitz-dom (node)**:   - no site added: the select list's two modules, blitz-dom `node::select_list` and blitz-paint `render::select_list`, hold no tracing or log token (gate entry 5), and the chunk's added `packages/*.rs` lines hold none (gate entry 4) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: add the select list's two modules and the chunk's added package lines as holding no tracing or log token (select-popup)"
    rationale: "Report Gates: the added-lines census and the census over the two modules each green, exit 1; Coverage, select list: instrumentation none added, none required (gate entries 2, 4, 5)"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 127
    section: "## 6. Log Coverage"
    change: "Add a row: - observed absent — an option's label in a sink-installing host's log · searched: `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, under `#[cfg(unix)]` and the state directory `hl-popup`, reading the host at `trace`: 0 option needles, its `command` span lines counted (examples/seven_guis/tests/host_log.rs:282-393; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: add host_log's popup test, 0 option needles at trace in a sink-installing host (select-popup)"
    rationale: "Report Outcome (security): `host_log`, `mcp_log` 0 needles at `trace`; Symbols / APIs: `hl-popup`; Deviations: `#[cfg(unix)]` on the new `host_log` test; New text lists the test and its span count"
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 127
    section: "## 6. Log Coverage"
    change: "Add a row: - observed absent — an option's label in the `mcp` role's log · searched: `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, reading the role at `trace`: 0 option needles; it records nothing, so no `ml-popup` directory exists (examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: add mcp_log's popup test, 0 option needles at trace, recording nothing (select-popup)"
    rationale: "Report Outcome (security): `host_log`, `mcp_log` 0 needles at `trace`; Reverted / negative API facts: `mcp_log`'s popup test records nothing and no `ml-popup` directory exists"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 127
    section: "## 6. Log Coverage"
    change: "Add a row: - The seven_guis shared module holds `option_needles`, a select's option labels as needles for the popup log tests (examples/seven_guis/tests/common/mod.rs:425-453; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: add option_needles to the seven_guis shared module's log-check readers (select-popup)"
    rationale: "Report New text, by line: `examples/seven_guis/tests/common/mod.rs` 425-453 `option_needles`; Outcome (security): the popup log tests read 0 needles"
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 131
    section: "## 6. Log Coverage"
    change: "Add a row: - observed absent — a print, an env read, a subscriber or a listener in the popup checks `select_popup`, `select_popup_painted` and `stand_act_popup` (a `session_common` reader) · searched: the silence census over those files, reading 0, gate entry 2 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "log-coverage: add a silence row for the three popup checks (select-popup)"
    rationale: "Report Expected amendments, `obs-plan`: new silence rows for three checks; Outcome (obs): silence census 0; Gates: the census over `select_popup` and its fellows, exit 1, last line 0"
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a continuation line:  Since 2026-10-31-select-popup a shot of an open list carries a page's option labels and group `label`s as pixels, a boundary widening held for P2's halt, the shot's own fields unmoved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "pii-scrubbing-compliance: a shot of an open list carries option and group labels as pixels, the widening held for P2's halt (select-popup)"
    rationale: "Report Schema / config: new pixel content in every shot of an open list; Coverage, the list's pixels: instrumentation shot's fields unmoved, PII labels in pixels, boundary widening, P2's halt"
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 14
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a row: - `size` on a `multiple` select is a new attribute read, parsed into a whole number capped at 65534, carrying no text out and moving no redaction shape (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "pii-scrubbing-compliance: add the size read on a multiple select, a capped whole number carrying no text out (select-popup)"
    rationale: "Report Schema / config: new attribute read, `size` on a `multiple` select, a parse into a capped whole number, no text carried out, no redaction shape moved; Symbols / APIs, `size`: capped at 65534"
    fact: F13
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 15
    section: "## 8. PII Scrubbing & Compliance"
    change: "Replace «and it is not painted» with «and, since 2026-10-31-select-popup, it is also painted, as a group's header in an open list, leaving as pixels in a shot and on no log line or field», the rest of the row standing, and add its origin: (the header as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "pii-scrubbing-compliance: retire «not painted» for an optgroup's label, now painted as a header of an open list (select-popup)"
    rationale: "Report Retired: the limit «a group's label is unpainted»; Counts, `group-label`: a group's `label` painted as a header; Coverage: no log site, a label leaving only as a painted pixel and the snapshot's existing `name`"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 15
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a row: - An open list's row crosses the snapshot and the diff by its own `bounds`, `differs` naming `bounds` within its closed set, no new field, key or carrier added (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "pii-scrubbing-compliance: add an open row's crossing by its own bounds, no new field, key or carrier (select-popup)"
    rationale: "Report Schema / config: the list crosses the ratified carriers as a row's own `bounds` in the snapshot and the diff, `differs` naming `bounds`, a closed set inside the ratified crossing; no key name, node key or carrier added"
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 15
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a row: - `SelectListRow.label` holds page text in a public read type and is never logged; a label leaves only as a painted pixel and the snapshot's existing `name` (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "pii-scrubbing-compliance: add SelectListRow.label as page text never logged (select-popup)"
    rationale: "Report Coverage, `BaseDocument` public readers: `SelectListRow.label` page text in a read type, never logged; Coverage, select list: no log site, a label leaves only as a painted pixel and the snapshot's existing `name`"
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 15
    section: "## 9. CI Integration"
    change: "In the session state directory row's seven_guis names, add `hl-popup` after `hl-cells`, the rest of the row standing; `cc-popup` stays under `cc-*`, and no `ml-popup` is added, none existing"
    sidecar: "ci-integration: name hl-popup among the seven_guis session state directories (select-popup)"
    rationale: "Report Symbols / APIs, Test-only state directories: `cc-popup` and `hl-popup` under `CARGO_TARGET_TMPDIR`, no `ml-popup`; the row enumerates `hl-*` by name and covers `cc-*` by its glob"
note: "No scoped detector fired (report: no dependency, log site, span or field added); the §3 body sites obs-plan:76, :92 and :95 have no picked entity and are not proposed here, and the `position_in_containing_block` panic's row is security-plan's."
