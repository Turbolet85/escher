proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "Add after line 14, as a continuation of the sink's item:  since 2026-10-31-select-popup a select's open list is read at `RUST_LOG=trace` by two standing checks, each opening the list and choosing a row, 0 needles in each — seven_guis `host_log` through the `escher-session` host's own sink, in the state directory `hl-popup` under `CARGO_TARGET_TMPDIR`, `#[cfg(unix)]` as every host-driving test there, and seven_guis `mcp_log` through the `mcp` role, which records nothing and has no state directory of its own (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; the shared `option_needles`, examples/seven_guis/tests/common/mod.rs:425-453; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Logging & Monitoring: the two standing log checks of an open select list added, `host_log` in `hl-popup` and `mcp_log`, 0 needles at trace (2026-10-31-select-popup)."
    rationale: "The file records each sink-installing host's log reading and its state directory (`hl-client`, `hl-cells`); the report adds one more each: Symbols / APIs names `hl-popup` (host_log.rs:296) and no `ml-popup`, Reverted says `mcp_log`'s popup test records nothing, Spec claims disproved says the host test carries `#[cfg(unix)]`, and Outcome's (security) row reads `host_log`, `mcp_log` 0 needles at `trace`, met."
    basis: "escher-0.3.0/chunks/2026-10-31-select-popup/report.md:155"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "Add after line 14, as a continuation of the sink's item:  the select list of 2026-10-31-select-popup, an engine overlay on every document, remote pages included, adds no log site, log field or span and moves no redaction shape: a label leaves only as a painted pixel and the snapshot's existing `name`, `SelectListRow.label`, page text in a public read type, is never logged, and escher-driver's `info_span!` sites stay 2 with `stand_act_spans` reading eight fields (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Logging & Monitoring: the select list's silence recorded, no log site, log field or span, no redaction shape moved, a row's label never logged (2026-10-31-select-popup)."
    rationale: "A new surface carrying page text, remote pages included, whose log silence this category should state: Schema / config says no log field or span and no redaction shape moved, Coverage of new surfaces says no log site and `SelectListRow.label` never logged, and Outcome's (obs) and (security) rows read 2 `info_span!` and eight `stand_act_spans` fields."
    basis: "escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110"
note: "D-security-input, D-security-auth and D-security-deps found no violation in this file (no input surface, auth or secret text here; report Dependencies: none); no passage here names a Retired item; line 12's dated `focussed node` reading stands, the report's Decisions calling that count neither moved nor unmoved; the screenshot pixels' boundary widening (P2's halt) is not stated here as ratified."
