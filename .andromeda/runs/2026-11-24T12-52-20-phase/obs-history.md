@history_item n=1 marker=amd-e5lmdcaj
**amd-e5lmdcaj** — (2026-10-07-command-and-refusal-schema) set the standing rule that the span's cause field takes a `Cause` name and that no `Command`, `Call` or `ArgValue` is ever fielded with `Debug`, nor an argument by its schema name. A canvas verb's new argument (canvas id, level) and its command variant fall under this rule.

@history_item n=2 marker=amd-3iczuidb
**amd-3iczuidb** — (2026-10-07-act-by-id) added `Outcome` to the `Debug`-deriving types that nothing prints, logs or fields. A new canvas outcome, which holds draw commands and possibly glyph or authored text, is one more such type (row decision/debug-types-never-logged).

@history_item n=3 marker=amd-oqhtag4e
**amd-oqhtag4e** — (2026-10-07-refusal-detection) set up the session's record of ids: bounded, with no reader outside the crate, no `Debug`, never printed. It is the basis of row decision/session-id-record-unprinted, which a canvas read by stable element id must leave unprinted.

@history_item n=4 marker=amd-cschmkve
**amd-cschmkve** — (2026-10-07-driver-command-spans) built the `command` span: INFO at `escher_driver`, entered before `validate`, eight fields each recorded only where it applies, and a never-fielded list (any argument, `Command`, `Outcome`, the screen text). escher-driver takes `tracing` ungated. These are the span and field rows a new verb must keep, adding no ninth field.

@history_item n=5 marker=amd-u2nw2mep
**amd-u2nw2mep** — (2026-10-09-founder-rulings) the founder ratified the closed span's line and ruled that a diff's three sizes may reach a log while its content never does. A diffable canvas dump's added, removed and changed counts may reach a log only on that ruling, and the dump's lines never.

@history_item n=6 marker=amd-dnjkg5io
**amd-dnjkg5io** — (2026-10-10-driver-cli) the command line writes each answer as one JSON line on stdout, an answer and never a log line, and an `Outcome` is written in one place, `to_json`. Standing rule: the next surface adds no span or log field that holds what a call supplied or what a screen reads. Basis of contract/escher-session-stdout/command-answer.

@history_item n=7 marker=amd-gftbr5vi
**amd-gftbr5vi** — (2026-10-10-driver-cli) typed text was measured in a sink-installing host's log at `trace` under both builds, 0 needles. This set the pattern the row decision/sink-host-read-at-trace generalizes, so a new canvas verb's answer needs its own needle reading in `host_log` and `mcp_log`.

@history_item n=8 marker=amd-vm35icer
**amd-vm35icer** — (2026-10-07-driver-session) trap: a zero read from a host that installs no sink says nothing about one that does. A canvas read measured only in process does not prove the escher-session host's log clean.

@history_item n=9 marker=amd-3jlpr7el
**amd-3jlpr7el** — (2026-10-10-mcp-surface) ratified the `tool_call` span: one per MCP `tools/call`, exactly two closed fields `tool` and `ending`, and `command` as its child. A new MCP tool for the canvas read joins `tool`'s closed set and adds no field (the tool-call-span rows).

@history_item n=10 marker=amd-hk2vxiq4
**amd-hk2vxiq4** — (2026-10-12-self-description) `help` joined both spans' closed sets with no field added. The self-description and its remedies are an answer, never a log line. A new verb's `help` entry follows this.

@history_item n=11 marker=amd-wivka2yb
**amd-wivka2yb** — (2026-10-12-headless-screenshot) the screenshot verb's precedent for a paint-derived read: an accepted shot records `verb` alone, and the image, its byte size and its width and height are never fielded. 0 image bytes were measured on host and mcp stderr at `trace`. A canvas dump of scene commands is the nearest analogue, and its size and contents stay unfielded.

@history_item n=12 marker=amd-2iwce5bg
**amd-2iwce5bg** — (2026-10-31-select-popup) verbs' help and meaning texts are the program's own `'static` text, add no log site or span field, and the `command` span keeps its eight fields (row contract/command-span/field-count). Help and meaning text for the canvas verb sit under the same rule.

