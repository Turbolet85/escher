proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 10
    section: "## 2. A11y Strategy"
    change: "In the changed-set's first sentence, after «a select's changed choice mark it» and its `apply_select_choice` aside, add «and opening or closing a single select's list marks that select», and restate «node creation, hover, active, scroll and layout do not» as «node creation, hover, active, scroll and layout do not while no list stands; where a list stands, a page scroll or a move of the select's box that closes it marks its select through `reconcile_select_list` at the next resolve and a wheel elsewhere marks it through `close_select_list`, while a highlight or a wheel move inside the list marks nothing — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»; and in the next sentence add a list's open and close to what reaches the refresh above."
    sidecar: "a11y-strategy §2: the changed set gains a new kind of site, a single select's list opening or closing marks the select, and «scroll and layout do not» is limited to no list standing (2026-10-31-select-popup)."
    rationale: "Report, Counts / qualifiers moved, `changed-set` names a11y-strategy:10 as changed (open and close mark the select); Spec claims disproved, `changed-set` states the page-scroll, move and wheel marks and that a highlight or a wheel move inside the list marks nothing."
    basis: .andromeda/a11y-plan-detail/a11y-strategy.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 24
    section: "## 2. A11y Strategy"
    change: "Add a bullet after this item: «- A single select's `expanded` is its list's state: the builder writes `set_expanded(open)` on a single select and reads no `aria-expanded` there, the one carve-out from the `aria-expanded` read above, every other element keeping that read; `:open` and `flush_open_state` are unmoved, so neither an author's `aria-expanded` nor an author's `open` steers it (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:628-651); the tree reads the list open on the select, its focus staying there, a `ComboBox` with its chosen option `selected` and the highlight unread (tests/blitz-tests/tests/select_popup.rs:565-614); the snapshot gains no key for it, `NODE_FIELDS` staying thirteen, and what an assistive technology reads of it is not measured — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "a11y-strategy §2: new item, a single select writes `expanded` from its list's state and reads no `aria-expanded`, every other element keeping the read (2026-10-31-select-popup)."
    rationale: "Report, Symbols / APIs, Tree (`accessibility.rs:238-242`) and Bridge (`NodeState` gains no field), and Coverage of new surfaces, `tree expanded on a single select`; line 23 states the `aria-expanded` read with no carve-out, and the pre-pass's `expanded` no-change holds only for every other element, so the fact takes its own item, line 23 kept as upstream's reads."
    basis: .andromeda/a11y-plan-detail/a11y-strategy.md:23
note: "No retired item stands in this file; D-a11y-surface and D-a11y-obs-schema fire nothing here (no log field or schema moved; the list's §5–§7 coverage lies in those categories); the body's a11y-plan:29 and :37 carry the same two claims and are the body pass's, outside this file."
