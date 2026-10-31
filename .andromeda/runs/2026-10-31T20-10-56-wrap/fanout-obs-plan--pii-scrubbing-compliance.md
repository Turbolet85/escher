proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 15
    section: "## 8. PII Scrubbing & Compliance"
    change: "- An `optgroup`'s non-empty `label` is its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and, since 2026-10-31-select-popup, it is painted as a header in an open list, its pixels in a `screenshot` of that list; the added lines hold no log site, print or env read, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md; the header at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "§8: an optgroup's label is now painted as a header in an open list; the retired «a group's label is unpainted» ends at this row."
    rationale: "The report's Retired bullet names «a group's label is unpainted», and its `group-label` fact and Expected amendments (`obs-plan` §8, «a group's label painted») move it; this row's «it is not painted» still states the limit as standing, and the report's sweep lists no hit here, its listing reaching its patterns' wording and no further."
    basis: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 16
    section: "## 8. PII Scrubbing & Compliance"
    change: "- An open select list crosses the ratified carriers and adds none: an option's label and a group's `label`, page text, are new painted pixels in every `screenshot` of an open list, and a label otherwise leaves only as the snapshot's existing `name`; a row's own `bounds` reads in the snapshot and the diff, `differs` naming `bounds`, a closed set inside the ratified crossing; no key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span is added and no redaction shape moves; the pixels are a boundary widening, carried to the founder's answer card at this wrap's P2 on his recorded word, that report's inputs#I4 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "§8: new row, a page's option and group labels as pixels in a shot of an open list, the boundary widening carried to the founder's card."
    rationale: "Report Schema / config states the crossing and the new pixel content, Coverage of new surfaces marks the list's pixels «boundary widening, P2's halt», and Expected amendments has the playbook's Boundary widening halt once at P2 on inputs#I4; §8 holds no statement of this crossing."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 16
    section: "## 8. PII Scrubbing & Compliance"
    change: "- A select list's labels reach no log: `SelectListRow.label` holds page text in a public read type and is never logged, the two new `select_list` modules and the added `packages/*.rs` lines hold no log site, the census reading 0, and a list opened and chosen through the `escher-session` host and through the `mcp` role reads 0 option needles on stderr at `trace` (`option_needles`; `host_log`, unix only; `mcp_log`, which records nothing), `stand_act_spans` still reading the `command` span's eight fields (packages/blitz-dom/src/node/select_list.rs:93-108; examples/seven_guis/tests/common/mod.rs:425-453; examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "§8: new row, the select list's labels reach no log, with the host and mcp readings at trace that hold it."
    rationale: "Report Coverage of new surfaces (`PII no log site`; `SelectListRow.label` page text in a read type, never logged), Outcome's (security) and (obs) acceptance lines and the gate entries read 0 needles and 0 added log tokens; D-obs-pii holds, and §8 records each such reading as a row."
    basis: packages/blitz-dom/src/node/select_list.rs:93-108
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 16
    section: "## 8. PII Scrubbing & Compliance"
    change: "- The driver's restated `click`, `type` and `press` help and `covered`'s meaning, with its clause on an option of a closed select, are the program's own fixed `'static` text and hold no call text; the `command` span's fields stay eight (`stand_act_spans`) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "§8: new row, the driver's restated help and `covered` text hold no call text."
    rationale: "Report Coverage of new surfaces, `driver help and covered text`, gives this PII reading (`no call text`), and Schema / config states the texts are the program's own `'static` text."
note: "No raw PII in a log site on the report's readings, so D-obs-pii, D-obs-instrumentation and D-obs-stack raise nothing here; the first D-new-detail row's closing clause on the founder's card should be restated to his ruling once P2's halt has his answer."
