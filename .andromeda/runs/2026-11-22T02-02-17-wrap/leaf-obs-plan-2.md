@leaf_group group=obs-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/rules/observability.md line=22 class=omission
leaf-text: embedders drain JS errors with `take_js_errors` (≤256 retained between drains).
source: ctr-7tmk6vzc «Embedders drain JS messages with take_messages.»
replace-with: embedders drain JS errors with `take_js_errors` (≤256 retained between drains) and JS messages with `take_messages`.
@leaf_edit n=2 leaf=.claude/rules/observability.md line=23 class=omission
leaf-text: and chains the previous hook.
source: dec-5qungz5i «WASM builds install console_error_panic_hook, a workspace dependency, as their panic hook.»
replace-with: and chains the previous hook; WASM builds install `console_error_panic_hook` (a workspace dependency) as their panic hook.
@leaf_edit n=3 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: `SelectListRow.label` holds page text in a public read type and is never logged;
source: dec-55k4uacy «The buffered render's mutation buffer holds page content in process memory only from a render to its replay, its Debug printing its length alone, and nothing logs, fields or prints its contents.»
replace-with: The buffered render's mutation buffer holds page content in process memory only from a render to its replay, its `Debug` printing its length alone, and nothing logs, fields or prints its contents. `SelectListRow.label` holds page text in a public read type and is never logged;
