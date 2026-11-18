@leaf_group group=architecture+layout-templates
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_edit n=1 leaf=.claude/docs/services/seven_guis.md line=38 class=stale
leaf-text: a click right after a `scroll` landing on the shown overlay scrollbar
source: dec-sivea3ow «Click after scroll waits out the scrollbar fade — A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing.»
replace-with: a driver click right after a `scroll` advancing the virtual clock past the overlay scrollbar's fade first, so it is not taken as a thumb drag
