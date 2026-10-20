proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy
    change: >-
      In the escher-driver bullet, schema_tests.rs (`schema::tests`) pins the verb table as twelve verbs in a stated
      order and three levels — the eight instance-level verbs, `screenshot` after `scroll` since
      2026-10-12-headless-screenshot and `lint` after `screenshot` since 2026-10-19-visual-lint, then `start`, `status`,
      `stop`, then `help` (eleven until 2026-10-19-visual-lint, which renamed the test
      `the_twelve_verbs_are_named_in_order_each_at_its_level`); replaces «eleven verbs in a stated order» and «the seven
      instance-level verbs».
    sidecar: >-
      test-plan §4 escher-driver: schema_tests pins twelve verbs, eight instance-level, `lint` the eighth (2026-10-19-visual-lint gated).
    rationale: >-
      The report's Symbols bullet: `schema::LINT` placed in `VERBS` after `SCREENSHOT`, «`VERBS` now holds twelve verbs in
      three levels: eight instance, three session, one tool» (schema.rs:458-461); Counts bullet `verbs12` · `instance8`
      list test-plan.md:146 (@c2480, @c2642) stale; `renamed` names
      `schema::tests::the_eleven_verbs_are_named_in_order_each_at_its_level` → `…the_twelve_verbs…`.
    basis: .andromeda/test-plan.md:146
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the escher-driver bullet's schema_tests.rs clause, the verbs are named in order, each at its level — eight
      instance-level (`screenshot` the seventh, after `scroll`, since 2026-10-12-headless-screenshot; `lint` the eighth,
      after `screenshot`, since 2026-10-19-visual-lint), then `start` · `status` · `stop`, then `help` at the tool level,
      twelve (eleven until 2026-10-19-visual-lint); replaces «seven instance-level … eleven».
    sidecar: >-
      test-plan §1 escher-driver: the schema's verbs eight instance-level and twelve in all, `lint` the eighth.
    rationale: >-
      Same claim as the §4 schema_tests site; the report's Counts bullet lists test-plan.md:22 (@c3454/@c3500) stale under
      `verbs12` and `instance8`.
    basis: .andromeda/test-plan.md:22
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the escher-driver bullet, describe.rs 2's count row reads `[12, 13, 12, 3, 8, 4, 15]` since
      2026-10-19-visual-lint (`[11, 13, 12, 3, 8, 4, 15]` since 2026-10-19-session-recording until then).
    sidecar: >-
      test-plan §1 escher-driver: describe.rs's count row `[12, 13, 12, 3, 8, 4, 15]` (help's `verbs` 11 → 12).
    rationale: >-
      The report's Schema / config bullet: «`help`'s `verbs` 11 → 12. Basis: `schema_tests.rs` and `describe.rs`'s `counts`
      (`[12, 13, 12, 3, 8, 4, 15]`)»; describe.rs:145 is a New-text row. A site the report's verbs12 row list for :22 does
      not name (found by this detector's sweep, «its count row reads `[11, 13, …]`»).
    basis: .andromeda/test-plan.md:22
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy
    change: >-
      In the Command line bullet, each of the eight instance-level verbs is accepted and answers exactly its fields
      (seven until 2026-10-19-visual-lint) — and since 2026-10-19-visual-lint `lint_answers_one_line_of_findings`: a
      hosted `lint` answers exit 0 and one line `{"findings":[]}` (examples/seven_guis/tests/cli_commands.rs:364-385).
    sidecar: >-
      test-plan §6 Command line: eight instance-level verbs accepted; cli_commands' lint test.
    rationale: >-
      The report's Counts bullet `instance8` lists test-plan.md:204 @c671 «each of the seven instance-level verbs is
      accepted» stale; Outcome names `cli_commands::lint_answers_one_line_of_findings` (cli_commands.rs:364-385).
    basis: .andromeda/test-plan.md:204
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-binary: the whole command line is twelve verbs — the eight instance-level ones with their arguments as flags
      (`screenshot` the seventh since 2026-10-12-headless-screenshot; `lint` the eighth since 2026-10-19-visual-lint, no
      argument, its accepted answer one line `{"findings":[…]}`), then `start`, `status`, `stop`, then `help`; replaces
      «eleven verbs — the seven instance-level ones».
    sidecar: >-
      key session-lifecycle session-binary: twelve verbs, eight instance-level, `lint` the eighth.
    rationale: >-
      The report's Symbols bullet (the command line `escher-session lint --session <dir>`, the usage line `… | screenshot
      [--id <id>] | lint | start …`) and Counts bullets `verbs12` · `instance8` listing the key file's :9 (@c119, @c138) stale.
    basis: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-start: `run` records each answered call of the eight instance verbs (seven until 2026-10-19-visual-lint,
      `lint` kept like every instance verb, its answer kept whole).
    sidecar: >-
      key session-lifecycle session-start: a recording keeps the eight instance verbs' calls, `lint` among them.
    rationale: >-
      The report's Symbols bullet «Recording: a `lint` step is kept like every instance verb, its answer kept whole.
      `recording.rs` is unchanged, since it reads `Level::Instance` from `VERBS`»; Counts bullets `verbs12` · `instance8`
      list the key file's :5 (@c4298/@c4313) stale.
    basis: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the escher-driver bullet's mcp_tests.rs clause, the listing is the twelve verbs of the table — eleven until
      2026-10-19-visual-lint, ten until 2026-10-12-headless-screenshot, nine until 2026-10-12-self-description (the test
      renamed `the_listing_is_the_twelve_verbs_of_the_table`).
    sidecar: >-
      test-plan §1 escher-driver: the MCP listing pinned at twelve verbs.
    rationale: >-
      The report's `renamed` bullet (`mcp::tests::the_listing_is_the_eleven_verbs_of_the_table` → `…the_twelve_verbs…`)
      and Counts bullet `verbs12` listing test-plan.md:22 @c7062/7073 stale.
    basis: .andromeda/test-plan.md:22
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the apps bullet's `tests/mcp_server.rs` clause, `tools/list` offers the verbs in order, twelve since
      2026-10-19-visual-lint (eleven since 2026-10-12-headless-screenshot until then).
    sidecar: >-
      test-plan §1 apps: mcp_server's tools/list twelve verbs.
    rationale: >-
      The report's Counts bullet `verbs12` lists test-plan.md:10 (@c5000) stale; Symbols: MCP `lint` a tool reached
      through the table.
    basis: .andromeda/test-plan.md:10
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy
    change: >-
      In the MCP bullet, `tools/list` offers the verbs in order — twelve since 2026-10-19-visual-lint, eleven since
      2026-10-12-headless-screenshot until then.
    sidecar: >-
      test-plan §6 MCP: tools/list twelve verbs.
    rationale: >-
      The report's Counts bullet `verbs12` lists test-plan.md:205 (@c330) stale.
    basis: .andromeda/test-plan.md:205
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      In the cold-agent pipe's «The server and the check's judge» bullet, the agent talks to `escher-session mcp` (the
      twelve verbs, `lint` after `screenshot` since 2026-10-19-visual-lint; eleven until then).
    sidecar: >-
      test-plan §3 cold-agent pipe: the server's twelve verbs.
    rationale: >-
      The report's Harness / gate surface bullet: the cold-agent pipe's `VERBS` gains `lint` (scripts/cold-agent.sh:48);
      Counts bullets `verbs12` · `coldagent` list test-plan.md:126.
    basis: .andromeda/test-plan.md:126
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      In the same bullet, `valid_step` admits only `tool` ∈ the twelve verbs (eleven until 2026-10-19-visual-lint).
    sidecar: >-
      test-plan §3 cold-agent pipe: valid_step admits the twelve verbs.
    rationale: >-
      The report's Schema / config bullet: «The cold-agent pipe's verb names are twelve, `lint` after `screenshot`, at
      scripts/cold-agent.sh:48»; test-plan.md:126 (the judge clause) restates the count.
    basis: .andromeda/test-plan.md:126
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      In the Verdict bullet, `isolated` = the init tools exactly the twelve `mcp__escher__{verb}` tools (eleven until
      2026-10-19-visual-lint).
    sidecar: >-
      test-plan §3 cold-agent verdict: `isolated` requires the twelve tools.
    rationale: >-
      The report's Harness / gate surface bullet: «The pipe's `isolated` reading now requires twelve tools»; Counts bullets
      `verbs12` · `coldagent` list test-plan.md:127 stale.
    basis: .andromeda/test-plan.md:127
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      In the cold-agent Proof bullet, keep the 2026-10-15 live run's reading as dated — `isolated` (the eleven
      `mcp__escher__*` tools of that day, one server `escher`) — and add that the pipe requires twelve since
      2026-10-19-visual-lint, its contract tests' fake answering `lint`, the live run not re-fired.
    sidecar: >-
      test-plan §3 cold-agent proof: the live run's eleven tools dated; twelve required since 2026-10-19-visual-lint.
    rationale: >-
      The report's Counts bullets `verbs12` · `coldagent` list test-plan.md:130 stale; the site is a measured live run of
      2026-10-15, so a bare eleven → twelve would misstate what was measured — the correction dates it and states the
      new requirement (Harness / gate surface bullet: `isolated` now requires twelve tools).
    basis: .andromeda/test-plan.md:130
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy
    change: >-
      In the Cold-agent pipe bullet, `FAKE` speaks the twelve tools (eleven until 2026-10-19-visual-lint), answering
      `lint` with `{"findings":[]}` (.github/scripts/test_cold_agent.py:100-101); the test count 33 unmoved.
    sidecar: >-
      test-plan §4 test_cold_agent: the fake speaks twelve tools, `lint` answered with no findings.
    rationale: >-
      The report's Schema / config bullet: «The contract test's fake answers `lint` with `{"findings":[]}`
      (test_cold_agent.py:100-101), and its comment reads "the twelve tools" (:60)»; Counts bullet `verbs12` lists
      test-plan.md:151 stale.
    basis: .andromeda/test-plan.md:151
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy
    change: >-
      In the Command line bullet, `cli_commands` (13, 12 until 2026-10-19-visual-lint, 11 until
      2026-10-14-quality-gates, 10 until 2026-10-12-headless-screenshot, 8 until 2026-10-12-self-description).
    sidecar: >-
      test-plan §6: cli_commands 12 → 13 (the lint test).
    rationale: >-
      The report's Counts bullet `clicmds` (12 → 13) lists test-plan.md:204 @c193 stale; the added test is
      `cli_commands::lint_answers_one_line_of_findings` (cli_commands.rs:364-385).
    basis: .andromeda/test-plan.md:204
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the apps bullet, `tests/cli_commands.rs` (13 tests, 12 until 2026-10-19-visual-lint, 11 until
      2026-10-14-quality-gates, …).
    sidecar: >-
      test-plan §1: cli_commands 12 → 13.
    rationale: >-
      The report's Counts bullet `clicmds` lists test-plan.md:10 @c3102 «(12 tests» stale.
    basis: .andromeda/test-plan.md:10
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy
    change: >-
      In the escher-driver bullet, seven_guis' `cli_commands` (13 since 2026-10-19-visual-lint, 12 since
      2026-10-14-quality-gates, …).
    sidecar: >-
      test-plan §4: cli_commands 12 → 13.
    rationale: >-
      The report's Counts bullet `clicmds` lists test-plan.md:146 @c9628 «(12 since 2026-10-14-quality-gates» stale.
    basis: .andromeda/test-plan.md:146
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-proof: seven_guis `cli_commands` 13 (12 until 2026-10-19-visual-lint, 11 until 2026-10-14-quality-gates, …).
    sidecar: >-
      key session-lifecycle session-proof: cli_commands 12 → 13.
    rationale: >-
      The report's Counts bullet `clicmds` lists the key file's :11 @c2511 stale.
    basis: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy
    change: >-
      In the MCP bullet, `mcp_server` (6, 5 until 2026-10-19-visual-lint, …), since 2026-10-19-visual-lint a `lint` tool
      call answering its line of findings (`a_lint_tool_call_answers_its_line_of_findings`,
      examples/seven_guis/tests/mcp_server.rs:340-358).
    sidecar: >-
      test-plan §6 MCP: mcp_server 5 → 6, a lint tool call.
    rationale: >-
      The report's Counts bullet `mcpsrv` (5 → 6) lists test-plan.md:205 @c158 stale; Deviations names the added test
      `mcp_server::a_lint_tool_call_answers_its_line_of_findings` (mcp_server.rs:340-358).
    basis: .andromeda/test-plan.md:205
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the apps bullet, `tests/mcp_server.rs` (6 tests, 5 until 2026-10-19-visual-lint, 4 until
      2026-10-12-headless-screenshot …), its flows including a `lint` tool call.
    sidecar: >-
      test-plan §1: mcp_server 5 → 6.
    rationale: >-
      The report's Counts bullet `mcpsrv` lists test-plan.md:10 @c4782 «`tests/mcp_server.rs` (5 te…» stale.
    basis: .andromeda/test-plan.md:10
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-proof: seven_guis `mcp_server` 6 (5 until 2026-10-19-visual-lint, 4 until 2026-10-12-headless-screenshot, 2
      until 2026-10-12-self-description).
    sidecar: >-
      key session-lifecycle session-proof: mcp_server 5 → 6.
    rationale: >-
      The report's Counts bullet `mcpsrv` lists the key file's :11 @c2717 stale.
    basis: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy
    change: >-
      In the MCP bullet, in process in both layout modes, `stand_mcp_tools` (6, 5 until 2026-10-19-visual-lint, …), since
      2026-10-19-visual-lint a `lint` tool call answering the reader's findings and parenting its `command` span
      (tests/blitz-tests/tests/stand_mcp_tools.rs:706-766).
    sidecar: >-
      test-plan §6 MCP: stand_mcp_tools 5 → 6, a lint tool call.
    rationale: >-
      The report's Counts bullet `mcpsrv` lists test-plan.md:205 @c1037 stale; Deviations names the added test
      `stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command` (stand_mcp_tools.rs:706-766).
    basis: .andromeda/test-plan.md:205
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the tests/blitz-tests bullet, `stand_mcp_tools` 6 (5 until 2026-10-19-visual-lint, 4 until
      2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description), its sixth a `lint` tool call answering the
      reader's findings and parenting its `command` span.
    sidecar: >-
      test-plan §1 blitz-tests: stand_mcp_tools 5 → 6.
    rationale: >-
      The report's Counts bullet `mcpsrv` lists test-plan.md:27 @c38114 «`stand_mcp_tools` 5» stale.
    basis: .andromeda/test-plan.md:27
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-proof: `stand_mcp_tools` 6 (5 until 2026-10-19-visual-lint, 4 until 2026-10-12-headless-screenshot, 2 until
      2026-10-12-self-description).
    sidecar: >-
      key session-lifecycle session-proof: stand_mcp_tools 5 → 6.
    rationale: >-
      The report's Counts bullet `mcpsrv` lists the key file's :11 @c2928 stale.
    basis: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the tests/blitz-tests bullet, `stand_lint` 6 (5 until 2026-10-19-visual-lint's gated part), its sixth
      `the_lint_verb_answers_what_the_reader_reads`: the `lint` verb through `Session::run` answers, in both layout
      modes, the findings `Session::lint` reads — 7 on the planted page, 0 on each lean task at boot
      (tests/blitz-tests/tests/stand_lint.rs:308-330).
    sidecar: >-
      test-plan §1 blitz-tests: stand_lint 5 → 6, the lint verb through Session::run.
    rationale: >-
      The report's Counts bullet `standlint` (5 → 6) lists test-plan.md:27 @c50653 stale; Outcome names
      `stand_lint::the_lint_verb_answers_what_the_reader_reads` (stand_lint.rs:308-330), 7 findings on the page, 0 on each
      task, in both modes.
    basis: .andromeda/test-plan.md:27
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      In the Crate-local test helpers' blitz-tests (session checks) bullet, add the builder `lint()` and the reader `linted(session, call)` (a
      lint's findings, `None` for a refused call or any other outcome; tests/blitz-tests/tests/session_common/mod.rs:115-127)
      since 2026-10-19-visual-lint, `act`'s arm covering `Linted`; and `stand_lint` reads `lint` and `linted` too, the
      reader count 28 unmoved.
    sidecar: >-
      test-plan session_common: `lint()` and `linted()` added; stand_lint reads them.
    rationale: >-
      The report's Symbols bullet «Test helpers»: `lint()` (:115-118), `linted()` (:120-127), the `act` reader's arm covers
      `Linted` (:196-199); Counts bullet `standlint` lists test-plan.md:103 @c3133 stale, «The reader count, 28, is
      unmoved».
    basis: .andromeda/test-plan.md:103
    dependent-of: D-tests-coverage
  - detector: D-retired
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the tests/blitz-tests bullet's `stand_lint` clause, replace «on the in-process `Session::lint` (in process only)»
      with «on `Session::lint` in process and, since the gated part, on the `lint` verb through `Session::run` — the
      twelfth verb on every carrier, recorded, on the founder's F1 word (option 1)».
    sidecar: >-
      test-plan §1: the lint's «in process only» retired — `lint` is a verb on every carrier (founder's F1, 2026-10-20).
    rationale: >-
      The report's Retired bullet: the in-process-only carriage is superseded by the founder's own word, option 1
      (inputs#I7), and every «in process only … no verb … carries it» statement is now false; Counts bullet `lintverb`
      lists test-plan.md:27 @c50762/50778 stale.
    basis: .andromeda/test-plan.md:27
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the tests/blitz-tests bullet's `stand_act_spans` clause, 34 calls per mode (32 before 2026-10-19-visual-lint, 31
      before that), eight instance verbs read back where it read seven; since 2026-10-19-visual-lint it also reads a
      lint's line and a refused no-font lint's line, `cause=fonts-unavailable`.
    sidecar: >-
      test-plan §1: stand_act_spans 32 → 34 calls per mode, a lint's span lines read.
    rationale: >-
      The report's Counts bullet `spancalls`: «`stand_act_spans`' calls per mode, 32 → 34, with eight verbs read back where
      it read seven», test-plan.md:27 @c31178 stale, and «the check also reads a lint's line and a refused no-font lint's
      line is new fact».
    basis: .andromeda/test-plan.md:27
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      In the escher-driver bullet's json.rs 7 clause, add: since 2026-10-19-visual-lint a lint's outcome written as
      `{"findings":[…]}`, each finding keyed `id` · `defect` · `bounds` then its reason's keys, every reason pinned
      (`every_reason`), a ratio that is not finite written `null`; the count 7 unmoved.
    sidecar: >-
      test-plan §1 escher-driver: json.rs's tests pin a finding's written form.
    rationale: >-
      The report's Symbols bullet «The finding's written form» (json.rs:255-309, `Outcome::to_json` at json.rs:360-364)
      and its New-text rows json.rs 625-697 (`every_reason`) and 765-788 — test code, no new test function (the tally
      +4 is the four named new tests) — and Coverage «tests unit (json, …)»; the unit-tier coverage of the new written
      form is unrecorded at test-plan.md:22.
    basis: .andromeda/test-plan.md:22
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy
    change: >-
      In the Session host ↔ what it writes bullet, since 2026-10-19-visual-lint `host_log` and `mcp_log` each also drive
      one `lint` and still read 0 needles on stderr at `trace`; their counts (2 each) unmoved.
    sidecar: >-
      test-plan §5: host_log and mcp_log drive a lint, 0 needles.
    rationale: >-
      The report's Counts bullet `logflows` («`host_log` and `mcp_log` each drive one `lint`»; test-plan.md:187 @c2077 «may
      gain the lint; that is new fact») and Coverage «`host_log` and `mcp_log` read 0 needles with a lint on their flows».
    basis: .andromeda/test-plan.md:187
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      In the agent-run Proof bullet, add a re-count at 2026-10-19-visual-lint's gated part, `scripts/agent-run.sh`
      unchanged: `run stand`'s `run.end` reads passed 192 · failed 0 · ignored 7 over 47 files — +2 passed (`stand_lint`'s
      and `stand_mcp_tools`' lint tests) — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md.
    sidecar: >-
      test-plan §3 agent-run proof: run stand 190 → 192 · 0 · 7.
    rationale: >-
      The report's Counts bullet `runstand` «190 · 0 · 7 → 192 · 0 · 7», test-plan.md:119 @c10865 stale; Outcome
      `run stand` 192 · 0 · 7.
    basis: .andromeda/test-plan.md:119
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      In the Test-data bootstrap bullet, `run stand` 192 · 0 · 7, as measured at
      escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md (190 · 0 · 7 at 2026-10-20-select-by-id).
    sidecar: >-
      test-plan §3 bootstrap: run stand 192 · 0 · 7.
    rationale: >-
      The report's Counts bullet `runstand` lists test-plan.md:132 stale.
    basis: .andromeda/test-plan.md:132
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration
    change: >-
      In the Local baseline's `cargo test --workspace` row, add a re-count at 2026-10-19-visual-lint's gated part: 204
      result lines, 1087 passed · 0 failed · 12 ignored, from 204 · 1083 — +4 passed (`stand_lint` 1, `stand_mcp_tools`
      1, `cli_commands` 1, `mcp_server` 1), no result line added.
    sidecar: >-
      test-plan §9 local tally: 204 · 1087 · 0 · 12.
    rationale: >-
      The report's Counts bullet `tally` «204 · 1083 · 0 · 12 → 204 · 1087 · 0 · 12», test-plan.md:340 @c19517 stale;
      Outcome «the tally 204 · 1087 · 0 · 12»; the four new tests are named in Outcome and Deviations.
    basis: .andromeda/test-plan.md:340
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration
    change: >-
      In the Per-OS test tallies bullet, add a re-read at 2026-10-19-visual-lint's gated part on CI#39387656399, the
      pre-CI commit `ea3153f34` (green 17/17): linux 204 · 1087 · 0 · 12; macos and coverage 185 · 1084 · 0 · 12; windows
      185 · 1041 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 165 · 672 · 0 · 21 — as measured at
      escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md.
    sidecar: >-
      test-plan §9 per-OS tallies re-read on CI#39387656399.
    rationale: >-
      The report's Counts bullets `citallies` and `tally` (test-plan.md:334 @c4895–5106 and @c4968 stale, each move
      stated) and Cross-project bullet CI#39387656399 green 17/17.
    basis: .andromeda/test-plan.md:334
  - detector: D-tests-coverage
    severity: warning
    section: §10 Quality Gates & Coverage Targets
    change: >-
      In the Flakiness budget bullet, the stand repeat's latest reading is CI#39387656399, 165 · 672 · 0 · 21, +0 · +12
      (four new tests × three rounds; the 47 `stand_*.rs` files unmoved), before it CI#39371468367's 165 · 660 · 0 · 21.
    sidecar: >-
      test-plan §10 stand repeat 165 · 672 · 0 · 21.
    rationale: >-
      The report's Counts bullet `citallies` lists test-plan.md:353 «165 · 660 → 165 · 672» stale (basis evidence/ci.md
      entry 19), and Harness bullet «Its +12 is four new tests × three rounds».
    basis: .andromeda/test-plan.md:353
    dependent-of: D-tests-coverage
note: >-
  D-tests-framework and D-tests-obs-harness read no drift — no runner or framework changed, and agent-run's five commands,
  status shape and log format are unchanged (the cold-agent pipe's verb list moved, not the harness). One site beyond the
  report's row list: test-plan.md:22's describe.rs count row `[11, 13, 12, 3, 8, 4, 15]` (proposal 3). Line citations
  past inserted lines (host_log.rs, mcp_log.rs, session_common/mod.rs, stand_act_spans.rs:542-554) are left to the
  citation sweep.
