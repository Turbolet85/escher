proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 38
    section: "## Error Handling"
    change: "New row after line 38, in its own `<!-- detail: values · errors -->` block - - Key press after a focused control's anonymous block is dropped: a key press panics at `position_in_containing_block` (packages/blitz-dom/src/node/node.rs:1775) when the focused control's anonymous block was dropped because an ancestor gained `hidden`, the stack through `handle_dom_event`'s `absolute_position`, reachable by a remote page — reproduced on a `select` and a plain `button` with no list opened; `node.rs` and `events/mod.rs` unchanged against that chunk's base, no controlled run at the base taken, so «predates this chunk» rests on that and on the stack; unfixed at that chunk, `select_popup` skipping the key press in that one row, its comment saying why — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "2026-10-31-select-popup: new panic-path row, a key press after a focused control's anonymous block is dropped, remote-reachable and unfixed."
    rationale: "The report's Spec claims disproved, `panic-pos`, finds no row for this panic in this file's panic-path list and says the owed text is a new row in security-plan §Error Handling; its Outcome routes the same row to P2."
    basis: packages/blitz-dom/src/node/node.rs:1775
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "New row after line 14, in its own `<!-- detail: values · errors -->` block - - `covered`'s meaning since 2026-10-31-select-popup also says «an option of a closed select lies under it: click the select first, then the option», its name, remedy and order unchanged and the cause set still eight; a fixed `'static` text holding nothing a call supplies, with no cause, key or carrier added, pinned by `schema_tests`, `refusal` and `json` — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "2026-10-31-select-popup: `covered`'s meaning gains the closed-select clause, name, remedy, order and the cause set of eight unchanged."
    rationale: "The report's Symbols / APIs, Driver, states the new clause and Schema / config calls it the program's own `'static` text; line 9 lists what `covered`'s meaning states and lacks it. `covered-text` counts the standing readings as no-change, so this adds a row and edits none of them; a separate row keeps line 9 under the 2 000-char limit."
    basis: packages/escher-driver/src/refusal.rs:76-81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "In line 9 replace «and covers a `multiple` select: its match alone, a text opening on a space typing nothing» with «and covers any select: its match alone, a text opening on a space typing nothing»; the rest of the line stands."
    sidecar: "2026-10-31-select-popup: `type`'s help statement widened from a `multiple` select to any select."
    rationale: "The report's Symbols / APIs, Driver, says `type` on any select focuses with no click and a text opening on a space types nothing there, and that `type`'s fixed help was restated; the `multiple`-only qualifier no longer bounds it. The replacement is shorter, so line 9 cannot pass its limit."
    basis: .andromeda/security-plan-detail/error-handling.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "New row after line 14, in its own `<!-- detail: values · errors -->` block - - `click`'s, `type`'s and `press`'s help were restated at 2026-10-31-select-popup, each still a fixed `'static` text holding nothing a call supplies: a click on a select opens its list or closes one standing, on a closed select `press` reads `enter` and a space as opening it, and `type` on any select focuses with no click, so it fires no DOM focus events on a single select either; `VERBS` stays 12, causes 8 and `describe`'s count row unmoved — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "2026-10-31-select-popup: `click`'s, `type`'s and `press`'s fixed help restated for the select list, the verb and cause counts unmoved."
    rationale: "The report's Symbols / APIs, Driver and Schema / config, restate these three help texts and leave the counts unmoved; this file records the driver's help texts at line 9 and has no entry for this restatement. A separate row keeps line 9 under the 2 000-char limit."
    basis: packages/escher-driver/src/schema.rs:222-226
note: "D-security-input, D-security-auth and D-security-deps found no drift in this file (no error-handling input surface, no auth or secret change, Dependencies none); the panic's fix and owner stay with P5, and the body's §Panic paths has no broad line for the new row, which this pass does not propose because only the category file is in scope."
