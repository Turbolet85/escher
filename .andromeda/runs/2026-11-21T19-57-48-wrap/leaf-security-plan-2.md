@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: its `expanded` the closed bool of the engine's own state
source: dec-i4odk2pc «An author's `open` on a select and `aria-expanded` on a single select steer nothing: a drop-down's expanded state is the engine's own closed state, a single list box states none, and `open` on a select does not match `:open`.»
replace-with: a drop-down's `expanded` the closed bool of the engine's own state and a single list box stating none
@leaf_edit n=2 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: Enter opens its list and a Space with no typed text standing opens it
source: dec-lkdzy7gi «Enter and Space open a drop-down's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.»
replace-with: on a drop-down Enter opens its list and a Space with no typed text standing opens it
@leaf_edit n=3 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: and on a single select opens its list or closes one standing
source: dec-i3d6xc6s «Only a drop-down, a single select of display size 1, opens a list, and a disabled, inert or hidden one, one with no row to show, and one under a non-invertible transform open none.»
replace-with: and on a drop-down opens its list or closes one standing
@leaf_edit n=4 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: its stated limits: an option of a closed single select has no click arm and stays refused `covered`, under a group too, and a single select's `size` is not read.
source: dec-hsllrojk «A single select of display size above 1, a list box, falls back to no option and may stand with nothing chosen; an option of a closed drop-down has no click arm and stays refused covered.»
replace-with: its stated limits: a single select of display size above 1, a list box, falls back to no option and may stand with nothing chosen, and an option of a closed drop-down has no click arm and stays refused `covered`, under a group too.
@leaf_edit n=5 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: a single select's list is engine-painted
source: dec-lkdzy7gi «Enter and Space open a drop-down's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.»
replace-with: a drop-down's list is engine-painted
@leaf_edit n=6 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: a disabled, inert or hidden select or one with no row to show opens none
source: dec-i3d6xc6s «Only a drop-down, a single select of display size 1, opens a list, and a disabled, inert or hidden one, one with no row to show, and one under a non-invertible transform open none.»
replace-with: only a drop-down, a single select of display size 1, opens a list, and a disabled, inert or hidden one or one with no row to show opens none
@leaf_edit n=7 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: as does the select's removal or its turning disabled, hidden or inert
source: dec-fzaablnf «A select removed, disabled, hidden, made inert or made a list box with its list open closes the list with no panic, and a dropped option is no row.»
replace-with: as does the select's removal or its turning disabled, hidden, inert or a list box
@leaf_edit n=8 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: size` on a `multiple` select is read by
source: dec-llddfqft «The `size` attribute on a select is read as a hint capped at 65534, with `0` or a non-numeric value giving no hint.»
replace-with: size` on a select is read by
