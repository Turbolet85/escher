@history_item n=1 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (4. Span / Trace Coverage: 1 row(s)) — added `+ decision/canvas-read-opens-no-span`: `Session::canvas` opens no span by design, running inside the canvas verb's `command` span. Any canvas-lint read this chunk builds over the recorded draw commands has to follow it: no span of its own, reached only inside a call's `command` span.

@history_item n=2 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (3. Observability Harness Contract: 2 row(s)) — `~ decision/sink-host-read-at-trace` added "canvas needle" to the needles `host_log` / `mcp_log` must read at 0 at trace. `~ decision/part-answer-never-logged` added the canvas part, and that part is PROVISIONAL (delegate the escher overseer, inputs#I3). If canvas findings reach a carrier, they become new needles for the trace reads. The PROVISIONAL mark is still open, so no wrap may discharge it on its own judgement.

@history_item n=3 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (4. Span / Trace Coverage: 2 row(s); 6. Log Coverage: 2 row(s)) — `~ contract/command-span/settled` added canvas to the verb-alone list. `~ contract/tool-call-span/tool` and `~ contract/command-span/verb` moved from fifteen to sixteen. `+ contract/command-span/canvas` was added, ratified under the founder's F1 B «Глагол canvas на всех каналах». These are the counts and per-verb line rows that a new verb would move. The scope says "no new verb", so for this chunk they should stay at sixteen, and lint and canvas lines should carry no finding.

@history_item n=4 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the visual lint recorded silent and spanless) — recorded `lint.rs`, `text_extent`, `stand_lint` silent (census 0). `Finding` / `Reason` derive Debug and hold ids, closed words and numbers only, never logged, fielded or printed. Added the in-process `Session::lint` and `Harness::text_contrast` (the lint's contrast reader) to the span-absent list. It also set the "no new field" rule. A canvas finding type joins the same never-logged Debug class (`decision/debug-types-never-logged`). A contrast check reuses that reader.

@history_item n=5 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — the `lint` verb runs inside its call's `command` span. An accepted lint records `verb` alone. Findings and their count are never fielded. The founder ruled F1 «Новая команда lint» and F2 «Без поля» (no new span field). This is the base of `contract/command-span/lint` and `contract/command-answer/linted`: canvas defect words added inside the lint answer must leave the span line holding no finding or count.

@history_item n=6 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the lint's «PROVISIONAL: no new field» on the founder's own word, «Без поля (Рекомендую)». The no-span-field rule for lint is ratified, and a canvas lint must not reopen it.

@history_item n=7 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and scrub limits by hand, since its pin reads none of these claims. A kept check at rest holds a lint's findings. If the lint answer or a carrier changes, the statement and the at-rest list (Recording directory, `contract/kept-check/answer`) have to be edited by hand. The scope names both.

@history_item n=8 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the lint findings walk in `lint.rs` (`readings`, `placement`) was factored under the cognitive-complexity ceiling of 15 with private items only, and recorded silent. Extending the lint walk with canvas checks has to stay under that ceiling and keep its census at 0.

@history_item n=9 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — `Harness::text_contrast` (`contrast`, `TextContrast`) is silent, opens no span and mutates nothing. This is the reader a candidate canvas text-contrast check would reuse. It must stay on the span-absent list.

@history_item n=10 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (4. Span / Trace Coverage: 1 row(s)) — `+ decision/layout-read-opens-no-span`: a new driver read was added with no instrumentation by design, and the row was written so the absence is not read as drift. This is the precedent the canvas read's no-span row copies.

@history_item n=11 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (3. Observability Harness Contract: 2 row(s)) — `~ decision/checks-silent` and `~ decision/headless-stand-no-sink` gained `stand_layout`'s own recording subscriber as a named exception. A new canvas-lint check that installs a recorder or a sink has to be named in these exception lists, or it breaks the rows.

@history_item n=12 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (4. Span / Trace Coverage: 1 row(s)) — `~ decision/layout-read-opens-no-span`: reached by its verb, the read runs inside the call's `command` span. The same in-process-versus-verb wording applies if canvas lint findings are reached through `lint` or `canvas`.

@history_item n=13 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (6. Log Coverage; 4. Span / Trace Coverage; 3. Observability Harness Contract; 8. PII) — moved the verb count fifteen to sixteen and added `+ contract/command-span/layout`. Extended `~ decision/sink-host-read-at-trace` with layout-answer key needles. Added `+ contract/layout-answer/content` / `logging`, ratified by the founder «Глагол `layout --id`». This is the full precedent for what changes when a carrier gains a content class: needle set, span line row, content and logging contracts.

@history_item n=14 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (8. PII Scrubbing & Compliance: 1 row(s)) — `~ contract/layout-answer/logging` added the cold-agent transcript as a carrier on a live run. A lint answer carrying canvas findings would likewise cross to the model provider in the transcript. That carriage must be named.

@history_item n=15 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (3. Observability Harness Contract: 2 row(s); 6. Log Coverage: 2 row(s)) — `~ decision/part-answer-never-logged` added the graph part. `~ contract/command-span/graph` was widened so a part read's line holds no part number or count. This is the precedent the canvas part followed. A canvas finding on any paged answer keeps the same never-fielded rule.

@history_item n=16 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built) — the `command` span was built: target `escher_driver`, INFO, entered before `validate`, with eight fields each recorded only where it applies. It never fields any argument, screen text or Outcome. This is the base of `contract/command-span/span`, `/line`, `/field-count` and `decision/driver-spans-carry-no-content`. The field count stays at eight unless a fork rules otherwise.

