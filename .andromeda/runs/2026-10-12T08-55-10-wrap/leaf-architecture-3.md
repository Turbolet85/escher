group: architecture-3
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «diffs two snapshots by stable element id into the nodes added, removed and changed (`Snapshot::diff` → `SnapshotDiff` of `DiffNode`s)»
  source: .andromeda/architecture.md:136 «`differs` the readings that differ from before the step, in node-key order (`Reading::ALL`)»
  replace-with: «diffs two snapshots by stable element id into the nodes added, removed and changed (`Snapshot::diff` → `SnapshotDiff` of `DiffNode`s, a changed node naming in `differs` the readings that differ from before the step, in node-key order — `Reading`, `Reading::ALL` — empty for an added node)»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`VERBS`, nine verbs in two levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session, each with its argument and result shapes; a returned node is keyed by the twelve `NODE_FIELDS`, and two served help texts say the state words and the two limits of the boolean model)»
  source: .andromeda/architecture.md:133 «`VERBS` lists ten verbs in three levels (`Level` is `Instance` · `Session` · `Tool`»
  replace-with: «`VERBS`, ten verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session, `help` of the tool itself, each with its argument and result shapes; a returned node is keyed by the thirteen `NODE_FIELDS` — `differs` on a changed node only — each key with its help, the boolean model's two limits stated in its node keys' help, and two served help texts say the state words and those two limits)»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «every edge is a typed `SessionError` (thirteen)»
  source: .andromeda/architecture.md:133 «since 2026-10-12-self-description `remedy() -> &'static str`»
  replace-with: «every edge is a typed `SessionError` (thirteen), each with a fixed remedy (`remedy()`)»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «An outcome, a refusal and a session error each have one written form, a line of JSON (`to_json`, hand-written)»
  source: .andromeda/architecture.md:133 «`describe() -> String` (since 2026-10-12-self-description) writes the tool's self-description, the answer of `help`, as one line of JSON»
  replace-with: «An outcome, a refusal and a session error each have one written form, a line of JSON (`to_json`, hand-written; a session error's carries its remedy), and `describe` writes the tool's self-description, the answer of `help`, as one line of JSON from the tables alone — verbs, node keys, key names, busy classes, causes and session errors with their remedies, and exit statuses (`EXIT_STATUSES`)»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05`, `tools/list` the nine verbs of `VERBS`, `tools/call` run on the one session it holds in process — through `Session::run` for an instance verb, `start` · `status` · `stop` answered by the server»
  source: .andromeda/architecture.md:133 «speaks newline-delimited JSON-RPC 2.0 in the handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`»
  replace-with: «handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05` (`PROTOCOL_VERSIONS`, `2025-11-25` answered when the asked one is not among them), `tools/list` the ten verbs of `VERBS`, `help` last, each described by its help and its result fields, `tools/call` run on the one session it holds in process — through `Session::run` for an instance verb, `start` · `status` · `stop` · `help` answered by the server»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «`start <task>`, the six instance verbs, `status`, `stop`»
  source: .andromeda/architecture.md:138 «`stop` and, since 2026-10-12-self-description, `help` — each with `--session <dir>` exactly once but `help`, which needs none, admits at most one»
  replace-with: «`start <task>`, the six instance verbs, `status`, `stop`, and `help`, which needs no session»

- leaf: CLAUDE.md:44   class: stale
  leaf-text: «while authored text, an open value set, a new carrier or a new exit still is»
  source: .andromeda/architecture.md:101 «riding those ratified carriers, while text from a page, a call or a screen, a new carrier or a new exit still widens»
  replace-with: «and the program's own fixed `'static` table text — the `help` self-description, a session error's remedy, each tool's description with its result fields — rides the command line's stdout and the `mcp` role's stdio as ratified carriers (his ruling of 2026-10-12), while text from a page, a call or a screen, an open value set, a new carrier or a new exit still is»

- leaf: CLAUDE.md:80   class: omission
  leaf-text: «| Driver command and refusal schema (verb table · argument kinds and bounds · validation · causes and remedies) | `packages/escher-driver/src/{schema,command,refusal}.rs` (unit tests inline)»
  source: .andromeda/architecture.md:257 «since 2026-10-12-self-description describe (`describe`, the self-description written from the tables alone)»
  replace-with: «| Driver command and refusal schema (verb table · argument kinds and bounds · validation · causes and remedies · node keys and exit statuses · the `help` self-description) | `packages/escher-driver/src/{schema,command,refusal,describe}.rs` (unit tests inline)»

- leaf: CLAUDE.md:85   class: omission
  leaf-text: «checks `tests/blitz-tests/tests/stand_mcp_tools.rs` and»
  source: .andromeda/architecture.md:263 «since 2026-10-12-self-description `stand_mcp_masked` — a password typed through an MCP `type` reads `MASKED_VALUE`»
  replace-with: «checks `tests/blitz-tests/tests/stand_mcp_{tools,masked}.rs` and»

- leaf: CLAUDE.md:100   class: omission
  leaf-text: «`start {task}`, then `snapshot` · `click --id` · `type --id --text` · `press --key` · `advance --ms` · `scroll --id` · `status` · `stop`;»
  source: .andromeda/architecture.md:138 «`stop` and, since 2026-10-12-self-description, `help` — each with `--session <dir>` exactly once but `help`, which needs none, admits at most one»
  replace-with: «`start {task}`, then `snapshot` · `click --id` · `type --id --text` · `press --key` · `advance --ms` · `scroll --id` · `status` · `stop`, and `help` — the self-description, which reaches no session and needs no `--session`;»

- leaf: CLAUDE.md:101   class: stale
  leaf-text: «its tools the nine verbs»
  source: .andromeda/architecture.md:133 «`tools/list` answers the ten verbs of `VERBS` in order, `help` last»
  replace-with: «its tools the ten verbs, `help` among them»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «one verb table in two levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance and `start` · `status` · `stop` of the session»
  source: .andromeda/architecture.md:133 «`VERBS` lists ten verbs in three levels (`Level` is `Instance` · `Session` · `Tool`»
  replace-with: «one verb table in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance, `start` · `status` · `stop` of the session and `help` of the tool itself»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «the driver's nine verbs as MCP tools over stdio»
  source: .andromeda/architecture.md:133 «`tools/list` answers the ten verbs of `VERBS` in order, `help` last»
  replace-with: «the driver's ten verbs as MCP tools over stdio»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «newline-delimited JSON-RPC 2.0 in three handshake revisions»
  source: .andromeda/architecture.md:133 «speaks newline-delimited JSON-RPC 2.0 in the handshake revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`»
  replace-with: «newline-delimited JSON-RPC 2.0 in four handshake revisions, `2025-11-25` its default since the self-description»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and — still to come — the driver's self-description and the headless screenshot.»
  source: .andromeda/architecture.md:133 «built at 2026-10-12-self-description and off it: the help the driver serves about itself»
  replace-with: «the driver's self-description (built — `help`, the tenth verb, asked of the tool itself on the command line and over MCP and answered with no session run: `describe`'s one line of JSON written from the tables alone — the verbs, the node keys each with its help, key names, busy classes, refusal causes and session errors each with its remedy, and the exit statuses — the program's own fixed text riding the ratified carriers, ratified by the founder (2026-10-12); every MCP tool described with its result fields, and a changed node in a returned diff naming the keys whose reading differs, `differs`), and — still to come — the headless screenshot.»
