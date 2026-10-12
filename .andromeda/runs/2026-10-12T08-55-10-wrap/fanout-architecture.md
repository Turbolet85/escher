proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      The crate re-exports 45 names, where it said 41: beside those named, `describe` from the new private module `describe`, and `EXIT_STATUSES`, `ExitStatus` and `NodeFieldSpec` from the command and refusal schema, which now accounts for 28 of them where it was 25. `SESSION_ERRORS` and `ErrorSpec` stay crate-private and are not re-exported. Re-point the lib.rs citation to the file, since the report gives no new range.
    sidecar: >-
      Driver session: re-exports 41 → 45 (describe, EXIT_STATUSES, ExitStatus, NodeFieldSpec).
    rationale: >-
      The report's Symbols / APIs bullet says escher-driver's re-exports went from 41 to 45, counted over every `pub use` of lib.rs (evidence/counts.md). The Counts bullet's `reexp41` row marks `architecture.md:133` as changed.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      `VERBS` lists ten verbs in three levels, where it listed nine in two. `Level` is now `Instance` · `Session` · `Tool`, and `name()` returns `instance` · `session` · `tool` (packages/escher-driver/src/schema.rs:232-239). The order is the six instance verbs, the three session verbs, then `help`. `help` is the row `HELP` (packages/escher-driver/src/schema.rs:475-531): level `Tool`, no argument, and seven result fields, each with its help: `verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`. `FieldKind` gains `Words` and `Entries`, nine kinds where it had seven.
    sidecar: >-
      Driver session: VERBS ten verbs in three levels (Level::Tool, Level::name()), the HELP row, FieldKind nine kinds.
    rationale: >-
      The report's Symbols / APIs bullet (`schema.rs`) lists a tenth verb `HELP` with `Level::Tool`, `Level::name()`, and `FieldKind::Words` and `FieldKind::Entries` (nine kinds). Counts `verbs9` and `levels2` mark `architecture.md:133` as changed.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      `NODE_FIELDS` is now `[NodeFieldSpec; 13]`, each entry `{ name, help }` (packages/escher-driver/src/schema.rs:32-107): the twelve keys in their order, then a thirteenth, `differs`, after `bounds`, which is present on a changed node only. Each key's help states its reading: `enabled` follows the attribute's presence; `value` names the fixed mask; `selected` · `pressed` · `invalid` are read from `aria-selected` · `aria-pressed` · `aria-invalid`, a mixed toggle reads pressed false and a grammar or spelling error reads invalid true; `bounds` stay the same however far content is scrolled. So the node keys' help states the boolean model's two limits beside the crate doc. The acting verbs' `added` and `changed` nodes take the thirteen fields, replacing "the twelve fields", and the schema.rs:32-37 citation becomes 32-107.
    sidecar: >-
      Driver session: NODE_FIELDS thirteen NodeFieldSpec entries with help, `differs` after `bounds`.
    rationale: >-
      The report's Symbols / APIs bullet (`schema.rs`) and its node keys' help bullet. It also says the crate doc now states that the boolean model's limits are in the node keys' help (`lib.rs:30-41`, `:64-68`).
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      Add the exit-status table: `ExitStatus { status, name, meaning }` and `EXIT_STATUSES: [ExitStatus; 4]` (packages/escher-driver/src/schema.rs:539-577), with `0` accepted · `1` refused · `2` usage · `3` session-error. The meanings are the ones the `command_line` doc comment stated. The command line's four status constants read the table, and `help` serves it.
    sidecar: >-
      Driver session: EXIT_STATUSES / ExitStatus table registered.
    rationale: >-
      The report's Symbols / APIs bullet (`schema.rs`, `cli.rs`) adds a public type and a public constant. The contract does not name either.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      Add `describe() -> String` (packages/escher-driver/src/describe.rs:11-58). It writes the tool's self-description as one line of JSON through the crate's `json` writer. Its keys are `HELP`'s seven result fields in order, every key name read from `HELP.fields` and every text from the tables: 10 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 13 session errors. It reads no environment and prints nothing. Its callers are the command line's `help` line and the MCP `help` tool. The "Not built" list's as-measured chain gains escher-0.1.0/chunks/2026-10-12-self-description/report.md for the self-description, the remedies, `differs` and the fourth MCP revision.
    sidecar: >-
      Driver session: describe() registered — the self-description served by `help`.
    rationale: >-
      The report's Symbols / APIs bullet lists `escher_driver::describe()` as a new public function, and Schema / config says the answer of `help` is a new document. Neither appears in the contract.
    basis: packages/escher-driver/src/describe.rs:11-58
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      `SessionError` stays thirteen variants, now read from a crate-private table `SESSION_ERRORS: [ErrorSpec; 13]` (packages/escher-driver/src/error.rs:74-158), each row `ErrorSpec { kind, meaning, remedy, repeats }`; `repeats` is not served. `kind()` reads the table. A new public `remedy() -> &'static str` reads it too. `Display` is the row's meaning, with `Io` and `NotSettled` adding their detail as before. Two rows share a remedy, each naming the earlier row: `dead` uses `no-session`'s and `answer-too-large` uses `not-settled`'s. `StateDirTooLong`'s message states its bound as a figure with no path: at most 94 bytes as given on Linux, 90 on every other target (packages/escher-driver/src/error.rs:56-64). Its remedy is to name a shorter or a relative path. Re-point the error.rs:12-203 citation to the file.
    sidecar: >-
      Driver session: SessionError's table SESSION_ERRORS (crate-private), remedy(), the StateDirTooLong bound in its message.
    rationale: >-
      The report's Symbols / APIs bullets for `error.rs` and `StateDirTooLong` add a public method and a fixed table. The contract states kind() and to_json() only.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      The written forms change in three places. `SessionError::to_json` now writes `{"error":{"kind","message","remedy"}}`, where it wrote `{"error":{"kind","message"}}`. A changed node is written with `differs` after `bounds`: a list of closed words, the node-key names, read from `NODE_FIELDS` by index (packages/escher-driver/src/json.rs:109-128). An added node is written without `differs`. `json::list` joins `string` and `Object` as `pub(crate)` and is read by `describe`.
    sidecar: >-
      Driver session: session error JSON gains remedy; a changed node's JSON gains differs; json::list pub(crate).
    rationale: >-
      The report's Schema / config bullets say a session error's written form gains `remedy` and a changed node's gains `differs`. Symbols / APIs (`json.rs`) says `json::list` is now pub(crate). Counts `errform` marks `architecture.md:133` as changed.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      `SessionCommand` is exactly `Start { app }` · `Status` · `Stop` · `Help`, with `Help`'s spec being `HELP`. `validate_session` admits the session-level and tool-level rows and returns `Help` for `help`; an argument to `help` is refused `malformed` with `Fault::Unnamed`. `validate` is unchanged and refuses `help` as `unknown-verb`, so `Session::run` runs no `help`.
    sidecar: >-
      Driver session: SessionCommand::Help; validate_session admits the Tool level.
    rationale: >-
      The report's Symbols / APIs bullet (`command.rs`). The contract says validate_session covers "the three session-level rows" and lists `SessionCommand` with three variants.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      The command line's reader gains a `help` ending (packages/escher-driver/src/cli.rs:159-164). `help` with no argument and at most one `--session <dir>`, which is not read, writes `describe()`'s line on stdout with status `0` and reaches no session. `help` with an argument is refused `malformed`, status `1`. `help --session` with no directory, or given twice, is usage, status `2`. The four status constants read `EXIT_STATUSES`. The usage line lists `help` after `stop`.
    sidecar: >-
      Driver session: command_line's help ending; statuses read from EXIT_STATUSES.
    rationale: >-
      The report's Symbols / APIs bullet (`cli.rs`) and Harness / gate surface: the `escher-session` binary gains the verb `help` and its argv grammar gains the `help` ending.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      `serve_mcp` speaks four handshake revisions (`PROTOCOL_VERSIONS`): `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`. `initialize` answers the asked revision when it is spoken, else `LATEST_VERSION` = `2025-11-25`; this replaces "one of the three, else `2025-06-18`". The current revision `2026-07-28` is answered `2025-11-25` and is not taken up, a residual on the founder's fork 4A. `tools/list` answers ten tools with `help` last. Each tool's `description` is the verb's help, then `. Returns: `, then each result field as `{name} ({kind}): {help}` joined by `; ` (packages/escher-driver/src/mcp.rs:344-353). The server answers a `help` tool call itself, whether or not a session is held. Its text block is `describe()`'s line and its ending `accepted`. While a session is held the call counts in that session's `served`. Its `tool_call` span reads `tool` = `help` and `ending` = `accepted`, with no `command` child. The server still answers four methods, and `server/discover` still gets `-32601`.
    sidecar: >-
      Driver session MCP: four revisions, 2025-11-25 by default; ten tools with result fields in their descriptions; the help tool answered by the server.
    rationale: >-
      The report's Symbols / APIs bullet (`mcp.rs`), and Schema / config, which records the `initialize` default moving 2025-06-18 → 2025-11-25 and `tools/list` going from 9 to 10 tools. Counts `rev0618` (×3) and `verbs9` mark `architecture.md:133` as changed. The founder's answer 4A is in the report's Cross-project bullet (inputs#I3, inputs#I5).
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      The `command` span keeps its eight fields and `tool_call` keeps its two. `help` is a new word in `command`'s `verb` set, recorded when a `help` handed to `Session::run` is refused `unknown-verb`, and in `tool_call`'s `tool` set. The sentence "a session-level verb handed to run is refused `unknown-verb` with its table word recorded" becomes "a session-level or tool-level verb".
    sidecar: >-
      Driver session spans: `help` joins the closed sets of command.verb and tool_call.tool; no field added.
    rationale: >-
      The report's Schema / config bullet: no span field added; `help` is a new word of `tool_call`'s `tool` and of `command`'s `verb` closed sets.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: >-
      Add `pub enum Reading`, eleven variants `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Bounds`, with `Reading::ALL` (packages/dioxus-native-dom/src/snapshot_diff.rs:32-74). It is re-exported from the crate (packages/dioxus-native-dom/src/lib.rs:36), taking the crate's `pub use` names from 15 to 16, and through dioxus-native. `DiffNode` gains `differs: Vec<Reading>` (packages/dioxus-native-dom/src/snapshot_diff.rs:124-126). On a changed node it lists the readings that differ from before the step, in node-key order, filled at the one site that decides `changed` (`differing`, packages/dioxus-native-dom/src/snapshot_diff.rs:174-209). A node that only moved reads `[Bounds]`, and an added node reads it empty.
    sidecar: >-
      Dioxus DOM bridge: Reading enum and DiffNode::differs registered.
    rationale: >-
      The report's Symbols / APIs bullet (`dioxus-native-dom`) adds a public enum and a public field. The contract states `DiffNode`'s shape without `differs` and does not name `Reading`.
    basis: packages/dioxus-native-dom/src/snapshot_diff.rs:124-126
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: >-
      In the sentence on escher-driver's `Outcome::to_json`, which says it writes `added` and `changed` as nodes keyed by `NODE_FIELDS` and `removed` as ids, add that a changed node also carries `differs` after `bounds`, as closed node-key words.
    sidecar: >-
      Dioxus DOM bridge: the diff's JSON restatement gains differs.
    rationale: >-
      This is a second statement of the diff's written form that the primary on the Driver session `to_json` changes. Per the report's Schema / config bullet, a changed node's written form gains `differs`.
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → dioxus-native-dom
    change: >-
      The module snapshot_diff re-exports `SnapshotDiff`, `DiffNode` and `Reading`, where it named the first two only.
    sidecar: >-
      Existing Scopes dioxus-native-dom: snapshot_diff re-exports Reading.
    rationale: >-
      The report's Symbols / APIs bullet: `Reading` is re-exported from `lib.rs:36`, dioxus-native-dom's `pub use` names going from 15 to 16.
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → escher-driver
    change: >-
      Thirteen private modules, where it said twelve: add describe (`describe`, the self-description written from the tables). In schema, say the verb table `VERBS` comes in its three levels with the `help` row, the node-key specs carry their help, and the exit-status table `EXIT_STATUSES` is added. In error, add `SessionError` with its crate-private table of kinds, meanings and remedies. In command, add `validate_session`'s `Help`. cli gains the `help` ending, and mcp serves the `help` tool and four handshake revisions.
    sidecar: >-
      Existing Scopes escher-driver: thirteen modules (describe), three levels, exit-status and session-error tables.
    rationale: >-
      The report's Crates / modules bullet says escher-driver gains the module `describe`, counted in lib.rs as twelve modules going to thirteen. Counts `levels2` marks `architecture.md:257` as changed. `modules12` did not fire on the wording "twelve private modules", so this detector's own read covers that site.
    basis: .andromeda/architecture.md:257
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: >-
      Add the test target `stand_mcp_masked`: a password typed through an MCP `type` reads `MASKED_VALUE` and never the typed text, role `PasswordInput` with a non-empty name, in both layout modes (tests/blitz-tests/tests/stand_mcp_masked.rs:80-156). It declares no `mod session_common;`. `stand_mcp_tools` gains two checks: every cause a tool call reaches is served with its remedy, `covered` on an overlay fixture (tests/blitz-tests/tests/stand_mcp_tools.rs:417-522); and `help` is answered by the server and parents no `command` span (tests/blitz-tests/tests/stand_mcp_tools.rs:524-576). `stand_diff` checks `differs` on every step, and the Flight Booker's Book names eight nodes that only moved, each `differs` = `[bounds]` (tests/blitz-tests/tests/stand_diff.rs:489-527).
    sidecar: >-
      Existing Scopes blitz-tests: stand_mcp_masked target; stand_mcp_tools' remedy and help checks; stand_diff's differs.
    rationale: >-
      The report's Crates / modules bullet says blitz-tests gains the test target `stand_mcp_masked`, 34 files under `run stand`. Files and New text, by line give the new checks. Counts `mcptools` marks `architecture.md:263` (×2) as the row that names its files.
    basis: .andromeda/architecture.md:263
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: >-
      `tests/session_common/mod.rs` is read through `mod session_common;` by four of the session checks, `stand_settle`, the twelve `stand_act_*` and, since 2026-10-10-mcp-surface, `stand_mcp_tools`: eighteen readers, where it said seventeen.
    sidecar: >-
      Existing Scopes blitz-tests: session_common readers seventeen → eighteen (stand_mcp_tools), measured.
    rationale: >-
      Under Spec claims disproved by measurement, the report says "seventeen readers" at `architecture.md:263` is wrong. It measured 18 files declaring `mod session_common;`, `stand_mcp_tools` the eighteenth (CARRY 8). Counts `readers17` marks `architecture.md:263` as changed.
    basis: .andromeda/architecture.md:263
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Tests
    change: >-
      Add `stand_mcp_tools` to the checks that hold a `Session` and keep what they share in `tests/blitz-tests/tests/session_common/mod.rs`. The list now reads: the session checks, `stand_settle`, the twelve `stand_act_*` and `stand_mcp_tools`.
    sidecar: >-
      Conventions Tests: session_common's sharers include stand_mcp_tools.
    rationale: >-
      This is the same claim as the readers count, restated without its number. The report's Spec claims disproved bullet names `stand_mcp_tools` as the eighteenth reader.
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → CLIs
    change: >-
      Say `escher-session` takes the ten verbs of `VERBS`, where it said nine. The tenth is `help`, which needs no `--session` and admits at most one, unread. It answers the self-description line with status `0` and reaches no session. The four exit statuses keep their values and streams, now read from `EXIT_STATUSES` and served by `help`. `usage` means a `--session` that is absent where the verb needs one, or is given twice.
    sidecar: >-
      CLIs: escher-session's verbs nine → ten (help); exit statuses read from EXIT_STATUSES.
    rationale: >-
      The report's Symbols / APIs bullet (`cli.rs`), Deviations (the `usage` meaning), and Counts `verbs9` and `exitstat`. These name architecture's verb-count and exit-status sites, and where a row says where the meanings live it is a detector's read.
    basis: .andromeda/architecture.md:138
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Project Intent → Front page
    change: >-
      The README now says an MCP client gets ten driver commands as tools, and says how to ask for `help` on both surfaces (README.md:48). The help the driver serves about itself is built, so only the screenshot is still to come.
    sidecar: >-
      Front page: ten driver commands, help built, only the screenshot still to come.
    rationale: >-
      Deviations says `README.md:48` reads "ten driver commands" and says how to ask for `help` on both surfaces. Counts `frontpage` marks `architecture.md:210` (×2) as changed.
    basis: .andromeda/architecture.md:210
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: §Occupied Resources → Network ports and listeners
    change: >-
      On the `escher-session mcp` role, register: the `help` tool, which the server answers itself and which runs no session; the fourth handshake revision, `2025-11-25`, now the default; and what a result carries out. A result now also carries the tool's self-description and a session error's remedy, both the program's own fixed text held as `'static` data of the one table. These ride inside the ratified carrier on the founder's fork 1A («Внутри, с правилом», inputs#I3). A changed node's `differs` is closed node-key words, which is no widening under the 2026-10-10 rule. Still no listener, socket or port, and the session socket is unchanged.
    sidecar: >-
      Occupied Resources: the MCP role's help tool, fourth revision, and the fixed self-description and remedy text on the ratified carrier.
    rationale: >-
      The report's Symbols / APIs bullet (`mcp.rs`) and Schema / config add a tool and a revision to the stdio surface this row registers. Expected amendments says the "Boundary widening" pattern was answered by founder 1A (inputs#I3) and that the answer makes the wrap halt once.
  - detector: D-arch-decisions
    severity: escalate
    section: §Established Decisions → [Driver session]
    change: >-
      Record the founder's fork 1A («Внутри, с правилом», inputs#I3). Besides what a call and its answer already carry, the command line's stdout and the MCP role's stdio now carry the program's own fixed text, held as `'static` data of the one table: the `help` self-description, a session error's remedy, and each tool's description with its result fields. That text rides the ratified carriers. Text from a page, a call or a screen, a new carrier or a new exit still widens. A changed node's `differs` crosses as a closed set under the 2026-10-10 rule. No dependency was added, and `serde_json` still only reads.
    sidecar: >-
      [Driver session]: the founder's fork 1A recorded — fixed program text rides a ratified carrier; text from outside still widens.
    rationale: >-
      The chunk sends a new class of content, the self-description and remedies, over the ratified crossings. The decision's statement of what crosses does not include it. Cross-project (inputs#I3, and I5's rows: a playbook rule beside the closed-field rule, from founder fork 1A) and Expected amendments ("Boundary widening", answered 1A) give the ruling. Dependencies: none, and serde_json stays a reader.
note: >-
  Line numbers come only from the report: architecture.md:133, :138, :210, :257 and :263. The proposals for the Dioxus DOM bridge, the dioxus-native-dom scope row, Conventions → Tests, the listeners row and [Driver session] name a section and give no line. The Stack has no drift: the report's Dependencies bullet is "none". The D-arch-decisions proposal and the listeners row carry the boundary-widening halt the report forecasts at Validate. The playbook rule and the two residuals.md entries (forks 1A, 3A, 4A) are outside architecture and are not proposed here.
