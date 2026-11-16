@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand's TaskShell mount (`main#main > #task-shell`, header over `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport; it is the mount every session role boots and the base that `decision/fixed-document-skeleton-main-mount` and `decision/held-instance-reads-as-fresh-boot` read. A fixture app beside the stand for a panicking or logging handler must not move the stand's markup.

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became the third consumer of the `task_in_shell` mount, adding no markup, id, class, style, wrapper or order, and «a held instance reads as a fresh boot, the mount and pinned viewport hold after commands». That is the origin of `decision/held-instance-reads-as-fresh-boot` and `decision/session-roles-share-stand-boot`. A caught handler panic that leaves the instance poisoned, rebooted or held must keep this statement true, or amend it.

@history_item n=3 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the into-view scroll rule: every holding scroller moves, then the viewport, ratified by the founder (2026-10-07) for every document, read on the stand's `crud-list`. This is the origin of `decision/scroll-into-view-moves-every-scroller` (guard `stand_act_scroll`). An acting `scroll` whose answer gains the caused-errors field must leave this behaviour and its readings unchanged.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session <verb> … --session <dir>` with four endings: accepted, one uncoloured JSON line, exit 0; refused, exit 1; usage, exit 2; session error, exit 3. The desktop-native statement says a client command boots nothing. This is the origin of `contract/escher-session-exit-codes/{0,1,3}`. The scope admits no new exit status or session error, so a caught panic has to ride the exit-0 answer or an existing ending.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role: an MCP server over stdio, local, with no listener or auth surface, writing protocol lines only on stdout and echoing an admitted `id`. It narrowed «no ending writes back anything the caller typed» to the commands. Its crossing is the founder's ratified one of 2026-10-10. This is the origin of `decision/mcp-server-local-stdio`, `contract/escher-session-mcp/stdio` and `decision/no-caller-input-echoed`. The new field crosses this carrier, and the founder's field rule of 2026-10-10 decides whether it is a widening.

@history_item n=6 marker=2026-10-12-self-description
**2026-10-12-self-description** — `escher-session help` answers `describe()`'s seven keys in fixed table text, and each MCP tool description is the verb's help plus its result fields. This is the origin of `contract/escher-session-help/answer` and `contract/escher-session-mcp/tools`. The chunk's new answer field must enter both through the table text.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — precedent: adding content in band to an answer (the shot's image) was the founder's fork 1, ratified by his own word. It also set the MCP result bound: 1,048,576 bytes by the whole written form for an image result, by its text for a text-only one (`contract/escher-session-mcp/result-bound`). Captured panic or log content enlarges a text result, so it is bounded by this rule, and per-call capture needs a cap that fits it.

@history_item n=8 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>`: one owner-only replayable check (`check-{n}.json` 0600 in a 0700 dir), its session-error endings `record-dir-not-private` · `recording-failed`. It was a boundary widening, ratified by the founder (F1 A, F2 B). This is the origin of `decision/session-recording-replayable-check` and `contract/escher-session-record/check-file`. The scope's point on whether the new field joins the kept answer rests here.

@history_item n=9 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call its verb, arguments, acceptance, answer digest and the masked answer as on every carrier, replayed by `Check::replay`. Keeping the answer was the founder's widening F3 B. This is the direct precedent for P4's design point: whether the caused-errors field enters the kept answer and its digest, or is held out. Kept authored text (a panic payload or log message) would widen what a check file holds.

@history_item n=10 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — precedent: a new statement on the acting tools' `settled` result field in the MCP descriptions and `help` was taken as the program's fixed text inside a ratified carrier, no widening. The caused-errors field's help text and tool listing lines follow the same path.

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the host and `mcp` roles boot one of six served screens: the lean four, Cells by `boot_cells`, the load page by `boot_load`, each through its own boot. `start`'s word and the MCP `start` enum name those six. The cold-agent pipe runs `mcp --record`. Precedent and trap for the scope's fixture app beside the stand: a seventh served app would move the usage line's `apps:`, the `start` enum and the cold-agent apps. The scope says no new verb and leaves this open.

@history_item n=12 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (both entries) — the `graph` verb: ids, component names, handler event names, state names, closed flag words and booleans only, never a value (`contract/escher-session-graph/answer`). The chunk ties a caused error to the handler through this graph's node identity. Its sibling entry re-pointed citations into the task files, `dioxus_document.rs` and the driver's `schema.rs`. This chunk edits `schema.rs` and maybe `dioxus_document.rs`, so those citations will need re-pointing again.

@history_item n=13 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the tenth instance verb `actions` (`{"nodes"}` of id, owner, keyed, verbs with verb and cause; never a value, typed text or NodeId, unpaged), and the MCP listing of fourteen verbs in table order with `help` last. This is the origin of `contract/escher-session-actions/answer` and `contract/escher-session-mcp/tools`. The verb count stays at fourteen (no new verb). It is also the content-free answer precedent the new field's (owner id, event name) tie follows.

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (three entries) — the `coverage` role: a further consumer of the TaskShell mount through the same boot, adding no markup. Each check is read bounded at 1,048,576 + 1 bytes and replayed on a fresh boot; the answer is `{"unexercised":[{id,owner,event}],"checks","held"}`; a failed boot is exit 3. This is the origin of `contract/escher-session-coverage/{answer,check-bound}`. A kept check carrying the new field still has to read under that bound, and a replayed step that panics meets this role's endings. The third entry re-cited the skeleton and head notes after one added line in `dioxus_document.rs` beside the reached record in `handle_event`, the record the scope names as a capture candidate. That is a citation trap if the chunk edits that file.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the scrolling boxes exception entry) — an open select list's row is scrolled into view by writing the list's `scroll_y` at once, no scroller or viewport moving. This is the `except:` of `decision/scroll-into-view-moves-every-scroller`, which the acting `scroll` must keep in both layout modes.

@history_item n=16 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (121 rows, each verified against the code), the picked rows among them, on the founder's approval of 2026-11-12. From here on, amendments name rows by handle (`+`/`~`/`−`), as this chunk's wrap must for any contract row its answer field changes.

@history_item n=17 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (three ratification entries) — the `graph` verb, the coverage role as a TaskShell consumer, and the coverage role on the cli surface were moved from PROVISIONAL to ratified by the founder's own word. The handler identity and the coverage contracts this chunk rests on are therefore ratified, not delegate answers.

@history_item n=18 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage`: `agent-run.sh run session` runs the escher-session binary's flows in seven_guis. Precedent for this chunk's proof over the CLI and MCP carriers: a new carrier-level flow would land in that selection.
