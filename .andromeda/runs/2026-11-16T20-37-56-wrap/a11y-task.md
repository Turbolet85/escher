@task_prose part=session
The session put error text in the acting verbs' answer: each caused item now carries its record's message, location, source and, for an error, its cause chain, where before caused held only closed words and a count per kind and level.
The scoped capture keeps text from three sources (app, framework, engine), admits engine records at warn and error for the first time, scrubs a field by its name to [redacted] for every source, and installs a chaining panic hook once per process that keeps a panic's payload and location.
Every error boundary is now marked (root and own boundaries alike, through a boundary registry provided before the first build), and the step reads every boundary's new errors, not the root's alone.
The answer is bounded: 16 items kept, a text cut at 1024 bytes, 8 fields and 8 chain links kept, the rest counted in a new caused_past field on click, type, press, advance and scroll.
The sink's content-field scrub set gained password (11 to 12 names); six engine log sites now carry their values in fields under fixed messages.
Retired: the closed-words-only caused shape, the capture's count-only contract and the step's read of the root boundary mark alone.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: the scoped capture of log records during a driver step — what it admits by target and level, what text it keeps, the bounds it keeps it in, and its panic hook
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: escher's telemetry entry points — the sink's subscriber builder, now public, with the capture layer beside it
@task_file n=3 path=packages/escher-telemetry/src/format.rs
what: the sink's line format and its content-field scrub set (the field names whose values are redacted), now with password
@task_file n=4 path=packages/escher-driver/src/caused.rs
what: the caused answer builder — one item per panic, boundary error or log record, its order, its bounds and the counts past them
@task_file n=5 path=packages/escher-driver/src/execute.rs
what: the acting verbs' step settle — what a step reads before and after it to answer caused and caused_past
@task_file n=6 path=packages/escher-driver/src/json.rs
what: the driver's answer writer — how a caused item and caused_past are written on the answer line
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the driver schema — the acting verbs' result fields and their help, caused rewritten and caused_past added
@task_file n=8 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's unit checks — the field-kind table
@task_file n=9 path=packages/escher-driver/src/lib.rs
what: the session library's public surface — the caused types it exports
@task_file n=10 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture unit checks of acting-verb outcomes
@task_file n=11 path=packages/escher-driver/src/recording_tests.rs
what: the session recording unit checks — a kept check's recorded outcomes
@task_file n=12 path=packages/escher-driver/src/replay.rs
what: the session recording's replay — a kept check replayed and compared against its recorded answers
@task_file n=13 path=packages/dioxus-native-dom/src/boundary.rs
what: the Dioxus error boundary reading — the root error mark and the new boundary registry and boundary marks for every boundary
@task_file n=14 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document — its construction, which now provides the boundary registry at the root before the first build
@task_file n=15 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus document crate's public surface — the boundary types it exports
@task_file n=16 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling — the warning when a clicked link's href is not parseable, now a fixed message with href and url fields
@task_file n=17 path=packages/blitz-dom/src/form.rs
what: the engine's form submission — the warning for an unimplemented submission scheme, now with a url field
@task_file n=18 path=packages/blitz-dom/src/mutator.rs
what: the engine's document mutator — the iframe load warning when its src cannot be resolved, now with a src field
@task_file n=19 path=packages/blitz-dom/src/layout/mod.rs
what: the engine's layout — the text-node layout error, its field renamed text
@task_file n=20 path=packages/blitz-dom/src/lib.rs
what: the engine crate root — the new TRACING_COMPILED constant saying whether the engine's tracing is compiled in
@task_file n=21 path=packages/dioxus-native/src/assets.rs
what: the Dioxus native asset loading — its two warnings, now fixed messages with a request field
@task_file n=22 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check that every failure answers what failed, where and from which source — payloads, chains, boundaries, scrub, flood bounds and every carrier
@task_file n=23 path=tests/blitz-tests/tests/capture_panic_hook.rs
what: the stand check that the capture's panic hook installs once and a panic outside a capture reads as before
@task_file n=24 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check of a step's errors on its own session — the root boundary's Err now crossing as text in caused, and the same caused over the MCP carrier
@task_file n=25 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand checks' shared session helpers — now with the one decoder of an MCP tool result's text
@task_item n=1 part=touches
The password rows (Password value masked, Password text is never revealed, snapshot password input value): a new text carrier now crosses the answer — a field named password is redacted by name in caused and on the sink's lines, a typed password is asserted 0 times in the answer, and a masked control's snapshot value stays the mask; a message still holds what its author wrote.
@task_item n=2 part=touches
The 3. A11y Assertion Harness Contract overview's account of escher's sink: its redaction of escher's own content fields now also covers password (the set is 12 names), while the allowlist for blitz and accesskit_xplat targets is unchanged on the sink's lines.
@task_item n=3 part=touches
Snapshot readers behind the accessibility feature and Screen-reader integration behind the accessibility feature: the boundary registry and boundary marks are ungated by any feature, and the caused text rides every acting verb whatever the build.
@task_item n=4 part=touches
Stand checks repeated three rounds: a new stand check joins the stand-repeat leg by its name prefix and is repeated in its three rounds.
@task_item n=5 part=touches
The Dioxus document reached-handler record rows: the Dioxus document now also provides a boundary registry at its root before its first build and answers boundary marks beside the reached-handler record; a caused item still ties to a handler's id, owner and event, and an engine item from a link click ties to none.
@task_item n=6 part=touches
Dioxus render into a mutation buffer: the document's construction now registers every error boundary built under it before the first render, and the registry only grows for the document's life.
@task_item n=7 part=touches
MCP tool results · snapshot and the carriers of an answer: the MCP tool result carries the acting verb's answer line, now with error text and caused_past, decoded whole by the stand checks.
@task_item n=8 part=touches
The acting verbs' answer (click, type, press, advance, scroll): one new result field caused_past, and caused's item shape replaced — kind, level, source, target, message, cut, file, line, column, boundary, chain, fields and the handler tie.
@task_item n=9 part=touches
Driver schema defines no role or name set: the driver schema changed (caused's help rewritten, caused_past added) and still defines no role or name set.
@task_fact n=1 slug=screen-reader-support
fact: A step's caused answer carries error text (messages, chains, kept fields) yet never a password: a field named password is redacted by its name on every source, a masked control's value stays the mask, and a typed password crosses neither the answer nor the sink's lines.
@task_item n=1 part=untouched
Role mapping, accessible name precedence, ARIA state and property reads, and the accessibility tree's states and hidden content: no role, name, state or tree node moved.
@task_item n=2 part=untouched
Keyboard navigation: Tab order, focus moves, Enter and Space activation, range input keys and the select lists' open and closed keys did not change.
@task_item n=3 part=untouched
Visual design verification: contrast thresholds, the lint's contrast finding, stand colours, scrollbars and select list paint did not change.
@task_item n=4 part=untouched
Accessibility tier and platform adapter rows: no adapter, AccessKit wiring or assistive-technology reach changed.
@task_item n=5 part=untouched
The snapshot node keys, the snapshot text form, the snapshot diff and the UI graph: no reading of the snapshot or the graph changed.
@task_item n=6 part=untouched
Cognitive accessibility and the a11y CI leg and gate rows: no invalid-state, tab-order table or a11y leg moved.
