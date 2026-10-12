# Fan-out results — 2026-10-10-mcp-surface (wrap resumed in run 2026-10-11T08-35-17-wrap)

Seven Explore doc-agents, one parallel batch, each prompt the amendment-flow letter substituted (repo-relative paths, no
`{…}` left; detector counts 2 · 3 · 1 · 1 · 3 · 3 · 2 = 15 = the drift-base's `doc:` names). Contracts lines: test-plan,
obs-plan, a11y-plan (render exit 0); architecture `NOT MIGRATED` (line dropped); security-plan, design-system,
layout-templates carry no keyed contract (line dropped). Entity probe on every return: `&lt;` `&gt;` `&amp;` `&quot;`
`&#` — entities=0. Each return arrived as a message, so the lists below are a transcription of it (the handoff's
standing note); the `change` texts are kept whole, the rationale and basis cut to their citations.

## Verdict lines
- architecture — 14 proposals (2 primaries, 12 `dependent-of`). Stripping removed nothing.
- security-plan — 16 proposals. Stripping removed four orchestrator notes: no proposal for the cold-agent stub row
  (:73, the pipe unchanged) or for Data Protection (:186, the mask claim true); the `unkeyed_actionable` clause of the
  `id` row stands; the CLI row, Threat Model vector, Logging bullets and Secret Management entries named by section,
  the report giving no line.
- design-system — 0 proposals (`proposals: []`). Stripping removed one comment: every new surface's `tokens` flag is
  `n/a`, no rendered UI. Raw twin `.raw-fanout-design-system.md`.
- layout-templates — 2 proposals. Stripping removed nothing.
- test-plan — 5 proposals. Stripping removed two comments: D-tests-coverage no drift (tiers unit · integ · e2e
  present); the counts the report lists (805/11 at :329, 127/6 over 33 at :116, 59 at `session-lifecycle.md:11`, seven
  targets at :10) not proposed — "restated counts, not a coverage, runner or harness/log-format violation".
- obs-plan — 9 proposals. Stripping removed nothing.
- a11y-plan — 0 proposals (`proposals: []`). Stripping removed four comments: no interactive element; no a11y violation
  schema exists (§3 NOT YET MEASURED); the one keyed contract read and untouched; the plan's a11y §1 and §7 entries fall
  under neither detector. Raw twin `.raw-fanout-a11y-plan.md`.

## Validate — the opening rule
Every coordinate a proposal writes was checked: master lines (the doc's own) · source ranges the report's Changes state
(`mcp.rs:49-111` · `:128-168` · `:178-199` · `:201-230` · `:247-277` · `:319-339` · `:341-397` · `:399-423` ·
`:443-491` · `:493-496` · `:515-522` · `:554-588` · `:590-642`, `cli.rs:35-37` · `:78` · `:135-141`, `Cargo.toml:166`,
`packages/escher-driver/Cargo.toml:16`, `Cargo.lock:3100`, `stand_mcp_tools.rs:160-207`) · test ranges the report's
**New text, by line** lists as rows (`mcp_log.rs:47-240`, `mcp_server.rs:237-278`, `stand_mcp_tools.rs:209-381`,
`common/mod.rs:200-208` · `:219-327` · `:329-334`) · `evidence/live-trial.py` (named by the report, no line). Rejected
for a source the report does not carry: **0**. One collateral clause is not applied: A6's "its tools are
`mcp__escher__*`" (the report carries `--allowedTools mcp__escher` only).

## Proposals and dispositions
Key: **W1** = boundary widening answered by fork 1 (the MCP crossing, «Ратифицировать как есть», `inputs#I3`) · **W4** =
boundary widening answered by fork 4 (the `tool_call` span, «Span с tool и ending», `inputs#I3`) — both escalate
(playbook "Boundary widening", never routine) → the Validate card · **W3** = rests on fork 3 («Малое живое испытание
сейчас», `inputs#I3`), escalated as uneasy (no rule) → the card · **R** = routine, the rule named.

### architecture
- **A1** D-arch-resources · §Standard Contracts → Driver session — 41 re-exports with `serve_mcp(…)`; the JSON-RPC
  revisions, `initialize` / `ping` / `tools/list` / `tools/call`, `-32601` for the rest, notifications; bounds 65,536 /
  the `id` echo (i64 or ≤128-byte string) / 1,048,576; fixed protocol errors, no `data`; the result shape, no
  `structuredContent`; idle expiry on arrival; `command_line`'s boot `FnMut`; the `mcp` role and its endings; usage
  `; mcp role: mcp`. Basis architecture.md:133, mcp.rs ranges, cli.rs:135-141. → **escalate W1** (the bounds and the
  echo are fork 1's option text).
- **A2** dependent-of D-arch-resources · Driver session — "an MCP tool" leaves the Not built list. → **escalate W1**.
- **A3** dependent-of D-arch-resources · Driver session — the crate fields the `command` span's eight values and the
  `tool_call` span's two (`tool`, `ending`), parent of `command` by nesting; the `mcp` role's reply lines on stdout
  written by the hand `json` writer (readers cli · client · host · mcp); `Session::run`'s production callers host.rs
  and mcp.rs. → **escalate W4** (the span) **+ W1** (the reply stream).
- **A4** dependent-of D-arch-resources · §Standard Contracts → Dioxus DOM bridge — retire the two "no MCP command
  exists" clauses; the `snapshot` tool returns the screen text, an accepted acting call its JSON line with the diff,
  through the `mcp` role's stdout. → **escalate W1**.
- **A5** D-arch-resources · §Occupied Resources → Network ports and listeners — the `mcp` role's stdio JSON-RPC to the
  spawning client, local to the invoking user, no listener, socket or port — the socket still the one listener; the
  crossing (id and typed text in; ids, names, masked values, snapshot text out) ratified by the founder 2026-10-10.
  → **escalate W1**.
- **A6** D-arch-resources · §Occupied Resources → Names — the MCP server name `escher` (`serverInfo.name`, version =
  escher-driver's `CARGO_PKG_VERSION`), beside the stub's `stub`. → **apply R** "Accurate this-chunk addition" + the
  plan's entry; the clause "its tools are `mcp__escher__*`" not applied (opening rule).
- **A7** dependent-of D-arch-resources · §Occupied Resources → Process-wide state — five → seven seven_guis targets
  spawning `escher-session` (+ `mcp_server`, `mcp_log`, through `Speaker`); the `mcp` role one process, at most one
  session, no listener, no thread; its stderr the binary's `init` sink, a third role reaching that install site.
  → **apply R** "Accurate this-chunk addition" + the plan's entry.
- **A8** dependent-of D-arch-resources · §Existing Scopes → escher-driver — eleven → twelve private modules with `mcp`;
  `json`'s forms also written by `mcp`. → **apply R** "Accurate this-chunk addition"; carries the sweep's `claim false`
  rows architecture.md:257 (both) with R2.
- **A9** D-arch-decisions · §Established Decisions → [Driver session] — four crates (+ `serde_json`, workspace, no
  feature, reader only, lock one line, no adapter) **and** the MCP crossing ratified by the founder 2026-10-10.
  → **escalate W1** (the crossing half); the `serde_json` half is fork 2's answer («serde_json, только чтение»), applied
  with it.
- **A10** dependent-of D-arch-decisions · §Stack and Technologies — `serde_json` "1" a root workspace entry, 1.0.151,
  escher-driver's MCP request reader. → **apply R** "Accurate this-chunk addition" + the plan's entry (fork 2).
- **A11** dependent-of D-arch-decisions · Driver session — "no serialization crate entered (…re-opened at the MCP
  surface)" → `json`'s `string` / `Object` `pub(crate)`; `serde_json` reads MCP requests only. → **apply R** (fork 2).
- **A12** dependent-of D-arch-decisions · §Occupied Resources → Names — escher-driver's entry depends on four crates.
  → **apply R**; settles the sweep's `claim false` row architecture.md:154.
- **A13** dependent-of D-arch-resources · §Conventions — "its one command span" → two hand-built spans, `command` and
  `tool_call` (parent of `command` on an MCP call). → **escalate W4**.
- **A14** dependent-of D-arch-resources · §Inherited Defaults → Optional capabilities — "escher-driver (its one command
  span)" → its `command` and `tool_call` spans, ungated. → **escalate W4**.

### security-plan
- **S1** D-security-input · §Input Validation (new row: MCP server) — the stdio JSON-RPC boundary: 65,536-byte line,
  −32700 / −32600, the `id` echo form, fixed protocol errors, four methods, notifications, `tools/call` params and
  `call_of`'s kind check, then `validate` / `validate_session`; `start` outside the apps refused; key order (a repeated
  key collapses, `Repeated` cannot arise); 1,048,576 result bound; idle expiry on arrival; replies by `json.rs` only;
  gate entry 10 count 0; results carry ids, names, values, snapshot text by the ratified crossing. → **escalate W1**.
- **S2** dependent-of · §Input Validation → Driver command schema — three surfaces through `validate` (+ MCP
  `tools/call`); outcome also an MCP tool result; the MCP server's idle-expiry check a second clock reader; a second
  span `tool_call`. → **escalate W1 + W4**.
- **S3** dependent-of · `id` row — a third exit (MCP tool results); a third source of a named id (a tool argument).
  → **escalate W1**.
- **S4** dependent-of · accessible-names row — MCP tool results a further exit; `mcp_log` 0 / 0 / 0 at `trace`.
  → **escalate W1**.
- **S5** dependent-of · control-states row — MCP tool results a further carrier of the three readings, still closed
  words and booleans. → **escalate W1**.
- **S6** dependent-of · password / file value row — MCP tool results a further exit; the mask holds by construction
  (snapshot block equal to `Session::run`'s text), a typed password over MCP not separately run. → **escalate W1**.
- **S7** dependent-of · CLI arguments row — the first word `mcp`, its usage shapes, its endings, `FnMut` boot.
  → **escalate W1** (the group validates with S1; the row itself is the CLI's inventory).
- **S8** dependent-of · §Threat Model Summary → Attack surface — the vector "local stdio — the driver's MCP server";
  its results reached the model provider through the client in the live trial. → **escalate W1**.
- **S9** dependent-of · §API Security — the `mcp` role serves the nine verbs over stdio; still one listener.
  → **escalate W1**.
- **S10** dependent-of · §Error Handling → Error format — refusals and session errors also MCP `isError: true` results;
  protocol errors a fixed code and message, no `data`; the admitted `id` echo the one value written back; no reply
  holds the refused id, the tool name or the filler. → **escalate W1**.
- **S11** dependent-of · §Logging & Monitoring → the sink — the `tool_call` span line (`tool`, `ending`, parent of
  `command`), neither field in `CONTENT_FIELDS`; the role's 9 / 10 lines at `info`, 0 needles at `trace`.
  → **escalate W4**.
- **S12** dependent-of · §Logging & Monitoring → Stdout output — the `mcp` role writes protocol lines only, no ESC.
  → **escalate W1** (with the group).
- **S13** D-security-auth · §Secret Management → Development — the live trial a second user of the operator's Claude
  Code login (same source, no new secret). → **escalate W3** (uneasy: the 2026-10-06 ratification names the cold-agent
  run only).
- **S14** dependent-of D-security-auth · §Secret Management → What counts as secret — the same. → **escalate W3**.
- **S15** D-security-auth · §Authentication & Authorization (table) — a row "Local stdio (driver MCP server)": no
  handshake, token or peer check; answers only the process holding its stdio. → **escalate W1** (fork 1's "local to the
  invoking user, no listener or auth surface").
- **S16** D-security-deps · §Dependency Security → Pinning — `serde_json` recorded: workspace entry, no
  `preserve_order`, reader of untrusted stdin, writes nothing, no package enters, audit green. → **apply R** "A proposal
  graded `escalate` by a detector's own severity, where that detector reports its invariant holding and the plan's
  reviewed list names the change" (no ban list exists; the plan names "§Dependency Security: `serde_json` named, its
  hand review"); fork 2's word recorded in its sidecar entry.

### layout-templates
- **L1** D-layout-surface · §Surface: cli → the `escher-session` entry — the third role `escher-session mcp`, its
  stdio, revisions, tools, endings, usage; "no ending writes back anything the caller typed" scoped to the commands
  (the role echoes the admitted `id`). → **apply R** "Accurate this-chunk addition" + the plan's entry.
- **L2** dependent-of · §Surface: desktop-native → seven_guis Home and TaskShell — the `mcp` role a further consumer of
  the TaskShell mount through the same boot, no markup added. → **apply R** "Accurate this-chunk addition".

### test-plan
- **T1** D-tests-obs-harness · §3 → Stand log format — the `mcp` role's `tool_call` span line, parent of `command`; its
  stdout protocol lines only; 9 / 10 lines at `info`; exercised by `mcp_log` and `stand_mcp_tools`. → **escalate W4**.
- **T2** dependent-of · §2 → Process-lifecycle checks — what a sink-installing host writes gains the `tool_call` line.
  → **escalate W4**.
- **T3** dependent-of · §5 → Session host ↔ what it writes — `mcp_log` covers the `mcp` role at `trace` under both
  builds. → **escalate W4**.
- **T4** D-tests-framework · §2 → Process-lifecycle checks — the `cfg(unix)` gate scoped to socket checks;
  `mcp_server` / `mcp_log` through `Speaker` run on windows (CI#38124028789 attempt 2). → **apply R** "Accurate
  this-chunk addition".
- **T5** dependent-of · §3 → session-checks (key) — the same in the key file. → **apply R**.

### obs-plan
- **O1** D-obs-instrumentation · §4 — the second span `tool_call` (`tool`, `ending`), per `tools/call`, parent of
  `command`; session verbs a `tool_call` with no `command` child; "the span is `run`'s" retired as the only span.
  → **escalate W4**.
- **O2** dependent-of · §2 → Telemetry mechanism — two span macros; the `mcp` role's reach. → **escalate W4**.
- **O3** dependent-of · §6 → escher-driver — the `tool_call` closed-span line; the role's 9 / 10 lines at `info`.
  → **escalate W4**.
- **O4** dependent-of · §8 → Scrubbing — two `escher_driver` emitters; no scrub field added. → **escalate W4**.
- **O5** D-obs-stack · §3 → the `escher-session` bullet — the `mcp` role: no subscriber of its own, protocol lines on
  stdout, the sink stderr-only, a third reach of `init`, two install sites still. → **apply R** "Accurate this-chunk
  addition" + the plan's entry.
- **O6** dependent-of · §3 → headless-stand bullet — `serve_mcp`'s replies a further stdout write (never a log line).
  → **apply R**.
- **O7** dependent-of · §6 → examples — the role's endings and usage. → **apply R**.
- **O8** D-obs-stack · §3 → headless-stand bullet — `stand_mcp_tools`' in-process `Recorder` a second exception to
  "installs no subscriber". → **apply R**.
- **O9** D-obs-stack · §3 → headless-stand bullet — seven_guis targets 5 → 7; `mcp_log` reads the role at `trace`; the
  shared module gains `Speaker`. → **apply R** (obs-plan:70's count).

## Raised by the orchestrator (check 5 — expected amendments no detector proposed; the sweep's `claim false` rows)
- **R1** architecture §Project Intent → Front page — the README now says an MCP client drives the apps by spawning
  `escher-session mcp` (the MCP surface no longer "still to come"). → **apply R** (the report's Files: README; the plan's
  entry).
- **R2** architecture §Existing Scopes → escher-driver (:257) — "it depends on blitz-test-harness, dioxus-native-dom and
  `tracing`" → four, with `serde_json`; the sweep's `claim false` rows :257 (both) settle with A8. → **apply R**.
- **R3** test-plan counts and checks — seven integration targets (:10, :66), `run stand` 127 · 0 · 6 over 33 (:116),
  workspace 170 · 805 · 0 · 11 (:329), escher-driver 59 unit tests (:21 and key `session-lifecycle.md:11`), the three new
  checks, `Speaker`, `session-proof`. → **apply R** (the report's Counts / Harness).
- **R4** a11y-plan §1 — the stand graph re-read with `serde_json` named as escher-driver's fourth dependency, no
  adapter (gate entry 9 count 0). → **apply R**.
- **R5** a11y-plan §7 — MCP tool results a further reader of the snapshot model (a `snapshot` result is `to_text`'s
  output). → **escalate W1** (a further exit of the snapshot text).
- **R6** design-system §Surface: cli → Component Patterns — the `mcp` role's stdout: protocol lines only, no colour, no
  ESC, no terminal question. → **apply R** (the report's (design) outcome; the plan's entry).

## Checks 2 · 3 · 4 · 6
- Cross-contradiction: none — obs §3 (O5–O9) and test-plan §3 (T1) state the same role and line; A7 and O9 the same
  count.
- Intent-consistency: the scope record is absent (scope clean, 0 recorded); the report's eight Deviations each carry a
  justification inside the plan's intent — no escalation.
- Absence needs evidence: "no listener, port or socket" rests on the report's Symbols / APIs and the role's code range;
  "no package enters" on gate entry 8 (count 0); "no adapter" on gate entry 9 (count 0) — each cited.
- Disproved claims: (1) argument order — plan text; the measured key order lands in S1's row (matched); (2) one reply
  per line with an id — plan text, no master states it; S1 states the `-32700` answer with `id: null` (matched).

## Escalation resolved (the Validate card, one halt)
- W1 (19) and W4 (12) — 29 proposals: **Confirm all** — the operator's answer, carrying the founder's own confirm
  («Подтвердить всё», 2026-10-11 09:37Z (corrected 2026-10-12, the overseer's estimate; ES33), by question dialog in the overseer session, relayed by the overseer;
  `relay-1.md`). Both forks recorded ratified; no PROVISIONAL mark written.
- W3 (S13 · S14): **Apply on fork 3** — the same answer, the founder's label «Записать по форку 3»; the text scoped to
  the one live trial.
- No playbook rule proposed: the widening class stays never-routine, and no new pattern emerged.
- Every proposal now reads **apply**: 46 detector proposals + 6 raised = 52.