@history_item n=13 marker=amd-twhohqtg
**amd-twhohqtg** — (2026-10-31-select-popup) a shot of an open list carries page option labels as pixels, a boundary widening ratified by the founder. This is the precedent for authored text crossing in painted form, relevant to the founder's fork on authored text in glyph runs.

@history_item n=14 marker=amd-vrhzfubf
**amd-vrhzfubf** — (2026-10-19-visual-lint) the founder ruled on the `lint` verb: a new verb, no new span field («Без поля»). An accepted `lint` records `verb` alone and its findings and their count are never fielded. The verb and tool word counts rose by one. This is the template for adding a verb with no field.

@history_item n=15 marker=amd-4q2wobhh
**amd-4q2wobhh** — (2026-10-20-large-screen-in-parts) a part read records `verb` alone, and a part's number and the part count are never fielded. Relevant if the canvas dump is paged against the 10,000-byte snapshot budget.

@history_item n=16 marker=amd-wv4t22sd
**amd-wv4t22sd** — (2026-11-07-ui-graph) the `graph` verb joined `Session::run` under the same one `command` span. `Outcome::Graphed` is written by `to_json` alone and never logged or fielded, and the `session_common` readers grew. This is the pattern for a read verb's harness-contract entry (row decision/graph-answer-never-logged).

@history_item n=17 marker=amd-tlxklvv6
**amd-tlxklvv6** — (2026-11-07-ui-graph) the graph answer's carried set at rest in the Recording directory row, and the §8 content item, were a boundary widening, later ratified (see amd-spdvdmvv). A canvas answer kept in a check extends the same at-rest list and needs its own content contract and ruling.

@history_item n=18 marker=amd-spdvdmvv
**amd-spdvdmvv** — (2026-11-15T05-21-42-wrap) the founder ratified the graph answer's carried set, which had been PROVISIONAL on the overseer's delegate confirm. A carrier widening waits for the founder's own word, not a delegate's.

@history_item n=19 marker=amd-jsqwtiq2
**amd-jsqwtiq2** — (2026-11-07-ui-graph) graph's `command` and `tool_call` lines: the verb table and tool word counts rose by one. A graph line holds no node, state, id or name. `stand_act_spans` drives one call per mode, `host_log` and `mcp_log` read 0 needles, and the `tool_call` parents one `command` child. This is the checklist a 16th verb repeats.

@history_item n=20 marker=amd-htjat4nf
**amd-htjat4nf** — (2026-11-07-ui-graph) dioxus-native-dom's new graph modules were recorded as log-silent by a silence census. New read code in blitz-paint and blitz-test-harness for the canvas scene needs the same census row.

@history_item n=21 marker=amd-kn6x2k72
**amd-kn6x2k72** — (2026-11-07-ui-graph) the cold-agent pipe's `VERBS` and its fake server answer gained `graph`, uncounted. A new verb must also enter the pipe's verb list and its fake server.

@history_item n=22 marker=amd-quujnooh
**amd-quujnooh** — (2026-11-08-what-can-i-do-here) the actions answer's carried set at rest and in the PII boundary: a boundary widening on pattern `rul-izni57qv`, ratified by the founder; a typed sentinel and `MASKED_VALUE` read 0 times. This is the precedent for the canvas answer's content row (contract/actions-answer/logging).

@history_item n=23 marker=amd-k5v64vz2
**amd-k5v64vz2** — (2026-11-08-what-can-i-do-here) `actions` joined the `command` verbs and the `tool_call` tools. An actions line holds nothing of its answer, an accepted read records `verb` alone, and an argument is refused `malformed` (row contract/command-span/actions).

@history_item n=24 marker=amd-ei5tfcg5
**amd-ei5tfcg5** — (2026-11-08-what-can-i-do-here) the actions read opens no span and changes nothing in the document; `actionable_nodes` is read-only. This is the precedent for a canvas read that "runs no step and changes nothing" (row decision/actions-read-changes-nothing).

