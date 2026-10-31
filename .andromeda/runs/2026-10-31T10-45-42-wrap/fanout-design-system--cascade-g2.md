proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "- In that sheet a `multiple` select lays out as rows: `select[multiple] > option` and `select[multiple] > optgroup > option` are `display: block`, `padding: 0 2px` and `white-space: nowrap`, and `select[multiple] > optgroup` is `display: block` — engine defaults, no design token (packages/blitz-dom/assets/default.css:161-188)"
    sidecar: "2026-10-31-engine-debts: desktop-native tokens gain the default sheet's rows and groups for a `multiple` select."
    rationale: "The report's Schema / config lists these default-sheet rows as added, and its Expected amendments carries them to design-system §Surface: desktop-native; the entity beside it states the same sheet's single-select layout."
    basis: packages/blitz-dom/assets/default.css:161-188
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "- In that sheet a `multiple` select's chosen row takes the engine-default pair `#c8c8c8` background and `#000000` text under `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked`, Chromium's inactive selected pair, 12.56:1 as measured by `select_interaction` — hardcoded because the engine holds no tokens (packages/blitz-dom/assets/default.css:161-188)"
    sidecar: "2026-10-31-engine-debts: desktop-native tokens gain the engine-default chosen-row pair `#c8c8c8` on `#000000` text, 12.56:1."
    rationale: "The report's Schema / config names the new engine-default chosen-row pair and its measure, its Coverage reads the pair as hardcoded by design with the engine holding no tokens, and its Expected amendments carries the pair as new text."
    basis: packages/blitz-dom/assets/default.css:161-188
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "- In that sheet a closed select's group is transparent to layout: `select:not([multiple]) > optgroup:not([hidden])` is `display: contents`, and its chosen option under `select:not([multiple]) > optgroup > option:checked` is `display: inline` — engine defaults, no design token (packages/blitz-dom/assets/default.css:161-164; packages/blitz-dom/assets/default.css:166-168)"
    sidecar: "2026-10-31-engine-debts: desktop-native tokens gain the default sheet's rules for a closed select's `optgroup` and its chosen option."
    rationale: "The report's Schema / config lists both rules as added, and they extend to a group the single-select layout this entity states, which itself still holds."
    basis: packages/blitz-dom/assets/default.css:161-168
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read; a later working-route chunk owns them (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "2026-10-31-engine-debts: desktop-native records that no select popup is painted, a group's label stays unpainted and `size` is unread, owed to a later chunk."
    rationale: "The report's F8 and F7 state no popup, no opening key and the unpainted label, Reverted names no `size` read, and the P5 card's rows 1 and 2 give them to the popup entry; beside this entity it bounds the box the select lays out as."
    basis: docs/capabilities.md:196-198
    fact: F4
note: "D-design-tokens fired nothing: no Coverage row reads `hardcoded✗`, the chosen-row pair reading hardcoded by design and Cells' outline the stand accent ✓; F5 to F8 and the pair's line in Color Palette's engine-painted defaults belong to the other group's categories."
