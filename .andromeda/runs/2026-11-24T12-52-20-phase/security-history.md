@history_item n=1 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — (Threat Model Summary) added decision/canvas-content-crosses-as-shot-pixels: a canvas's app-painted content crosses only as shot pixels on every carrier, the model provider included, with no new verb, field, cause, exit, log field or span, and a kept check keeps no pixels; ratified on the founder's F1 «Ратифицировать для всех каналов». This chunk's draw-command verb is exactly the new carrier/field that row rules out, so it must be amended (or replaced) through a founder fork, not extended silently.

@history_item n=2 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — (Dependency Security) added decision/harness-blitz-paint-custom-widget: blitz-test-harness turns on blitz-paint's custom-widget feature in every build selection, adding no package to Cargo.lock. A read of a widget's recorded `Scene` beside `screenshot` in the harness rests on this feature set; any new crate (e.g. `anyrender_serialize` into the harness/driver) is a dependency change this row's "no package added" framing does not cover.

@history_item n=3 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — (Input Validation) added decision/custom-widget-content-box-frame (paint size, scene placement, pointer frame in the content box) and decision/custom-widget-document-font-context (the widget handed the document's live font context, "no input crosses"). The dump's coordinates are in this content-box frame, and glyph runs carry font data from this shared context — the font-by-reference/no-inline-data premise and the "authored text in glyph runs" fork bear on it.

@history_item n=4 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added decision/custom-widget-click-synthesis (click synthesized on a custom-widget target), ratified as an engine change. The chunk's boundaries keep click synthesis, placement, pointer frame and font hand-off as built; the stand's one canvas is Circle Drawer, which the dump measures.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a custom widget's attributes replayed at set-up) behind `custom-widget`, `set_custom_widget` replays each existing attribute, page content included, to `Widget::attribute_changed`, the value's check staying the widget's own. Authored attribute text can reach a widget and so its painted scene (glyph runs), which matters for what a draw-command dump carries out.

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the screen's pixels in band) `screenshot` added as the eleventh verb: an optional `id` looked up as a click's, `FieldKind::Png`, `fonts-unavailable` refusing before paint, a crop of no pixel `off-screen`, image never fielded, answer under the unchanged 1,048,576-byte bound; pixels ruled a boundary widening (no closed value set). Direct precedent for a new canvas read beside `screenshot`: per-verb schema, field kind, refusal order, carriers and the widening question.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (engine's password mask, nine causes, no image in a log, `png`) a shot's image reaches no log (first 64 base64 chars 0 times at `trace`, both builds); `png` reviewed as the shot encoder with no package entering the lock. Sets the logging test pattern (`host_log`/`mcp_log` needles) and the dependency-review pattern a dump verb must repeat.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (closed-field rule) founder's ruling: a new field of a closed value set inside a ratified carrier is no widening; a field holding authored text, an open value set, a new carrier or a new exit still is (now decision/boundary-widening-rule, decision/closed-value-field-no-widening). Command kinds/counts may sit inside the rule; glyph-run text or glyph ids (authored text in another form), float coordinates and font digests are the open-value/authored-text classes that come to the founder.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a shot of an open list holds labels as pixels) page text carried as pixels in a shot was itself ruled a boundary widening and ratified separately. Precedent that authored text crossing in a new form (here: glyph runs in a dump) needs its own ratification even when pixels of it already cross.

@history_item n=10 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built `validate`: a closed table of verbs and bounded argument kinds, one answer per call in fixed order, no panic/unwrap on call input, no clock; rule for later chunks: every surface reaches `validate` first and every refusal text stays fixed and free of call content. A sixteenth verb (or a new argument on an existing one) enters this table and its rules (decision/closed-driver-verb-schema, decision/single-validation-gate).

@history_item n=11 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — after `validate` an unknown id is refused `stale` or `not-found`, and the session keeps a bounded id record fed only from its own snapshots; widening the verb table was ratified as widening a validated surface. A canvas-id-taking read resolves through this same path (decision/unknown-id-refused-stale-or-not-found, decision/session-id-record-bounded).

@history_item n=12 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — a caller-supplied id is looked up anew on every call through `get_node`, never by index, with no node handle kept between calls; nothing public runs a command not from `validate`. The canvas read named by stable id must keep this (decision/id-lookup-anew-each-call, decision/no-node-id-between-calls).

@history_item n=13 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — one INFO `command` span per call, fixed words, booleans and counts only; never any argument, refusal fault, screen text or its length. A dump verb records `verb` (and `cause` when refused) alone; a command count or byte size on the span would be a new log field question.

@history_item n=14 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's span-line class and that a diff's three sizes may reach a log while its content never does; standing rule: a count may reach a log, a node/id/name/value field is a new crossing question. Bears on whether a draw-command diff's or dump's counts are logged (decision/diff-counts-only-in-logs).

@history_item n=15 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's sink drops every outside-target record whole at every level and redacts content-named fields of `escher_` targets; trap: a "nothing in the logs" check must run against a sink-installing host (`host_log`). A dump read's no-log claim must be proven there, not in a sinkless process (decision/telemetry-sink-target-allowlist, decision/log-sink-redacts-and-allowlists).

@history_item n=16 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (the driver session socket) Unix-domain socket in a 0700 state dir, closed grammar, one connection at a time, ratified; standing: a command carrying screen content reopens the crossing question. The dump verb is new screen content on this carrier (decision/driver-session-unix-socket-only, decision/session-socket-closed-grammar).

@history_item n=17 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session socket carries calls and answers) second widening: ids, names, masked values, snapshot text and diff out, no NodeId; answers bounded at 1,048,576 bytes, oversize reply; the command line became an external input surface with closed argv and no echo. The dump's carriers, bounds and CLI flags inherit these rows (decision/session-socket-carries-calls, decision/cli-closed-argv-reader, decision/no-echo-of-caller-input).

@history_item n=18 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (MCP server's input and crossing, ratified) stdio-only MCP, bounded lines and results, typed args before `validate`, no auth; "the next carrier is a new question". A new MCP tool for the dump extends `tools/list` and results under these bounds; what the client sends on to a model provider is the client's (decision/mcp-result-crossing, decision/mcp-results-client-responsibility).

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (`serde_json` reviewed; `tool_call` span) `serde_json` chosen as read-only (no `preserve_order`), writing nothing; replies hand-written. If the dump is produced via serde (anyrender's `RenderCommand` derives or `anyrender_serialize`), that departs from decision/serde-json-read-only and needs its own decision.

@history_item n=20 marker=2026-10-12-self-description
**2026-10-12-self-description** — (`help` and fixed program text) `help` ratified; the program's own fixed table text (verb help, result field name/kind/help) crosses the carriers; session errors as one fixed table. A new verb adds `describe.rs` help and MCP listing text under decision/mcp-listing-fixed-text and decision/session-errors-fixed-table.

@history_item n=21 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the refusal cause set narrowed to eight (`time-unavailable` retired), closed; new verb shapes are refused within the set, never by adding a cause. A dump verb (e.g. a non-canvas id, a widget-less element, fonts) must map onto the eight (decision/closed-refusal-cause-set).

@history_item n=22 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — kept checks at rest: 0700 directory, 0600 files, 400 records / 1,048,576 bytes ending the recording, strict reader `Check::from_bytes`, unix only. A new instance verb is recorded with its answer; dump bytes count against the check bound (decision/recording-directory-owner-only, decision/recording-bounds-end-recording).

@history_item n=23 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check keeps each answer whole, masked as on every carrier, a shot kept as width/height and never pixels; two committed stand-Counter checks hold no screenshot step. Whether a kept check holds a canvas dump (and how, given "a kept check keeps no pixels" for canvas content) is decided here (decision/kept-check-answer-whole, decision/committed-counter-checks).

@history_item n=24 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: `lint` on every carrier) precedent for an in-process reader turned verb: twelfth verb, no argument, findings of ids/closed words/numbers, never fielded, kept with answer whole, cold-agent tool set updated; ratified as a widening onto every ratified carrier. The template for a sixteenth verb's checklist across schema, MCP, recording, cold-agent and logs.

@history_item n=25 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `part` (1 to 1000) and paged answers under the 10,000-byte snapshot budget, a part's number and count never fielded. The dump's size is measured against this same budget; paging it would reuse `part` (contract/driver-argument-kinds/part).

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the UI graph joins what the carriers take out) graph answer crossing on socket, command line, MCP and kept check ratified, its carried set listed in the threat model and `docs/capabilities.md`. A dump verb likewise updates the carried sets on each boundary and the capability statement.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the thirteenth verb, graph) verb count and instance-verb count moved in schema, MCP, settle row; argument refused `malformed`, no cause added; read verbs wait on no load and move no time. Counts in rows (contract/recorded-check/tool, settle rows) move again with a sixteenth verb.

