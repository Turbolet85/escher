@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=28 class=omission
leaf-text: under the same feature `DioxusDocument::unkeyed_actionable()` returns the ones that read a positional path, each with its `remedy()`;
source: fct-77ewqepn «`unkeyed_actionable` and `DioxusDocument::actionable_nodes` read one actionable rule, its listener leg the vdom walk's exact current listeners holding an event other than `mounted`»
replace-with: under the same feature `DioxusDocument::unkeyed_actionable()` returns the ones that read a positional path, each with its `remedy()`, and `DioxusDocument::actionable_nodes()` lists the actionable nodes (`ActionableNode`: `id`, `owner`, `keyed`); both read one actionable rule (the crate-private `Readings`), its listener leg the vdom walk's exact current listeners holding an event other than `mounted`, so an element whose listener a re-render removed is read by neither, the `data-dioxus-id` mark serving event dispatch alone;
