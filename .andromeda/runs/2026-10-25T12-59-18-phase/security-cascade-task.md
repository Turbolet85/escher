# Task — security · 2026-10-25-cold-agent-bar

## The chunk
Extends the 7GUIs stand (example code) so the driver serves every 0.2.0 check: `cells` booted by `start` (the stand's one screen over the snapshot text budget, so `snapshot`'s `part` is reached), the flight booker's one-way/return choice made a real `select`, and a load in flight carried on the stand (how — and whether that adds a provider, resource or crossing — is a P4 fork).
Gives the cold-agent pipe new goals over v020-04 … v020-09 (recording and replay, lint, `advance`, the load wait, the select, the screen in parts) and moves its contract tests with it.
Runs one live cold-agent run on the operator's Claude Code login over `escher-session mcp`, counting wrong calls from the server's `tool_call` span lines and replaying the agent-written check on fresh children; the wrong-call bar is the founder's P4 fork.
Re-pins the stand's moved claims in the tests that hold them (Tab order and Enter/Space tables, contrast rows, element ids and actionable keys, snapshot, diff and act checks, every-screen-under-budget); no new driver verb, argument, refusal cause, session error, carrier, log field or span, and no engine change for every document is intended.

## Reached files
- examples/seven_guis/src/session_host.rs · edits the `start` task list
- examples/seven_guis/src/stand.rs · edits the headless boot, `options`
- examples/seven_guis/src/tasks/cells.rs · edits its keys
- examples/seven_guis/src/tasks/flight_booker.rs · edits the one-way / return choice
- examples/seven_guis/tests/flows/flight.sh
- tests/blitz-tests/tests/common/mod.rs · edits `tab_order`, `ENABLED_TAB_ORDER`
- tests/blitz-tests/tests/stand_keyboard.rs · edits the Tab order and Enter/Space tables
- tests/blitz-tests/tests/stand_contrast.rs · edits the contrast rows
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_actionable_keys.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_snapshot_text.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- tests/blitz-tests/tests/stand_diff.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_load_wait.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- scripts/cold-agent.sh · edits `TASKS`, the `stand` task's prompt and goals
- .github/scripts/test_cold_agent.py

