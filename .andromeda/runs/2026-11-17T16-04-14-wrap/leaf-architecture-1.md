@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dire
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/stack
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/servi
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/servi
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/servi
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: `scroll_to` and `scroll_by` clamping between its scroll minimum and maximum;
source: dec-iue6vazk «After layout rounding every scroll container's offset is clamped to its scroll range, the viewport and a running scroll animation are clamped too, and a removed subtree's scroll state is reset»
replace-with: `scroll_to` and `scroll_by` clamping between its scroll minimum and maximum, and after layout rounding every scroll container's offset, the viewport and a running scroll animation clamped to their scroll range and a removed subtree's scroll state reset, so a list whose content shrinks after a delete reads offset 0 (an upstream behaviour change taken);
