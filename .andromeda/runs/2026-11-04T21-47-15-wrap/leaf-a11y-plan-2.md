@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnabled
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=28 class=omission
leaf-text: `DioxusDocument::unkeyed_actionable()` returns the ones that read a positional path, each with its `remedy()`.
source: fct-77ewqepn «the listener mark over-includes an element whose listener was removed (conservative).»
replace-with: `DioxusDocument::unkeyed_actionable()` returns the ones that read a positional path, each with its `remedy()`; its listener mark over-includes an element whose listener was removed (conservative), so such an element still needs its key.
