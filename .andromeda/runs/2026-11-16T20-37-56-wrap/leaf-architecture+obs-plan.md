@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summary
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=6 class=stale
leaf-text: which prints nothing and counts records
source: dec-5sc4lslw «escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing»
replace-with: which prints nothing and keeps the step's records and panics in process, a content-named field reading `[redacted]` and the message and other fields kept as written
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=6 class=stale
leaf-text: only at warn and error, engine and `escher_` targets not at all
source: dec-64gbwyvv «The scoped log capture admits the Dioxus framework targets (dioxus*, generational_box, warnings) and the engine targets only at warn and error, escher_ targets never, and every other outside target»
replace-with: and the engine targets only at warn and error, `escher_` targets never
@leaf_edit n=3 leaf=.claude/docs/services/escher-telemetry.md line=6 class=omission
leaf-text: every other outside target at every level. Not its job
source: ctr-j5z4y2tn «Each kept text is cut at 1024 bytes, saying how much; at most 8 fields a record and 16 records a source are kept, the rest counted.»
replace-with: every other outside target at every level, each kept text cut at 1024 bytes saying how much, at most 8 fields a record and 16 records a source kept and the rest counted. Not its job
@leaf_edit n=4 leaf=.claude/docs/services/escher-telemetry.md line=15 class=omission
leaf-text: and `init_with_writer(ServiceIdentity, W: MakeWriter)` (a caller-given sink).
source: ctr-chc4sbyd «Public: returns the sink's subscriber with the capture layer, built uninstalled, at a given LevelFilter.»
replace-with: and `init_with_writer(ServiceIdentity, W: MakeWriter)` (a caller-given sink); public `subscriber` returns the sink's subscriber with the capture layer, built uninstalled, at a given `LevelFilter`.
@leaf_edit n=5 leaf=.claude/docs/services/escher-telemetry.md line=26 class=stale
leaf-text: `request` · `error` · `panic.payload`
source: dec-4zd5yain «Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted»
replace-with: `request` · `error` · `password` · `panic.payload`
@leaf_edit n=6 leaf=.claude/docs/services/escher-telemetry.md line=26 class=retired
leaf-text: No engine call site is edited, and no filter layer is added: the drop sits in the formatter.
source: dec-5qlvv542 «Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited.»
replace-with: Engine and framework warn and error sites write fixed messages and carry their values in named fields (`href` · `url` · `src` · `text` · `request`), so the field-name scrub reaches them; no filter layer is added: the drop sits in the formatter.
@leaf_edit n=7 leaf=.claude/docs/services/escher-telemetry.md line=29 class=omission
leaf-text: it spawns no thread.
source: dec-xxvmhvjw «escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.»
replace-with: it spawns no thread. The capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
