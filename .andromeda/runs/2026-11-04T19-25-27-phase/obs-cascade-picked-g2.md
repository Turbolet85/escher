@account n=1 entity=log-coverage#5.1 v=not_in_scope
why: the examples' print sites and escher-session's fixed endings belong to a component the chunk only uses, and the probe adds no verb, exit or print to them
@account n=2 entity=log-coverage#6.1 v=not_in_scope standing=S3
why: a change adds no log site, span or field, so escher-driver keeps its 2 info_span! sites and stand_act_spans keeps its eight fields
@account n=3 entity=log-coverage#8.1 v=carried
why: it records the 2026-11-01 sync's dioxus-native-dom mounted-event guard as silent, and the chunk re-verifies the MountedData hazard in that crate at HEAD after that sync
@account n=4 entity=log-coverage#8.4 v=not_in_scope
why: blitz-dom node log sites and the select list's silence are in files the chunk does not reach
@account n=5 entity=log-coverage#8.9 v=carried
why: dioxus-native-dom is the probe's read surface, and its mutation_writer trace logs are the engine events the probe must neither depend on nor add to
@account n=6 entity=log-coverage#9.2 v=not_in_scope
why: this is an earlier search over the examples slice that the chunk does not redo
@account n=7 entity=log-coverage#9.3 v=not_in_scope
why: this is the 0.2.0 requirement record's silence over its own test files, which the chunk does not change
@account n=8 entity=log-coverage#9.5 v=not_in_scope
why: this is host_log's select-list option reading, which the probe does not touch
@account n=9 entity=log-coverage#9.6 v=not_in_scope
why: this is mcp_log's select-list option reading, which the probe does not touch
@account n=10 entity=log-coverage#9.7 v=not_in_scope
why: these are the seven_guis shared module's option needles for the popup log tests, which the probe check does not read
@account n=11 entity=log-coverage#9.8 v=not_in_scope standing=S1
why: each new stand or engine check installs no subscriber, reads no env var, prints nothing and binds no listener, and the silence census reads 0 over its file
@account n=12 entity=log-coverage#9.9 v=not_in_scope standing=S3
why: a change adds no log site, span, field or env read, so escher-driver keeps its 2 info_span! sites
@account n=13 entity=log-coverage#9.11 v=not_in_scope standing=S3
why: a change adds no log site, span, field or env read, so escher-driver keeps its 2 info_span! sites and stand_act_spans keeps its eight fields
@account n=14 entity=error-capture-reporting#1.4 v=carried
why: the chunk re-verifies the MountedData panic, and it must add no panic hook, capture or reporting while doing so
@account n=15 entity=pii-scrubbing-compliance#1.1 v=carried
why: the reading states how the MountedData panic surfaces in a sink-installing binary, where std's chained hook prints the raw message past the scrub
@account n=16 entity=pii-scrubbing-compliance#2.1 v=carried
why: the sink redacts panic.payload under escher_telemetry::panic and prints only safe fields under the dioxus_native engine prefix, which is what the reading's capture statement and the probe's engine-event independence rest on
@account n=17 entity=pii-scrubbing-compliance#3.1 v=not_in_scope
why: this is the sink's reach and its drop of third-party targets, and the probe installs no sink
