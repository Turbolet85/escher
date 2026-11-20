@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the cli surface's output form: `escher-session` prints one uncoloured JSON line per command on stdout, no ESC byte, no terminal test, exit statuses 0 accepted · 1 refused · 2 usage · 3 session error (the founder's answers, 2026-10-10). This is the origin of decision/driver-output-for-agents; any README wording of the driver's output or exit codes must match it.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: newline-delimited JSON-RPC 2.0 lines only on stdout, a reply per non-notification request, uncoloured, its end writing nothing there. The source of contract/escher-session-mcp/stdout and contract/escher-session-stdout/mcp-line; the README's description of the MCP tools rests on it.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` (the self-description as one uncoloured JSON line, status 0, reaching no session), the exit statuses read from the schema's `EXIT_STATUSES` table, and the `mcp` `help` tool result as that line. The source of contract/escher-session-commands/help and contract/escher-session-mcp/help-result; README statements of what `help` answers must follow it.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the shot's answer line `{"width","height","png"}` and, over `mcp`, an `image` block then a text block (the founder's fork 1). This is the exception beside contract/escher-session-mcp/other-results (any other result is the command's JSON line); a README that counts or describes the tools keeps that split.

@history_item n=5 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — paged `snapshot [--part]` answers `{"text":…}` / `{"text":…,"part":n,"parts":m}`; over `mcp` a paged snapshot result became the JSON line (it had been the screen text alone). This is where contract/escher-session-mcp/other-results took its current reading.

@history_item n=6 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — named the recording's `record-dir-not-private` and `recording-failed` among the exit-3 session errors and added no new exit. The four exit statuses still stand; any README listing of exit codes must not grow.

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the cli serves six tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), listed in the usage line's `apps:` and the MCP `start` enum. This is the precedent for the served-task count that README §Status states for 0.3.0.

@history_item n=8 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — the served-task set moved from six to seven (`order` added in 0.3.0, operator Q1 «В 0.3.0, до прогона»), with every contract/escher-session-tasks/* row re-counted. A README stating 0.3.0 closed must say seven tasks, not six.

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — added `escher-session graph` (one uncoloured JSON line; over `mcp` a text block of that line) and the pending limit «An unpaged graph» in docs/capabilities.md (residual `res-7cc5bi4k`); it also re-pointed citation ranges into docs/capabilities.md and schema.rs. This is one of the driver commands that README's pinned tool count (`.github/scripts/test_capability_statement.py`) covers. docs/capabilities.md stays 0.3.0's statement.

@history_item n=10 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added `escher-session actions`: its line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or NodeId (the source of contract/actions-line/content), plus the stated limit «An unpaged actions answer». This is another command in the README's count, and any README description of `actions` must keep that content rule.

@history_item n=11 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added the `coverage` role (one uncoloured JSON line, the four exit statuses, none added) and its three stated limits, with the canvas count deferred to 0.4.0. That limit is part of what the README says 0.4.0 builds (the headless canvas). The record was PROVISIONAL and ratified at 2026-11-15T05-21-42-wrap.

@history_item n=12 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified, by the founder's own word, the graph answer line and the coverage role, which had been PROVISIONAL. This is the precedent that the README's 0.3.0 surfaces stand ratified, not provisional.

@history_item n=13 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added contract/escher-session-commands/layout-answer and layout-arguments: `layout` on every carrier, its one-line answer, and a missing id refused `malformed`. It is the newest driver command, and the README's pinned count of driver commands as tools must include it.

@history_item n=14 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — re-pointed the list-bullet font citation from `packages/blitz-dom/src/layout/construct.rs:1106-1112` to `packages/blitz-dom/src/layout/list.rs:96-102` after the citation sweep found the old range claim-false. This is exactly the `construct.rs:1106` → `list.rs:96` site in the backlog res-xe7pyput; the scope says `rec.py find` does not find it in design-system now, so P3 must re-read it before calling it gone.

@history_item n=15 marker=2026-10-24-u55-door
**2026-10-24-u55-door** — split design-system into its body and 8 category files (map .andromeda/design-system.map.tsv). This is the origin of the `*-detail/`-style category files that the backlog's hand-read citations point into; they no longer stand.

@history_item n=16 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows (364 records → 137 rows; citations verified against the code; 1 guard re-pointed). This is why the category files and their file:line citations no longer stand and why the backlog's sites are read through against rows. It is also why the CLAUDE.md stack line has no design-system row to amend.

@history_item n=17 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the earlier chunk of this name moved the stand's radii, shadows, padding, colours and font registration out of design-system to layout-templates and architecture, PROVISIONAL on a delegate F2 answer, with a pointer atomic with each removal. It was ratified by the founder's own word at 2026-10-27-upstream-sync-engine-debts. The precedent for this chunk: forks go to the founder, the least-change option stands until he answers, and a superseded provisional record is kept and never reworded.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — ratified F2's stand-row move by the founder's own word «Подтвердить (Рекомендую)» and carried the ratification as one new entry per touched doc, leaving the provisional entries' words unchanged. It also re-pointed citations after blitz-vibey-script's split (old paths read claim-false while each claim held at its new site). This is the pattern for reading the backlog citations through before any site is called gone.
