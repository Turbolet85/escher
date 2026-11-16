@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=60
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=42
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=12 class=stale
leaf-text: The headless stand (`seven_guis::stand`, booted in-process by the `stand_*` checks) installs no subscriber
source: dec-64bjtz2h «A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing»
replace-with: The headless stand (`seven_guis::stand`, booted in-process by the `stand_*` checks) installs no sink, its acting steps running only under escher-telemetry's scoped capture, which prints nothing
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=12 class=omission
leaf-text: a global `Registry` + `EnvFilter` (`RUST_LOG`, default `warn`) + one non-ANSI fmt layer
source: ctr-a5orotqf «The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset; it filters the sink layer alone, the capture layer beside it unfiltered.»
replace-with: a global `Registry` + `EnvFilter` (`RUST_LOG`, default `warn`, filtering the sink layer alone, the capture layer beside it unfiltered) + one non-ANSI fmt layer
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=12 class=omission
leaf-text: plus the `log` → `tracing` bridge (`LogTracer`)
source: dec-64gbwyvv «The scoped log capture counts the Dioxus framework targets (dioxus*, generational_box, warnings) only at warn and error, engine and escher_ targets not at all, and every other outside target at every level.»
replace-with: plus the `log` → `tracing` bridge (`LogTracer`); the scoped log capture beside it counts the Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) only at warn and error, engine and `escher_` targets not at all, and every other outside target at every level
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=49 class=stale
leaf-text: install no subscriber, read no env var and no clock and print nothing
source: dec-5sc4lslw «escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which counts records and prints nothing; its telemetry is its spans alone.»
replace-with: install no sink — setting only escher-telemetry's scoped capture around an acting step, which counts records and prints nothing — read no env var and no clock and print nothing
@leaf_edit n=5 leaf=.claude/rules/observability.md line=18 class=stale
leaf-text: the driver library `escher-driver` — its session and its command schema — and their checks install none
source: dec-66qxmezv «The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step, which prints nothing»
replace-with: the driver library `escher-driver` — its session and its command schema — and their checks install no sink, the driver setting only escher-telemetry's scoped capture around an acting step, which prints nothing
@leaf_edit n=6 leaf=.claude/rules/observability.md line=18 class=omission
leaf-text: filter from `RUST_LOG`, default `warn`
source: ctr-7fb47irx «Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.»
replace-with: filter from `RUST_LOG`, default `warn`, applied to the sink layer alone, never to the capture layer beside it
@leaf_edit n=7 leaf=.claude/rules/observability.md line=18 class=omission
leaf-text: `log` records bridged into the same formatter
source: dec-64gbwyvv «The scoped log capture counts the Dioxus framework targets (dioxus*, generational_box, warnings) only at warn and error, engine and escher_ targets not at all, and every other outside target at every level.»
replace-with: `log` records bridged into the same formatter; the scoped log capture counts the Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) only at warn and error, engine and `escher_` targets not at all, and every other outside target at every level
@leaf_edit n=8 leaf=.claude/rules/observability.md line=24 class=stale
leaf-text: The crate installs no subscriber.
source: dec-5sc4lslw «escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which counts records and prints nothing; its telemetry is its spans alone.»
replace-with: The crate installs no sink, setting only escher-telemetry's scoped capture around an acting step, which counts records and prints nothing.
