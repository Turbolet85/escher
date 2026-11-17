@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand registers the bundled DejaVu Sans for every generic with system fonts off on native (`build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`); the isolated mount must boot with this same font context so its text measures, and its bounds compare, as the stand's do.

@history_item n=2 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's font registration now lives in architecture §Standard Contracts → Headless stand beside `font_ctx()`, not as a design-system value; the custom `FontContext` item (contract/native-config/font-context) stands. The isolated boot's "stand font" is that architecture contract, and any new boot path must reuse it rather than restate a typography value here.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2's stand-row move ratified; stand lines leave design-system) — the stand-only values (font registration, colours, type, select padding, served-page groups) are held in layout-templates and architecture, ratified by the founder; surface's cli item naming the served tasks was kept "as is". An isolated-mount carrier that names components or tasks extends that kept cli item, and stand values it reads belong to layout-templates, not this plan.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable`; the reason behind decision/no-generic-family-lays-out-sans. An isolated document built without the stand font context would trip this refusal, so the isolated boot must carry the stand's `font_ctx`.

@history_item n=5 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — added decision/unavailable-families-use-default-font (guard `font_family_fallback`): with no listed family available, text lays out in the engine default font. A trap for the isolated mount: if its document lacks the stand font, readings silently differ from the in-app reading rather than fail.

@history_item n=6 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the default user-agent sheet reaches every document once, through `DocumentConfig::default()` (the row decision/default-ua-sheet-via-default-config); a new isolated-mount document must be built through the default config, or its styling and so its bounds diverge from the in-app subtree.

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle answers every member of the animating set the same way, none waited on, returning `Settled` with `animating` read off the document's flag (decision/settle-does-not-wait-on-animation); the isolated mount's settle inherits this rule unchanged.

@history_item n=8 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock` its documents read, settle never advancing it (decision/animation-driven-by-virtual-clock); the isolated document must read the same harness clock so its readings are deterministic like a stand task's. Ratified at Epoch 2 close (2026-10-18-engine-debts ratification entry).

@history_item n=9 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — ratified the one-virtual-clock motion rule among three; the precedent for how a delegate mark is lifted by a new entry, not by rewording — the form this chunk's wrap follows when it records the founder's lift of the 24 delegate marks.

@history_item n=10 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — on every Dioxus document, windowed and headless, a render is recorded into a mutation buffer unborrowed then replayed under one borrow (decision/buffered-render-replay), with the limit that a render-time call sees the previous render's document. A component rooted alone through `Harness::from_vdom` must keep this path; a document built around an already-built vdom (the cry-c4emakc4 boundary-marks branch) touches the same build order.

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid's into-view move carries no animation) — the async `scroll_to` from a key handler panicked `RefCell already borrowed` and did not ship; the precedent that led to the buffered render, a trap to watch if an isolated Cells component re-enters the vdom at mount.

@history_item n=12 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the cli graph answer: one uncoloured JSON line `{"nodes":[…],"states":[…]}`, the same on socket, mcp and kept check, carrying ids, owner ids, component names, state names, handler event names, closed flag words and booleans only (contract/escher-session-commands/graph-answer, contract/ui-graph-line/content); plus the unpaged-graph limit (`res-7cc5bi4k`). The isolated mount names its component across this graph's component boundary and must check its root's component name in this answer.

@history_item n=13 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the cli surface's graph answer line; ratified — the coverage role) — the graph line and the coverage role were lifted from PROVISIONAL to ratified by the founder's word, the text unchanged; the precedent for recording a lift of delegate marks as its own entry, which this chunk's wrap does for 24 operations.

@history_item n=14 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` verb on every ratified carrier as a boundary widening (pattern `rul-izni57qv`) ratified by the founder; its line carries ids, owner ids, verb words, closed cause words and booleans, never a value, typed text or `NodeId` (contract/actions-line/content); the precedent for how a new verb or argument crossing every carrier is ratified before it is built, as this chunk's fork requires.

@history_item n=15 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added contract/escher-session-commands/layout-answer and contract/escher-session-commands/layout-arguments (a missing or bad id refused `malformed`, status 1). This chunk puts `layout` into the cold-agent pipe and lifts the 15 delegate marks of this wrap; an isolated mount's props or component argument would follow the same refused-malformed rule, not a usage ending.

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the coverage role replays kept checks on a fresh session, argv files read bounded and parsed strictly, a bad file refused `malformed` holding nothing of it, no exit status added; the precedent for validating untrusted input (as props must be) to a typed refusal and for a kept check carrying a new step.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the cli serves a closed table of six tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), the usage line's `apps:` and the MCP `start` tool's `enum` listing them, an unknown task a usage ending; the precedent for the fork's "closed table of named components and prop sets on `start`" option, and for the cold-agent pipe this chunk extends with `layout`.

@history_item n=18 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the cli output form: one uncoloured JSON line per command on stdout, a usage error one fixed line on stderr with stdout empty, a session error its fixed message on stderr, exit 0/1/2/3 (the escher-session exit status, exit codes and stderr rows); any carrier the isolated mount gets must keep this form.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role: JSON-RPC lines only on stdout, a tool result's text the JSON line, an `Err` ending's fixed message on stderr with status 3 (contract/escher-session-mcp/other-results, contract/escher-session-mcp/err-ending); an MCP tool for an isolated mount would answer under this rule.

@history_item n=20 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from the schema's `EXIT_STATUSES` table served by `help`, a session error's object carrying its fixed `remedy` (decision/exit-statuses-from-schema-table, contract/escher-session-session-error/remedy); a new verb or refusal must enter the schema's tables, not a new exit.

@history_item n=21 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording's `record-dir-not-private` and `recording-failed` sit in the exit-3 class with no exit of their own (contract/escher-session-exit-status/3); the precedent that new session errors join an existing class, never a new status.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part]` answers `{"text":…}` or `{"text":…,"part":n,"parts":m}` (contract/escher-session-commands/snapshot); the isolated mount's reading equality is stated over the snapshot, so its comparison reads this shape, paged for a large component.

@history_item n=23 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (decision · contract · absence · overview), each verified against the code; every row this chunk binds (logical viewport from scale, root-relative units, the font-context contract) dates from this conversion with no later amendment, so they stand as the code showed at that read.