@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph answer's size under the answer bound) graph kept unpaged (Cells 293,869 bytes), the size a stated limit owned by residual res-7cc5bi4k; later paged. Precedent for measuring the dump's size and stating a limit or paging it.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (a recorded graph answer at rest) kept checks gain the graph answer at rest, ratified. A dump answer kept in a check would need the same data-protection row.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the verb counts the checks read) cold-agent `valid_step` verb set and recorded-check tool set move with each verb. The dump verb must join the cold-agent `VERBS` and the instance verb set (decision/cold-agent-check-validation, contract/recorded-check/tool).

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the reserved virtual-node id form) the id grammar reserves `{owner}///{key}`, no served id holding it; a stable-id contract change ratified. The scope defers virtual nodes and grouping (v040-11), so the dump must not serve `///` ids.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph verb's silence in the logs) `graph` records `verb` alone; `host_log`/`mcp_log` drive one call with 0 needles at trace under both builds. The dump verb owes the same silence proof (decision/no-screen-content-in-logs).

@history_item n=33 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (actions answer in the threat model's carried sets / on the socket and in kept checks) `actions` crossing ratified on every carrier, carried sets updated, replay of a kept step owed. Same carriers' update set as this verb.

@history_item n=34 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the verb table at fourteen / validation and answer lines / the shared target check) `actions` tenth instance verb, new `FieldKind`, `malformed` on argument, one shared `check_target` for every id-taking verb and the actions read. A canvas-id verb either joins `check_target` and the actions listing or states why not (decision/actions-read-shares-target-check, contract/actions-answer/verbs).

