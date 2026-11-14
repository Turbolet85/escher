@history_item n=1 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built the closed verb table and bounded argument kinds behind `validate`, one answer per call in a fixed order, no clock read, a refused call changing nothing, and a refusal holding nothing of the call; a seed argument on `start` is a new entry in this closed table and must be bounded and validated the same way (decision/closed-driver-verb-schema, decision/command-schema-no-clock).

@history_item n=2 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — fixed the `command` span at eight fixed-word or count fields, never fielding any argument; a seed value must stay out of the span (scope: no log field carrying a seed's value), and adding a field is a crossing question (decision/one-command-span-per-call, decision/command-span-content-free).

@history_item n=3 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's span-line record class with the standing rule that a field holding any content is a new crossing question; any span field naming a seed would reopen it (decision/closed-span-lines).

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the socket, its access and input rows) — registered the Unix socket as the one local listener, no authentication, and the driver reading no env var beyond `RUST_LOG`; the seed must enter by no new listener or env read (decision/driver-reads-only-rust-log, decision/driver-session-unix-socket-only).

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's sink drops outside-target records and redacts content-named fields; trap: a "nothing in the logs" check over a process that installs no sink passes vacuously — a seed-out-of-logs proof must run on a sink-installing host such as `host_log` (decision/telemetry-sink-target-allowlist).

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line and the socket's calls) — put `validate`/`validate_session` behind the command line and socket `call`, added session-level `start` and the `name` kind under the founder's ratification, with the standing rule that every later surface's input passes `validate` first and no ending echoes caller input; a seed on `start` rides these carriers (decision/cli-closed-argv-reader, decision/no-echo-of-caller-input, contract/cli-roles/serve).

@history_item n=7 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings; the closed-field rule) — set the founder's rule that a closed-value field inside a ratified carrier is no widening while authored text, an open value set, a new carrier or a new exit is; the scope's P4 fork on whether a seed argument widens is decided against this rule (decision/boundary-widening-rule, decision/closed-value-field-no-widening).

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing) — MCP tool arguments typed before `validate` (i64 a number, a wrong kind refused `malformed`), the `start` tool on the MCP carrier; a seed argument there must fit these value kinds and bounds (contract/mcp-tool-arguments/value-kinds, decision/mcp-bounded-lines).

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` reviewed; the `tool_call` span) — `serde_json` admitted as a reader only, and every dependency an escher crate takes is reviewed in Pinning; a PRNG crate for the seed would need the same review (decision/serde-json-read-only).

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and fixed text) — `help`'s self-description and the MCP listing are the program's own fixed table text inside the ratified carriers; a seed argument's help text must stay fixed text (decision/mcp-listing-fixed-text).

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** (a session error's remedy and its table) — session errors come from one fixed table holding no path; any seed-related failure must be a cause or table row, never call content (decision/session-errors-fixed-table).

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness `VirtualClock` documents read, `advance` moving it exactly, the overlay fade on the document's clock so two shots of one state read the same pixels; the direct determinism precedent for a run-scoped source beside `with_time`, and the `Date`-anchored-at-`SystemTime` premise sits in the same reach (decision/advance-moves-virtual-clock-exactly, decision/time-moves-only-on-advance).

@history_item n=13 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the session reads the wall clock for the bounded load wait alone; restates which clocks the driver reads, a list a seeded run must not grow (decision/driver-installs-no-subscriber).

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check, its directory, its reader and `--record`) — the kept check's start record, the strict bounded closed-key reader and `--record` placement; a recorded seed adds a key or argument to the `start` record and must pass the same strict reader (contract/recorded-check/start-record, decision/recorded-check-strict-reader, contract/cli-flags/record).

@history_item n=15 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors) — wire v3's closed grammar and fixed replies; a seed on `start` crosses inside that grammar (decision/session-socket-closed-grammar, contract/session-socket-request/call).

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the replay runs each kept step through `Session::run`, and the two committed stand-Counter checks are compared byte for byte with a fresh recording; a seed recorded in the start record changes those committed files and what replay reproduces (decision/replay-through-session-run, decision/committed-counter-checks, decision/kept-check-call-content).

@history_item n=17 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the precedent for adding an argument kind (`part`, a bounded whole number) to the validated table as a founder-ratified widening onto every carrier, with the value never fielded in a span (contract/driver-argument-kinds/part, contract/cli-flags/part).

@history_item n=18 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the committed `docs/capabilities.md` statement and its pin state the carried sets; the scope names the statement and its pin as touched if the seed crosses (decision/capability-statement-mcp-crossing).

@history_item n=19 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the secret search covers `README.md`, `docs/capabilities.md` and the pin; an edit to them for the seed must keep the search at 0 hits.

@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs) — the stand's served apps are a closed `TASKS` table, the MCP `start` enum the program's own fixed text; a randomness-reading fixture for the proof would extend that table (contract/cli-roles/serve).

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`URLSearchParams` on the script crossing) — a bootstrap script realizing a page API with the native helper deleted from `globalThis` before any page script, ratified as a realization on the script crossing, no widening; the template for replacing `Math.random` in blitz-vibey-script's bootstrap (decision/url-search-params-bootstrap, decision/search-params-native-parser-hidden, decision/bootstrap-native-helper-removed).

@history_item n=22 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (CSS property accessors on `CSSStyleDeclaration`) — a script-reachable addition recorded as a realization on the registered script-to-DOM crossing per the 2026-10-06 precedent; supports reading a seeded `Math.random` the same way.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script runtime's module split) — moved blitz-vibey-script's runtime into `runtime/bootstrap/*.js` and `runtime/*.rs`; where a `Math.random` replacement would land and which citations it would move.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port, two test-only TCP binds) — no served TCP/UDP port, the Unix socket the one listener; the seed must add none (decision/session-socket-sole-listener).

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Dioxus bridge's dispatch and the signal registry) — precedent for an app-facing source in dioxus-native-dom provided as a root context at `ScopeId::ROOT`, holding no value, no log site or env read; the shape a seeded app source would take.

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role) — the coverage role replays caller-named kept checks on a fresh in-process session through the strict reader; a seed recorded in a check must reach this second replay path too (decision/coverage-check-files-untrusted, decision/recorded-check-untrusted).

@history_item n=27 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness's dependency taken as a reviewed Pinning bullet, the harness taking no `tracing` and no `style`; a PRNG dependency in `HarnessOptions`' crate follows this review (decision/test-harness-no-tracing-no-style, decision/workspace-inherited-dependency-versions).

@history_item n=28 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the whole-workspace advisories and licence gate and the environment-values list (the library reading none beyond `RUST_LOG`); a new crate for seeding must pass the licence gate and add no env read (decision/licence-gate, decision/dependency-audit-cargo-deny).

@history_item n=29 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (advisories published after a green audit) — standing rule: run `bash .github/scripts/ci-leg.sh audit` locally before a push; applies if the chunk adds a dependency (decision/dependency-audit-ci-gate).
