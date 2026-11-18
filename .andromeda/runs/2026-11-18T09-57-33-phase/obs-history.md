@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set up escher-telemetry: the chaining panic hook (it logs ERROR `panic` at `escher_telemetry::panic` with `panic.payload` redacted, then std's previous hook still prints the raw message) and the content-named scrub set. A panicking control on the new app goes through this hook, and the authored-text boundary is judged against this set (rows contract/escher-telemetry-panic-hook/previous-hook, contract/escher-telemetry-events/panic).

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — said the headless stand `seven_guis::stand` and its in-process checks install no subscriber, read no env var and print nothing. The new app's stand checks boot through this stand and inherit the rule (row decision/headless-stand-no-sink).

@history_item n=3 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — created the cold-agent pipe log: python3-json events, `verdict.json`, sitting outside escher's sink, no content-named key, transcript raw and never printed. A new verdict count has to fit this log (rows decision/cold-agent-log-no-user-content, decision/cold-agent-transcript-raw).

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — made `escher-session` the second sink installer, under the same `service.name=seven_guis`. Its trap: a zero reading from a host that installs no sink says nothing about one that does. The new served app runs in that binary, so it must be read through the sink-installing host, not just in process.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — made the sink drop every target outside the engine and `escher_` prefixes, whole and at every level, including WARN and ERROR. Standing rule: a target is admitted only together with a scrub rule for it. The new app's log calls under a `seven_guis` target never reach the sink and reach the agent only through the scoped capture; admitting that target would need a scrub rule and a ruling (row decision/sink-target-allowlist).

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made a command's answer, refusal or session error one JSON line on stdout, never a log line. Standing rule: a new surface adds no span or log field holding what a call supplied or a screen reads. The new app's `caused` answers travel as answers, and its controls add no log field (rows contract/escher-session-stdout/command-answer, decision/debug-types-never-logged).

@history_item n=7 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the `tool_call` span with exactly two closed fields, `tool` and `ending`, and put the `mcp` role on the sink with protocol lines only on stdout. A verdict count of `caused` calls must not add a field to this span; the pipe can only read what exists (rows decision/tool-call-span-two-closed-fields, contract/mcp-role-stdout/json-rpc).

@history_item n=8 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — moved the pipe onto `escher-session mcp` with `RUST_LOG=info` on the live child, and made `server.log` read only by counting span names. Verdict fields became counts, booleans or closed words, and the reasons `no-check`, `check-failed` and `check-cannot-fail` were added. This is the precedent for where and how a new verdict count is read (rows decision/cold-agent-artifacts-never-printed, decision/cold-agent-verdict-holds-no-content).

@history_item n=9 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — keeps each call's answer whole in the recording file, masked as on every carrier. A count of calls whose answer carried `caused`, read from the live child's kept checks, reads these answers at rest and must turn them into bools and counts without printing or copying them (rows contract/kept-check/answer, contract/recording-file/answer).

@history_item n=10 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — published `docs/capabilities.md`, which restates the driver's spans and the scrub's limits. Its pin reads none of those claims, so a change there is made by hand. The scope moves the statement's served-app and verdict counts (row decision/capability-statement-updated-by-hand).

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the verdict fields, the live child's recording and the six apps' carriage) — added `wrong_call_bar_pct`, the per-verb counts, `part_reads`, `recorded_sessions` and `recording_kept`. It set the reasons order, `capability-unused` after `task-unmet`, and the owner-only `record/` (mode 700, never printed or copied). It also put the six apps' screens in the transcript to the provider, on the founder's word. A seventh app's carriage and a `caused` count extend this register (rows contract/cold-agent-verdict/reasons, decision/cold-agent-kept-checks-at-rest).

@history_item n=12 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs and the served stand's silence) — introduced `main`'s six slugs, the private `Served` table, the usage line's `apps:` and the MCP `start` enum, all as fixed text. Under a census the served stand's boots and `LoadNet` were shown silent. A seventh slug moves that table, the usage line (row contract/escher-session-exit-codes/2) and the enum, and the census has to be re-run over the new app.

