proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 10
    section: "## Threat Model Summary"
    change: "At the end of the Trust boundary, add: Since 2026-10-31-select-popup a page's markup, a remote page's included, also reaches the engine-painted select list on every document: its option and group labels as the list's rows, and `size` on a `multiple` select as a capped whole number, each held under Input Validation. A remote page can also reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`), listed under Error Handling's panic paths (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "threat-model-summary boundary-remote-content: a remote page reaches the select list, its `size` read and the key-press panic."
    rationale: "Report, Coverage of new surfaces: `select list, engine overlay (every document, remote pages included)`, and `size on a multiple select` validated by `parse_unsigned_integer` capped at 65534; Spec claims disproved, `panic-pos`: «A remote page can reach it». D-security-input does not fire: the report states the validation of each surface."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 27
    section: "## Threat Model Summary"
    change: "Add a continuation line at the end of the Trust boundary: Since 2026-10-31-select-popup an answer's `screenshot` of an open list, on the socket and MCP carriers alike, also takes a page's option labels and group `label`s out as pixels; the shot's fields are unmoved and `cli_commands` pins a hosted shot equal to an in-process one; a boundary widening, put to the founder on his answer card at that chunk's wrap on his recorded word (inputs#I4) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "threat-model-summary boundary-session-socket: option and group labels leave as pixels in every shot of an open list, a widening on the founder's answer card."
    rationale: "Report, Schema / config: «New pixel content in every shot of an open list: an option's label and a group's `label`, painted»; Coverage of new surfaces, `the list's pixels on the screenshot carriers`: **boundary widening, P2's halt**; Expected amendments, Playbook *Boundary widening*."
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 27
    section: "## Threat Model Summary"
    change: "Add a continuation line at the end of the Trust boundary: The select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff (`differs` naming `bounds`, a closed set inside the ratified crossing) and as pixels in a shot; `NodeState` gains no field, `NODE_FIELDS` stays thirteen, and there is no `expanded` node key, on the operator's word (inputs#I6) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "threat-model-summary boundary-session-socket: the select list crosses only as a row's `bounds` and a shot's pixels; no `expanded` node key."
    rationale: "Report, Schema / config: «The list crosses the ratified carriers as: a row's own `bounds` in the snapshot and the diff ... and pixels in a `screenshot`»; Symbols / APIs, Bridge: `NodeState` gains no field, `NODE_FIELDS` stays thirteen; Decisions & corrections: no `expanded` node key, inputs#I6."
    fact: F14
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 37
    section: "## Threat Model Summary"
    change: "Add a continuation line at the end of the Trust boundary: Since 2026-10-31-select-popup a result's `image` block of an open list also holds a page's option labels and group `label`s as pixels, and a snapshot or diff a row's own `bounds`, the crossing the boundary-session-socket group states; what a client does with them, a model provider included, is the client's (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "threat-model-summary boundary-mcp-stdio: the MCP carrier's occurrence of the select list's crossing."
    rationale: "Report, Schema / config and Coverage of new surfaces, `the list's pixels on the screenshot carriers`: the crossing is on every ratified carrier, and this entity states the MCP carrier's carried set on its own."
    dependent-of: D-new-detail
note: "No detector fires in this group: the report states each new surface's validation, no auth, session, token or key surface moved, and Dependencies are none. F13's «put to the founder» wording awaits his answer at P2's halt (inputs#I4, CARRY cry-musoudbr). The panic's detail (F1) is error-handling's, its owner P5's; the widening rule row and the select rows are input-validation's."
