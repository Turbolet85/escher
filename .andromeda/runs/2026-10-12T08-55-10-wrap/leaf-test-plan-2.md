group: test-plan-2
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`VERBS`, nine verbs in two levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session,»
  source: .andromeda/test-plan.md:143 «schema.rs pins the verb table — ten verbs in a stated order and three levels (`Level::Instance` · `Level::Session` · `Level::Tool`, `Level::name` reading `instance` · `session` · `tool`»
  replace-with: «`VERBS`, ten verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session, `help` of the tool,»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «a returned node is keyed by the twelve `NODE_FIELDS`»
  source: .andromeda/test-plan.md:143 «since that chunk, the thirteen node keys named in order, each with its help (`differs` after `bounds`)»
  replace-with: «a returned node is keyed by the thirteen `NODE_FIELDS`, `differs` after `bounds`»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «every edge is a typed `SessionError` (thirteen)»
  source: .andromeda/test-plan.md:143 «error.rs pins, since that chunk, that each of the thirteen session errors states a kind, a meaning and a remedy»
  replace-with: «every edge is a typed `SessionError` (thirteen), each stating a kind, a meaning and a remedy»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05`»
  source: .andromeda/test-plan.md:10 «since 2026-10-12-self-description the newest revision `2025-11-25` spoken — `2026-07-28` answered with it»
  replace-with: «handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`, a later revision answered with the newest»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`tools/list` the nine verbs of `VERBS`»
  source: .andromeda/test-plan.md:21 «the listing is the ten verbs of the table — nine until 2026-10-12-self-description»
  replace-with: «`tools/list` the ten verbs of `VERBS`»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «`start` · `status` · `stop` answered by the server»
  source: .andromeda/test-plan.md:21 «since that chunk `help` is answered by the server with or without a session»
  replace-with: «`start` · `status` · `stop` and `help` answered by the server, `help` with or without a session»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «`start <task>`, the six instance verbs, `status`, `stop`»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «then `start <task>`, `status`, `stop`, then, since 2026-10-12-self-description, `help`»
  replace-with: «`start <task>`, the six instance verbs, `status`, `stop`, `help`»

- leaf: CLAUDE.md:100   class: omission
  leaf-text: «`advance --ms` · `scroll --id` · `status` · `stop`; one line of JSON on stdout»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «since 2026-10-12-self-description `help` takes no argument and at most one `--session <dir>`, not read, and answers `describe()`'s line on stdout, status `0`, reaching no session»
  replace-with: «`advance --ms` · `scroll --id` · `status` · `stop`, and `help` (the tool's description, reaching no session); one line of JSON on stdout»

- leaf: CLAUDE.md:101   class: stale
  leaf-text: «its tools the nine verbs»
  source: .andromeda/test-plan.md:201 «`tools/list` offering the verbs in order — ten since that chunk»
  replace-with: «its tools the ten verbs»

- leaf: CLAUDE.md:85   class: omission
  leaf-text: «checks `tests/blitz-tests/tests/stand_mcp_tools.rs` and»
  source: .andromeda/test-plan.md:26 «since 2026-10-12-self-description `stand_mcp_masked` 1 — a password typed over MCP through `serve_mcp` in process»
  replace-with: «checks `tests/blitz-tests/tests/stand_mcp_{tools,masked}.rs` and»

- leaf: CLAUDE.md:80   class: omission
  leaf-text: «causes and remedies) | `packages/escher-driver/src/{schema,command,refusal}.rs`»
  source: .andromeda/test-plan.md:143 «describe.rs that the document holds `help`'s seven fields in order and every text in it is its tables'»
  replace-with: «causes and remedies · the self-description `help` answers) | `packages/escher-driver/src/{schema,command,refusal,describe}.rs`»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «one verb table in two levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance and `start` · `status` · `stop` of the session»
  source: .andromeda/test-plan.md:143 «schema.rs pins the verb table — ten verbs in a stated order and three levels (`Level::Instance` · `Level::Session` · `Level::Tool`, `Level::name` reading `instance` · `session` · `tool`»
  replace-with: «one verb table of ten verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session and `help` of the tool»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «the driver's nine verbs as MCP tools over stdio»
  source: .andromeda/test-plan.md:21 «the listing is the ten verbs of the table — nine until 2026-10-12-self-description»
  replace-with: «the driver's ten verbs as MCP tools over stdio»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «newline-delimited JSON-RPC 2.0 in three handshake revisions»
  source: .andromeda/test-plan.md:201 «since that chunk the newest revision `2025-11-25` spoken and `help` answered before any `start`»
  replace-with: «newline-delimited JSON-RPC 2.0 in four handshake revisions, the newest `2025-11-25`»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «whose `initialize` asked `2025-11-25`, answered `2025-06-18`)»
  source: .andromeda/test-plan.md:201 «answered `2025-06-18`, and the client went on — the trial's reading, taken before the server spoke `2025-11-25`»
  replace-with: «whose `initialize` asked `2025-11-25`, answered `2025-06-18` — a reading taken before the server spoke `2025-11-25`)»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and — still to come — the driver's self-description and the headless screenshot.»
  source: .andromeda/test-plan.md:200 «since 2026-10-12-self-description `help` answers status `0` and one line equal to `describe()`, reaching no session»
  replace-with: «the driver's self-description (built — `help`, a tool-level verb answered with no session, one line equal to `describe()` through the command line and as an MCP tool, every refusal cause and session error carrying its remedy), and — still to come — the headless screenshot.»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «(`Snapshot::diff` → `SnapshotDiff` of `DiffNode`s)»
  source: .andromeda/test-plan.md:24 «`DiffNode::differs` — a changed node names exactly the readings that differ, in `Reading::ALL` order, and an added node none»
  replace-with: «(`Snapshot::diff` → `SnapshotDiff` of `DiffNode`s, a changed node naming exactly the readings that differ, `DiffNode::differs`)»

- leaf: CLAUDE.md:13   class: omission
  leaf-text: «the stand checks share `tests/common/`»
  source: .andromeda/test-plan.md:26 «and `tests/session_common/mod.rs`, what the checks that hold a session share»
  replace-with: «the stand checks share `tests/common/` and the checks that hold a session `tests/session_common/`»
