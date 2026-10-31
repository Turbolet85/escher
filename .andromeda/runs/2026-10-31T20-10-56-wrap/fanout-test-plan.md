proposals:
  - detector: D-retired
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-start: `type` replaces after the focusing click on a control that takes one, and a select of either kind, single or `multiple`, is focused with no click, so `type` fires no DOM focus events on a single select and a text opening on a space types nothing there; the parenthesis naming a `multiple` select alone ends."
    sidecar: "§3 → Session lifecycle, session-start: `type` focuses any select with no click, no longer a `multiple` select alone (select popup; `Target.multiple` and `focus_multiple_select` retired)."
    rationale: "The report's Retired bullet names `Target.multiple` and `focus_multiple_select`; its Driver bullet states `Target.select`, `focus_select` and that `type` on any select focuses with no click; its `type-focus` row lists session-lifecycle:5 as changed. The contract's parenthesis still asserts the retired multiple-only mechanism as standing."
  - detector: D-new-detail
    severity: warning
    section: "§3 → Session lifecycle"
    file: new
    near: "session-proof, beside `stand_act_select` 10, the fifteenth, and beside `host_log` and `mcp_log`"
    change: "session-proof gains `stand_act_popup` 12, the sixteenth driver-action file, `Session::run` in both layout modes; seven_guis `host_log` 4 becomes 5 with `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, unix only, and `mcp_log` 2 becomes 3 with `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing; each reads no option at `trace`."
    sidecar: "§3 → Session lifecycle, session-proof: `stand_act_popup` 12 added; `host_log` and `mcp_log` each gain a select-list silence test (select popup)."
    rationale: "The report's Coverage of new surfaces gives `stand_act_popup` 12 and `cli_commands`, `host_log`, `mcp_log` +1 each; its Outcome reads the three new checks green in both layout modes and `host_log`, `mcp_log` 0 needles at `trace`; Deviations and Reverted state the `cfg(unix)` gate and no recording in `mcp_log`. Its `act-files` and `cli-count` rows move session-lifecycle:11's words and `cli_commands`, and no row moves this per-file proof. §3 has no category file; this detail belongs in the key's session-proof row, not a new file."
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness hold: every new surface carries tests in the report's Coverage, the runner stays `cargo test` through `ci-leg.sh` and `agent-run.sh`, and no harness, status or log format moved. Outside these detectors, for the orchestrator's sweep: test-plan:190 keeps `22 stand_* readers` and `six non-stand readers` and test-plan:151 keeps its `22 stand_* files` beside the `mod-decl` move 28 to 31, the split not stated in the report; §3 Input helpers' `scroll_into_view` still reads every scrolling box then the viewport, where the report's Seen-through-the-list bullet has a row write the list's `scroll_y` and return. The D-new-detail target is the key's session-proof row, so `file: new` is the rule's label for a section with no category file, not a request for a new file."
