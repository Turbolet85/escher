@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=23 class=stale
leaf-text: the driver's `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver, a limit its help states, its fix owned by residual `res-hfvjj77l`.
source: ctr-ublownab «Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere.»
replace-with: the driver's `press` takes an optional `ctrl`, false when absent and held as the host's action key (Meta on macOS, Control elsewhere), so a non-adjacent `multiple` set can be built through the driver.
