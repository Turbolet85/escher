@leaf_group group=obs-plan-1
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=49 class=omission
leaf-text: install no sink — setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing — read no env var
source: dec-66qxmezv «installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing»
replace-with: install no sink — setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing, and its chaining panic hook, which prints nothing — read no env var
