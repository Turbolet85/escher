proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 10
    section: "## 1. A11y Scope Summary"
    change: "Add after this row: - **Select list** — an engine overlay on every document, remote pages included, that a closed single select opens: one `BaseDocument` field `select_list`, answered first by `hit_with_scrollbar` and painted by `paint_scene` after the root's render, with SC 2.1.1, 2.4.3 and 1.4.3 asserted in the harness by `select_popup`, `select_popup_painted`, `stand_act_popup` and `stand_keyboard`'s list walk; what an assistive technology reads of it stays unmeasured and the windowed list was read and not run (packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-paint/src/render.rs:246-247) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "a11y-scope-summary: the select list recorded as a new scope entity, an engine overlay on every document, asserted for SC 2.1.1, 2.4.3 and 1.4.3 in the harness."
    rationale: "Report Coverage of new surfaces, «select list, engine overlay (every document, remote pages included)», and Outcome (a11y): SC 2.1.1, 2.4.3 and 1.4.3 met in the harness, what an assistive technology reads unmeasured."
    fact: F1
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 23
    section: "## 1. A11y Scope Summary"
    change: "The named limits read «(the focus fixup firing no `blur`, a single select's `size` unread, `aria-activedescendant` unread)», dropping «no popup painted and Enter and Space opening nothing on a single select» and «a `multiple` select's `size` unread»; the closing provenance reads «the named limits at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "a11y-scope-summary: the capability statement's named limits lose the three retired select limits; a single select's `size` stays a limit."
    rationale: "Report Retired names «no popup is painted», «Enter and Space open nothing on a single select» and «a `multiple` select's `size` is unread»; Counts rows `no-popup`, `enter-space` and `size-unread` each list a11y-scope-summary:23 changed; **`size`** keeps a single select's `size` unread."
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:23
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 10
    section: "## 2. A11y Strategy"
    change: "The marking clause reads «a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing mark it»; the exclusion reads «node creation, hover, active, scroll and layout do not, with no list open; where a list stands, a page scroll or a move that closes it marks its select through `reconcile_select_list` at the next resolve and a wheel elsewhere marks it through `close_select_list`, while a highlight or a wheel move inside the list marks nothing»."
    sidecar: "a11y-strategy: the changed set gains a select list's open and close as a new marking site, and the scroll and layout exclusion takes the standing-list exception."
    rationale: "Report Counts row `changed-set` lists a11y-strategy:10 changed; Spec claims disproved, `changed-set`: a scroll or move that closes a list and a wheel elsewhere mark the select, a highlight or wheel move inside it marks nothing, and with no list open the old rule holds."
    basis: .andromeda/a11y-plan-detail/a11y-strategy.md:10
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 29
    section: "## 2. A11y Strategy"
    change: "The broad line reads «an in-document mutation, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing mark it; node creation, hover, active, scroll and layout do not, save a scroll or move that closes a standing list, and a highlight inside the list marks nothing, so a pointer move alone reaches no refresh», its harness-settle clause unchanged."
    sidecar: "a11y-plan §2: the changed-set broad line carries a select list's open and close and the standing-list exception."
    rationale: "Report Counts row `changed-set` lists a11y-plan:29 changed, the broad line that a11y-strategy:10 details; Spec claims disproved, `changed-set`."
    basis: .andromeda/a11y-plan.md:29
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 23
    section: "## 2. A11y Strategy"
    change: "The merged read reads «`aria-expanded` (a boolean, on every element but a single select, which reads none and writes `expanded` from its list's open state)»; the rest of the row stands."
    sidecar: "a11y-strategy: upstream's `aria-expanded` read takes the single-select exception."
    rationale: "Report Symbols, Tree: a single select writes `set_expanded(open)` and reads no `aria-expanded`, every other element keeping that read. Counts row `expanded` keeps other rows as true on every other element, but this row states the read with no element scope, so it now overclaims."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 19
    section: "## 4. ARIA Patterns & Roles"
    change: "Add after this row: - **Single select's state** — a single select's pattern is `ComboBox` with `expanded` written from its list's open state, a closed bool of engine state with no `aria-expanded` read, and the chosen option `selected` (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "aria-patterns-roles: a single select's `ComboBox` pattern recorded with `expanded` from the list's open state and the chosen option `selected`."
    rationale: "Report Coverage of new surfaces, «tree expanded on a single select»: a closed bool from engine state, no author attribute; `ComboBox`, `expanded`, chosen option `selected`."
    fact: F13
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 21
    section: "## 4. ARIA Patterns & Roles"
    change: "The limit reads «a group's label is named in the tree and painted as a header on an open list», and the row's provenance reads «as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "aria-patterns-roles: «a group's label is unpainted» retired; the label is painted as a header on an open list."
    rationale: "Report Retired names «a group's label is unpainted»; Counts row `group-label` lists aria-patterns-roles:21 changed."
    basis: .andromeda/a11y-plan-detail/aria-patterns-roles.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 21
    section: "## 4. ARIA Patterns & Roles"
    change: "Add after this row: - **Option group's header row** — in an open list a group's header row chooses nothing, its options stand under it, and a group with no label reads the whole list's box (packages/blitz-dom/src/document.rs:995-1012; tests/blitz-tests/tests/select_popup.rs:805-855) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "aria-patterns-roles: an open list's group header row recorded, choosing nothing, and an unlabelled group reading the whole list's box."
    rationale: "Report Deviations, details chosen at implement: a group with no label reads the whole list's box; `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing` pins the header row."
    fact: F14
note: "D-a11y-surface and D-a11y-obs-schema raise nothing in this group: the report's coverage row gives the list SC 2.1.1, 2.4.3 and 1.4.3 and no log field or violation schema moved; the report's a11y-plan:55 and :64 (§3) and a11y-plan:113, :123 and :202 (§5, §7) are outside these three categories."
