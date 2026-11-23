@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — (escher-telemetry: stderr subscriber, allowlist scrub) set the rule that tracing is compiled only with the `tracing` feature in the engine and upstream crates, and the engine-target allowlist (`blitz`, `dioxus_native`, `stylo_taffy`, …) printing only the seven safe fields. This is the basis of decision/engine-tracing-feature-gated, decision/telemetry-via-tracing and decision/engine-target-safe-fields. Any log site the placement fix adds in blitz-paint or blitz-dom events, or the font-context handoff adds, would sit behind the feature and print only the safe fields.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — (telemetry installer named; the headless stand installs none) put on record that the headless stand and its in-process checks install no subscriber, read no env and print nothing. This is the origin of decision/headless-stand-no-sink. The chunk's new red/green canvas checks (placement, pointer frame, canvas text) run on that headless stand and must keep to it.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — (one println! in the stand checks: the re-exec child's captured ids) is the origin of the re-run-child exception in decision/checks-silent: stdout that a re-run child prints and the parent reads into an assertion is not a log. A new check may print only under this named exception.

@history_item n=4 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — (the headless-stand census covers the checks' shared module) extended the silence census to the checks' shared modules (`tests/blitz-tests/tests/common/mod.rs`). This is why decision/checks-silent names "their shared modules". Any helper the chunk adds to common or fixtures falls inside the census.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (a second sink installer; the scrub's reach scoped to the measurement) made `escher-session` the second sink installer beside `seven_guis_native` and scoped "a headless boot has no escher sink" to in-process boots. This is the basis of decision/scrub-sink-reach and the split in decision/headless-stand-no-sink. Trap recorded: a zero read from a host that installs no sink says nothing about one that does. A canvas shot through the host is read under the host's sink.

@history_item n=6 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — (outside-target records dropped) introduced the three outcomes: engine prefixes scrubbed to the safe fields, `escher_` prefixes with content-named fields redacted, every other target dropped. Standing rule: a target is admitted only together with a scrub rule for it. If canvas text shaping through parley or a new blitz-paint site emits under a new target, the sink drops it. This bears on decision/engine-target-safe-fields and decision/scrub-sink-reach.

@history_item n=7 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — (the executor is silent) established that `Command`, `Call`, `ArgValue` and `Outcome` derive Debug and that nothing prints, logs or fields them. Rule kept: field a cause by `Cause::name`, never a value with Debug. This is the origin of decision/debug-types-never-logged and decision/driver-spans-carry-no-content. The rule binds any widget or font-context type the chunk adds that holds page text.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the driver's command span as built; `tracing` ungated in escher-driver) built the `command` span with its eight fields and its never-fielded list. It also found that a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` receives engine events. This is the origin of decision/engine-tracing-by-invocation, contract/command-span/line, field-count and decision/command-span-no-call-content. Any blitz-dom event or blitz-paint log site the chunk touches shows up under the workspace build.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the closed span's line, PROVISIONAL; a span's fields under the scrub) defined the closed-span line shape and judged every span pair by the scrub rule (engine target: safe fields only). This defines what contract/command-span/line and contract/tool-call-span/line print.

@history_item n=10 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — (the closed span's line ratified) records that the founder ratified the closed-span line as a second record class. That ratification underpins contract/command-span/line and contract/escher-driver-span-lines/default-level. The chunk adds no span or field (scope: "No new verb, argument, carrier, cause, … log field or span").

@history_item n=11 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — (OTel export out of escher 0.1.0) records the founder's choice of no OTel export: no OTel crate, no egress. This is the basis of absence/telemetry-backend. The parley dev-dependency must bring no exporter.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session host writes the command span; the command line writes answers, not logs) made a command's answer, refusal or session error one line of JSON on stdout, an answer and never a log line. It also measured the workspace-built host's one engine INFO line. This is the origin of contract/escher-session-stdout/command-answer, contract/escher-session-exit-codes/0 and /1, and contract/escher-session-stderr/telemetry-not-installed. Standing rule: the next surface adds no field holding what a call supplied or a screen reads.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the `tool_call` span, ratified) added the `tool_call` span (`tool`, `ending`), with a shot's call ending in an image block then text. This is the origin of contract/tool-call-span/line and the MCP half of contract/command-answer/shot. Its fields are closed words only, a boundary widening the founder ratified.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (`screenshot` in the spans' closed sets, nine causes, no image in a log) added the screenshot verb. An accepted shot records `verb` alone and a refused one `verb` and `cause`; the image, its byte size and width/height are never fielded; `fonts-unavailable` is returned before anything is painted. It measured 0 image bytes on host/mcp stderr at `trace`, both builds. This is the origin of contract/command-span/screenshot, contract/refusal-cause/fonts-unavailable, decision/shot-image-leaves-only-in-answer and contract/command-answer/shot. The canvas text from the bundled fonts must keep the `fonts-unavailable` contract and the shot contract that the scope says stands.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — (the workspace host's engine line printed twice) re-read the workspace-built host: the `blitz_dom::document` `focussed node` INFO line printed twice. Why it fires twice is recorded, not established. This shows how engine events reach the sink under the workspace build (decision/engine-tracing-by-invocation). The content-box pointer-event change in blitz-dom events may move such engine lines.

@history_item n=16 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (the stderr readings) read a client's stderr, the host and the workspace-built windowed `seven_guis_native` by build and level, 28 runs, 0 needles. This is the measured baseline behind decision/engine-tracing-by-invocation and decision/session-logs-carry-no-page-content.

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (eight causes) set the cause set at eight names, `time-unavailable` retired and `fonts-unavailable` kept, with the executor returning six. This closed set holds contract/refusal-cause/fonts-unavailable. The chunk adds no cause.

@history_item n=18 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder: one-virtual-clock) ratified the eight cause names, `fonts-unavailable` among them, as the founder's word. The closed cause set the scope keeps unmoved rests on this ratification.

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (recording opens no span and logs nothing; the recording-directory row) put kept checks at rest: call content in a `0600` file, never a log. This is the origin of contract/kept-check/arguments, accepted and answer-digest.

@history_item n=20 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (the replay observed silent, each kept answer at rest, the committed checks) made the recorder write each answer whole as `answer`, a shot's without `png`. It also committed `counter-{cli,mcp}.json` as public checks. This is the origin of contract/kept-check/answer, decision/committed-kept-checks-clean and decision/committed-checks-hold-no-private-data. A committed kept check must not take a screenshot step, and the stand's kept checks keep their readings unless a ruling moves one.

@history_item n=21 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: the `lint` verb inside the command span, no field) wrote `Outcome::Linted { findings }` by `to_json` alone, never logged or fielded. This is the origin of contract/command-answer/linted. Canvas text reaching lint or text_extent would stay inside this rule.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the chunk's surfaces in §6, no log site added, and the census's carve-out) recorded `no site added` rows for blitz-dom engine changes. It also set a trap: an inline `#[cfg(test)]` module (blitz-shell `event.rs`) is held out of the added-lines census under `packages/`, so a census 0 does not speak for it. The chunk's engine changes in blitz-paint `render.rs`/`lib.rs` and blitz-dom `events/mod.rs` and `document.rs` need the same reading.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the five new checks read silent) is the precedent for recording new engine checks without a `stand_` prefix as an `observed absent` row in §6 log-coverage, read by the silence census. The chunk's new placement, pointer and canvas-text checks follow this form under decision/checks-silent.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (citations re-pointed past the chunk's added lines) re-pointed citations into blitz-dom `document.rs` and `events/pointer.rs` and escher-driver `execute.rs`. Trap: this chunk's added lines in `document.rs` (the public `font_ctx()`), `events/mod.rs` and blitz-paint `render.rs` will move cited lines, which then need the same re-point.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the line citations the select list's added lines moved) re-pointed citations into blitz-paint `render.rs`, blitz-dom `document.rs` and `events/pointer.rs`. This is the same re-point obligation the chunk's edits to these files will trigger.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list adds no log site or span) is the precedent for a blitz-paint render feature (`render::select_list`) added with no tracing or log token, read by the added-lines census. That census also left escher-driver's 2 `info_span!` and `command`'s eight fields unmoved. The canvas placement fix in `draw_custom_widget` and `process_custom_widget_node` carries the same obligation.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list's checks are silent) recorded `select_popup`, `select_popup_painted` and `stand_act_popup` as silent in both layout modes. Its census over the pinned paint code (`render::select_list`, `stroke_text_as` in `text.rs`) found no token. This is the precedent for the chunk's painted-pixel checks under decision/checks-silent.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list read through the host, the role and a hosted shot) read a hosted shot of an open list equal to the in-process one. `host_log` and `mcp_log` read 0 option needles at `trace`. This is the precedent for reading painted page text in a shot through the host without it reaching a log (decision/session-logs-carry-no-page-content, decision/shot-image-leaves-only-in-answer).

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a page's labels as pixels and the size read, the widening ratified) made a shot that carries page text as pixels a boundary widening, ratified with the shot's fields unmoved. This is the origin of decision/open-list-shot-carries-labels. It is the nearest precedent for F3, canvas text leaving as pixels in a shot, which the scope settles as inside the ratified crossing.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed load reaches its handler, and where its error text stops) records a standing trap: a trait default can hide a fix, because a default method stopped at `CountedHandler` with no gate reading it. Sweep the implementers of a trait a change extends. This applies directly to the chunk's new defaulted `Widget` method that hands the font context: every implementor (`FpsWidget`, `examples/custom_widget.rs`, `wgpu_texture`, the stand's widget) must be swept.

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the upstream take read at `e42ee1f25` is silent) recorded that `set_custom_widget`'s replay and blitz-paint's `blend` and layers open no span. This is the standing §4 statement on the custom-widget and paint code this chunk changes.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (citations re-pointed past the take's line moves) re-pointed citations into blitz-paint `render.rs`, blitz-dom `document.rs` and the telemetry-strategy engine-event line. The re-point covers the files this chunk edits.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the Dioxus runtime probe's silence row) is the precedent for a blitz-tests target with no `stand_` prefix. It got a log-coverage `observed absent` row from the silence census, while the §3 body took no row. This is a template for the chunk's new checks.

