# Cascade compare — a11y-plan · phase · 2026-10-24-a11y-re-read
E items 119 · held 35 · lost at classify 2 · lost at strong 1 · beside a placed fact - · cascade only 11 · body 44 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### a11y-scope-summary#1.9 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:16
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:16) The chunk adds no crate or dependency, so the stand's graph must still hold no platform adapter after it.
  > neither builds a platform adapter — `accesskit_xplat` and `accesskit_winit` are in neither graph, `accesskit` was already in both through blitz-dom, and the lockfile did not change at that chunk — so no assistive technology is given a tree by the stand binary still

### a11y-scope-summary#1.9 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:17
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:17) The headless halves (the shell's tree carrying stable ids, the refresh) are what `stand_accessibility_ids` in the a11y leg re-asserts; the adapter and its windowed witness remain the residuals line's, not this chunk's.
  > the adapter on the stand binary and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next), the headless halves, the tree the shell hands the adapter carrying stable ids only and the refresh, proven on the stand at 2026-10-14-stand-a11y-assertions (§2, §7)

### a11y-scope-summary#1.9 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:18
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:18) The latest re-read of the stand graph, after 0.2.0's last engine take, is the state the chunk must leave unchanged — no adapter on any stand binary.
  > re-read on the upstream take read at `f9cbb2756` and `tokio` 1.54.1: no package enters or leaves `Cargo.lock`, and the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still

### a11y-strategy#1.2 · lost at strong · .andromeda/a11y-plan-detail/a11y-strategy.md:10
- (.andromeda/a11y-plan-detail/a11y-strategy.md:10) Any new row that reads the changed set (a select's keyboard move, a Tab step) must match what marks it — a select's changed choice marks it, a choice of the chosen option marks nothing.
  > a mutation of an in-document node, a focus or checked change, a text control's input and a select's changed choice mark it — `apply_select_choice` marking nothing for a choice of the chosen option (packages/blitz-dom/src/events/keyboard.rs:271-297); node creation, hover, active, scroll and layout do not; `has_changes()` reads true while it is non-empty.

### a11y-strategy#1.2 · lost at strong · .andromeda/a11y-plan-detail/a11y-strategy.md:10
- (.andromeda/a11y-plan-detail/a11y-strategy.md:10) A re-read or new row that reads `has_changes()` as a step's change must drain the set first, since a settle neither reads nor drains it and a fresh document's set is non-empty.
  > A harness settle (`Harness::settle`, the driver session's `act`) neither reads nor drains the set, so a step settled through it leaves the refresh above, and a later diff, exactly what the step marked; and the set is non-empty on a freshly booted harness document — the boot's own in-document mutations mark it and nothing has drained it — so a check that reads `has_changes()` as "this step changed something" drains first (packages/blitz-test-harness/src/settle.rs:129-191; tests/blitz-tests/tests/stand_settle.rs:78-119; as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md and its evidence/red-first.md)

### screen-reader-support#4.1 · lost at classify · .andromeda/a11y-plan-detail/screen-reader-support.md:37
- (.andromeda/a11y-plan-detail/screen-reader-support.md:37) The CRUD Tab-order re-read must keep the list a focusable third stop whose rows are not focusable, with the actionable-key check still returning nothing.
  > since 2026-10-13-stand-keyboard-harness the list carries `tabindex="0"`, is focusable and is the third CRUD Tab stop, its rows still carrying no `tabindex` and not focusable, and the check still returns nothing — the list reads its author key

## Held

- a11y-scope-summary#1.11
- a11y-strategy#1.1
- a11y-strategy#2.2
- a11y-strategy#2.3
- a11y-strategy#2.4
- aria-patterns-roles#2.1
- aria-patterns-roles#2.2
- aria-patterns-roles#2.3
- aria-patterns-roles#2.5
- aria-patterns-roles#2.7
- aria-patterns-roles#2.9
- ci-integration#1.0
- cognitive-accessibility#1.2
- keyboard-navigation#1.1
- keyboard-navigation#1.5
- keyboard-navigation#2.3
- keyboard-navigation#2.4
- keyboard-navigation#3.3
- keyboard-navigation#3.4
- keyboard-navigation#4.9
- keyboard-navigation#5.8
- keyboard-navigation#7.1
- keyboard-navigation#7.2
- keyboard-navigation#7.3
- keyboard-navigation#7.4
- screen-reader-support#1.2
- screen-reader-support#1.3
- screen-reader-support#1.4
- screen-reader-support#2.1
- screen-reader-support#3.1
- screen-reader-support#5.1
- visual-design-verification#1.4
- visual-design-verification#1.5
- visual-design-verification#1.6
- visual-design-verification#8.1

## Cascade only

- a11y-strategy#2.1
- aria-patterns-roles#2.4
- aria-patterns-roles#2.6
- aria-patterns-roles#2.8
- keyboard-navigation#1.2
- keyboard-navigation#1.3
- keyboard-navigation#1.4
- keyboard-navigation#2.7
- keyboard-navigation#3.2
- keyboard-navigation#4.4
- screen-reader-support#6.2
