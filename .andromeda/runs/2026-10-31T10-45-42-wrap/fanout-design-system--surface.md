proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New list item after line 9, in group desktop-native-tokens: - Since 2026-10-31-engine-debts the same sheet lays out a select's groups and a `multiple` select's rows: under a single-choice select a group takes no box of its own and a chosen option inside it shows alone, as one directly under the select does (`select:not([multiple]) > optgroup:not([hidden]) { display: contents; }` · `select:not([multiple]) > optgroup > option:checked { display: inline; }`); under a `multiple` select each row and each group is a block with a box of its own (`select[multiple] > option, select[multiple] > optgroup > option { display: block; padding: 0 2px; white-space: nowrap; }` · `select[multiple] > optgroup { display: block; }`); `size` is unread, so a `multiple` select lists every row, a group's `label` is named in the tree and unpainted, and no popup is painted — engine defaults, no design token (packages/blitz-dom/assets/default.css:161-188; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "2026-10-31-engine-debts: Surface desktop-native tokens gain the default sheet's group and `multiple` row layout."
    rationale: "Report **Schema / config** names these default-sheet additions and **Expected amendments** carries design-system §Surface: desktop-native, the default sheet's rows and groups; **Symbols / APIs** F7 and F8 and report line 279 state the unpainted label, the absent popup and the unread `size`; line 9 of the file states only the single-choice reading, so this detail is new."
    basis: packages/blitz-dom/assets/default.css:161-188
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New list item after the sheet's rows and groups item, in group desktop-native-tokens: - Since 2026-10-31-engine-debts a chosen row of a `multiple` select takes the sheet's chosen-row pair, background `#c8c8c8` and text `#000000` (`select[multiple] > option:checked, select[multiple] > optgroup > option:checked { background-color: #c8c8c8; color: #000000; }`), Chromium's inactive selected pair, 12.56:1 as measured by `select_interaction` — an engine default hardcoded by design, the engine holding no tokens (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:691-712)"
    sidecar: "2026-10-31-engine-debts: Surface desktop-native tokens gain the engine-default chosen-row pair for a `multiple` select."
    rationale: "Report **Schema / config** adds the new engine-default chosen-row pair and **Expected amendments** states the pair is new text under design-system §Surface: desktop-native; **Coverage of new surfaces** flags its tokens as engine default, hardcoded by design with 12.56:1 measured, which D-design-tokens does not read as `hardcoded✗`."
    basis: packages/blitz-dom/assets/default.css:161-188
note: "D-design-tokens and D-retired fire on nothing here: the only hardcoded flag reads hardcoded by design, not `hardcoded✗`, the Cells outline is a stand's held accent and so not this file's, and no Retired item is stated in this file; the moved `type`, `press` and `off-screen` fixed text has no site in this file."
