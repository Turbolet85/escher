proposals:
  - detector: D-cascade
    severity: escalate
    file: .andromeda/security-plan-detail/input-validation.md
    near: 38
    section: "## Input Validation"
    change: "In the screenshot clause, after a password input painted masked, add: since 2026-10-31-select-popup every shot of an open select list also holds, as pixels, an option's label and a group's `label`, the shot's fields unmoved and a hosted shot pinned equal to an in-process one by `cli_commands`; page text in the pixels on this carrier, a boundary widening put to the founder's answer card at that chunk's wrap on his recorded word (inputs#I4, CARRY `cry-musoudbr`), its ratification written only from his answer."
    sidecar: "input-validation, Driver session socket: a shot of an open list holds option and group labels as pixels, a boundary widening for the founder's answer card."
    rationale: "Report **Schema / config** names the new pixel content in every shot of an open list, **Coverage of new surfaces** marks it boundary widening, P2's halt, and **For the later phases** has validate halt once for his answer card citing inputs#I4."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 39
    section: "## Input Validation"
    change: "After the clause on a closed value set inside the ratified carrier, add: since 2026-10-31-select-popup an open list's row reads its own `bounds` in the snapshot and the diff, `differs` naming `bounds`, a closed set inside the ratified crossing and so no widening under the same rule; `NodeState` gains no field and `NODE_FIELDS` stays thirteen."
    sidecar: "input-validation, Driver session socket: an open list's row crosses as its own `bounds`, inside the ratified crossing."
    rationale: "Report **Schema / config**: the list crosses the ratified carriers as a row's own `bounds` in the snapshot and the diff; report **Bridge**: `NodeState` gains no field, `NODE_FIELDS` stays thirteen."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 39
    section: "## Input Validation"
    change: "Add a row after the Driver session socket row: **Boundary:** Driver session socket (escher-driver), an open select list · **What to validate:** The highlighted row · **How (stack-specific):** The highlighted row crosses no structured carrier: it is in no snapshot and no tree, and the lint does not read its contrast — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: the highlighted row crosses no structured carrier."
    rationale: "Report **Tree** and **Coverage of new surfaces**: the highlight is unread by the tree; report inputs#I7 lists the highlight absent from the snapshot and the tree and a highlighted row's contrast unread by the lint."
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 173
    section: "## Input Validation"
    change: "Add a row after the `rowspan` row: **Boundary:** Markup numeric attributes · **What to validate:** `size` on a `multiple` select · **How (stack-specific):** Parsed by `size_rows` through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value giving no hint; a valid value hints `height` of `size` × 1.2em and carries no text out; a single select's `size` stays unread (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/node/select_list.rs:459-475; tests/blitz-tests/tests/select_popup.rs:995-1032) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation, Markup numeric attributes: `size` on a `multiple` select, a capped parse carrying no text."
    rationale: "Report **`size`** and **Coverage of new surfaces**: `size on a multiple select`, validated by `parse_unsigned_integer`, capped at 65534, `0` and non-numeric read none. D-security-input holds: the new read is validated."
    basis: packages/blitz-dom/src/stylo.rs:1124-1139
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 133
    section: "## Input Validation"
    change: "Markup numeric attributes — parsed with default fallbacks; `rowspan` clamped to 1..=65534, `<ol start>` parsed as `usize`, `size` on a `multiple` select capped at 65534, `0` or a non-numeric value giving no hint."
    sidecar: "security-plan body, Markup numeric attributes: `size` joins the broad line."
    rationale: "The body's broad line for the Markup numeric attributes group, which gains F8's row; report **`size`**."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 183
    section: "## Input Validation"
    change: "Add a row after the `disabled` row: **Boundary:** Markup attributes · **What to validate:** A select's and a row's state when a list would open or choose · **How (stack-specific):** A disabled, inert or hidden select and a select with no row to show open no list; a disabled row changes nothing and leaves the list open; a group header chooses nothing; a non-invertible transform is held by the same check set (tests/blitz-tests/tests/select_popup.rs:443-472; tests/blitz-tests/tests/select_popup.rs:377-388; tests/blitz-tests/tests/select_popup.rs:805-855) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: a disabled, inert or hidden select opens no list; a disabled row and a group header choose nothing."
    rationale: "Report **Outcome**, the (security) acceptance on disabled, inert, disabled option and non-invertible transform, and **Deviations from intent**: a select with no row to show opens nothing."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 186
    section: "## Input Validation"
    change: "Add a row after the `open` row: **Boundary:** Markup attributes · **What to validate:** `open` and `aria-expanded` on a select · **How (stack-specific):** An author's `open` or `aria-expanded` steers nothing on a select: `:open` and `flush_open_state` are unmoved, and a single select writes its tree `expanded` from the engine's own state (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:628-651) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: an author's `open` or `aria-expanded` steers nothing on a select."
    rationale: "Report **Tree**: a single select writes `set_expanded(open)` and reads no `aria-expanded`; `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing."
    basis: packages/blitz-dom/src/accessibility.rs:238-242
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 210
    section: "## Input Validation"
    change: "Replace «and a click on a select's label re-dispatches to the select» with: and a click on a select's label re-dispatches to the select, but for a bound single select, whose label's click since 2026-10-31-select-popup focuses the select and opens nothing, asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing` (tests/blitz-tests/tests/select_popup.rs:428-441)."
    sidecar: "input-validation, `aria-label` · `<label for>`: a single select's label click focuses it and opens nothing, no re-dispatch."
    rationale: "Report `label-click` changes input-validation:210; report **Pointer**: the `label` arm focuses a bound single select and opens nothing; **Spec claims disproved**: the single-select half is now asserted and no longer re-dispatches."
    basis: tests/blitz-tests/tests/select_popup.rs:428-441
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 135
    section: "## Input Validation"
    change: "In the Markup attributes line, after «Enter activating buttons and links and Space buttons as a click», add: and each opening a single select's list, Space only with no typed text standing, a `multiple` select's `size` read through a capped parse; and after «names and descriptions as page text», add: an option's and a group's label also painted in an open list."
    sidecar: "security-plan body, Markup attributes: Enter and Space open a single select's list; `size` is read; option and group labels are painted."
    rationale: "Report **Keys**: `press_select_key` opens on Enter and on a Space with no typed text standing; **`size`**; **Schema / config**: an option's label and a group's `label`, painted. The line's broad reading of the Markup attributes group is otherwise stale; the labels' widening stands on the socket row's proposal."
note: "Input-validation lines the report changes but this group's picks do not hold (219, 248, 249, 256, 261, 262, 264, 265) got no proposal here and need their owner; F4, F6, F7, F9 and F12 have no picked entity to carry them and F13, F14 are threat-model-summary's, all for the new-detail pass or their group; D-security-input, D-security-auth and D-security-deps fired on nothing; the pixel proposal is the P2 widening halt, to be asked as an open question and not read as ratified."
