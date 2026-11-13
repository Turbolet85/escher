@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=13 class=stale
leaf-text: its colours the engine's own constants
source: dec-gejjncmn «decision/select-list-engine-colours · Select list colours are the engine's own — The open select list's border, highlight and disabled-text colours are the engine's own constants; its background and an ordinary row's»
replace-with: its border, highlight and disabled-text colours the engine's own constants and its background and an ordinary row's text colour the select's own page styles
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: a disabled, inert or hidden select or one with no row to show opens none
source: dec-i3d6xc6s «decision/select-list-open-gate · Which select opens a list — A disabled, inert or hidden select, a select with no row to show, and one under a non-invertible transform open no list.»
replace-with: a disabled, inert or hidden select, one with no row to show or one under a non-invertible transform opens none
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: a failed, empty or non-CSS stylesheet
source: dec-iigg6kvs «decision/failed-stylesheet-blocks-no-render · Failed stylesheet blocks no render — In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render.»
replace-with: a failed, empty, non-CSS or non-UTF-8 stylesheet
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: the snapshot's `enabled` reading the same presence
source: dec-igejw4vq «decision/disabled-read-by-presence · disabled read by presence on form controls — The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer»
replace-with: the snapshot's `enabled` and `clickable` readings reading the same presence
