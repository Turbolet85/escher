@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=28 class=omission
leaf-text: the snapshot field `clickable` says an element reacts to a click, read from the vdom walk's current listeners and not the listener mark, which over-includes
source: fct-uz4kdss3 «and `clickable` from the vdom walk's current `click` listeners on an element that is no disabled form control»
replace-with: the snapshot field `clickable` says an element reacts to a click, read from the vdom walk's current `click` listeners on an element that is no disabled form control and not the listener mark, which over-includes
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=25 class=omission
leaf-text: so `aria-disabled` moves no snapshot reading
source: fct-2hrkecxm «A password input's value reads as the fixed `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no snapshot field, tree node or painted frame»
replace-with: so `aria-disabled` moves no snapshot reading; a password input's value reads as the fixed `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no snapshot field, tree node or painted frame, a file input with a non-empty `value` reading the same marker, and the driver refuses a `click` or a `type` as `disabled` where the snapshot's `enabled` reads `Some(false)`, nothing dispatched
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: so the stand binary still gives no assistive technology a tree).
source: fct-xy7rrkj5 «`Adapter` creation must happen before the window is first shown and panics if it is already visible»
replace-with: so the stand binary still gives no assistive technology a tree). An AccessKit `Adapter` is created before the window is first shown — it panics if the window is already visible.
@leaf_edit n=4 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: when the document's changed set taken by that poll was non-empty
source: fct-xhpyuh5z «a harness settle neither reads nor drains it and a fresh harness document's set is non-empty, so a check reading `has_changes()` as "this step changed something" drains first.»
replace-with: when the document's changed set taken by that poll was non-empty (`changed_nodes`, the nodes written since the last `take_changed_nodes` — node creation, hover, active, scroll and layout marking none; a harness settle neither reads nor drains it and a fresh harness document's set is non-empty, so a check reading `has_changes()` as "this step changed something" drains first)
