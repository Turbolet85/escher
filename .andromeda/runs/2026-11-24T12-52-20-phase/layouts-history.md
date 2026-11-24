@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's TaskShell mount at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans (row decision/headless-stand-pinned-viewport); the canvas dump's float coordinates and its determinism across hosts are read at this pinned viewport and scale.

@history_item n=2 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — set the founder's rule that every element an agent can act on reads an author key, added as attributes only with no class, style, wrapper or order; the dump names its canvas by such a stable id (`circle-canvas`) and must add no markup to the stand.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — named the session host a further consumer of the TaskShell mount, adding no markup, a held instance reading as a fresh boot with mount and viewport holding after commands (rows decision/held-instance-reads-as-fresh-boot, decision/session-roles-share-stand-boot); a canvas read on the held instance runs no step and must leave this intact.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session <verb> [arguments] --session <dir>` the driver's command line with four endings (0 accepted one JSON line, 1 refused, 2 usage, 3 session error) and unknown first words refused `unknown-verb` (rows contract/escher-session-command/form, contract/escher-session-exit-codes/*, contract/escher-session-refusals/unknown-verb); a sixteenth verb or new argument inherits this grammar and usage line.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role over stdio, local, no listener or auth, protocol lines only on stdout, echoing admitted ids, and scoped "no ending writes back what the caller typed" to the commands (rows decision/mcp-server-local-stdio, contract/escher-session-mcp/stdio, decision/no-caller-input-echoed); the new verb's MCP tool rides this carrier, its crossing the founder's ratified one.

@history_item n=6 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` answering `describe()`'s seven-key fixed line and the MCP tool descriptions as the verb's help plus result fields (rows contract/escher-session-help/answer, contract/escher-session-mcp/tools); a new verb or argument must extend the help table and tool description.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot` (optional `--id`, element's visible part), `fonts-unavailable` refusal, the MCP image block and the 1,048,576-byte result bound (rows contract/escher-session-refusals/fonts-unavailable, contract/escher-session-mcp/result-bound); the closest precedent for a per-element canvas read beside `screenshot`, and its in-band image was a founder's fork — as this chunk's carrier crossing is.

@history_item n=8 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — gave Circle Drawer its author ids, `circle-canvas` among them, as `id:` lines alone (row contract/stand-element-ids/circle-canvas); the id the dump verb is named by.

@history_item n=9 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — swapped the cold-agent pipe to the driver's own MCP server under a fixed prompt naming no element id or app slug (rows decision/cold-agent-uses-driver-mcp, decision/cold-agent-prompt-names-no-id); a new tool shows up in the cold agent's tool list and the prompt must stay id-free.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>` keeping a session as one owner-only replayable check (`check-{n}.json` 0600 in a 0700 dir), with `record-dir-not-private` and `recording-failed` session errors, a founder-ratified boundary widening (rows decision/session-recording-replayable-check, contract/escher-session-record/*); a new verb's calls are kept here.

@history_item n=11 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call its verb, arguments, acceptance, answer digest and masked answer, a shot kept without its png (row contract/escher-session-record/check-file); precedent for how a heavy canvas dump is kept and replayed — the founder's F3 B widening.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, `escher-session lint`) — added an instance verb end to end: verb count, usage line, MCP tool order, kept under `--record`, `fonts-unavailable`; ratified by the founder's F1 «Новая команда lint» — the pattern a sixteenth verb follows, and the next entry «Canvas lint» extends.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answers `{"text"}` or `{"text","part","parts"}`, a part past the last empty (row contract/escher-session-snapshot/answer); the snapshot's size budget this chunk measures its dump against, and a paging precedent if the full level outgrows it.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — fixed the served tasks at six (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), `start` and the MCP enum naming them, Circle Drawer not among them; this is why a carrier verb reaches no stand canvas until Circle Drawer is served or a fixture serves one (scope premise).

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the mode list entry) — the engine's select list painted by `paint_scene` after the root's render and before the devtools overlays, with no row a layout box; prior change to the same `paint_scene` path this chunk reads widget scenes out of, and a painted-not-DOM precedent for what the snapshot reads.

@history_item n=16 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — the thirteenth verb `graph`, no argument (one named `malformed`), one uncoloured line carrying ids, names, closed words and booleans only, never a value; a ratified widening — precedent for a new read verb's shape and what it may carry (the dump's glyph text is this chunk's fork).

@history_item n=17 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — graph verb; coverage role; coverage as a TaskShell consumer) — the founder's own word replaced the overseer's delegate confirms; shows that a delegate-confirmed verb entry stays PROVISIONAL until his word.

@history_item n=18 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — the fourteenth verb `actions`, unpaged, ids and closed words only, never a value or typed text, the MCP listing in table order with `help` last; the latest precedent before `layout` for growing the verb table.

@history_item n=19 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, a consumer of the TaskShell mount; escher-session's coverage role) — the `coverage` role boots each kept check's app through the same boot and replays it (row decision/session-roles-share-stand-boot); a kept check holding the new verb is replayed by this role too.

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows, verified against the code; the picked rows' handles this chunk amends (`~`/`+`) date from this conversion.

@history_item n=21 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grew to fifteen with `layout --id`, MCP listing `layout` after `actions`, `help` last, and `decision/no-caller-input-echoed` gained its one exception (the layout answer's id); a sixteenth verb must re-word contract/escher-session-command/form and contract/escher-session-mcp/tools, and a `--id` answer echoing its id needs this exception extended.

@history_item n=22 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirm of the `layout --id` rows; the exception to the caller-input rule stands on his word.

@history_item n=23 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: cli) — served tasks gained `order` as the seventh (contract/escher-session-served-tasks/order); serving Circle Drawer would be a further change to this set, the founder's carrier fork depending on it.

@history_item n=24 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `graph` took `--part`, answering `part`/`parts` with part 1 alone carrying the states; precedent for paging a full-level dump that outgrows the budget.

@history_item n=25 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added decision/circle-drawer-circles-painted-on-canvas (circles from one `data-circles` attribute, never DOM nodes, crossing as shot pixels alone) and Circle Drawer to the clean-at-boot lint; the dump makes the canvas's commands cross as data, so this row's «shot pixels alone» must be re-worded by this chunk.

@history_item n=26 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** (state standing; ratified — Circle Drawer circles painted on a canvas) — the founder ratified the canvas row, its state standing; changing its «crosses as shot pixels alone» clause is a change to a founder-ratified decision and needs his word (the chunk's crossing fork).
