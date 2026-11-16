@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summa
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=6 class=omission
leaf-text: bridges `log` records and chains a panic hook.
source: dec-64gbwyvv «The scoped log capture counts the Dioxus framework targets (dioxus*, generational_box, warnings) only at warn and error, engine and escher_ targets not at all, and every other outside target at every level.»
replace-with: bridges `log` records and chains a panic hook; beside the sink it offers a scoped log capture, set by the driver around an acting step, which prints nothing and counts records — the Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) only at warn and error, engine and `escher_` targets not at all, every other outside target at every level.
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=12 class=omission
leaf-text: the default is `warn` when it is unset or unparsable.
source: ctr-7fb47irx «Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.»
replace-with: the default is `warn` when it is unset or unparsable; it filters the sink layer alone, never the capture layer beside it.
@leaf_edit n=3 leaf=.claude/docs/services/escher-telemetry.md line=29 class=omission
leaf-text: it spawns no thread.
source: dec-l4cbrm4r «Where the held default has no capture layer, the log capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls»
replace-with: it spawns no thread. Where the held default has no capture layer, the log capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls into its one-dispatcher interest mode.
@leaf_edit n=4 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: install no subscriber — a headless boot made in process has no escher sink
source: dec-64bjtz2h «A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing»
replace-with: install no global subscriber — a headless boot made in process has no escher sink, its acting steps running only under this crate's scoped capture, which prints nothing
@leaf_edit n=5 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: each install a recording subscriber of their own, not this sink)
source: dec-5oq63xdv «A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder.»
replace-with: each install a recording subscriber of their own, not this sink, opening an empty log capture first so the capture's dispatchers are registered before the recorder)