@history_item n=25 marker=amd-d3eh2inx
**amd-d3eh2inx** — (2026-11-08-what-can-i-do-here) `actions_needles` in the seven_guis shared module and `stand_act_spans`' call count from 36 to 38. `host_log` and `mcp_log` read 0 needles at `trace` under both builds. A canvas verb needs its own needle set and call-count move.

@history_item n=26 marker=amd-outdow3i
**amd-outdow3i** — (2026-11-08-what-can-i-do-here) the cold-agent pipe's VERBS reached fourteen, with `actions` uncounted. This count history is what a 16th verb extends.

@history_item n=27 marker=amd-vqwvafxb
**amd-vqwvafxb** — (2026-11-16-layout-explained) added decision/layout-read-opens-no-span: a new driver read with no instrumentation by design is stated in §4 so the absence is not read as drift. A canvas read needs the like row.

@history_item n=28 marker=amd-wjfn5pka
**amd-wjfn5pka** — (2026-11-17-layout-on-the-carriers) the layout read, once reached by a verb, runs inside that call's `command` span and opens none of its own. This is the latest precedent for a read put on the carriers.

@history_item n=29 marker=amd-ypcavwq4
**amd-ypcavwq4** — (2026-11-17-layout-on-the-carriers) contract/command-span/verb went from fourteen to fifteen words, and contract/command-span/layout was added (verb alone when accepted, verb and cause when refused). A 16th verb moves the count again and adds a per-verb line contract.

@history_item n=30 marker=amd-vstyxann
**amd-vstyxann** — (2026-11-17-layout-on-the-carriers) contract/tool-call-span/tool went from fourteen to fifteen, and contract/command-span/settled's verb-alone list gained layout. Both rows change again for a canvas verb.

@history_item n=31 marker=amd-etvpmur5
**amd-etvpmur5** — (2026-11-17-layout-on-the-carriers) the needle set of decision/sink-host-read-at-trace was extended with layout-answer keys. A canvas answer's keys and command lines likely join that needle set.

@history_item n=32 marker=amd-w7erycve
**amd-w7erycve** — (2026-11-17-layout-on-the-carriers) added contract/layout-answer/content and contract/layout-answer/logging on the founder's ruling «Глагол `layout --id`». This is the template for a canvas answer's content and logging contracts.

@history_item n=33 marker=amd-zbdqebxv
**amd-zbdqebxv** — (2026-11-17-a-component-in-isolation) contract/layout-answer/logging gained the cold-agent transcript to the model provider as a carrier. What a canvas answer carries to a model provider is the founder's fork for this chunk.

@history_item n=34 marker=amd-mxcnbjck
**amd-mxcnbjck** — (2026-11-17-a-component-in-isolation) the founder later ratified the delegate confirms on the layout carriers and the caused error field himself. A carrier ruling stays PROVISIONAL until his own word.

@history_item n=35 marker=amd-6r5naojb
**amd-6r5naojb** — (2026-11-18-cold-agent-bar) reversed the uncounted-verbs decision: graph, actions and layout became counted verbs, each required by capability-unused. Whether a canvas verb is counted, with a verdict field, is a choice this decision now frames.

@history_item n=36 marker=amd-bwz3xm3u
**amd-bwz3xm3u** — (2026-11-18-cold-agent-bar) verdict fields `graph_calls`, `actions_calls`, `layout_calls`, `list_opens` and `refused_by_cause` were added under decision/verdict-fields-closed. A canvas verb's verdict field would follow this form (row contract/cold-agent-verdict/refused-by-cause).

@history_item n=37 marker=amd-vsk5tp2q
**amd-vsk5tp2q** — (2026-11-18-cold-agent-bar) the cold-agent pipe reads `server.log` only to count span lines, and the rule was shortened so it no longer lists verbs. The count history of the rows contract/cold-agent-verdict/counts-agree and refused-by-cause.

