@history_item n=1 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — (§4, 1 row) ~ decision/canvas-read-opens-no-span: the rule now also says the lint's canvas label reading opens no span and logs nothing, running inside the lint call's command span. This is the latest form of the row this chunk rests on. Any in-process read of declared virtual nodes inside a canvas should follow the same pattern: no span of its own, no log, and it runs only inside an existing call's command span. If the read is not reached by any verb, the row's wording has to be extended for it, as the lint's reading was.

@history_item n=2 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — (§3, §4, §6) + decision/canvas-read-opens-no-span (Session::canvas has no span by design), + contract/command-span/canvas, ~ contract/command-span/settled and ~ contract/tool-call-span/tool (fifteen to sixteen verbs), ~ contract/command-span/verb (sixteen words), ~ decision/sink-host-read-at-trace (canvas needles added to host_log/mcp_log), ~ decision/part-answer-never-logged (canvas part, PROVISIONAL, delegate the escher overseer). Why it matters: this is the precedent for how one new canvas surface moves the verb counts, the per-verb span rows and the host/mcp needle set. The chunk claims no new verb, so those counts must stay put. If a declared name or key crosses a carrier (the names' crossing fork), the trace reads need a virtual-node needle class, as canvas needles were added. The part-answer row is still PROVISIONAL.

@history_item n=3 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (§3, §4, §6, §8) ~ decision/layout-read-opens-no-span (the read now runs inside the layout verb's command span and opens none of its own), + contract/command-span/layout, verb counts fourteen to fifteen, ~ decision/sink-host-read-at-trace (layout-answer key needles), + contract/layout-answer/content and /logging. The widening was ratified through «Глагол `layout --id` (Recommended)». Why it matters: layout explained (`Placer::App { canvas }`) is a later entry's boundary. A read that is first in-process-only and later reached by a verb has its span row reworded. Each new answer carrier gets a content row and a never-logged row.

@history_item n=4 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (§3, §4) + decision/layout-read-opens-no-span, added so that a driver read with no instrumentation by design is not read as drift. ~ decision/headless-stand-no-sink and ~ decision/checks-silent except gained the stand_layout recording subscriber. Why it matters: it is the origin of the «new read, no span, recorded as a decision» pattern. A new check that installs a recorder must be named in the headless-stand-no-sink except.

@history_item n=5 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (§3, §4, §6, §7, §8) ~ decision/headless-stand-no-sink (except: the error-text founder check installs escher's sink over an in-memory writer), ~ decision/driver-library-installs-no-subscriber (scoped capture plus chaining panic hook), ~ decision/driver-emits-no-event (the capture keeps records and panics in process), ~ decision/escher-target-content-named-set (password added), − decision/scrub-at-sink-not-call-sites → + decision/scrub-by-field-name, + decision/engine-sites-fixed-messages. Why it matters: these are the current forms of picked §3, §4 and §8 rows. Any engine or Dioxus warn/error site that a refused declaration might add must write a fixed message, with its values in named fields so the field-name scrub reaches them. A refused declaration must not panic: a panic would cross through the capture's panic hook.

@history_item n=6 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (§3, §4, §6) ~ decision/headless-stand-no-sink and ~ decision/driver-library-installs-no-subscriber now name escher-telemetry's scoped capture around an acting step as the one subscriber, which prints nothing. ~ decision/driver-emits-no-event. ~ contract/escher-sink-env/rust-log: EnvFilter applies to the sink layer alone. + decision/capture-framework-targets-warn-up. Why it matters: Dioxus and engine records at warn or above, for example from validating a declaration, reach the caused answer through the capture. They are not silent in the answer even though nothing prints.

@history_item n=7 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — (§3, §7, §8) ~ decision/scrub-by-field-name: the capture scrubs by the answer's own hidden set, and `error` crosses as written, cut at the text bound. Later ratified (2026-11-17-a-component-in-isolation). Why it matters: this is the current picked row's text. A declared name carried in an `error` field of a record would cross into a caused item unredacted.

@history_item n=8 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — (§6, §8) ~ contract/layout-answer/logging: the answer also crosses to the model provider in the transcript on a live cold-agent run. A third uncounted verb was added. The delegate confirms of the error-field and layout cards were ratified. Why it matters: the cold-agent transcript is a carrier that the names' crossing fork has to count when it lists what a declared name would cross.

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (§3, §6, §8, §9) The graph answer's carried set covers ids, component names, registered state and handler event names, closed flag words and booleans, never a value. It is a boundary widening, ratified (made PROVISIONAL here, ratified at 2026-11-15T05-21-42-wrap). SignalRegistry holds names and declarations, never a value or a NodeId beyond the dispatch chain, and its Debug prints counts alone. dioxus-native-dom's graph modules add no log site. Why it matters: this is the closest precedent for app-authored names (component and state names) crossing a ratified carrier as a founder-ratified widening, which bears on the names' crossing fork. It is also a precedent for a declaration registry in dioxus-native-dom whose Debug prints counts only, which fits if the declaration lives in the Dioxus renderer.

@history_item n=10 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — Ratified the graph answer's carried set and the coverage role's rows, which had been PROVISIONAL (delegate overseer), on the founder's own word. Why it matters: a delegate confirm of a widening stays PROVISIONAL until the founder's batch. If the names' crossing fork is answered by a delegate, it will carry the same mark.

@history_item n=11 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (§3, §4, §6, §8, §9) The actions answer carries ids, owner ids, verb words, closed cause words and booleans. It is a boundary widening on pattern `rul-izni57qv`, ratified, and it is at rest in the recording directory. Session::actions and dioxus-native-dom's actionable_nodes open no span. Why it matters: `owner` ids are the precedent for a node tied to its owner element, as a virtual id `{owner}///{key}` will be. A later entry that lists virtual nodes in actions goes through this widening pattern.

@history_item n=12 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (§6, §8) dioxus-native-dom's reached record holds stable ids and event names only, no NodeId, and its Debug prints its count alone. The coverage line's carried fields are a widening, ratified later. Why it matters: this is the precedent for a crate-private record in dioxus-native-dom (no NodeId, count-only Debug, no log site) that a validated declaration store should match.

@history_item n=13 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (§6, §8) MutationBuffer holds element ids, names and text in process memory only, and its Debug prints `len` alone. A new dioxus-native-dom item got a §6 no-site row and a §8 hold row. Why it matters: this is the form used to record a new in-process holder of app-authored text, which the declared names and keys will be.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (§8) An optgroup's non-empty `label` is its Role::Group name in the accessibility tree, inside the ratified name crossing. That stands on the P5 lean, not on fork F7's text, and the operator confirmed it as his own reading, not the founder's word. Why it matters: this is a direct precedent for the names' crossing fork. An author-written name that reaches the accessibility tree as a node name was read as inside the ratified name crossing. Writing that down is a ruling that needs a named owner and must not be assumed.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (§8) Option and group labels painted in an open list were ratified as a boundary widening. A row's `bounds` crossing inside the closed `differs` set was read as outside the widening under the field rule of 2026-10-10. SelectListRow.label is page text in a public read type, never logged. Why it matters: this precedent separates a region or bounds (closed numbers, not a widening) from authored text (a widening). It also covers a public read type that holds author text and is never logged, which is the shape of a declared node's name.

@history_item n=16 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — (§3, §4) Command, Call and ArgValue derive Debug, which prints ids and text, and nothing prints, logs or fields them. The standing rule is: never field such a type with Debug. Why it matters: this is the origin of decision/debug-types-never-logged. A declared virtual node type holding an author key and a name that derives Debug falls under it.

@history_item n=17 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — (§3, §4) Outcome joined the Debug-deriving types that nothing prints, logs or fields. Why it matters: decision/debug-types-never-logged (picked) grew by naming each new answer-bearing type. A new declaration type joins that set.

@history_item n=18 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — (§3) The session's record of ids is bounded (4096 ids), has no reader outside the crate and no Debug, and is printed, logged or fielded by nothing. Why it matters: this is the origin of decision/session-id-record-unprinted (picked). Virtual ids of the reserved form would enter that record once a screen reads them.

@history_item n=19 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (§2, §3, §4, §6, §8) What a command prints is its answer under the ratified crossing (the founder, 2026-10-10), never a log field. Standing rule: the next surface adds no span or log field that holds what a call supplied or a screen reads. Typed text was measured at 0 in a host's log under both builds, and the workspace build's engine line was measured. Why it matters: the scope dates the names' crossing rule to 2026-10-10. This entry is that rule's obs side. It also measures decision/engine-tracing-by-invocation (picked).

@history_item n=20 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (§2, §3, §4, §6, §8) The command span was built with eight fields and tracing ungated in escher-driver. A workspace build turns blitz-dom/tracing on, so a test child with the sink at info also receives engine events. The closed span's line was added PROVISIONAL, with fields judged by the same scrub. Why it matters: these entries are the origin of the picked rows decision/engine-tracing-by-invocation, contract/command-span/field-count and the escher-log-line/span-line contracts. Any blitz-dom tracing site that a Widget-trait declaration adds is compiled in under the workspace build.

@history_item n=21 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — (§6, §8) The closed span's line was ratified as a second record class. A diff's sizes may reach a log, its content never. Why it matters: this is the origin of the picked decision/closed-span-lines. A new record class or field is a boundary widening for the founder.

@history_item n=22 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — (§3, §6, §8) Outside targets are dropped whole at every level. The ESCHER_TARGET_PREFIXES and ENGINE_TARGET_PREFIXES rules were added. The rule for later chunks: a target is admitted only together with a scrub rule for it. Why it matters: this is the origin of the picked decision/sink-target-allowlist and the engine and escher target rows. A log site under a new target is silently dropped, and admitting it needs a scrub rule.

@history_item n=23 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (§3, §8) This added the second sink installer (escher-session) and measured ids and names leaking at debug/trace before the allowlist. Trap: a zero read from a host that installs no sink says nothing about one that does. Why it matters: in-process silence for virtual-node declarations does not prove host-log silence. decision/sink-host-read-at-trace requires the host and mcp reads.

@history_item n=24 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — (§1, §2, §3, §6, §7, §8) Bootstrap of escher-telemetry: the stderr sink, the chaining panic hook, the allowlist scrub with engine safe fields, and tracing gated on the cargo feature in engine and upstream crates. Why it matters: this is the origin of the picked rows decision/engine-tracing-feature-gated, decision/tracing-for-telemetry, decision/engine-target-safe-fields and contract/escher-telemetry-panic-hook/previous-hook. A blitz-dom custom-widget site added by this chunk must be feature-gated.

@history_item n=25 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — (§3) escher ships no OTel export: no OTel crate, no egress, no credential path. Why it matters: this is the basis of the picked absence/telemetry-backend. The chunk must add no exporter.

@history_item n=26 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — (§3) The seven_guis shared log-check module (Needles, stand_needles and others) states that assertion messages name a kind and a count, never a needle or a captured line. Why it matters: this is the origin of the picked decision/log-check-failures-name-kind-count. A virtual-node needle helper, if the crossing fork adds one, follows it.

@history_item n=27 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (§6, §8) The runtime probe's failure messages name a layout mode, a task or page and a link or index only, and its reading quotes no captured stderr, panic payload, signal value or node id. Why it matters: this is the origin of the picked decision/runtime-probe-leaks-no-read-value. P3's probe readings and evidence must quote no read value, consistent with the operator's condition that no raw log goes into evidence.

@history_item n=28 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (§7, §6, §8) NetHandler::error was added as a public trait method with a do-nothing default. Standing trap: a trait default can hide a fix, as this one stopped at CountedHandler. Sweep the implementers of a trait that a change extends. Why it matters: if the declaration fork puts the declaration on blitz-dom's `Widget` trait, a defaulted method can hide declarations at implementers that do not override it.

@history_item n=29 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — Added `Document::accessibility_tree`. Citations into blitz-dom `document.rs` and blitz-shell `window.rs` were re-pointed, with no log site added. Why it matters: the reserved `accessibility_tree` on `Widget` sits next to this method. A chunk that adds lines to these files moves obs-plan citations that must be re-pointed at wrap.

@history_item n=30 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (§4, §7) + decision/exploration-adds-no-span (an in-process driver call is instrumented only through the command spans of the calls it drives). + decision/exploration-catches-panics (a caught panic is reported without its payload). Why it matters: this is a precedent for an in-process read with no verb that is recorded as adding no span. It also shows the no-payload rule for panics on untrusted input.

@history_item n=31 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — (§3) ~ decision/checks-silent except now names the order page's one fieldless info log site among the exceptions, following stand_layout (2026-11-16-layout-explained) and the seeded-randomness re-run child (2026-11-14-seeded-randomness). Why it matters: the new virtual-node checks must stay silent. Any log site in pinned code, or any recorder, must be named as an exception in this row.

@history_item n=32 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — (§8) + decision/dropped-data-never-logged: the dropped data is a new class of user data. Why it matters: this is the obs side of the drag-and-drop input that the scope's CARRY cry-tjzqz3hz carries to this chunk's wrap (the security-summary gaps). It is written only at wrap, never by hand.

@history_item n=33 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — The masters became rows (decision, contract, absence, overview). Since then each amendment names a row by its handle. Why it matters: planned obs changes for this chunk are row edits by handle (+, ~ or −) in the areas above, not prose sections.
