@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnabled`
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=33 class=stale
leaf-text: the driver's `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver, a limit its help states, its fix owned by residual `res-hfvjj77l`.
source: ctr-ublownab «contract/driver-verbs/press · driver verbs · press — Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere.»
replace-with: the driver's `press` takes an optional `ctrl`, false when absent, held as the host's action key (Meta on macOS, Control elsewhere), so a non-adjacent `multiple` set can be built through the driver.