@history_item n=38 marker=amd-bcuh7k4j
**amd-bcuh7k4j** — (2026-10-25-capability-statement) `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of them, so a change to the span set is made there by hand (row decision/capability-statement-updated-by-hand). The scope names `docs/capabilities.md` for a new verb.

@history_item n=39 marker=amd-rbvxics4
**amd-rbvxics4** — (2026-10-19-session-recording) recording opens no span and logs nothing, and the Recording directory row (`0700`, one `0600` check) was a ratified at-rest widening. A canvas verb's answer kept in a check lands under these rows.

@history_item n=40 marker=amd-6x37n7n2
**amd-6x37n7n2** — (2026-10-19-recorded-check-replay) a kept check holds each answer whole, a shot's without `png`, and committed checks are the Counter scripts only, holding no private data. A heavy canvas answer (font or image data) follows the shot's precedent of keeping such data out (rows contract/recording-file/answer, decision/committed-checks-hold-no-private-data).

@history_item n=41 marker=amd-zogtm42c
**amd-zogtm42c** — (2026-10-25-cold-agent-bar) the host's served slugs come from the private `Served` table, and the usage line's `apps:` and the MCP `start` enum follow it. Circle Drawer is not served, so a carrier test of a canvas verb needs a served canvas or a fixture.

@history_item n=42 marker=amd-kdgejat5
**amd-kdgejat5** — (2026-11-18-served-error-page) contract/escher-session-exit-codes/2 reads "the served task names" with no count. Serving Circle Drawer moves no row text there.

@history_item n=43 marker=amd-g62h26kx
**amd-g62h26kx** — (2026-11-16-error-text-in-the-answer) set decision/driver-library-installs-no-subscriber (the scoped capture and chaining panic hook, printing nothing) and the exception to decision/answers-echo-nothing-typed. These are the rows a canvas read's answer and refusals must hold.

@history_item n=44 marker=amd-5rrv4k4t
**amd-5rrv4k4t** — (2026-11-16-error-field-in-the-answer) the current exception wording of decision/answers-echo-nothing-typed: a caused item's `error` field is shown as written. Ratified per amd-mxcnbjck.

@history_item n=45 marker=amd-z4s54auy
**amd-z4s54auy** — (2026-11-16-error-text-in-the-answer) current rule of decision/driver-emits-no-event: the scoped capture keeps the step's records and panics in process and prints nothing, and the driver's telemetry is its spans alone. A canvas read adds no event.

@history_item n=46 marker=amd-77iqla2t
**amd-77iqla2t** — (2026-11-16-error-field-in-the-answer) current rule of decision/scrub-by-field-name: by the content-named set in the sink and by the answer's hidden set in the capture. Any canvas field name reaching a log is judged by it.

@history_item n=47 marker=amd-q624ly3m
**amd-q624ly3m** — (2026-11-08-handler-coverage) the coverage role's replayed steps each write their own `command` span, and the coverage line is never logged (row decision/coverage-line-never-logged). A kept canvas step replays through the same path. Ratified per amd-fznt7mdc.

@history_item n=48 marker=amd-csae2x3f
**amd-csae2x3f** — (2026-11-14-auto-exploration) contract/escher-session-coverage/malformed: a check of another format is refused `malformed`. Relevant if the kept-check format moves to carry a canvas answer.

@history_item n=49 marker=amd-mwdyvgex
**amd-mwdyvgex** — (2026-11-20-driver-debts) the graph answer gained a part form, and decision/part-answer-never-logged names both part kinds. This is the precedent if a full-level canvas dump over the budget is paged.

@history_item n=50 marker=amd-magvjdff
**amd-magvjdff** — (2026-11-20-driver-debts) contract/command-span/graph covers a part read and a `part` outside 1 to 1000 refused `malformed`, holding no part number or count. This is the span-line wording for a paged read verb.

@history_item n=51 marker=amd-eqkseifg
**amd-eqkseifg** — (2026-11-24-upstream-sync-ahead-of-inside-the-canvas) the previous chunk of this epoch added decision/dropped-data-never-logged for a new class of user data. A new data class, such as a canvas's authored text in glyph runs, gets its own never-logged PII row.