@history_item n=34 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions read opens no span and adds no site) added the rule that the actions read changes nothing in the document. This is the origin of decision/actions-read-changes-nothing. The scope keeps the actions digests of the served tasks.

@history_item n=35 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions answer's carried set, at rest and in the PII boundary) defined the actions answer: shape, content, owner, cause, never logged. This is a ratified widening and the origin of contract/actions-answer/*. Any widget pointer-event change must leave the actions answer's carried set unmoved.

@history_item n=36 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — (the masters as rows) turned obs-plan's text into the rows the picked set holds (decision/contract/absence/overview). Each row was verified against the code. From here on, amendments name rows by handle.

@history_item n=37 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — (~ decision/checks-silent · except) widened the re-run-child exception to "ids or digests". A pixel or digest re-run child in a canvas check would fall under it.

@history_item n=38 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (~ decision/checks-silent · except, ~ decision/headless-stand-no-sink · except) named the layout stand check's own recorder as an exception. This is the precedent: a new check that installs a subscriber must be added to these rows' named exceptions.

@history_item n=39 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (~ decision/headless-stand-no-sink, ~ decision/driver-library-installs-no-subscriber, ~ contract/telemetry-sink/rust-log) set the current wording that acting steps run under escher-telemetry's scoped capture, which prints nothing. It also changed contract/escher-sink-env/rust-log so that RUST_LOG filters the sink layer alone, and added decision/capture-framework-targets-warn-up. Engine warn/error records during an acting step reach the caused answer.

