@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand registers the bundled DejaVu Sans for every generic on native (`build_single_font_ctx` through `HarnessOptions.font_ctx`); the origin of the «bundled fonts» capability §What it can do must state, beside the WASM bundled-font rows (decision/wasm-bundled-font-context, contract/native-config/wasm-font-context).

@history_item n=2 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the escher-session output form: one JSON line per command on stdout, usage/session errors fixed text on stderr, no colour, no ESC byte, no terminal test, exit classes 0·1·2·3; the base of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the stdout/stderr rows the statement describes — any prose on the driver's output must keep these words.

@history_item n=3 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: newline-delimited JSON-RPC 2.0 lines only on stdout, a tool result's text the JSON line (screen text for `snapshot`), `Err` ending on stderr status 3; underlies contract/escher-session-mcp/* rows and the README's MCP sentence («the fifteen driver commands as tools») the pin holds.

@history_item n=4 marker=2026-10-12-self-description
**2026-10-12-self-description** — `escher-session help` self-description line, the `EXIT_STATUSES` table and a session error's fixed `remedy`; underlies contract/escher-session-commands/help, contract/escher-session-mcp/help-result and contract/escher-session-session-error/remedy that the statement's driver sections cite.

@history_item n=5 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a screenshot whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; the shot's `{"width","height","png"}` line and the MCP image+text blocks (founder's fork 1); the source of decision/screenshot-never-skips-text, both fonts-unavailable contracts and the screenshot rows — the bundled-fonts claim and any font limit must agree with it.

@history_item n=6 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording's `record-dir-not-private` and `recording-failed` joined the exit-3 session-error class with no new exit status; a precedent that new 0.3.0 commands keep the four exit statuses, which the statement's endings prose must not contradict.

@history_item n=7 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — paged `snapshot [--part]` answer `{"text","part","parts"}` and its MCP result as that JSON line (founder's F1); source of contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result, and the paging precedent the unpaged graph/actions limits are stated against.

@history_item n=8 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the 0.2.0 cold-agent bar's amendment: escher-session serves six tasks (Cells and the load page added), the bundled DejaVu Sans for every boot; precedent for what the README's «done» and the next «Cold-agent bar» (v030-15) entry own, which this chunk must leave to that entry.

@history_item n=9 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's font registration moved out of design-system to architecture §Standard Contracts → Headless stand, the WASM bundled-font and custom `FontContext` items kept; the last README/records chunk, a precedent: the bundled-fonts claim cites architecture for the stand, design-system for WASM/`FontContext` (contract/native-config/font-context).

@history_item n=10 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2's stand-row move ratified; the stand-only lines leave design-system) — ratified the font-registration move and kept surface's cli item naming the six served tasks «as is» on the founder's word; a stand fact the statement restates stays where these entries put it.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay; what the tree, the snapshot and a shot read of an open list) — retired the «no select popup» limit; states the windowed list as read and not run, and a `screenshot` of a row crops to it; a precedent for a limit stated «read, not run» and for retiring a stated limit, both of which §Stated limits must reflect.

@history_item n=12 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a focused select past its dropped block) — added the row now decision/boxless-focused-select-keeps-focus and, under Kept, left the limit unstated in design-system: «a focused control that no longer shows keeping focus and taking keys» listed under a11y-plan §5; exactly the hidden-focus SC 2.4.3 gap CARRY `cry-vpudls5k` adds to §Accessibility reach — the statement names it, design-system still does not (no master amended by phase).

@history_item n=13 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — render-time `MountedData` calls on every Dioxus document with the by-construction limit (a call acts on the previous render's document, `set_focus_to` false for a node made that render); a 0.3.0 limit §Stated limits must carry with its standing (by construction, no owner).

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line; the unpaged graph, a pending limit; citation ranges re-read) — the `graph` answer (contract/escher-session-commands/graph-answer, graph-arguments, contract/ui-graph-line/content) and the stated limit «An unpaged graph» in docs/capabilities.md owned by residual `res-7cc5bi4k` (target next, Cells 293,869 bytes); a citation into docs/capabilities.md (the single select's list, :135-140 → :157-162) lives in design-system — a rewrite of the page moves it, a trap for the wrap's re-point.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line; the unpaged `actions` answer, a stated limit) — `actions` line, its content and stale rule, decided by the verbs' own `validate`/`check_target` (decision/actions-share-the-verbs-checks, contract/actions-line/*, actions-answer/arguments); the stated limit «An unpaged actions answer» beside «An unpaged graph», owner `res-7cc5bi4k`, target next — §What it can do («what can be done here») and §Stated limits must hold both.

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's answer line, endings and refusals; the coverage answer's three stated limits and their owners) — the `coverage` role, its shape, the 1,048,576 + 1 read bound and `malformed` refusals (contract/coverage-*, coverage-answer/arguments); three limits listed in the capability statement with owners: canvas handlers (0.4.0), an unreached screen (none, by construction), `stopPropagation` unread (residual, target next) — §Stated limits keeps each with its owner.

@history_item n=17 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (137), each verified against the code; every picked row handle this chunk rests on dates from it, and design-system holds what a planner must respect — the statement's claims cite these rows, not superseded section text.

@history_item n=18 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — + decision/unavailable-families-use-default-font beside decision/no-generic-family-lays-out-sans; the font-fallback rule the bundled-fonts and fonts-unavailable claims must be stated consistently with.

@history_item n=19 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer line; ratified — the coverage role's answer line, endings and refusals) — both PROVISIONAL cli records now ratified by the founder's own word; the statement may present graph and coverage as settled 0.3.0 capabilities.

@history_item n=20 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — + contract/escher-session-commands/layout-answer, + layout-arguments: the `layout --id` answer on every carrier, a missing id refused `malformed`; the «why an element sits where it does» capability §What it can do must state with its witness.
