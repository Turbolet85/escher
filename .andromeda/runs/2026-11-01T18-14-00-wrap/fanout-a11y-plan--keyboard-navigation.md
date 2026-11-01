proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 9
    section: "## 5. Keyboard Navigation"
    change: A focused control that no longer shows keeps focus and takes keys, its box taken away by `hidden`, by `style="display:none"` or by CSS alone, the fixup in the line above reading `is_inert()` alone; a key press where the control's anonymous block was dropped no longer panics, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still in the slab naming the block, and a key on a select past its dropped block still moves its choice; the limit owned by a `.andromeda/residuals.md` line, a line in `escher-0.4.0-incubator/deferred-from-0.3.0.md` and a `CARRY:` on «A11y re-read» (`chk-ajxm4sin`), its SC 2.4.3 re-assertion on the stand and served pages (packages/blitz-dom/src/document.rs:1493-1503; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-134; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:162-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: keyboard-navigation §5 focus order gains the limit the panic's fix leaves, a focused control that no longer shows keeps focus and takes keys, with its owner from the phase card's last row (inputs#I6).
    rationale: The report says the limit stands on no master row and owes a new sentence in a11y-plan §5 (report.md:58), with the fix, focus staying and the key acting at report.md:23, its naming as the limit that stays at report.md:85 and report.md:140, and its owner at report.md:81; lines 8 and 12 stay as the report marks them no-change.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 132
    section: "## 5. Keyboard Navigation"
    change: The check `key_press_after_container_lost_its_box` (3 tests) pins, in both layout modes and for `hidden`, `style="display:none"` and CSS alone, a key on a button whose anonymous block was dropped not panicking, a key on a select past its dropped block still moving its choice and a control in no anonymous block reading the same; `select_popup`'s row 3 now takes the one `harness.press` of every row, its skip gone, both red before the fix; the new check has no `stand_` prefix and joins neither the `a11y` leg nor the `stand-repeat` list, so `run stand` is unmoved (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-134; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:162-187; tests/blitz-tests/tests/select_popup.rs:513) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: keyboard-navigation §5 harness gains `key_press_after_container_lost_its_box` (3 tests) and `select_popup`'s row 3 pressing as every row does.
    rationale: The report adds the check and removes the row-3 skip (report.md:26, report.md:27, report.md:139), asserts the select's choice past its dropped block as deviation 3 (report.md:114), and leaves the a11y tally, `run stand` and the stand-repeat targets unmoved (report.md:36, report.md:37, report.md:60).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27
note: D-a11y-surface and D-a11y-obs-schema fire nothing here (no interactive element added, report.md:102-108; no violation schema moved, report.md:31); the body's §5 broad line near .andromeda/a11y-plan.md:107 owes the limit's sentence per report.md:58, outside this file's proposals, and the limit's owner rows land at P5 per report.md:81.