@history_item n=40 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (~ decision/driver-emits-no-event) is the first rewording of decision/driver-emits-no-event: the driver sets only the scoped capture around an acting step and emits no event.

@history_item n=41 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (3. Observability Harness Contract: 4 rows) gave decision/headless-stand-no-sink, decision/checks-silent and decision/driver-library-installs-no-subscriber their current wording. That wording includes the error-text founder check's in-memory sink and the chaining panic hook. Exceptions the chunk's checks add must extend these lists.

@history_item n=42 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (~ decision/driver-emits-no-event) gave the current wording: the capture keeps the step's records and panics in process and prints nothing.

@history_item n=43 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (~ decision/capture-framework-targets-warn-up) made the scoped capture admit engine targets at warn and error. Any engine warn site touched by the placement fix or by font-context access during an acting step crosses into the caused answer.

@history_item n=44 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (+ decision/engine-sites-fixed-messages, + decision/scrub-by-field-name) added the rule that engine and framework warn/error sites write fixed messages and carry values in named fields. Any engine log site the chunk adds in blitz-paint or blitz-dom must follow it.

@history_item n=45 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (~ contract/command-span/verb) moved the verb table to fifteen words, the count in the picked row. The scope adds no verb, so the count must stay fifteen.

@history_item n=46 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (~ decision/sink-host-read-at-trace) extended the needle set of the `trace` read of the host and the mcp role. A canvas shot read through the host stays under this read.
