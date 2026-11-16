@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=12 class=stale
leaf-text: counts the Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) only at warn and error, engine and `escher_` targets not at all
source: dec-64gbwyvv «The scoped log capture admits the Dioxus framework targets (dioxus*, generational_box, warnings) and the engine targets only at warn and error, escher_ targets never, and every other outside target at every level.»
replace-with: admits the Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) and the engine targets only at warn and error, `escher_` targets never
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=12 class=omission
leaf-text: which install the sink over an in-memory capture
source: dec-64bjtz2h «The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.»
replace-with: which install the sink over an in-memory capture, and the error-text founder check, which installs the sink over an in-memory writer
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=42 class=stale
leaf-text: `request` · `error` · `panic.payload`
source: dec-4zd5yain «Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted»
replace-with: `request` · `error` · `password` · `panic.payload`
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=42 class=omission
leaf-text: WARN and ERROR included, whatever `RUST_LOG` names.
source: dec-ldgd22yg «Scrubbing is by field name, in escher's own sink, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source, where a content-named field reads [redacted] and the»
replace-with: WARN and ERROR included, whatever `RUST_LOG` names. The scoped capture scrubs by the same field names for every source, a content-named field reading `[redacted]` and the message and other fields kept as written; engine and framework warn and error sites write fixed messages and carry their values in named fields (`href`, `url`, `src`, `text`, `request`), so the field-name scrub reaches them.
@leaf_edit n=5 leaf=.claude/docs/obs-summary.md line=43 class=stale
leaf-text: no other carrier takes the text
source: ctr-6htfgiyx «An `error` field is in the scrub set, so the sink prints its value as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item; a failed fetch's error text reaches no carrier as a field value.»
replace-with: the scoped capture keeps `url` and `error` as `[redacted]` in a caused item, the text reaching no carrier as a field value
@leaf_edit n=6 leaf=.claude/docs/obs-summary.md line=49 class=stale
leaf-text: which counts records and prints nothing
source: dec-5sc4lslw «escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.»
replace-with: which keeps the step's records and panics in process and prints nothing
@leaf_edit n=7 leaf=.claude/rules/observability.md line=18 class=stale
leaf-text: counts the Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) only at warn and error, engine and `escher_` targets not at all
source: dec-64gbwyvv «The scoped log capture admits the Dioxus framework targets (dioxus*, generational_box, warnings) and the engine targets only at warn and error, escher_ targets never, and every other outside target at every level.»
replace-with: admits the Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) and the engine targets only at warn and error, `escher_` targets never
@leaf_edit n=8 leaf=.claude/rules/observability.md line=18 class=omission
leaf-text: the driver setting only escher-telemetry's scoped capture around an acting step, which prints nothing
source: dec-66qxmezv «setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing; only a binary's own main installs the sink.»
replace-with: the driver setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing
@leaf_edit n=9 leaf=.claude/rules/observability.md line=18 class=omission
leaf-text: which install the sink over an in-memory capture
source: dec-64bjtz2h «The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.»
replace-with: which install the sink over an in-memory capture, and the error-text founder check, which installs the sink over an in-memory writer
@leaf_edit n=10 leaf=.claude/rules/observability.md line=24 class=stale
leaf-text: which counts records and prints nothing
source: dec-5sc4lslw «escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.»
replace-with: which keeps the step's records and panics in process and prints nothing
@leaf_edit n=11 leaf=.claude/rules/observability.md line=32 class=stale
leaf-text: `request` · `error` · `panic.payload`
source: dec-4zd5yain «Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted»
replace-with: `request` · `error` · `password` · `panic.payload`
@leaf_edit n=12 leaf=.claude/rules/observability.md line=32 class=stale
leaf-text: no other carrier takes the text
source: ctr-6htfgiyx «An `error` field is in the scrub set, so the sink prints its value as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item; a failed fetch's error text reaches no carrier as a field value.»
replace-with: the scoped capture keeps `url` and `error` as `[redacted]` in a caused item, the text reaching no carrier as a field value
@leaf_edit n=13 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: A new field carrying user content takes a name in the content set — or is not logged.
source: dec-izcyt3ou «Engine and framework warn and error sites write fixed messages and carry their values in named fields (href, url, src, text, request), so the field-name scrub reaches them.»
replace-with: A new field carrying user content takes a name in the content set — or is not logged. Engine and framework warn and error sites write fixed messages and carry their values in named fields (`href`, `url`, `src`, `text`, `request`), so the field-name scrub — in the sink and in the scoped capture, where a content-named field reads `[redacted]` and the message and other fields are kept as written — reaches them.
