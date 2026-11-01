proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 47
    section: "## 5. Keyboard Navigation"
    change: Add after this line a new item reading - A key press with focus on a control whose anonymous block was dropped no longer panics, for `hidden`, `style="display:none"` and CSS alone, in both layout modes, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: keyboard-navigation gains the end of the key-press panic past a dropped anonymous block, for `hidden`, `style="display:none"` and CSS alone, in both layout modes.
    rationale: The report's panic fix states this at report.md:23 and its security criterion reads it met in both modes the three ways at report.md:139; no picked line states it.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 9
    section: "## 5. Keyboard Navigation"
    change: Add after this line a new item reading - The limit that stays, a focused control that no longer shows keeps focus and takes keys, a select's choice still moving past its dropped block; the fixup reads `is_inert()` alone, on the founder's recorded choice (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: keyboard-navigation gains the limit the panic's fix leaves, on the founder's word at inputs#I3, «Убрать падение, фокус остаётся (Рекомендую)».
    rationale: report.md:58 reads that this limit stands on no master row and owes a new sentence in a11y-plan §5; report.md:23 and report.md:85 state it, with `DocumentMutator::flush` unedited.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:85
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 9
    section: "## 5. Keyboard Navigation"
    change: Add after this line a new item reading - The owner of the limit that a focused control no longer showing keeps focus and takes keys is a line in `.andromeda/residuals.md`, a line in `escher-0.4.0-incubator/deferred-from-0.3.0.md` and a `CARRY:` on the entry A11y re-read (`chk-ajxm4sin`), its SC 2.4.3 re-assertion on the stand and served pages — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: keyboard-navigation names the owner of the focus limit, the residual, the deferred line and the `CARRY:` on A11y re-read (`chk-ajxm4sin`).
    rationale: The phase card's last row names this owner at report.md:81, and report.md:85 points the limit to that row.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:81
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 47
    section: "## 5. Keyboard Navigation"
    change: Add after this line a new item reading - `key_press_after_container_lost_its_box` holds 3 tests in both layout modes, a button past its dropped block, a select's choice past it and a control in no anonymous block reading the same, no `stand_` prefix, in neither the `a11y` nor the `stand-repeat` leg (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-134; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:162-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: keyboard-navigation gains the new check `key_press_after_container_lost_its_box`, 3 tests in both layout modes, in neither the `a11y` nor the `stand-repeat` leg.
    rationale: report.md:27 adds the check with no `stand_` prefix, report.md:60 reads that it joins no leg, and the report's last section lists its three tests.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 107
    section: "## 5. Keyboard Navigation"
    change: Add at the end of this broad line the sentence - The fixup reads `inert` alone, so a focused control that no longer shows, its anonymous block dropped, keeps focus and takes keys, the key press no longer panicking.
    sidecar: a11y-plan §5's focus line states the limit the panic's fix leaves, the fixup itself unmoved.
    rationale: report.md:58 reads that the limit stands on no master row and owes a new sentence in a11y-plan §5, the fixup at a11y-plan.md:107 being unmoved; this is the broad line the group's new item details.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:58
    dependent-of: D-new-detail
note: D-a11y-surface and D-a11y-obs-schema find no drift, the report adding no interactive UI element and moving no violation schema; F4 and F7 are placed beside keyboard-navigation#3.3 as no keyboard-harness entity is picked here, and §5's keyboard-harness line on `select_popup` holds by the task but sits outside this listing.