## Needs
- The cold-agent pipe's command line: the verb set, argument count and the task allowlist checked before any precondition or `claude` call; the task reaching the agent only as the fixed prompt, naming no element id and no app slug — the new goals must keep that shape.
- The cold-agent check as untrusted input: only the reply's last fenced json block, bounded in bytes and steps, closed keys, `tool` one of the twelve verbs, else never replayed; a valid check replayed on fresh `escher-session mcp` children, every step validated again, a per-reply bound, each child killed and reaped; `check.json` never printed.
- The cold-agent pipe's registered crossings: the spawned `claude` client, the driver's own MCP server as its tools, and the model provider on the live run — the screen's carriage to the provider ratified for one run only, so whether a second run carrying new screens (Cells' contents, the select's options and value) needs the founder's word again.
- The operator's Claude Code login as the development secret: held by the `claude` CLI outside the repo, no API key, no env read by the pipe, no CI secret; committed live evidence holds no key, auth header, credential-file name, token field or e-mail address.
- The cold-agent pipe's logging: events and verdict hold counts, booleans and closed words; server log, check and raw transcript never printed; each committed transcript and check host-path-masked (hygiene gate), no host temp path even as prose.
- The MCP stdio boundary (`escher-session mcp`, `serve_mcp`): bounded request lines and results, a paged `snapshot` result's JSON line, screenshot image blocks, lint findings, fixed protocol errors — what the client does with results, a model provider included, is the client's.
- The driver command schema and `validate`: the closed table of twelve verbs and seven bounded argument kinds, `part` a whole number 1 to 1000 with `Outcome::Part`, eight refusal causes with fixed remedies — reached through the stand, none added.
- The widening rule for ratified carriers: a new field of a closed value set is not a widening, while authored text, an open value set, a new carrier or a new exit is — Cells' cell contents and the select's option labels are page text crossing the already ratified name and value crossings.
- The select's markup-read rules, reached indirectly through the stand's new `select`: closed key match, the typeahead buffer never carried out, the choice leaving only as events' value, the snapshot's unmasked `value` and the options' accessibility nodes, the driver's `type` on a select as the same typeahead.
- The load wait and the absent request timeout: settle never waits on a load, an acting step waits at most `LOAD_WAIT`; the stand boots offline and the fixtures hold a load with the test-only `HeldNet` stand-in.
- The `file:`/`data:` net provider with no path restriction and no size cap or timeout: no agent- or user-supplied URL routed through it without a recorded decision — binds how the stand carries a load in flight.
- Occupied resources registration: a new provider, listener, port, env var or workspace crate is an arch and security-plan registration first, a test-only one registered as test-only — binds the load-in-flight fork.
- Stable element ids and actionable keys on the new stand screens: every actionable element keyed, ids leaving by the platform adapter, driver answers, MCP results and kept checks, no `NodeId` crossing.
- Masked values unchanged: a password's or file input's value as the fixed mask on every carrier, a masked `type` recorded as the stand-in.
- The driver session socket and its state directory, used by `start` of a new task and by `flight.sh`: owner-only `0700`/`0600`, wire `v3`, bounded request and answer, idle expiry.
- The `escher-session` command line, driven by `flight.sh`: closed argv reader, `validate` before the session is asked, four endings with fixed exit statuses, nothing the caller typed written back.
- Session recording and kept checks, if the acceptance's replay means the driver's `--record` check: `0700` directory, `0600` `create_new` check, strict `Check::from_bytes`, `Check::replay` through `Session::run` alone; only the two committed Counter checks are public, any other check stays owner-only and uncommitted.
- escher's telemetry sink on `escher-session` and its `mcp` role: no element id, accessible name or typed text on stderr at any level; the `command` and `tool_call` spans hold fixed words and counts only, no part number, part count or screen text — the wrong-call count reads those span lines.
- Typed refusals and session errors: fixed tables, no call content, of a refusal only its cause's name logged.
- The example apps' input validation: the 7GUIs inputs validated or falling back — Cells' formulas and the flight booker's new select included.
- `serde_json` as the reader of the MCP server's stdin, a kept check's bytes and the replay's answer line, writing nothing; no package added to the lock.

## Not needed
- The browser app, rdme's markdown rendering and the WPT runner's local file reads: the chunk touches none of them.
- Script execution (`blitz-vibey-script`, the script fetcher, `isTrusted`): the stand runs no page scripts.
- Remote content parsing rows (fonts, `@font-face` sniffing, SVG, stylesheets, iframes, favicons, legacy attributes): the stand is offline example markup, no new parse path.
- The browser urlbar, link navigation, external-browser opening and form submission methods: not reached by the stand or the pipe.
- The browser's sqlite history, the HTTP cache, cookies, clipboard and the native file dialog: untouched.
- CI publish, signing material, the PR-triggered WPT post-results workflow and signed artifacts: no workflow or release change.
- The dependency audit's advisory ignores, licence table, pinning and supply-chain rows: no dependency change is intended.
- `unsafe` code in `stylo_taffy` and layout, and the engine's panic paths: no engine change is intended.
- The upstream apps' `fmt::init()`, the WPT runner's `env_logger` and their unscrubbed URL logs: not escher's sink, not touched.
- The example CLIs (`screenshot`, `paint_bench`, `preact_script`, `multi_doc`) and their env reads: not reached.
- The `agent-run.sh`, `apt-install.sh` and `coverage_floor.py` command lines: their argument rules do not move.
- The fixture pages' outbound requests and subresource integrity (graphite, google, servo): not stand pages.
- The test-only client listener `eab-{pid}` and its `TMPDIR` read: unchanged by this chunk.