@history_item n=35 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the unpaged actions answer's size; silence in the logs) actions answer unpaged under the bound (Cells 106,088 bytes), limit owned by res-7cc5bi4k; 0 needles in host/mcp logs. Size and log-silence measures the dump verb repeats (decision/actions-answer-unpaged).

@history_item n=36 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the coverage role reads caller-named kept checks as untrusted input through the bounded strict reader and replays them. A new verb's kept steps flow through this reader too (decision/coverage-check-files-untrusted, decision/recorded-check-untrusted).

@history_item n=37 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — kept check opening record gained `format` (other values refused `malformed`), explored paths kept in the same owner-only medium. If the dump changes what a check holds, the format/start-record contract is the place (contract/recorded-check/start-record, contract/kept-check-file/entry, contract/kept-check-file/medium).

@history_item n=38 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — added decision/fixture-name-closed-table (fixture state names checked whole against a closed table before boot). The scope's premise that a fixture could serve a canvas to the command line/MCP would ride this rule.

@history_item n=39 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (six served slugs) `escher-session` serves a closed six-entry `TASKS` table, the MCP `start` app enum listing them as fixed text; Circle Drawer is not among them. The scope premise: a dump verb on CLI/MCP/kept check reaches no stand canvas until served; serving it widens this closed table.

@history_item n=40 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the pipe's second live run, a third carriage) six apps' screens incl. a screenshot and lint findings reached the model provider, ratified as content widening of the 2026-10-15 crossing. What a dump carries to the provider is the same founder fork the scope names.

@history_item n=41 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the live child's kept checks) the live server records owner-only checks read for bools and counts only, never printed or committed (decision/cold-agent-checks-read-for-counts-only, decision/cold-agent-records-private). A dump in a live run's kept checks stays under this.

@history_item n=42 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — cold-agent pipe on the escher server: the screen to the provider, the agent's check as untrusted input (last fenced json block, ≤ 65,536 bytes, ≤ 400 steps, known verb). A new verb joins this tool set and validation (decision/model-reply-untrusted-input).

@history_item n=43 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — decision/cold-agent-screen-to-provider gained layout answers after the pipe's tool set took `layout`; contract/driver-session-reply/no-attribute-text gained its authored-text exception (record message, caused `error`). Adding the dump to the cold-agent tool set amends the provider row again; authored glyph text would be a further exception.

@history_item n=44 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** — decision/cold-agent-screen-to-provider gained graph and actions answers on the founder's F4; decision/cold-agent-events-metadata-only counts refusals per closed cause. Precedent: each new answer crossing to the model provider was ruled by the founder.

@history_item n=45 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — decision/cold-agent-screen-to-provider gained acting answers' caused text (authored text) on the founder's F5. Closest precedent for authored text reaching the provider — the "authored text in glyph runs" fork.

@history_item n=46 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — caused text crossing replaced the closed-words-only shape; decision/socket-carries-screen-reading and decision/no-screen-content-in-logs extended; authored text ruled a widening. Shows how an authored-text class is added to the carried set and kept out of logs.

