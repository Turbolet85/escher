@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=57
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: `aria-expanded` read on every element but a single select, whose `expanded` is the closed bool of the engine's own state, an author's `aria-expanded` or `open` on it steering nothing
source: dec-i4odk2pc «An author's `open` on a select and `aria-expanded` on a single select steer nothing: a drop-down's expanded state is the engine's own closed state, a single list box states none»
replace-with: `aria-expanded` read on every element but a single select, a drop-down's `expanded` the closed bool of the engine's own state and a single list box stating none, an author's `aria-expanded` or `open` on it steering nothing
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: Enter opens its list and a Space with no typed text standing opens it
source: dec-lkdzy7gi «Enter and Space open a drop-down's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.»
replace-with: Enter opens a drop-down's list and a Space with no typed text standing opens it
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: a click on a select focuses it and, on a single select, opens its list or closes one standing
source: dec-i3d6xc6s «Only a drop-down, a single select of display size 1, opens a list, and a disabled, inert or hidden one, one with no row to show, and one under a non-invertible transform open none.»
replace-with: a click on a select focuses it and, on a drop-down (a single select of display size 1), opens its list or closes one standing
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: or a click opens a single select's engine-painted list
source: dec-lkdzy7gi «Enter and Space open a drop-down's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.»
replace-with: or a click opens a drop-down's engine-painted list
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: a disabled, inert or hidden select, one with no row to show or one under a non-invertible transform opens none
source: dec-i3d6xc6s «Only a drop-down, a single select of display size 1, opens a list, and a disabled, inert or hidden one, one with no row to show, and one under a non-invertible transform open none.»
replace-with: only a drop-down, a single select of display size 1, opens a list, and a disabled, inert or hidden one, one with no row to show or one under a non-invertible transform opens none
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: a select removed, disabled, hidden or made inert with its list open closes it with no panic
source: dec-fzaablnf «A select removed, disabled, hidden, made inert or made a list box with its list open closes the list with no panic, and a dropped option is no row.»
replace-with: a select removed, disabled, hidden, made inert or made a list box with its list open closes it with no panic
@leaf_edit n=7 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: `size` on a `multiple` select is read through `parse_unsigned_integer`
source: dec-llddfqft «The `size` attribute on a select is read as a hint capped at 65534, with `0` or a non-numeric value giving no hint.»
replace-with: `size` on a select is read through `parse_unsigned_integer`
@leaf_edit n=8 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: an option of a closed single select has no click arm
source: dec-hsllrojk «A single select of display size above 1, a list box, falls back to no option and may stand with nothing chosen; an option of a closed drop-down has no click arm and stays refused covered.»
replace-with: an option of a closed drop-down has no click arm
@leaf_edit n=9 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: under a group too, and a single select's `size` is not read
source: dec-hsllrojk «A single select of display size above 1, a list box, falls back to no option and may stand with nothing chosen; an option of a closed drop-down has no click arm and stays refused covered.»
replace-with: under a group too, and a single select of display size above 1, a list box, falls back to no option and may stand with nothing chosen
