proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 10
    section: "## Threat Model Summary"
    change: "After line 10 add the sub-item:   - **The select list, since 2026-10-31-select-popup:** a click, Enter or Space on a single select in any document, a remote page's included, opens a list the engine paints over the page, its rows holding the page's option labels and group `label`s; keys and elements read through closed matches, every node through `get_node`, every edge finite, and a select with no row to show opens nothing; hostile `size`, 2,000 options and a 5,500-character label keep every box finite; a list closes with no panic when its select is removed, disabled, hidden or made inert, while a key pressed after an ancestor gained `hidden` still panics at `position_in_containing_block`, for any control and with no list opened, which a remote page can reach (the Error Handling panic paths); per-input checks are listed under Input Validation (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Threat Model Summary, remote web content: the engine-painted select list added as remote-reachable surface, with its bounds and the panic beside it (2026-10-31-select-popup)."
    rationale: "Report Coverage of new surfaces, `select list, engine overlay (every document, remote pages included)`; Outcome's (security) rows, including «met to its letter, with a defect beside it»; Spec claims disproved, `panic-pos`; Expected amendments name security-plan §Threat Model Summary. The Coverage row's «one reader of the state» is left out because Deviations records the field read past that reader in two places."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:10
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 27
    section: "## Threat Model Summary"
    change: "After line 27 add the sub-item:   - **An open select list's crossing, since 2026-10-31-select-popup:** a `screenshot`'s PNG in an answer, of the whole screen or of one row cropped, holds new pixel content where a list stands open: an option's label and a group's `label`, painted — a page's text in pixels, a boundary widening by the playbook, at that chunk's wrap brought to the founder's answer card on his recorded word (the report's inputs#I4); a row's own `bounds` reads in the snapshot and the diff, `differs` naming `bounds`, a closed set inside the ratified crossing; a label otherwise leaves only as the snapshot's existing `name` and has no log site; no key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span is added and no redaction shape moved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Threat Model Summary, session socket: an open select list's painted labels in a screenshot answer recorded as a boundary widening at P2's halt, and a row's bounds as a closed set inside the ratified crossing (2026-10-31-select-popup)."
    rationale: "Report Schema / config states the list's crossing of the ratified carriers and the new pixel content; Coverage, `the list's pixels on the screenshot carriers`, marks it «boundary widening, P2's halt»; Expected amendments, Playbook Boundary widening, halts once on inputs#I4; the Addendum's inputs#I9 sends the widening card to the operator for the founder."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:27
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 37
    section: "## Threat Model Summary"
    change: "After line 37 add the sub-item:   - **An open select list's crossing, since 2026-10-31-select-popup:** a result's `image` block of an open list carries an option's label and a group's `label` as painted pixels to the client, and a snapshot or diff carries a row's own `bounds` — the pixels a page's text, a boundary widening by the playbook, at that chunk's wrap brought to the founder's answer card on his recorded word (the report's inputs#I4), the `bounds` a closed set inside the ratified crossing; what a client does with them, a model provider included, is the client's; the cold-agent run over the list belongs to `v030-15` and is unread here (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Threat Model Summary, MCP stdio: an open select list's painted labels in an image block recorded as a boundary widening at P2's halt, and a row's bounds as a closed set inside the ratified crossing (2026-10-31-select-popup)."
    rationale: "Report Schema / config says the list crosses the ratified carriers as a row's `bounds` and pixels in a `screenshot`; Coverage, `the list's pixels on the screenshot carriers`, marks it «boundary widening, P2's halt», the MCP image block being one such carrier (this file's line 36); the P5 list says `v030-15` owns the cold-agent run, the list's effect unread here."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:37
note: "Three things outside these proposals: the widening clause in the near-27 and near-37 changes is worded as pending, so P2's answer card should settle its final wording; the panic's own row is §Error Handling's with its owner at P5, this file only pointing to it; D-security-input, D-security-auth, D-security-deps and D-retired found nothing in this file (no dependency or auth change, no retired item stated here)."