@history_item n=17 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the span's `cause` field takes a `Cause::name`. Rule: never field a `Command`, `Call` or `ArgValue` with Debug, nor an argument by its schema name. This applies to any new lint or canvas argument (e.g. a region or bound) the chunk might add. It is the root of `decision/command-span-no-call-content`.

@history_item n=18 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Outcome` joins the Debug-deriving types that nothing prints, logs or fields. An Outcome fielded with Debug would print an id, name or value. This is the origin of `decision/debug-types-never-logged`, which covers any new finding variant.

@history_item n=19 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the session's record of ids is bounded at 4096, has no reader outside the crate and no Debug, and is never printed or fielded. This is the origin of `decision/session-id-record-unprinted`.

@history_item n=20 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `fonts-unavailable` cause, returned before anything is painted. A shot's image leaves only in its answer and is never logged, which established the shot line and the shot-image rows. A canvas check that needs font advances (`Widget::fonts`) meets the same no-font condition. It should reuse this closed cause, not mint a new one.

@history_item n=21 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — each command's answer, refusal or error is one JSON line on stdout, an answer and never a log line. Standing rule: the next surface adds no span or log field that holds what a call supplied or a screen reads. This governs `contract/escher-session-stdout/command-answer` and the exit-code / stderr rows.

@history_item n=22 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span was ratified with its two closed fields, `tool` and `ending`. This is the basis of `decision/driver-span-fields-print`. No content field may be added.

@history_item n=23 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined the spans' closed sets. The self-description and remedies are answers on stdout, never log lines. If `describe.rs` changes for a new carrier or defect word, the help text stays the program's own fixed text and adds no log site.

@history_item n=24 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span. The Recording directory row (0700, one 0600 check per session) and the check-file contract were added. This is the base of `contract/recording-file/check-file` and the kept-check rows, which a changed lint answer flows into.

@history_item n=25 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check records the answer whole (a shot's without png). It also added the "Committed checks" row (Counter only, public). This is the base of `contract/kept-check/answer`, `contract/recording-file/answer` and `decision/committed-checks-hold-no-private-data`. Canvas findings in a lint answer would rest whole in kept checks. The committed Counter checks must not gain private data.

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set, ratified) — the Recording directory's at-rest content list grew with a new answer's carried set, as a ratified boundary widening. This is the precedent: a lint answer gaining canvas defect words extends that at-rest list.

@history_item n=27 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set, at rest and in the PII boundary) — a same-pattern widening (rule `rul-izni57qv`) for a new answer's carried set, ratified by the founder. This is a second precedent for how a carrier's content change is ruled and recorded.

@history_item n=28 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (3. Observability Harness Contract: 3 row(s); 4. Span / Trace Coverage: 1 row(s); 6. Log Coverage: 2 row(s)) — `~ decision/headless-stand-no-sink`, `~ decision/driver-library-installs-no-subscriber` and `~ decision/driver-emits-no-event` changed to name escher-telemetry's scoped capture around an acting step. `~ contract/telemetry-sink/rust-log` and `~ contract/escher-sink-env/rust-log` changed so the filter applies to the sink layer alone. A canvas lint is a non-acting read: it runs under no capture and must add no subscriber.

@history_item n=29 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. Observability Harness Contract: 4 row(s); 4. Span / Trace Coverage: 1 row(s)) — current wording of `decision/checks-silent`, `decision/headless-stand-no-sink` and `decision/answers-echo-nothing-typed` (caused-item exception). `decision/driver-library-installs-no-subscriber` gained the chaining panic hook. `decision/driver-emits-no-event` now says the capture keeps records and panics in process. These rows bind the chunk's new checks and driver code.

@history_item n=30 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (3. Observability Harness Contract: 1 row(s)) — `~ decision/answers-echo-nothing-typed` except clause: an `error` field is shown as written, cut at the text bound. This is the row's current exception. Canvas findings must echo nothing a caller typed. Ratified by the founder at 2026-11-17-a-component-in-isolation.

@history_item n=31 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Observability Harness Contract: 1 row(s)) — `~ decision/checks-silent` except clause gained "the order page logs". This is the current wording. A planted canvas fixture that pins code adding a log site would need another named exception.

@history_item n=32 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` widened the re-run-child exception to "ids or digests". A canvas-lint check that re-runs a child (e.g. for cross-OS digests) falls under it.

@history_item n=33 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-session` became the second sink installer under `service.name=seven_guis`. Trap: a zero read from a host that installs no sink says nothing about one that does. This is the basis of `decision/scrub-sink-reach`. It is also why canvas content must be read at trace through the sink-installing host, not only in process.

@history_item n=34 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — only the windowed binary installs telemetry, and the headless stand installs none. This is the origin of `decision/headless-stand-no-sink` for the in-process stand checks (both layout modes) the chunk extends.

@history_item n=35 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — every area's master text became rows (315 records → 281 rows), verified against the code. Every picked row's handle dates from this conversion. Later entries amend rows by handle.
