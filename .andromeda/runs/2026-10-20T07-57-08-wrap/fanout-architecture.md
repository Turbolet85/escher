proposals:
  - detector: D-arch-resources
    severity: escalate
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The verb-table sentence reads: `VERBS` lists twelve verbs in three levels — first the eight asked of the held instance, in order `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` and, since 2026-10-19-visual-lint (its gated part), `lint` (no argument, one named refused `malformed` with `Fault::Unnamed`; result `findings`, `FieldKind::Entries`, always present; its help states the six defect words, each reason''s keys and its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines) (packages/escher-driver/src/schema.rs:310-335; packages/escher-driver/src/schema.rs:458-461), a widening of the validated verb set on the founder''s F1 «Новая команда lint (Рекомендую)» (inputs#I7).'
    sidecar: 'arch §Driver session: VERBS twelve, instance verbs eight, `lint` the eighth instance verb (2026-10-19-visual-lint gated).'
    rationale: 'Report Changes → Symbols / APIs: «`lint` is the eighth instance verb and the twelfth verb», `schema::LINT` placed after `SCREENSHOT`; Counts moved `verbs12` / `instance8` list architecture.md:196 @c9111 «`VERBS` lists eleven verbs» and the seven-instance clause as stale.'
    basis: packages/escher-driver/src/schema.rs:310-335
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The `describe()` sentence reads its table counts as the report measured them: 12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 15 session errors (the line now says «11 verbs» and «9 causes»).'
    sidecar: 'arch §Driver session: `describe` counts 11 → 12 verbs (and causes read 8, as `counts` measures).'
    rationale: 'Report Changes → Schema / config: «`help`''s `verbs` 11 → 12. Basis: `schema_tests.rs` and `describe.rs`''s `counts` (`[12, 13, 12, 3, 8, 4, 15]`)»; `verbs12` lists architecture.md:196 @c14768 «11 verbs» stale. The same `counts` reads 8 causes where the line says 9 — corrected on the same measured basis.'
    basis: packages/escher-driver/src/describe.rs
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: '`Command` is exactly `Snapshot` · `Click { id }` · `Type { id, text }` · `Press { key, shift }` · `Advance { ms: u32 }` · `Scroll { id }` · `Screenshot { id: Option<String> }` · `Lint` (the last since 2026-10-19-visual-lint, validated from `("lint", [])`; packages/escher-driver/src/command.rs:175-176), eight variants.'
    sidecar: 'arch §Driver session: `Command::Lint`, the eighth variant.'
    rationale: 'Report Changes → Symbols / APIs: «`Command::Lint` (command.rs:175-176, `spec` at :190, validated `("lint", [])` at :267). The `Command` enum now has eight variants.»'
    basis: packages/escher-driver/src/command.rs:175-176
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: '`Outcome` is exactly `Screen` · `Acted` · `Advanced` · `Scrolled` · `Shot` · `Linted { findings: Vec<Finding> }` (the last since 2026-10-19-visual-lint, packages/escher-driver/src/execute.rs:75-79); the executor''s `lint` arm calls `Session::lint` (packages/escher-driver/src/execute.rs:261-263), so it dispatches nothing, runs no step and settles nothing, and is refused `fonts-unavailable` exactly as `Session::lint` is, before anything is read; `record_result` returns early for it as for `Screen` and `Shot` (packages/escher-driver/src/execute.rs:276).'
    sidecar: 'arch §Driver session: `Outcome::Linted`, a sixth variant; the `lint` arm runs `Session::lint`, no step.'
    rationale: 'Report Changes → Symbols / APIs: «`Outcome::Linted { findings: Vec<Finding> }` (execute.rs:75-79) is a sixth `Outcome` variant, and a public API change. Its arm calls `Session::lint` (execute.rs:261-263), so it runs no step and settles nothing»; the line''s `Outcome` enumeration (architecture.md:196) lists five.'
    basis: packages/escher-driver/src/execute.rs:75-79
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The written-forms sentence adds a lint''s outcome: `{"findings":[…]}` (packages/escher-driver/src/json.rs:360-364), each finding an object keyed `id` · `defect` · `bounds` (`[x, y, width, height]`, as a node''s bounds), then its reason''s keys — `other` and `area` for an overlap, `visible` for a clip, `text` then `visible` (left out when no part can be seen) for a truncation, `ratio` (its shortest decimal, `null` when not finite) and `required` for contrast, none for off-screen and zero-size (packages/escher-driver/src/json.rs:255-309) — holding stable ids, closed words and numbers only.'
    sidecar: 'arch §Driver session: the finding''s hand-written JSON form and `{"findings":[…]}`.'
    rationale: 'Report Changes → Symbols / APIs → «The finding''s written form (json.rs:255-309)» and «`Outcome::to_json` writes `{"findings":[…]}` (json.rs:360-364)»; the contract enumerates each outcome''s written form (architecture.md:196) and holds none for a lint.'
    basis: packages/escher-driver/src/json.rs:255-309
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The `command` span sentence reads: `snapshot`, an accepted `screenshot` and, since 2026-10-19-visual-lint, a `lint` record `verb` alone (a refused one `verb` and `cause`), no field added — the founder''s F2 «Без поля (Рекомендую)» (inputs#I7); the never-fielded list adds a lint''s findings and their count.'
    sidecar: 'arch §Driver session: the `lint` verb''s command span records `verb` and, refused, `cause` only (F2).'
    rationale: 'Report Changes → Symbols / APIs: «Its `command` span records `verb` and, refused, `cause` only — F2 option 1»; Schema / config: «No new … span field». The span sentence at architecture.md:196 names `snapshot` and `screenshot` only.'
    basis: packages/escher-driver/src/execute.rs:276
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: '`tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot, `lint` after `screenshot` since 2026-10-19-visual-lint), `help` last; a `lint` tool result is its JSON line through the existing default arm (packages/escher-driver/src/mcp.rs:58 the one changed line, its rustdoc).'
    sidecar: 'arch §Driver session: MCP `tools/list` twelve verbs, `lint` after `screenshot`.'
    rationale: 'Report Changes → Carriers → MCP: «a tool; its result is the JSON line through the existing default arm (mcp.rs — only the rustdoc changed, "the eight instance verbs", mcp.rs:58)»; `verbs12` lists architecture.md:196 @c44821/44832/44842 «`tools/list` answers the eleven verbs» stale.'
    basis: packages/escher-driver/src/mcp.rs:58
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The recording sentence''s «a call no record can spell (a verb outside the seven instance verbs …)» reads «the eight instance verbs», and a `lint` step is kept like every instance verb, its answer kept whole (recording.rs unchanged, reading `Level::Instance` from `VERBS`).'
    sidecar: 'arch §Driver session: a recorded check spells eight instance verbs; a `lint` step is kept.'
    rationale: 'Report `instance8`: architecture.md:200 (@c952/2181 «the seven instance verbs», the recorded check''s `tool`) stale; Changes → Carriers → Recording: «a `lint` step is kept like every instance verb, its answer kept whole. `recording.rs` is unchanged».'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: '`Check::from_bytes` reads «… closed keys, the eight instance verbs, argument names from the verb''s schema …» where it reads «the seven instance verbs».'
    sidecar: 'arch §Driver session: `Check::from_bytes` admits the eight instance verbs.'
    rationale: 'Report `instance8` lists architecture.md:200 @c2181 stale (the second of its two occurrences on that line); `lint` is an instance verb (Changes → Symbols / APIs).'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The load-wait sentence reads: the five acting verbs wait on every carrier, and `snapshot`, `screenshot` and `lint`, which run no step, wait on nothing.'
    sidecar: 'arch §Driver session: `lint`, running no step, waits on no load.'
    rationale: 'Report Changes → Symbols / APIs: the `lint` arm «runs no step and settles nothing» (execute.rs:261-263); architecture.md:203 names only `snapshot` and `screenshot` as the verbs that wait on nothing (its «five acting verbs» no-change per `verbs12`).'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → CLIs
    change: 'The `escher-session` sentence reads: the verbs are the twelve of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `screenshot [--id <id>]`, `lint`, `start <task> [--record <dir>]`, `status`, `stop` and `help`; the usage line reads `… | screenshot [--id <id>] | lint | start …`.'
    sidecar: 'arch §CLIs: `escher-session` takes the twelve verbs, `screenshot` and `lint` listed.'
    rationale: 'Report Changes → Carriers → The command line: «`escher-session lint --session <dir>`; the usage line now reads `… | screenshot [--id <id>] | lint | start …`»; `verbs12` lists architecture.md:219 (@c2256 «the verbs are the ten of `VERBS`», also missing `screenshot`, stale before this chunk) stale.'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → CLIs
    change: 'The cold-agent `run` sentence reads «only the escher server''s twelve tools» (scripts/cold-agent.sh:48, `lint` after `screenshot`).'
    sidecar: 'arch §CLIs: the cold-agent session gets the escher server''s twelve tools.'
    rationale: 'Report Changes → Harness / gate surface: «The cold-agent pipe''s `VERBS` gains `lint` (scripts/cold-agent.sh:48) … The pipe''s `isolated` reading now requires twelve tools»; `verbs12` lists architecture.md:222 @c10057 stale.'
    basis: scripts/cold-agent.sh:48
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → CLIs
    change: 'The verdict''s isolated reading reads «the twelve `mcp__escher__{verb}` tools, one server `escher` connected».'
    sidecar: 'arch §CLIs: the cold-agent `isolated` reading requires twelve tools.'
    rationale: 'Report `verbs12` lists architecture.md:222 @c10443 «the eleven `mcp__escher__{verb}` tools» stale; Harness / gate surface: «The pipe''s `isolated` reading now requires twelve tools».'
    basis: scripts/cold-agent.sh:48
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: §Occupied Resources → Network ports and listeners
    change: 'The session-socket entry adds: since 2026-10-19-visual-lint both carriers (the socket''s `call` answer and the `mcp` role''s result) and a kept check also take a lint''s findings out — stable ids, the closed defect words and numbers, no `NodeId`, page text, colour or control value — still the one listener and no port, the wire''s bounds unchanged; a boundary widening ratified by the founder (2026-10-20, F1 «Новая команда lint (Рекомендую)», by dialog in the overseer session, relayed verbatim by the overseer, inputs#I7).'
    sidecar: 'arch §Occupied Resources: the socket and the `mcp` role carry a lint''s findings, on the founder''s F1 word.'
    rationale: 'Report Expected amendments: «The findings now cross every ratified carrier — the socket, the command line, MCP and a kept check — as ids, closed words and numbers … The founder''s ratification is inputs#I7, F1 option 1»; the socket entry enumerates what crosses (ids, names, values, snapshot text, diff, a shot) and holds no finding. No new listener, port or env var (Changes → Crates: none).'
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: escalate
    section: §Established Decisions → [Driver session]
    change: 'The driver-session decision adds: since 2026-10-19-visual-lint a lint''s findings cross on the socket, the command line''s stdout, the `mcp` role''s stdio and a kept check, as the `lint` verb''s answer — ids, closed defect words and numbers — with no new carrier, exit, path argument, cause, session error, span field or dependency, a boundary widening ratified by the founder (2026-10-20, F1 «Новая команда lint (Рекомендую)» = option 1 «New verb, all carriers, recorded», and F2 «Без поля (Рекомендую)» = option 1 «No new field», by dialog in the overseer session, relayed verbatim by the overseer, inputs#I7) — superseding the delegate''s PROVISIONAL «In process only» (inputs#I2).'
    sidecar: 'arch §Established Decisions: the lint''s carriage on every carrier, recorded, on the founder''s F1/F2 word.'
    rationale: 'Report Retired: «The in-process-only carriage of the lint … is superseded by the founder''s own word, option 1 (inputs#I7)»; Decisions & corrections: «It ratifies the lint''s carriage, a boundary widening». The decision records each prior widening of what the carriers take out (calls, fixed text, the in-band shot) and none for findings. Dependencies: none, so no stack drift.'
  - detector: D-retired
    severity: escalate
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The visual-lint bullet''s head reads: «The visual lint — `Session::lint` in process and, since 2026-10-19-visual-lint''s gated part, the `lint` verb, the eighth instance verb, on every carrier (the socket''s `call`, the command line, an MCP tool, `help`''s listing) and recorded, on the founder''s F1 «Новая команда lint (Рекомендую)» (inputs#I7)», in place of «The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part)».'
    sidecar: 'arch §Driver session: the visual lint is no longer in process only; its `lint` verb is built.'
    rationale: 'Report Retired: the in-process-only carriage is superseded (inputs#I7); `lintverb` and `title` list architecture.md:202 (the whole bullet) stale.'
  - detector: D-retired
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The visual-lint bullet''s sentence «No verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, span or field carries it, and `VERBS` stays eleven» reads: «The `lint` verb carries it — `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` written form, an MCP tool and a recorded step — its `command` span recording `verb` and, refused, `cause` alone, no field added (F2 «Без поля (Рекомендую)»), and `VERBS` holds twelve».'
    sidecar: 'arch §Driver session: the lint''s no-carrier sentence retired; the verb, outcome, form and tool named.'
    rationale: 'Report Retired: «Every master statement "in process only — no verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step … carries it, `VERBS` staying eleven" is now false»; `outcome` lists architecture.md:202 @c2608 stale.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The causes sentence reads: `fonts-unavailable` from `screenshot` and `lint` among the verbs, a shot checked before anything is painted and a lint (and `Session::lint` in process) before anything is read.'
    sidecar: 'arch §Driver session: `fonts-unavailable` returned by `screenshot` and the `lint` verb.'
    rationale: 'Report `lintverb` lists architecture.md:196 @c18681 «from `screenshot` alone among the verbs … and from the in-process `Session::lint`» stale: «the `lint` verb now also returns `fonts-unavailable`»; Changes → `Outcome::Linted`: «refused `fonts-unavailable` exactly as `Session::lint` is».'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The record-of-ids sentence reads: the record is fed from the snapshot `Session::lint` reads — in process or through the `lint` verb, which takes no argument — and from every snapshot a call takes once `validate` has admitted it.'
    sidecar: 'arch §Driver session: the record of ids fed by `Session::lint`, in process or through `lint`.'
    rationale: 'Report `lintverb` lists architecture.md:196 @c33714 «the in-process `Session::lint`» stale.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: 'The re-exports sentence names «the visual lint''s `Bounds`, `Defect`, `Finding` and `Reason`» in place of «the in-process lint''s …».'
    sidecar: 'arch §Driver session: re-exports name the visual lint, no in-process qualifier.'
    rationale: 'Same retired qualifier as architecture.md:196 @c33714 (Report Retired: the in-process-only carriage superseded); the re-exports clause on architecture.md:196 restates it in other words.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Existing Scopes → escher-driver
    change: 'The crate row''s lint module reads «lint (`Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds`, the visual lint, carried since 2026-10-19-visual-lint by the `lint` verb — execute holding `Outcome::Linted`, json the finding''s written form)» in place of «the in-process visual lint».'
    sidecar: 'arch §Existing Scopes: escher-driver''s lint module no longer in process only.'
    rationale: 'Report `title` lists architecture.md:354 @c3921 stale («in process only» qualifier); Changes → Symbols / APIs name `Outcome::Linted` (execute.rs:75-79) and the finding''s written form (json.rs:255-309).'
    dependent-of: D-retired
note: 'architecture.md:169 (`text_extent`''s one non-test caller `Session::lint`) stays true as the report dispositions it. Project Intent''s README sentence («the page saying eleven since 2026-10-13-…») is history of README.md, which this chunk did not change — left; if the README should name twelve tools, that is a README edit, not an arch amendment. The second proposal also corrects «9 causes» to 8 on the `counts` basis the report gives, a pre-existing mismatch.'
