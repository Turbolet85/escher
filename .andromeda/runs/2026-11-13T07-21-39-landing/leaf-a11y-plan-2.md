@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=33 class=stale
leaf-text: and a click on any other element — a CRUD row or the list — still clears focus
source: dec-2dpilzfr «Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control.»
replace-with: a click on a checkbox or radio toggles it, dispatches `input` and moves focus to it, and a click on any other non-interactive element — a CRUD row or the list — still clears focus
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: while the `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated
source: dec-3rwivubt «the signal registry and the reached-handler record are not gated.»
replace-with: while the `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) and the reached-handler record (`record_reached`, `reached_handlers`) are not gated