@history_item n=13 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's endings gain the coverage role; ratified 2026-11-15T05-21-42-wrap) — added `escher-session coverage <app> <check>...`, taking an app the binary boots. A seventh served app becomes a valid `<app>` there (row contract/escher-session-coverage/command-shape).

@history_item n=14 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — widened the exception in decision/checks-silent so that a re-run child may print ids or digests. It is the history of the named exceptions that any new stand check of the error page must either fit or extend.

@history_item n=15 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added decision/exploration-catches-panics: a panic in an explored call is caught, reported as `Panicked` with no payload, and the hook still runs. Exploring the new app's panicking control meets this row.

@history_item n=16 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — extended the exceptions in decision/checks-silent and decision/headless-stand-no-sink to `stand_layout`'s own recording subscriber. Precedent: a new check that installs a subscriber must be named in both rows' exceptions.

@history_item n=17 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (§3) — introduced escher-telemetry's scoped capture around an acting step, which prints nothing. It reworded decision/headless-stand-no-sink and decision/driver-library-installs-no-subscriber, and scoped RUST_LOG to the sink layer alone (contract/telemetry-sink/rust-log). The scoped capture is what makes the new app's `caused` exist.

@history_item n=18 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (§4, §6) — reworded decision/driver-emits-no-event for the scoped capture and added decision/capture-framework-targets-warn-up, along with the EnvFilter reach (contract/escher-sink-env/rust-log). Which levels and targets the capture admits decides whether the new app's log lines appear in `caused`.

@history_item n=19 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§3, §4) — `caused` now carries each record's text. It extended decision/answers-echo-nothing-typed (an author's message kept as written), added the error-text founder check, which installs escher's sink, to the exceptions in decision/headless-stand-no-sink and decision/checks-silent, put the chaining panic hook in decision/driver-library-installs-no-subscriber, and changed decision/driver-emits-no-event to keep records and panics in process. Taken together, these are the reason the new app's authored text crosses whole.

@history_item n=20 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§6, §7, §8) — made the capture admit engine targets at warn and above (decision/capture-framework-targets-warn-up). It set the `error` and `url` scrub-set promises, added `password` to decision/escher-target-content-named-set, and replaced the sink-only scrub with decision/scrub-by-field-name and decision/engine-sites-fixed-messages (fixed messages, values in named fields). The new app's authored text must avoid every hidden-set field name.

@history_item n=21 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — opened `error` in `caused`: the capture judges by the answer's own hidden set without `error`. That changed contract/telemetry-scrub-set/error, decision/scrub-by-field-name and decision/answers-echo-nothing-typed. An authored `Err` text carried in an `error` field would therefore cross to the provider as written, which the scope's no-hidden-field boundary has to account for.

@history_item n=22 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (§3) — widened the needle set in decision/sink-host-read-at-trace to layout-answer keys. Precedent: a new answer content class is held out of the host and `mcp` logs by its own needles. The new app's authored texts are candidate needles for `host_log` and `mcp_log`.

@history_item n=23 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — named the transcript's crossing to the model provider in contract/layout-answer/logging, on the operator's ruling, and had the founder ratify the delegate confirms of the error-field and layout cards. Precedent for recording that the authored `caused` text crosses to the provider on a live run, a boundary widening that needs the founder's word (here F5).

@history_item n=24 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (§6, 7 rows) — made graph, actions and layout counted verbs, reversing decision/graph-actions-layout-not-counted-verbs. It extended `capability-unused` to cover a list opening, made decision/cold-agent-server-log-read-by-span-name read `tool` and `ending` plus the command span's cause word, and allowed objects of closed words to counts in decision/cold-agent-verdict-holds-no-content. This is the register a `caused` count and its requirement extend.

@history_item n=25 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (§6, 6 rows) — added the verdict fields `graph_calls`, `actions_calls`, `layout_calls`, `list_opens` and `refused_by_cause`, and widened decision/verdict-fields-closed. It is the immediate template for a new verdict field and its contract row.

@history_item n=26 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (§4 and §3) — cut the inventory from contract/cold-agent-verdict/capability-unused ("a counted verb, a part read or a list opening") and from decision/cold-agent-server-log-counted-only ("count its span lines"). Both rows are worded without a list, so a new required count may or may not need to name itself in them. That is a wording decision this chunk makes.
