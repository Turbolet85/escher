proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 69
    section: "## 5. Keyboard Navigation"
    change: "- A key press with focus on a control whose anonymous block was dropped no longer panics, for `hidden`, `style=\"display:none\"` and CSS alone, in both layout modes: `BaseDocument::deallocate_anonymous_block` sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-134; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "keyboard-navigation: added that a key press on a control whose anonymous block was dropped no longer panics, in both layout modes"
    rationale: "report.md:23 states the fix and its three ways, and report.md:139 reads it met in both modes; no entity of this group states it"
    basis: packages/blitz-dom/src/document.rs:1493-1503
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 69
    section: "## 5. Keyboard Navigation"
    change: "- The limit that stays: a focused control that no longer shows keeps focus and takes keys, a select's choice still moving past its dropped block; the fixup reads `is_inert()` alone, on the founder's recorded choice (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "keyboard-navigation: recorded the limit that stays, a focused control that no longer shows keeps focus and takes keys"
    rationale: "report.md:23 has focus stay and the key still act with `DocumentMutator::flush` unedited, report.md:85 and :140 name the limit, and report.md:58 reads it on no master row"
    basis: tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 69
    section: "## 5. Keyboard Navigation"
    change: "- The limit that a focused control that no longer shows keeps focus and takes keys is owned by a line in `.andromeda/residuals.md`, a line in `escher-0.4.0-incubator/deferred-from-0.3.0.md` and a `CARRY:` on the entry A11y re-read (`chk-ajxm4sin`), its SC 2.4.3 re-assertion on the stand and served pages — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "keyboard-navigation: named the owners of the focused-control limit, the residuals line, the deferred line and the A11y re-read carry"
    rationale: "report.md:81 gives the card's row and its owners on the operator's word, and report.md:85 names that row as the limit's owner"
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:81
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 129
    section: "## 5. Keyboard Navigation"
    change: "- `key_press_after_container_lost_its_box` (3 tests) pins, in both layout modes, a key on a button past its dropped anonymous block, a key on a select moving its choice past it, and a control in no anonymous block reading the same; it has no `stand_` prefix and is in neither the `a11y` nor the `stand-repeat` leg (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-134; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:162-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "keyboard-navigation: added the check `key_press_after_container_lost_its_box`, 3 tests in both layout modes, in neither the `a11y` nor the `stand-repeat` leg"
    rationale: "report.md:27 states the new check, its count and its prefix, and report.md:60 has it join no leg, with `a11y` 14 · 114 · 0 · 0 and `stand-repeat`'s seventeen targets unmoved"
    basis: tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187
    fact: F7
note: "The three proposals at line 69 read in the order F4, F5, F6; report.md:58 also owes the limit a new sentence in a11y-plan §5, which is the body's broad line and no proposal here; D-a11y-surface and D-a11y-obs-schema do not fire, the chunk adding no interactive UI element (report.md:101-109) and moving no violation schema (report.md:31)."
