proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 54
    section: "## Conventions"
    change: "In the list of checks that hold a `Session` and read `mod session_common;`, «the fifteen driver-action checks `stand_act_*`» becomes «the sixteen driver-action checks `stand_act_*`»; nothing else on the line moves."
    sidecar: "conventions: the driver-action checks `stand_act_*` are sixteen, `stand_act_popup` added by 2026-10-31-select-popup."
    rationale: "The report's Counts / qualifiers moved, `act-files`, reads `stand_act_*` files fifteen to sixteen (`git ls-tree`: 15 to 16) and lists conventions:54 among its changed sites."
    basis: .andromeda/architecture-detail/conventions.md:54
note: "No New facts item names conventions, so this group places no F{n}; the new modules on conv-visibility-and-naming's rule are F27's in existing-scopes and the public read types F12's in standard-contracts."
