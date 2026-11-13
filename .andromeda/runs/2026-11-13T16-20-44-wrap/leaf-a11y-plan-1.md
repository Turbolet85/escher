@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=19 class=omission
leaf-text: links only with `href`;
source: dec-xnppuiyn «decision/empty-alt-image-decorative · Empty-alt image read as decorative — An img whose alt is the empty string is decorative and reads the generic container role in the accessibility tree, not image.»
replace-with: links only with `href`; an `img` whose `alt` is the empty string decorative, reading the generic container role and not image (ratified as upstream's behaviour);
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=20 class=omission
leaf-text: the `inert` read ratified by the founder, 2026-10-18).
source: dec-iinqqgma «decision/hidden-checkable-label-toggle · Native checkable state kept with no box — A native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when the control is hidden by display none.»
replace-with: the `inert` read ratified by the founder, 2026-10-18); a native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when it is hidden by `display: none` (ratified as upstream's behaviour).
