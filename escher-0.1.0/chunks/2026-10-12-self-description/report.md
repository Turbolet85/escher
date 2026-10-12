# Report — 2026-10-12-self-description

**Chunk:** Self-description — the verb list, help and schemas served by the command line and MCP from the one schema
**Date:** 2026-10-12
**Commits:** `bf973d33 docs(provenance): founder words relayed verbatim; ES33 time; incubator ignored` (between chunks) · `22fc4553 chore(2026-10-12-self-description): operator pre-CI commit, for the run this chunk verdict reads` (`git log --since=2026-10-11T09:57:48Z`, the last wrap's own commit `5698e08e` excluded)

**This wrap** runs P1 only and stops, by the operator's directive (`inputs#I6`), which relays the founder ruling F252 D2: it is the first wrap since the cascade deploy, so its P2 cascade runs WHOLE mode, `cascade.py leaves --all`, to clear the leaf drift accumulated, as a split wrap.

## Changes (structured — detectors read this)
- **Files:** basis `gate.py scope` (wrap P1): `scope: clean — changed 18 · listed 18 · recorded 0`, base `bf973d33` (the parent of the pre-CI commit `22fc4553`).
  - new: `packages/escher-driver/src/describe.rs` · `tests/blitz-tests/tests/stand_mcp_masked.rs`;
  - modified: `packages/escher-driver/src/{schema,error,refusal,json,cli,mcp,command,lib}.rs` · `packages/dioxus-native-dom/src/{snapshot_diff,lib}.rs` · `tests/blitz-tests/tests/{stand_diff,stand_act_scroll,stand_mcp_tools}.rs` · `examples/seven_guis/tests/{cli_commands,mcp_server}.rs` · `README.md`;
  - not touched though listed as optional: `examples/seven_guis/tests/common/mod.rs` (no two checks needed a shared reader);
  - beside the source: `escher-0.1.0/verification-matrix.json` (v010-14 `implemented`), the chunk folder (`plan`, `research`, `scope`, `inputs/`, `evidence/{controls,counts,operator-pass}.md`), the phase and implement run dirs, `.andromeda/friction-log.ndjson`, `.andromeda/master-route.md` and `escher-0.1.0/working-route.md` (phase's promotion), `.claude/session-handoff.md`.
- **Symbols / APIs:** coordinates from the new-text listing below.
  - `escher_driver::describe() -> String` (`describe.rs:11-58`): the self-description as one line of JSON through the crate's `json` writer; its keys are `help`'s seven result fields in order (`verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`), every key name read from `HELP.fields` and every text from the tables (`VERBS`, `NODE_FIELDS`, `KEY_NAMES`, `BUSY_CLASSES`, `CAUSES`, `EXIT_STATUSES`, `SESSION_ERRORS`); reads no environment, prints nothing. Its callers: `cli.rs` (the `help` line), `mcp.rs` (the `help` tool), its own tests, and the checks.
  - `schema.rs`: a tenth verb `HELP` (`schema.rs:475-531`, name `help`, `Level::Tool`, no argument, seven result fields each with help, kinds `Entries` · `Words`); `VERBS` = the six instance verbs, the three session verbs, then `help`; `Level::Tool` with `Level::name()` → `instance` · `session` · `tool` (`schema.rs:232-239`); `FieldKind::Words` and `FieldKind::Entries` (nine kinds); `NODE_FIELDS` is now `[NodeFieldSpec; 13]`, each `{ name, help }` (`schema.rs:32-107`), the twelve keys in their order and a thirteenth, `differs`, after `bounds`; `ExitStatus { status, name, meaning }` and `EXIT_STATUSES: [ExitStatus; 4]` (`schema.rs:539-577`) — `0` accepted · `1` refused · `2` usage · `3` session-error, the meanings the `command_line` doc comment stated.
  - The node keys' help: `role` and `name` "as the accessibility tree reads it" (no role set); `enabled` keeps the presence reading ("the reading follows the attribute's presence, whatever its value"); `value` names the fixed mask for a password or file input; `selected` · `pressed` · `invalid` read by `aria-selected` · `aria-pressed` · `aria-invalid`, `pressed` "a mixed toggle reads false" (no `aria-checked`), `invalid` "a grammar or spelling error reads true" (no kind claimed); `bounds` "a box reads the same bounds however far its content is scrolled"; `differs` "on a changed node only: the keys whose reading differs from before the step, in this order; a node that only moved reads bounds alone".
  - `error.rs`: a crate-private table `SESSION_ERRORS: [ErrorSpec; 13]` (`error.rs:74-158`), `ErrorSpec { kind, meaning, remedy, repeats }` (`repeats` not served); `SessionError::kind()` reads it and a new public `SessionError::remedy() -> &'static str` too; `Display` is the row's meaning, `Io` and `NotSettled` adding their detail as before. Two rows share a remedy, each later row naming the earlier: `dead` repeats `no-session`'s ("start a session in this state directory, or name the directory a running session uses"), `answer-too-large` repeats `not-settled`'s ("the call ran and is not rolled back: take a snapshot to read the screen as it stands") — read true by the operator.
  - `StateDirTooLong`'s message states the bound: on Linux "…: as given, it is at most 94 bytes", on every other target 90 (`error.rs:56-64`); its remedy "name a shorter path for the state directory, or a relative one, which counts only its own bytes".
  - `json.rs`: `SessionError::to_json` writes `{"error":{"kind","message","remedy"}}`; a changed node is written with `differs` after `bounds` (its words the node-key names, read from `NODE_FIELDS` by index, `json.rs:109-128`), an added node without it; `json::list` is now `pub(crate)`.
  - `command.rs`: `SessionCommand::Help` (spec `HELP`); `validate_session` admits the `Session` and `Tool` levels (`verb_at` takes a level list) and returns `Help`; an argument to `help` is refused `malformed`, `Fault::Unnamed`. `validate` is unchanged and refuses `help` `unknown-verb`, so `Session::run` runs no `help` and its `command` span records the table's word `help`.
  - `cli.rs`: `Line::Help` — `help` with no argument and at most one `--session <dir>` (not read) answers `describe()`'s line on stdout, status `0`, reaching no session; `help x` → refused `malformed`, status `1`; `help --session` with no directory or given twice → usage, status `2`. The four status constants read `EXIT_STATUSES[0..3].status`. The usage line lists `help` after `stop`.
  - `mcp.rs`: `PROTOCOL_VERSIONS` = `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`, `LATEST_VERSION` = `2025-11-25`, answered to any revision not spoken (`2026-07-28` and `2099-01-01` included); `tools/list` lists ten tools, `help` last; each tool's `description` is the verb's help, then `. Returns: ` and each result field as `{name} ({kind}): {help}` joined by `; ` (`description`, `mcp.rs:344-353`); a `help` tool call is answered by the server, held session or not, its text block `describe()`'s line, accepted, its `tool_call` span `tool` = `help`, `ending` = `accepted`, no `command` child; a `help` call while a session is held counts in that session's `served`, as any tool call does. The methods answered stay four; `server/discover` stays `-32601`.
  - `refusal.rs`: `Cause::Covered`'s doc comment now reads the `contain: paint` reading (`refusal.rs:20-21`); no served text changed.
  - `dioxus-native-dom`: `pub enum Reading` (eleven variants: `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Bounds`) with `Reading::ALL` (`snapshot_diff.rs:32-74`); `DiffNode::differs: Vec<Reading>` (`snapshot_diff.rs:124-126`), filled at the one site that decides `changed` (`differing`, `snapshot_diff.rs:174-209`, `NodeState` destructured whole), empty for an added node; `Reading` re-exported from `lib.rs:36` (dioxus-native-dom's `pub use` names 15 → 16, counted over `lib.rs` at `bf973d33` and HEAD).
  - escher-driver re-exports 41 → 45: `describe`, `EXIT_STATUSES`, `ExitStatus`, `NodeFieldSpec` (counted over every `pub use` of `lib.rs`; `evidence/counts.md`). The crate doc says ten verbs in three levels, the tool describes itself through `help`, the boolean model's two limits are stated in the node keys' help, and a changed node names `differs` (`lib.rs:30-41`, `:64-68`).
  - Ports · sockets · env vars · listeners: none added — the lock and preservation probes green.
- **Crates / modules:** escher-driver gains the module `describe` (twelve modules → thirteen, counted in `lib.rs`: `cli` `client` `command` `describe` `error` `execute` `host` `json` `mcp` `refusal` `schema` `session` `wire`); blitz-tests gains the test target `stand_mcp_masked` (34 files under `run stand`). No crate added or removed.
- **Dependencies:** none — `git diff --quiet bf973d33 -- Cargo.lock Cargo.toml` and the four manifests: exit 0 (gate entry, green). `serde_json` stays a reader: `describe.rs` and `mcp.rs` hold no `serde_json::to_`, `Serialize`, print, `unwrap`, `expect`, `panic!` or `unreachable!` outside their test modules (the no-panic probe, count 0).
- **Schema / config:**
  - a session error's written form gains `remedy` — `{"error":{"kind","message","remedy"}}`;
  - a changed node's written form gains `differs` (a list of closed words, the node-key names) after `bounds`; an added node's is unchanged;
  - the answer of `help` is a new document (7 keys; 10 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 13 session errors — `describe.rs` tests), every value a fixed string of the table;
  - the MCP `initialize` default revision `2025-06-18` → `2025-11-25`;
  - the `tools/list` result 2,984 → 8,844 bytes (9 → 10 tools), measured over `escher-session mcp` built from `bf973d33` and from this tree (`evidence/counts.md`);
  - no span field added: the `command` span keeps its eight fields, `tool_call` its two; `help` is a new word of `tool_call`'s `tool` and of `command`'s `verb` closed sets (a `help` handed to `Session::run` is refused `unknown-verb` with `verb` = `help`).
- **Spec-master edits:** none (implement wrote none; the masters stand at HEAD `22fc4553`).
- **Counts / qualifiers moved:** each with its basis; master sites from `cascade.py sweep --pre-pass` over `report-patterns.toml` (this run dir), the figure its `standing` count.
  - verbs nine → ten, levels two → three. `verbs9`: 10 standing · `architecture.md:133` ×2 · `:210` · `security-plan.md:81` · `:203` · `layout-templates.md:74` ×2 · `test-plan.md:10` · `:21` ×2 · `:143` · `:201` · `registries/contracts/test-plan/session-lifecycle.md:9` — every row states the driver's verb count: changed (10). `levels2`: 4 standing · `architecture.md:133` · `:257` · `security-plan.md:81` · `test-plan.md:143` — changed (4).
  - revisions three → four, default `2025-06-18` → `2025-11-25`. `rev0618`: 3 standing · `architecture.md:133` ×3 · `layout-templates.md:74` ×2 · `test-plan.md:201` — `architecture.md:133` and `layout-templates.md:74` state the revisions spoken and the default: changed; `test-plan.md:201` records the live trial's reading (asked `2025-11-25`, answered `2025-06-18`): a measured fact of that trial, no-change. `rev0728`: control unfired over the seven masters at `22fc4553` (`cascade.py` refusal line) — no master states the current revision `2026-07-28`.
  - `server/discover` unchanged. `discover`: 4 standing · `architecture.md:133` ×2 · `security-plan.md:83` ×2 · `test-plan.md:10` · `:201` — all true as they stand (`-32601`): no-change.
  - node keys twelve → thirteen, each with help. `nodef12`: 1 standing · `security-plan.md:78` — changed.
  - field kinds seven → nine. `kinds7`: 1 standing · `test-plan.md:143` — changed.
  - a session error's written form gains its remedy. `errform`: 2 standing · `architecture.md:133` · `security-plan.md:346` — changed (2). `statedir`: control unfired over the seven masters at `22fc4553` — no master names the kind `state-dir-too-long` by that spelling.
  - re-exports 41 → 45. `reexp41`: 1 standing · `architecture.md:133` — changed.
  - escher-driver unit tests 59 → 66. `unit59`: 1 standing · `registries/contracts/test-plan/session-lifecycle.md:11` — changed.
  - workspace 805 · 0 · 11 over 170 result lines → 821 · 0 · 11 over 171 (`target/ci-logs/test.log`, the implement run's second fast leg). `ws805`: 2 standing · `test-plan.md:330` (the count) — changed · `design-system.md:76` (a citation `render.rs:805-812`, a true claim sharing the token) — no-change.
  - `run stand` 127 · 0 · 6 over 33 files → 131 · 0 · 6 over 34. `stand127`: 2 standing · `test-plan.md:116` ×2 (the count) — changed · `test-plan.md:330` ("127 result lines" of an earlier chunk's dated reading) — no-change.
  - `session_common` readers: the masters say seventeen; measured 18 (`grep -l 'mod session_common;' tests/blitz-tests/tests/*.rs`, CARRY 8). `readers17`: 3 standing · `architecture.md:263` ("seventeen readers") · `obs-plan.md:70` ("seventeen of the eighteen checks") — changed (2) · `test-plan.md:24` ("seventeen on the snapshot builder", another count) — no-change.
  - escher-driver modules twelve → thirteen. `modules12`: control unfired over the seven masters at `22fc4553` — the masters do not state the module count by "twelve modules" in that wording; a detector's own read of the escher-driver rows is the reach.
  - the front page. `frontpage`: 1 standing · `architecture.md:210` ×2 ("the help the driver serves about itself and the screenshot named…") — changed.
  - the password over MCP. `pwmcp`: 1 standing · `security-plan.md:126` ("a password typed over MCP not separately r[un]") — changed (now measured, `stand_mcp_masked`).
  - the exit statuses, now a table. `exitstat`: 5 standing · `architecture.md:138` · `:210` · `design-system.md:396` · `test-plan.md:50` — the four endings unchanged, now read from `EXIT_STATUSES` and served by `help`: changed where a row says where the meanings live (detector's read), else no-change · `test-plan.md:201` (the trial's `client_exit 0` reading) — no-change.
  - `Cause::Covered`'s stale doc wording. `covered`: control unfired over the seven masters — the code comment alone carried it.
  - the `tool_call` span (`toolcall`, 25 standing over `architecture` · `security-plan` · `test-plan` · `obs-plan` and the session-lifecycle key) and `stand_mcp_tools` (`mcptools`, 11 standing), `session_common` (`sescommon`, 13 standing): listed in the trail `cascade-2026-10-12-self-description-pre-pass.json`; the rows that state `tool`'s closed set (`obs-plan.md:107` ×4, `:181` ×3) gain the word `help`; the rest name the span or the file and stand.
  - A listing reaches its patterns' wording and no further — the detectors' own sweeps stay the control for sites worded otherwise.
- **Dev-tool versions:** none — no host tool installed or upgraded this chunk.
- **Harness / gate surface:** none changed (`scripts/`, `.github/` untouched: the preservation probe green). The `escher-session` binary gains a verb (`help`), its argv grammar the `help` ending above.
- **Cross-project / external claims:**
  - CI on the pushed sha: `22fc455309db` · CI#38225228282 push completed/success · verdict green · checks 16/16 · wall 665 s (operator entry 19, `evidence/operator-pass.md`).
  - `inputs.py verify` (wrap P1, before the report): `inputs: 6 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 1 · unreachable 0 · n/a 4 · uncited 2 · unparsed 0`.
    - `inputs#I1` · `inputs/I1-relay-1.md.txt` · copy message · n/a — the operator's take-up arguments: who decides which fork.
    - `inputs#I2` · `inputs/I2-relay-2.md.txt` · copy message · n/a — the operator's four technical answers and notes.
    - `inputs#I3` · `inputs/I3-self-description-founder-answers.md.txt` · copy no-repo · unchanged — the founder's answers 1A 2A 3A 4A, relayed verbatim by the overseer.
    - `inputs#I4` · `inputs/I4-relay-3.md.txt` · copy message · **copy altered — the snapshot no longer hashes to its sha256** — DRIFT, and what moved: the operator pass masked, on the operator's word, the one host path in it (the overseer's relay file's absolute path → `<overseer>/relays/self-description-founder-answers.md`) and in its twin `.andromeda/runs/2026-10-12T06-49-46-phase/relay-3.md`, nothing else — one rewrite each, 567 → 523 bytes, sha256 `9327fa099d307234c58d728378f62a4adbdc081f7694df3953a7892a7b9171cb` → `f550ddef995c20adf13effeda43e5649657eb0dbfbb5b50c6b198f6c81a3887c`, both recorded in `evidence/operator-pass.md`. The manifest `inputs/andromeda-inputs.json` still records `9327fa09…`, the snap's figure, left as it was on the operator's "nothing else". The words of the message are unchanged ("The record of my words is the message, the folder prefix is the host." — the operator), so every fact the plan rests on I4 (steps 3, 13; `plan.md:11` · `:83` · `:196` · `:482`) stands. Open for this wrap: whether the manifest's recorded sha is re-recorded to `f550ddef…` or the altered verdict stays on record — the operator's word (owner: this wrap, at P2 Validate).
      - I4 names the phase's first review card, `.andromeda/runs/2026-10-12T06-49-46-phase/p5-card-1.md`: **superseded** by the second card (I5); its rows are not carried.
    - `inputs#I5` · `inputs/I5-relay-4.md.txt` · copy message · n/a — the operator's own word ("yes — apply.", operator authority) approving `.andromeda/runs/2026-10-12T06-49-46-phase/p5-card-2.md`, the governing card. Its `## Beyond the plan` rows, verbatim:

      | proposal | owner | rides |
      |---|---|---|
      | Append to `.andromeda/playbook.md` a rule beside the closed-field rule: the program's own fixed text, held as `'static` data of the one table, rides a ratified carrier; text from a page, a call or a screen, a new carrier or a new exit still widens | this chunk's wrap | founder fork 1A («Внутри, с правилом») |
      | Record in `.andromeda/residuals.md`: the actionable-key check as a driver command, not in 0.1.0 | this chunk's wrap | founder fork 3A («Не в 0.1.0, в остатки») |
      | Record in `.andromeda/residuals.md`: taking up MCP revision `2026-07-28` (stateless, `server/discover`) | this chunk's wrap | founder fork 4A («Не сейчас, в остатки») |

    - `inputs#I6` · `inputs/I6-relay-1.md.txt` · copy message · n/a — snapped at this wrap (cited here and above): the operator's directive relaying the founder ruling F252 D2 — P1 only; P2's cascade in WHOLE mode (`cascade.py leaves --all`) as a split wrap; the I4 sha accounted for (above).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - "seventeen readers" of `tests/session_common/mod.rs` (`architecture.md:263`) and "seventeen of the eighteen checks that hold a session" (`obs-plan.md:70`): measured 18 files declaring `mod session_common;` (`grep -l 'mod session_common;' tests/blitz-tests/tests/*.rs` → 18; `stand_mcp_tools` is the eighteenth since 2026-10-10-mcp-surface, CARRY 8). `stand_mcp_masked` declares no `mod session_common;` and does not move it.
- **Expected amendments (from plan):** one line per entry of `plan.md`'s list (`plan.md:515-542`); searches are the sweep's patterns above.
  - architecture §Standard Contracts → Driver session (ten verbs in three levels; `help`, `describe` and the seven keys; thirteen node keys with help, `differs`; `EXIT_STATUSES`; the session errors' table and remedy; four revisions, `2025-11-25` by default; descriptions with result fields; re-exports from 41) — carried: Symbols / APIs · Schema / config · Counts. Sites: `verbs9` `architecture.md:133` ×2 · `levels2` `:133` · `rev0618` `:133` ×3 · `errform` `:133` · `reexp41` `:133` · `levels2` `:257`; `nodef12` 0 rows in architecture (its one row is `security-plan.md:78`).
  - architecture §Existing Scopes (escher-driver modules twelve → thirteen; blitz-tests gains `stand_mcp_masked`) — carried: Crates / modules. Sites: `modules12` control unfired (0 in the masters by that wording); `mcptools` `architecture.md:263` ×2 (the blitz-tests row naming its files).
  - architecture §Project Intent → Front page (the self-description built) — carried: Files (`README.md:48`) · Counts. Sites: `frontpage` `architecture.md:210` ×2.
  - security-plan §Input Validation (the Driver command schema row: ten verbs, the tool level, `help` by `validate_session`; the CLI arguments row: the `help` ending; the MCP server row: four revisions, ten tools, descriptions with result fields, `server/discover` still `-32601`; the password row: measured) — carried: Symbols / APIs · Counts. Sites: `verbs9` `security-plan.md:81` · `:203` · `levels2` `:81` · `nodef12` `:78` · `discover` `:83` ×2 (no-change) · `pwmcp` `:126`.
  - security-plan §Error Handling (a session error's written form with its remedy; the `StateDirTooLong` bound; pattern "Boundary widening", answered by the founder 1A, `inputs#I3`, whose own answer makes the wrap halt once) — carried: Schema / config · Symbols / APIs. Sites: `errform` `security-plan.md:346`; `statedir` control unfired (0).
  - test-plan §1 · §4 · §6 (the unit counts, the new check file) and §3 session-binary (the `help` ending) — carried: Counts. Sites: `unit59` `registries/contracts/test-plan/session-lifecycle.md:11` (a test-plan key file) · `ws805` `test-plan.md:330` · `stand127` `test-plan.md:116` ×2 · `verbs9` `test-plan.md:10` · `:21` ×2 · `:143` · `:201` and `session-lifecycle.md:9` · `kinds7` `test-plan.md:143` · `exitstat` `test-plan.md:50`.
  - obs-plan §4 (`tool_call`'s `tool` and `command`'s `verb` closed sets gain `help`) and §3 (the no-subscriber census gains `stand_mcp_masked`; the `session_common` readers re-counted, CARRY 8) — carried: Schema / config · Spec claims disproved · Counts. Sites: `toolcall` `obs-plan.md:107` ×4 · `:181` ×3 (the closed sets) · `:30` ×2 · `:70` ×2 · `:106` · `:300`; `readers17` `obs-plan.md:70`; `mcptools` `obs-plan.md:70` ×2 · `:107` ×2 · `:180`.
  - layout-templates §Surface: cli → Primary screens (`escher-session help`; the `mcp` role's revisions and ten tools) — carried: Symbols / APIs. Sites: `verbs9` `layout-templates.md:74` ×2 · `rev0618` `:74` ×2.
  - design-system §Surface: cli → Component Patterns (the `help` answer line) — carried: Symbols / APIs (one line of JSON, no ESC, status `0`). Sites: `exitstat` `design-system.md:396`; `ws805` `design-system.md:76` (a citation, no-change).
  - a11y-plan §7 (the node keys' help and `differs` as readers of the snapshot model; the password typed over MCP measured) — carried: Symbols / APIs (the node keys' help words) · Outcome (`stand_mcp_masked`). Sites: `mcptools` `a11y-plan.md:275`; `pwmcp` 0 rows in a11y-plan (its one row is `security-plan.md:126`).
  - `matrix#v010-13` notes — the acceptance restated "the ten verbs of `VERBS`" on the founder's fork 2A (`inputs#I3`): ledger-note — owner P7.3.
  - Beyond the masters, on the founder's answers (`inputs#I3`) and the governing card's rows (`inputs#I5`, carried above): the playbook rule (fork 1A) and the two `.andromeda/residuals.md` entries (forks 3A, 4A) — carried: Cross-project bullet, I5's rows; owner this wrap. Their files are outside the sweep: `.andromeda/playbook.md` and `.andromeda/residuals.md` keep their own read at P2.
- **Coverage of new surfaces:**
  - `escher-session help` (the command line) → validation `validate_session` + the reader's one-`--session` rule✓ · instrumentation n/a (no session run; the command line installs no span) · PII n/a (every text a fixed string of the table; `help --verb click` writes back nothing it named — `cli_commands`) · tests unit (`cli.rs` rows) / integ (`cli_commands`, `mcp_server`) · a11y n/a · tokens n/a (one uncoloured line, no ESC byte)
  - the MCP `help` tool → validation `validate_session` (an argument refused `malformed`)✓ · instrumentation `tool_call` span (`tool` = `help`, `ending`)✓ · PII n/a (fixed text) · tests unit (`mcp.rs`) / integ (`stand_mcp_tools`, `mcp_server`) · a11y n/a · tokens n/a
  - the MCP revision `2025-11-25` → validation the closed revision table✓ · instrumentation n/a (no log line on `initialize`) · PII n/a · tests unit / integ (`mcp_server`) · a11y n/a · tokens n/a
  - a session error's `remedy` → validation n/a (an output) · instrumentation n/a · PII n/a (fixed strings; no path — the bound is a figure) · tests unit (`error.rs`, `json.rs`) / integ (`cli_commands`: `no-session` and `state-dir-too-long`) · a11y n/a · tokens n/a
  - `DiffNode::differs` and its written form → validation n/a (computed from two snapshots, a closed enum) · instrumentation n/a (the `command` span keeps its counts only) · PII n/a (closed words) · tests unit (`snapshot_diff.rs`, `json.rs`) / integ (`stand_diff`, `stand_mcp_masked`) · a11y a reader of the snapshot model✓ · tokens n/a

## Deviations from intent
- The row types are named `NodeFieldSpec` (the node-key row) and `ExitStatus` (the exit-status row), the readings enum `Reading`, and `Level` gained `name()` for its word — the plan named none of them.
- The session errors' table (`SESSION_ERRORS`, `ErrorSpec`) is crate-private: the plan's re-export list (step 11) names four items and not it; it reaches a caller through `describe()` and `SessionError::remedy()`.
- Two remedies are shared (`dead` · `no-session`; `answer-too-large` · `not-settled`), each later row naming the earlier — the plan's step 3 rule (`inputs#I4`); read true by the operator after implement.
- The `usage` exit status's meaning reads "a `--session` that is absent where the verb needs one or is given twice" — `help` needs none.
- `stand_diff` checks `differs` on every step of the file against its own reading (in `assert_exact`), beyond the plan's Book step and one value step; `stand_mcp_masked` also asserts the `type` result's changed node names `differs`.
- A `help` call over MCP while a session is held counts in that session's `served`, as every tool call does — the plan did not say.
- `README.md:48` says "ten driver commands" and how to ask for `help` on both surfaces.
- scope record: none — `gate.py scope` clean, 0 recorded (wrap P1: `scope: clean — changed 18 · listed 18 · recorded 0`).

## Decisions & corrections
- The operator accepted implement's report with its departures: "the two shared remedies read true".
- The operator ordered the operator pass's mask of the overseer path in the phase relay and its snapped copy, and nothing else: "The record of my words is the message, the folder prefix is the host." The first hygiene firing read red on that one file (`P1 … relay-3.md:1 ×1 · home`), the second clean.
- The founder ruling F252 D2 (`inputs#I6`): the first wrap after the cascade deploy runs the cascade WHOLE mode, `cascade.py leaves --all`, as a split wrap; this session runs P1 only.
- Sweep hazard: `cascade.py sweep --pattern` takes its ids after ONE flag (`--pattern A B C`); the flag given once per id keeps only the last (measured at this P1: 18 flags ran 1 pattern).
- Sweep hazard: the token `805` (the workspace count) also matches a citation range `render.rs:805-812` in `design-system.md:76`.
- Implement's source edits made by a python string replacement escape the PostToolUse format hook: the fast leg read red on a rustfmt diff in `cli.rs` and `cli_commands.rs`, fixed by `cargo fmt` and one re-run (implement fix-loop).
- A relay a phase writes verbatim can carry a host path of a sibling directory: hygiene's P1 refuses it at the operator pass (measured here); the snapped copy is not read by P1.

## Outcome
- Acceptance criteria, each against the diff:
  - **v010-14** — met: `escher-session help` answers status `0` and one line equal to `describe()` (`cli_commands::help_answers_the_tools_description_and_reaches_no_session`); over MCP, `help` first after `initialize` and before any `start` answers the document (`mcp_server::the_newest_revision_is_spoken_and_help_answers_before_any_start`) and the same bytes as the command line (`mcp_server::help_reads_the_same_bytes_through_mcp_and_the_command_line`); `tools/list` lists the ten verbs with each description holding its result fields (`mcp::tests::the_listing_is_the_ten_verbs_of_the_table`); every cause provoked reads `remedy` equal to its cause's — seven through the binary (`cli_commands`: `unknown-verb`, `malformed`, `disabled`, `not-found`, `off-screen`, `stale`, `time-unavailable`), seven through MCP with `covered` on the overlay fixture (`stand_mcp_tools::every_cause_a_tool_call_reaches_is_served_with_its_remedy`); together the eight. Matrix `v010-14` reads `implemented` with these refs.
  - the password over MCP — met: the sentinel 0 times in both results; the input reads `MASKED_VALUE`, role `PasswordInput`, a non-empty name, both layout modes (`stand_mcp_masked`).
  - the Book's eight nodes read `differs` = `[bounds]`, both modes; `differs` lists exactly what differs in node-key order — met (`stand_diff::the_flight_bookers_book_names_eight_nodes_that_only_moved`, `assert_exact` on every step, `snapshot_diff` unit tests).
  - revisions — met: `2025-11-25` → `2025-11-25`, `2026-07-28` → `2025-11-25`, `server/discover` `-32601` (`mcp` unit rows, `mcp_server`).
  - the Linux bound — met: 94 bytes as given starts and stops; 95 ends `state-dir-too-long`, status `3`, message naming 94, remedy the table's; every session error's written form carries kind, message and remedy (`cli_commands::a_state_directory_of_94_bytes_holds_a_session_and_one_of_95_is_refused_with_its_bound`, `json` unit test). The macOS figure 90 is read, not measured.
  - fixed text only, nothing typed written back — met (`cli_commands`, `mcp_server`; the no-panic probe 0).
  - the `help` span — met: `tool` = `help`, `ending` = `accepted`, no `command` child (`stand_mcp_tools::help_is_answered_by_the_server_and_parents_no_command`); the `mcp` role's stderr at trace holds 0 sentinels, ids and names (`mcp_log`, green).
  - one line, no ESC, the four endings keep their statuses and streams; no driver code asks whether a stream is a terminal — met (`cli_commands`, `mcp_server`, the terminal probe 0).
  - no dependency, feature or package moves; nothing outside the write set — met (the lock probe and the preservation probe, exit 0).
  - `ci-leg.sh fast` and `doc` pass, and CI on the pushed sha reads green — met (below).
- Gates (implement's second full block on the restored tree, then the operator pass):
  - `cargo test -p escher-driver --locked` — green (exit 0, lacks FAILED; 66 passed)
  - `cargo test -p dioxus-native-dom --locked` — green
  - `cargo build --locked -p seven_guis --bin escher-session && target/debug/escher-session help` — green (exit 0, contains `"verbs":[` and `"exit_statuses":[`) — the smoke of the changed binary
  - `cargo test -p blitz-tests --locked --test stand_mcp_tools --test stand_mcp_masked --test stand_diff --test stand_act_scroll` — green
  - `cargo test -p seven_guis --locked --test cli_commands --test mcp_server --test mcp_log` — green
  - `cargo test --workspace --locked --test stand_mcp_tools --test stand_mcp_masked --test stand_diff --test cli_commands --test mcp_server --test mcp_log` — green
  - `cargo test -p seven_guis --locked --test cli_flow --test host_binary --test host_log --test host_timer` — green
  - `cargo test -p blitz-tests --locked --test stand_act_spans --test stand_act_refused --test stand_act_obstructed --test stand_act_diff --test stand_act_filled --test stand_session_lifecycle --test stand_snapshot_text --test stand_accessibility_ids --test stand_actionable_keys` — green
  - `git diff --quiet bf973d33… -- Cargo.lock Cargo.toml …` (the lock probe) — green (exit 0)
  - `sed '/^#\[cfg(test)\]/,$d' …describe.rs …mcp.rs | grep -c -E 'println!|…'` (the no-panic probe) — green (exit 1, last line 0)
  - `cat packages/escher-driver/src/*.rs | grep -c -E 'IsTerminal|is_terminal|isatty'` (the terminal probe) — green (exit 1, last line 0)
  - `git diff --quiet bf973d33… -- …execute.rs …session.rs …` (the preservation probe) — green (exit 0)
  - `bash scripts/agent-run.sh boot` — green
  - `bash scripts/agent-run.sh run stand` — green (`run.end` 131 passed · 0 failed · 6 ignored)
  - `bash .github/scripts/ci-leg.sh fast` — green (red at implement's first firing on a rustfmt diff, green after `cargo fmt`)
  - `bash .github/scripts/ci-leg.sh doc` — green
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) — red at its first firing (one host path in the phase relay), green at its second after the mask: `hygiene: clean` (`evidence/operator-pass.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` (leg operator) — run by hand as written (it moves history): exit 0, pushed `bf973d33..22fc4553` (`evidence/operator-pass.md`)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (leg operator) — green: `22fc455309db verdict: green · checks 16/16 · wall 665 s · runs CI#38225228282` (`evidence/operator-pass.md`)
  - smoke: the boot path changed (a new answer); its `role = 'smoke'` entry ran green as a gate, twice.
  - mutation controls (plan step 1): 11 of 11 reddened their named test, 11 of 11 restores sha-equal (`evidence/controls.md`).
- Watches: none folded.
- Outcome basis: the operator pass ran — the pre-CI commit `22fc4553` (`git log --format='%h %s' bf973d33..HEAD`: that one commit), the final HEAD's CI run CI#38225228282 green, recorded in `evidence/operator-pass.md`; implement's P4 report (this session) for the rest; between implement and this report the operator's directive of the mask (above). Post-implement artifacts: `evidence/operator-pass.md`.
- Process hygiene: implement's census — nothing of its runs left running (the `escher-session` hosts and `mcp` children the checks spawn, the scratch base build, the controls' cargo runs); the operator pass's fast leg, push and CI poll ended with their entries. Re-measured at this P1 by process name: 0 processes named `escher-sessio*`, `seven_guis_nati*`, `cli_commands*`, `mcp_server*` or `stand_mcp*` (`ps -eo pid,comm`, exit 1). The host also runs another project's cargo-mutants (not this chunk's).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: bf973d33 (the parent of the oldest pre-CI commit 22fc4553) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### README.md — added 1 line(s) in 1 range(s)
added: 48
### examples/seven_guis/tests/cli_commands.rs — added 131 line(s) in 8 range(s)
added: 4-10 · 16-17 · 33-48 · 106 · 118 · 343 · 378-381 · 436-534
- 33-47 @36 «fn remedied(line: &str) -> String {»
  - 38-41 «let stated = CAUSES»
  - 42-45 «assert!(»
- 437-465 @438 «fn help_answers_the_tools_description_and_reaches_no_session() {»
  - 442-457 «for (row, args) in rows.into_iter().enumerate() {»
  - 461-464 «assert!(»
- 467-534 @469 «fn a_state_directory_of_94_bytes_holds_a_session_and_one_of_95_is_refused_with_its_bound() {»
  - 475-485 @477 «let at = |length: usize| -> (PathBuf, String) {»
  - 486-490 «let run = |args: &[&str]| {»
  - 497-500 «let _started = Started {»
  - 509-514 «assert_eq!(»
  - 516-519 «assert!(»
  - 521-524 «assert!(»
  - 525-528 «assert!(»
  - 529-532 «assert!(»
### examples/seven_guis/tests/mcp_server.rs — added 76 line(s) in 5 range(s)
added: 2-5 · 20 · 123 · 126 · 239-307
- 239-249 @241 «fn initialized(speaker: &mut Speaker, id: u32, asked: &str) -> String {»
  - 242-247 «speaker.send(»
- 251-284 @252 «fn the_newest_revision_is_spoken_and_help_answers_before_any_start() {»
  - 254-258 «let rows = [»
  - 259-266 «for (id, (asked, answered)) in (1u32..).zip(rows) {»
  - 268-273 «assert!(»
  - 275-280 «assert!(»
- 286-306 @288 «fn help_reads_the_same_bytes_through_mcp_and_the_command_line() {»
  - 293-297 «assert_eq!(»
  - 300-305 «assert!(»
### packages/dioxus-native-dom/src/lib.rs — added 1 line(s) in 1 range(s)
added: 36
### packages/dioxus-native-dom/src/snapshot_diff.rs — added 148 line(s) in 10 range(s)
added: 32-75 · 91-93 · 101-103 · 121 · 124-126 · 174-211 · 219 · 320 · 372 · 378-430
- 32-57 @34 «pub enum Reading {»
- 59-74 «impl Reading {»
  - 60-73 @61 «pub const ALL: [Reading; 11] = [»
- 174-209 @176 «fn differing(»
  - 182-191 @183 «let NodeState {»
  - 193-205 «[»
### packages/escher-driver/src/cli.rs — added 44 line(s) in 21 range(s)
added: 12 · 18 · 22 · 24 · 26 · 28 · 53-56 · 63 · 110-111 · 153 · 159-164 · 187 · 302 · 344 · 362 · 565-572 · 665
       761-763 · 767-770 · 779-780 · 793-794
  - 159-164 «if command == Some(SessionCommand::Help) {»
### packages/escher-driver/src/command.rs — added 36 line(s) in 18 range(s)
added: 154-155 · 167-168 · 178 · 203 · 230-232 · 234 · 242 · 248-249 · 251 · 544 · 553 · 569 · 571 · 593-594 · 614
       793-796 · 802 · 804-813
### packages/escher-driver/src/describe.rs — new file · 222 line(s)
- 7-9 «use crate::schema::{»
- 11-58 @20 «pub fn describe() -> String {»
  - 21-52 «let values = [»
  - 54-56 «for (field, value) in HELP.fields.iter().zip(values) {»
- 60-66 «fn words(items: &[&str]) -> String {»
  - 61-65 «json::list(items.iter().map(|item| {»
- 68-76 «fn verb(verb: &VerbSpec) -> String {»
- 78-86 «fn arg(arg: &ArgSpec) -> String {»
- 88-95 «fn field(field: &FieldSpec) -> String {»
- 97-222 @98 «mod tests {»
  - 104-109 «fn document() -> serde_json::Map<String, Value> {»
  - 111-116 «fn entries<'d>(document: &'d serde_json::Map<String, Value>, key: &str) -> &'d Vec<Value> {»
  - 118-152 @119 «fn the_document_holds_helps_seven_fields_in_order() {»
  - 154-221 @155 «fn every_text_in_it_is_its_tables() {»
### packages/escher-driver/src/error.rs — added 213 line(s) in 6 range(s)
added: 42-159 · 161-180 · 183-188 · 194 · 196-198 · 254-318
- 42-54 @45 «pub(crate) struct ErrorSpec {»
- 56-61 @60 «const STATE_DIR_TOO_LONG: &str =»
- 62-64 @63 «const STATE_DIR_TOO_LONG: &str =»
- 66-68 @67 «const START_HERE: &str =»
- 70-72 @71 «const READ_THE_SCREEN: &str =»
- 74-158 @75 «pub(crate) const SESSION_ERRORS: [ErrorSpec; 13] = [»
  - 76-81 «ErrorSpec {»
  - 82-87 «ErrorSpec {»
  - 88-93 «ErrorSpec {»
  - 94-100 «ErrorSpec {»
  - 101-106 «ErrorSpec {»
  - 107-113 «ErrorSpec {»
  - 114-119 «ErrorSpec {»
  - 120-126 «ErrorSpec {»
  - 127-132 «ErrorSpec {»
  - 133-138 «ErrorSpec {»
  - 139-144 «ErrorSpec {»
  - 145-150 «ErrorSpec {»
  - 151-157 «ErrorSpec {»
  - 161-179 @162 «const fn spec(&self) -> &'static ErrorSpec {»
  - 254-305 @255 «fn the_thirteen_errors_each_state_a_kind_a_meaning_and_a_remedy() {»
  - 307-317 @308 «fn the_state_directorys_message_states_its_bound() {»
### packages/escher-driver/src/json.rs — added 60 line(s) in 20 range(s)
added: 7 · 13 · 109-128 · 132 · 167-176 · 181 · 193-194 · 216 · 228 · 237 · 304-305 · 310 · 475 · 479 · 481 · 496 · 521
       529-536 · 651 · 670-673
  - 115-127 «let index = match reading {»
  - 167-176 «if let Some(differs) = parts.differs {»
### packages/escher-driver/src/lib.rs — added 22 line(s) in 5 range(s)
added: 30-41 · 64-68 · 96 · 112 · 119-121
### packages/escher-driver/src/mcp.rs — added 76 line(s) in 18 range(s)
added: 19 · 35 · 38 · 53-55 · 60-64 · 67-68 · 81-82 · 344-356 · 401 · 620-623 · 635 · 889 · 893-894 · 937 · 1033
       1042-1046 · 1049-1057 · 1199-1221
- 344-353 @346 «fn description(verb: &VerbSpec) -> String {»
  - 347-351 «let fields: Vec<String> = verb»
  - 1199-1220 @1200 «fn help_is_answered_by_the_server_with_or_without_a_session() {»
### packages/escher-driver/src/refusal.rs — added 2 line(s) in 1 range(s)
added: 20-21
### packages/escher-driver/src/schema.rs — added 290 line(s) in 19 range(s)
added: 32-107 · 183-186 · 200-201 · 227-239 · 475-534 · 536-577 · 591 · 593 · 597-598 · 602-607 · 609-617 · 622 · 697
       731-743 · 775-781 · 783 · 843-844 · 846 · 851-898
- 32-39 @34 «pub struct NodeFieldSpec {»
  - 45-48 «NodeFieldSpec {»
  - 49-52 «NodeFieldSpec {»
  - 53-56 «NodeFieldSpec {»
  - 57-61 «NodeFieldSpec {»
  - 62-67 «NodeFieldSpec {»
  - 68-72 «NodeFieldSpec {»
  - 73-78 «NodeFieldSpec {»
  - 79-83 «NodeFieldSpec {»
  - 84-88 «NodeFieldSpec {»
  - 89-93 «NodeFieldSpec {»
  - 94-97 «NodeFieldSpec {»
  - 98-102 «NodeFieldSpec {»
  - 103-107 «NodeFieldSpec {»
  - 232-239 @233 «pub const fn name(self) -> &'static str {»
- 475-531 «pub(crate) const HELP: VerbSpec = VerbSpec {»
  - 478-481 «help: "describes the tool itself, as one document: every verb with its arguments and its \»
  - 483-530 «fields: &[»
- 539-548 @541 «pub struct ExitStatus {»
  - 552-557 «ExitStatus {»
  - 558-564 «ExitStatus {»
  - 565-571 «ExitStatus {»
  - 572-577 «ExitStatus {»
  - 852-881 @853 «fn the_thirteen_node_keys_are_named_in_order_each_with_its_help() {»
  - 883-898 @884 «fn the_four_exit_statuses_are_stated_in_status_order() {»
### tests/blitz-tests/tests/stand_act_scroll.rs — added 1 line(s) in 1 range(s)
added: 505
### tests/blitz-tests/tests/stand_diff.rs — added 89 line(s) in 5 range(s)
added: 9-11 · 20 · 220-260 · 478-481 · 489-528
  - 220-255 @221 «let misread = diff»
  - 256-259 «assert!(»
- 489-527 @490 «fn the_flight_bookers_book_names_eight_nodes_that_only_moved() {»
  - 492-525 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_mcp_masked.rs — new file · 156 line(s)
- 23-30 @24 «fn password_fixture() -> Element {»
  - 25-29 «rsx! {»
- 32-41 «fn boot(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {»
  - 33-40 «move |app| {»
- 43-47 «fn tool_call(id: usize, name: &str, arguments: &str) -> String {»
  - 44-46 «format!(»
- 49-78 @52 «fn result(reply: &str) -> (bool, String) {»
  - 53-55 «let (_, rest) = reply»
  - 58-76 «loop {»
- 80-156 @81 «fn a_password_typed_over_mcp_reads_the_mask_and_never_the_typed_text() {»
  - 83-155 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_mcp_tools.rs — added 199 line(s) in 3 range(s)
added: 12-19 · 24-27 · 391-577
- 391-406 @392 «fn covered_fixture() -> Element {»
  - 393-405 «rsx! {»
- 408-415 @409 «fn remedied(text: &str) -> Option<Cause> {»
  - 410-412 «let cause = CAUSES»
  - 413-414 «text.contains(&format!(r#""remedy":"{}""#, cause.remedy()))»
- 417-522 @418 «fn every_cause_a_tool_call_reaches_is_served_with_its_remedy() {»
  - 419-521 «for incremental in [false, true] {»
- 524-576 @525 «fn help_is_answered_by_the_server_and_parents_no_command() {»
  - 526-532 «let requests = [»
  - 536-539 «assert!(»
  - 540-545 «for index in [1, 3] {»
  - 548-552 «let driver: Vec<&Seen> = recorded»
  - 553-556 «assert!(»
  - 557-564 «let helps: Vec<&&Seen> = driver»
  - 566-575 «for (index, seen) in helps.iter().enumerate() {»
