proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "In `covered`'s measured limits, after the no-area sentence and before the turn to `off-screen`'s text, add: and, since 2026-10-31-select-popup, that an option of a closed select lies under it, «click the select first, then the option», pinned by `a_closed_option_stays_refused_covered_and_changes_nothing` (tests/blitz-tests/tests/stand_act_popup.rs:157-166), its name, its remedy and the cause order unchanged, the cause set still eight, a fixed `'static` text holding nothing a call supplies (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "error-handling: `covered`'s meaning gains the closed-option clause; name, remedy, order and the eight causes unchanged."
    rationale: "Report, Changes, Symbols / APIs, Driver: `refusal.rs` `covered`'s meaning gains «an option of a closed select lies under it: click the select first, then the option»; name, remedy and order unchanged; Schema / config restates it as the program's own `'static` text. The row enumerates this meaning's limits and lacks the clause."
    basis: packages/escher-driver/src/refusal.rs:76-81
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "Keep the row's `type` help clause on a `multiple` select and extend it: at 2026-10-31-select-popup `click`'s, `type`'s and `press`'s fixed help was restated, `click`'s now stating that a click on a select opens its list and `press`'s stating the keys on a closed select; `type` on any select focuses with no click, so it fires no DOM focus events on a single select either, and a text opening on a space types nothing there; each is still a fixed literal of the verb table holding nothing a call supplies, and the verb count and `describe`'s count row are unmoved."
    sidecar: "error-handling: the fixed help of `click`, `type` and `press` restated for the select list; `type` on any select focuses with no click."
    rationale: "Report, Changes, Symbols / APIs, Driver: `type` on any select focuses with no click, firing no DOM focus events on a single select either, a text opening on a space typing nothing there; `schema.rs` restates the three fixed help texts; `VERBS` 12 and `describe`'s count row unmoved. The row's help reading stops at a `multiple` select."
    basis: packages/escher-driver/src/schema.rs:240-243
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 36
    section: "## Error Handling"
    change: "- Key press after a hidden ancestor: a key press panics in `position_in_containing_block` (packages/blitz-dom/src/node/node.rs:1775), the stack through `handle_dom_event`'s `absolute_position`, when the focused control's anonymous block was dropped because an ancestor gained `hidden`; any control, reproduced on a `select` and a plain `button` with no list opened, and a remote page can reach it; not fixed at that chunk, its owner routed to that wrap's P5; that it predates the chunk rests on `node.rs` and `events/mod.rs` unchanged against the base and on the stack, no controlled run at the base having been taken; `select_popup` skips the key press in that one row — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "error-handling: new panic-path row, a key press after an ancestor of the focused control gained `hidden`, remote-reachable and not fixed."
    rationale: "Report, Spec claims disproved, `panic-pos`: the panic-path list holds no row for it and the owed text is a new row in security-plan §Error Handling; the fix's owner is P5's, so the row states the panic and leaves its ending open."
    basis: packages/blitz-dom/src/node/node.rs:1775
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "After the row's last reading add: since 2026-10-31-select-popup `host_log` and `mcp_log` each hold a test that opens a select's list and chooses in it through the host or the role, with 0 option needles at `trace` (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; the needles at examples/seven_guis/tests/common/mod.rs:425-453), `host_log`'s in the state directory `hl-popup` under `#[cfg(unix)]`, as every host-driving test there; `mcp_log`'s records nothing and no `ml-popup` directory exists (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "logging-monitoring: `host_log` and `mcp_log` read 0 option needles at `trace` over a list opened and chosen in."
    rationale: "Report, Outcome, Acceptance: `host_log`, `mcp_log` 0 needles at `trace`; Reverted / negative API facts: `mcp_log`'s popup test records nothing and no `ml-popup` directory exists; the row is where these process-level needle readings stand."
    basis: examples/seven_guis/tests/host_log.rs:296
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "After the row's last reading add: the select list of 2026-10-31-select-popup adds no log site, span or field: the two new `select_list.rs` modules, blitz-dom's `node` and blitz-paint's `render`, hold no tracing token by gate, `packages/escher-driver/src` still holds two `info_span!` and `stand_act_spans` still reads eight span fields, and an option's or a group's label leaves only as a painted pixel and the snapshot's existing `name`, `SelectListRow.label` never logged (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "logging-monitoring: the select list adds no log site, span or field; a label leaves only as a painted pixel and the snapshot's existing `name`."
    rationale: "Report, Coverage of new surfaces: instrumentation none added, no log site, a label only as a painted pixel and the snapshot's existing `name`; Schema / config: no log field or span; Outcome, Gates: the tracing-token grep over both modules green and two `info_span!`; Acceptance: `stand_act_spans` eight fields."
    fact: F16
note: "F2 and F3 fit error-handling's graceful degradation and no picked entity here carries them, so they stay with the new-detail pass; the report's `Focussed node` reading is neither moved nor unmoved, so logging-monitoring's dated re-reads stand; D-security-input, D-security-auth and D-security-deps found no drift in these four categories."
