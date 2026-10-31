proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "Replace the clause `off-screen`'s that a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows with: `off-screen`'s, restated at 2026-10-31-engine-debts, that under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view; the rest of the sentence (its remedy, the cause set (eight) and the cause names unchanged, a fixed `'static` text) stands."
    sidecar: "error-handling: `off-screen`'s transformed-box limit retired for the rotated bounding-box limit (2026-10-31-engine-debts, F1)."
    rationale: "The report's Retired bullet names `off-screen`'s transformed-box clause and says the word `untransformed` leaves packages/escher-driver; Symbols / APIs (driver and stand) gives the new meaning at refusal.rs:83-85 with its remedy, the eight causes and the names unmoved; Counts / qualifiers moved lists `visible-region` as changed at error-handling.md:9, and Expected amendments carries security-plan §Error Handling, `off-screen`'s text."
    basis: .andromeda/security-plan-detail/error-handling.md:9
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: "## Error Handling"
    change: "In the as-measured list, replace its no-area sentence and `off-screen`'s transformed-box sentence at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md with: its no-area sentence at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md, `off-screen`'s rotated-box sentence at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md."
    sidecar: "error-handling: the as-measured list stops naming `off-screen`'s retired transformed-box sentence and names the sentence that replaced it."
    rationale: "The passage still names the sentence the report's Retired bullet retires as a standing text with its measure; this is the second occurrence of that claim in the file, the edit being the sentence named and not a citation's digits."
    basis: .andromeda/security-plan-detail/error-handling.md:10
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "Replace the clause `type`'s help still says that it replaces and that on a select the text chooses the first option whose label starts with it with: `type`'s help still says that it replaces and that on a single select the text chooses the first option whose label starts with it, and since 2026-10-31-engine-debts that on a `multiple` select the text chooses its match alone, a text opening on a space typing nothing."
    sidecar: "error-handling: `type`'s help gains the `multiple` select's reading (2026-10-31-engine-debts, F6)."
    rationale: "The report's Symbols / APIs (driver and stand) lists `type`'s help among the fixed `'static` texts moved (schema.rs:239-240) and states F6's `type` on a `multiple` select; Deviations 6c states the opening space; Counts / qualifiers moved lists `type-on-select` as changed at error-handling.md:9, a single select's reading unmoved."
    basis: .andromeda/security-plan-detail/error-handling.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 12
    section: "## Error Handling"
    change: "New record after this line: - `press`'s help, since 2026-10-31-engine-debts, states a `multiple` select's limit, that a set of rows that are not adjacent needs Ctrl, which this verb does not carry; a fixed `'static` literal of the verb table holding nothing a call supplies, with no verb, argument, key name, result field, cause, session error or exit added (packages/escher-driver/src/schema.rs:261-263) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "error-handling: records `press`'s fixed help text on a `multiple` select's non-adjacent limit (2026-10-31-engine-debts, F6)."
    rationale: "The report's Symbols / APIs (driver and stand) moves `press`'s fixed text at schema.rs:261-263 and quotes the limit, and F6 states that no verb, argument, key name, result field, cause, session error or exit is added; line 9 holds the verb table's help texts as fixed literals and none of its rows states this one (`press-ctrl`: no site states `press`'s help)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 30
    section: "## Error Handling"
    change: "New record after this line: - Panic hazard in a spawned key-handler task: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics `RefCell already borrowed` (packages/dioxus-native-dom/src/events.rs:162); any app meets it, and no engine line moved for it; Cells calls its handles synchronously instead; measured headlessly through the harness's `poll(None)`, the windowed shell's same lines read and not run — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "error-handling: records the `MountedData` panic hazard measured at 2026-10-31-engine-debts, no engine line moved."
    rationale: "The report's Decisions & corrections states the hazard as outside the chunk's fixes, Reverted / negative API facts records the panic that kept step 15's async form from shipping, and Counts / qualifiers moved calls `mounted-hazard` a new fact at no standing site; this file's records hold the engine's panic paths and none states it. The report writes `events.rs:162` bare; the directory is read from the body's `NodeHandle` cite at .andromeda/security-plan.md:369."
    basis: .andromeda/security-plan.md:369
note: Line 9 already sits near the 2,000-char bound and two proposals grow it, so applying them needs a re-wrap into line 10; the report does not quote `type`'s new help words (schema.rs:239-240), so that proposal states the behaviour the report states, and the `MountedData` hazard's owner is still P5's to name.
