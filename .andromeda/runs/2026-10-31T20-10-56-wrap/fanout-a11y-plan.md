proposals:
  - detector: D-retired
    severity: warning
    section: "§5 Keyboard Navigation → Keyboard handling"
    change: "The single-choice `select` bullet at a11y-plan:123 should say: a focused single-choice `select` that is no disabled form control, its list closed, takes ArrowDown / ArrowUp to the next / previous enabled option, Home / End to the first / last, and a character with no Ctrl, Alt or Meta as typeahead kept for 1 s of the document's clock; Enter, and a Space with no typed text standing, open its list on the chosen option and fire nothing, a disabled, inert, hidden or empty select opening nothing; every other key does nothing; each change dispatches one `input` then one `change`, a key that changes nothing none; with the list open the engine paints it over the page and focus stays on the select, the list's keys move the highlight over enabled rows and choose nothing, Enter and Space choose the highlighted row and close, Tab in either direction chooses and closes before focus moves, Escape closes with the value unchanged, and a close clears the typed text; an `optgroup`'s options are the select's own in tree order, and a `multiple` select takes its own keys and row clicks. The words `no popup is painted` end."
    sidecar: "§5 Keyboard handling: the limits «no popup is painted» and «Enter and Space open nothing on a single select» end; the bullet states the open list's keys."
    rationale: "Report Retired names both limits; Counts `no-popup` lists a11y-plan:123 as changed; Symbols / APIs → Keys states `press_select_key`, `press_open_list_key` and `choose_current_and_close`, and Deviations states that a close clears the typed text."
    basis: "packages/blitz-dom/src/events/keyboard.rs:215-250"
  - detector: D-retired
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output"
    change: "At a11y-plan:202 replace `named in the tree and unpainted` with `named in the tree and painted as a header of an open list, heavier than a row by weight on every build, a header choosing nothing`; the bullet's other clauses and its closing «what an assistive technology reads of them is not measured» stand."
    sidecar: "§7 Accessibility tree output: «a group's label is unpainted» ends; an open list paints it as a header that chooses nothing."
    rationale: "Report Retired names «a group's label is unpainted»; Counts `group-label` lists a11y-plan:202 as changed; Symbols / APIs → Paint and Deviations (the operator's direction on a header heavier than a row on every build) state the header."
    basis: "tests/blitz-tests/tests/select_popup.rs:805-855"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Focus restoration"
    change: "In the pointer-click bullet at a11y-plan:113 replace `a click on a select focuses it` with `a click on a single select focuses it and opens its list, or closes one standing, each of two clicks on one point focusing; a click on a row of an open list chooses it, a disabled row changing nothing and leaving the list open and a group header choosing nothing; a press outside the list closes it with no event; a label's click on a bound single select focuses it and opens nothing; a click on a multiple select focuses it`; `a click on a multiple select's row focuses the select and acts on the row` and `an option of a closed single select takes no click` stand."
    sidecar: "§5 Focus restoration: a click on a single select opens or closes its list; a row's, a header's, an outside press's and a label's click stated."
    rationale: "Report Counts `click-help` lists a11y-plan:113 as changed; Symbols / APIs → Pointer states the `select`, `option`, `optgroup` and `label` arms and `close_list_outside`; Outcome (arch) reads two clicks open then close, each focusing; `closed-option` (F9) keeps a closed select's option taking no click."
    basis: "packages/blitz-dom/src/events/pointer.rs:415-420"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Test harness pattern"
    change: "Add a bullet: the select list's keyboard and focus coverage is headless — `select_popup`, `stand_act_popup` and two `stand_keyboard` tests, in both layout modes, assert SC 2.1.1 and 2.4.3 on it, the flight type walk holding with its list open and after each close and a click leaving focus on the select with the walk going on from it; `stand_keyboard` runs in the `a11y` leg, which stays fourteen files, `select_popup` and `stand_act_popup` in `stand-repeat`; the windowed list was read and not run."
    sidecar: "§5 Test harness pattern: the open list's SC 2.1.1 and 2.4.3 coverage and its checks recorded."
    rationale: "D-a11y-surface: the report adds an interactive element and §5's harness names no check of it; Coverage of new surfaces (`select list`) and Outcome (a11y, layouts) state the assertions and that the windowed list was read and not run; Harness / gate surface states the legs."
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:1082-1111"
  - detector: D-a11y-surface
    severity: warning
    section: "§6 Visual Design Verification → Color contrast"
    change: "Add a bullet: an open select list's colours are engine constants on `SelectListView`, no sheet's — highlight `#000000` on `#c8c8c8`, disabled text `#6d6d6d`, border `#999999`; `select_popup_painted` reads the highlight and a disabled row as painted against those constants and measures the plain pair at the default sheet and at the stand, so SC 1.4.3 is asserted on the list in the harness; a highlighted row's contrast is unread by the lint and a closed select's own pair stays unread; the list opens and closes with no motion."
    sidecar: "§6 Color contrast: the open list's constants, its measured pair and its two unread pairs recorded."
    rationale: "D-a11y-surface: §6 states nothing of the new element; Symbols / APIs lists the constants, Coverage of new surfaces asserts SC 1.4.3, Counts `ua-pair` states the plain pair measured and the closed pair unread, inputs#I7 names the lint limit, and Outcome (design) states no motion."
    basis: "tests/blitz-tests/tests/select_popup_painted.rs:161-187"
  - detector: D-a11y-surface
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output"
    change: "Add a bullet: a single select reads `ComboBox` and writes `expanded` from its list's open state, reading no `aria-expanded`, and an author's `open` or `aria-expanded` on it steers nothing; the tree's focus stays on the select and its chosen option reads `selected`; the highlighted row is in no snapshot and no tree; the snapshot gains no `expanded` key, an open list's row reading its own `bounds` in the list; what an assistive technology reads of the list is not measured."
    sidecar: "§7 Accessibility tree output: a single select's native `expanded`, the focus staying on it, the unread highlight and a row's bounds recorded."
    rationale: "D-a11y-surface: §7 states no tree reading of the new element; Symbols / APIs → Tree and Bridge, Schema / config (a row's own `bounds`), Coverage of new surfaces (`tree expanded on a single select`) and inputs#I6 (no `expanded` node key) state it, the assistive reading `unrunnable-here`."
    basis: "packages/blitz-dom/src/accessibility.rs:238-242"
  - detector: D-a11y-surface
    severity: warning
    section: "§2 A11y Strategy → Accessibility tree lifecycle"
    change: "In the `changed_nodes` bullet at a11y-plan:29, after `a select's changed choice mark it`, add `and a select list's opening or closing marks its select — a page scroll or a box move that closes a list through `reconcile_select_list` at the next resolve, a wheel elsewhere through `close_select_list``, and qualify the exclusion to read `node creation, hover, active, scroll and layout do not where no list stands, nor does a highlight or a wheel move inside a list, so a pointer move alone reaches no refresh`."
    sidecar: "§2 Accessibility tree lifecycle: opening and closing a select list mark the changed set; a scroll or a layout move that closes one is the exception to the exclusion."
    rationale: "Report Counts `changed-set` lists a11y-plan:29 as changed; Spec claims disproved → `changed-set` states the closing paths, that with no list open the exclusion holds, and that a highlight or a wheel move inside the list marks nothing."
    basis: "packages/blitz-dom/src/document.rs:891-907"
  - detector: D-a11y-surface
    severity: warning
    section: "§3 A11y Assertion Harness Contract"
    change: "In the `accessibility_merged_states` bullet at a11y-plan:55 replace `the merged aria-expanded (true and false and nothing else)` with `the merged aria-expanded (true and false and nothing else) on every element but a single select, which writes its list's open state and reads no aria-expanded, `select_popup` asserting that an author's attribute steers nothing`."
    sidecar: "§3 harness: the merged `aria-expanded` read excepts a single select, its native `expanded` asserted by `select_popup`."
    rationale: "Report Counts `expanded` lists a11y-plan:55 as changed; Symbols / APIs → Tree states that a single select reads no `aria-expanded` and every other element keeps that read."
    basis: "tests/blitz-tests/tests/select_popup.rs:628-651"
  - detector: D-a11y-surface
    severity: warning
    section: "§2 A11y Strategy → Role derivation (semantic HTML and ARIA)"
    change: "In the bullet naming upstream's six reads, qualify `aria-expanded` to read `aria-expanded (on every element but a single select, which writes its list's open state instead)`; the rest of the bullet stands."
    sidecar: "§2 Role derivation: the `aria-expanded` read excepts a single select."
    rationale: "The sweep for the claim the §3 change retires: this bullet states the `aria-expanded` read with no exception, and Symbols / APIs → Tree removes it from a single select."
    basis: "packages/blitz-dom/src/accessibility.rs:238-242"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§3 A11y Assertion Harness Contract → Contrast verification harness"
    change: "At a11y-plan:64 change `18 `STATE_ROWS`` to `20 `STATE_ROWS``; the rest of the passage stands."
    sidecar: "§3 Contrast verification harness: `STATE_ROWS` 18 → 20."
    rationale: "Report Counts `state-rows` states `STATE_ROWS` 18 → 20 and lists a11y-plan:64 as changed; §3 states the count as the current reading of the coverage §6 relies on."
    basis: ".andromeda/a11y-plan.md:64"
note: "D-a11y-obs-schema holds: the report's Schema / config moves no log field or span and neither schema changed. The `ua-pair` wording «A closed select's contrast pair is unmeasured» has no hit in the body; the `size`, `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select` retirements also have none, so any such site in the detail is the cascade's."