@history_item n=47 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — decision/socket-carries-screen-reading restated (caused scrubbed by the answer's own field names, error kept); delegate confirm later ratified. The dump's content joins this row's list.

@history_item n=48 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — decision/driver-installs-no-subscriber and decision/driver-no-subscriber-no-env gained the scoped-capture exception. A read verb runs no step, so it should stay outside that capture and add no env read or subscriber.

@history_item n=49 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added decision/session-layout-read-in-process (in-process read resolving its id via the run path, running no step/pump/tick/settle, answering ids, closed words and numbers) and named the layout answer in decision/no-screen-content-in-logs. A read-only, id-taking, no-step canvas read follows this shape.

@history_item n=50 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (Input Validation) `layout` became the fifteenth verb: socket-carries-screen-reading gained layout readings, the in-process read rule widened to every carrier, contract/recorded-check/tool to eleven instance verbs, contract/driver-verbs/layout added. The most recent template: this chunk is the sixteenth verb, moving the same rows.

@history_item n=51 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (Threat Model Summary) contract/actions-answer/verbs: every listed node lists `layout` with no cause; select-list crossing gained the layout reading. A canvas dump verb listed by `actions` changes this promise again.

@history_item n=52 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (Logging & Monitoring; Data Protection) contract/command-span/verb lists graph/actions/layout as verb-alone; decision/kept-check-layout-answer added. The dump verb joins the span list and needs a kept-check row.

@history_item n=53 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — decision/mask-holds-on-every-carrier guard gained `stand_masks_on_new_answers` covering each new answer. A dump answer must hold the mask (contract/driver-session-reply/masked-control-values, contract/mcp-result/masked-control-values), e.g. a password painted into a widget.

@history_item n=54 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — (delegate confirms ratified) the founder ratified the layout-verb and caused-error delegate confirms. Pattern: a delegate confirm of a crossing stands PROVISIONAL until his own word.

@history_item n=55 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — graph became paged (decision/graph-answer-paged, part 1 to 1000 under the 10,000-byte budget), contract/refusal-causes/malformed and contract/command-span/verb restated (never the part asked or count), kept-check call content gained graph parts. If the dump pages, it follows this shape and these rows move.

@history_item n=56 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — carried sets reconciled and `docs/capabilities.md` committed as the statement of the MCP crossing (decision/capability-statement-mcp-crossing), with a secret search over it. The scope touches `docs/capabilities.md` and the README tool count; the carried set there must name the dump.

@history_item n=57 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting steps wait on loads at most `LOAD_WAIT`; `snapshot` and `screenshot` wait on none. A canvas read waits on no load and moves no time (decision/bounded-load-wait, decision/time-moves-only-on-advance).

@history_item n=58 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — zip 2.4.2's three advisories ignored by ID: zip reaches the graph only through `anyrender_serialize 0.8.0` ← `browser`, reading archive entries into memory. The scope's premise weighs `anyrender_serialize` for the dump; taking it into escher-driver/the harness brings zip and its ignored advisories into escher's own crates (decision/advisory-ignores-by-id).

@history_item n=59 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (zip ignores gain RUSTSEC-2026-0429) the reach restated as `anyrender_serialize`'s `deserialize` opening a `ZipArchive` and never calling `extract`, owned by residual res-h7acz56c. A new consumer of `anyrender_serialize` changes the reach these ignore reasons assert.

@history_item n=60 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (advisories published after a green audit) CI audit can go red over an unchanged lock; run `ci-leg.sh audit` locally before a push (decision/local-audit-before-push).

@history_item n=61 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — (Dependency Security) decision/upstream-sync-dependency-set amended when the AnyRender 0.15 set entered from the registry. The `GlyphRunCommand`/`RenderCommand` shapes the dump reads, and `anyrender_serialize` 0.9.0, come from this set.

@history_item n=62 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `host_log` became a standing check of a recording session's clients at `trace` with 0 needles. The dump verb's no-log proof rides this guard (decision/no-screen-content-in-logs).

@history_item n=63 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — whole-workspace cargo-deny audit with licence gate, and `coverage_floor.py` input validation; per-crate floors later held (decision/per-crate-coverage-floors: escher-driver 97). A new verb in escher-driver must keep its floor, and any new dependency passes the licence gate.

@history_item n=64 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — snapshot text form writes names in `str`'s Debug form so a field never holds a line break or bare quote; standing rule that the first command exposing a new text form is a new crossing. A line-per-command dump with any text field follows this quoting (contract/snapshot-text/name-field) and asks the crossing question.
