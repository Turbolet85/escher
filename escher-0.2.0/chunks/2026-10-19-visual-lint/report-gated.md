# Report — 2026-10-19-visual-lint (gated wrap)

**Chunk:** Visual lint — `Session::lint` reads overlap, clipping, truncated text, off-screen, zero size and text
contrast by stable id with a measured reason, in process, fixture- and stand-proven; its one command gated on the
founder's F1 word.
**Date:** 2026-10-20
**Commits:** `ea3153f34 chore(2026-10-19-visual-lint): operator pre-CI commit, for the run this chunk's verdict reads`.
Basis: `git log --format='%h %s' 537a7ced8..HEAD`. The base is 537a7ced8, the parent of the oldest pre-CI commit after
the gated flip 165cfe443 (`gate.py scope` head line). The take-up's own report stands in `report.md`. This report
covers the gated part only: plan step 12, the founder's F1 word option 1.

## Changes (structured — detectors read this)
- **Files:** 22 code files plus the chunk's records. Basis: `gate.py scope`, `changed 22 · listed 21 · recorded 1`; the
  listing is `## New text, by line` below.
  - escher-driver: `packages/escher-driver/src/{schema,command,execute,json,describe,mcp,lib}.rs`, and its tests
    `{schema_tests,mcp_tests,cli_tests,recording_tests}.rs`.
  - blitz-tests: `tests/blitz-tests/tests/{session_common/mod,stand_lint,stand_act_spans,stand_mcp_tools,stand_act_scroll}.rs`.
  - seven_guis: `examples/seven_guis/tests/{cli_commands,mcp_server,host_log,mcp_log}.rs`.
  - The cold-agent pipe: `scripts/cold-agent.sh` and `.github/scripts/test_cold_agent.py`.
  - The chunk's records:
    - `evidence/founder-word.md` (new), `evidence/gate-amend-gated.md` (new) and `scope-record.md` (new).
    - `evidence/counts.md` and `evidence/ci.md` (each gains a gated-pass section).
    - `plan.md`: gate entries 7, 10 and 14, edited by the operator (inputs#I8).
    - `inputs/` I7–I9.
  - `escher-0.2.0/verification-matrix.json`: v020-05 `planned → implemented`, with its ref.
- **Symbols / APIs:**
  - **`lint` is the eighth instance verb and the twelfth verb.**
    - `schema::LINT` (packages/escher-driver/src/schema.rs:310-335) is placed in `VERBS` after `SCREENSHOT`
      (schema.rs:458-461). `VERBS` now holds twelve verbs in three levels: eight instance, three session, one tool.
    - It takes no argument, and a call naming one is refused `malformed` (fault `Unnamed`).
    - Its one result field is `findings`, `FieldKind::Entries`, always present. Its help states the six defect words
      and each reason's keys (schema.rs:322-334). The verb's help states the six readings and its limits: geometry
      only, no hit, right-to-left and vertical text read as left-to-right lines (schema.rs:313-320).
  - **`Command::Lint`** (packages/escher-driver/src/command.rs:175-176, `spec` at :190, validated `("lint", [])` at
    :267). The `Command` enum now has eight variants.
  - **`Outcome::Linted { findings: Vec<Finding> }`** (packages/escher-driver/src/execute.rs:75-79) is a sixth `Outcome`
    variant, and a public API change.
    - Its arm calls `Session::lint` (execute.rs:261-263), so it runs no step and settles nothing.
    - `record_result` returns early for it, as for `Screen` and `Shot` (execute.rs:276). Its `command` span records
      `verb` and, refused, `cause` only — F2 option 1.
    - `Session::run`'s rustdoc names it (execute.rs:121-123).
    - It is refused `fonts-unavailable` exactly as `Session::lint` is: before anything is read, when
      `can_paint_text` reads false.
  - **The finding's written form** (packages/escher-driver/src/json.rs:255-309), hand-written like every answer:
    - `bounds()`: a box as `[x, y, width, height]`, as a node's bounds are written (json.rs:255-269).
    - `ratio()`: an `f32` as its own shortest decimal, `null` when it is not finite (json.rs:271-278).
    - `finding()`: an object keyed `id` · `defect` · `bounds`, then the reason's keys (json.rs:280-308):
      - `other` and `area` for an overlap;
      - `visible` for a clip;
      - `text`, then `visible` (left out when no part can be seen) for a truncation;
      - `ratio` and `required` for contrast;
      - none for off-screen and zero-size.
    - `Outcome::to_json` writes `{"findings":[…]}` (json.rs:360-364).
    - A finding holds stable ids, closed words and numbers only: no `NodeId`, page text, colour or control value.
  - **The carriers**, each reached through the table with no code of its own:
    - **The command line:** `escher-session lint --session <dir>`; the usage line now reads
      `… | screenshot [--id <id>] | lint | start …`.
    - **The socket:** a `call` request, through `Session::run` on the host.
    - **MCP:** a tool; its result is the JSON line through the existing default arm (packages/escher-driver/src/mcp.rs —
      only the rustdoc changed, "the eight instance verbs", mcp.rs:58).
    - **`help`:** `describe()`'s `verbs` holds twelve entries.
    - **Recording:** a `lint` step is kept like every instance verb, its answer kept whole. `recording.rs` is
      unchanged, since it reads `Level::Instance` from `VERBS`.
    - **Remaining callers:** `Session::lint` keeps its in-process callers (the `stand_lint` tests), and the verb's arm
      is now a second non-test caller.
  - **Crate docs:** escher-driver's crate doc names twelve verbs, eight instance (lib.rs:31-33), and the verb on every
    surface (lib.rs:82-87).
  - **Test helpers** (tests/blitz-tests/tests/session_common/mod.rs): `lint()`, the call builder (:115-118); and
    `linted()`, which reads a lint's findings, `None` for a refused call or any other outcome (:120-127). The `act`
    reader's arm now covers `Linted` (:196-199).
- **Crates / modules:** none added or removed; escher-driver changed, as above.
- **Dependencies:** none.
- **Schema / config:**
  - The verb table gains its twelfth row. Its counts moved: instance verbs 7 → 8, and `help`'s `verbs` 11 → 12. Basis:
    `schema_tests.rs` and `describe.rs`'s `counts` (`[12, 13, 12, 3, 8, 4, 15]`).
  - No new `FieldKind`: `Entries` carries the keyed list. No new cause, session error, exit status, argument kind,
    node key or span field.
  - The cold-agent pipe's verb names are twelve, `lint` after `screenshot`, at scripts/cold-agent.sh:48 and
    .github/scripts/test_cold_agent.py:23. The contract test's fake answers `lint` with `{"findings":[]}`
    (test_cold_agent.py:100-101), and its comment reads "the twelve tools" (:60).
- **Spec-master edits:** none.
- **Retired:**
  - The in-process-only carriage of the lint, the delegate's PROVISIONAL «In process only (Most conservative)» on F1
    (inputs#I2), is superseded by the founder's own word, option 1 (inputs#I7).
  - Every master statement "in process only — no verb, `Command`, `Outcome` variant, written form, MCP tool,
    recorded step … carries it, `VERBS` staying eleven" is now false. The sites are under Counts / qualifiers moved,
    patterns `lintverb` · `title` · `outcome`.
  - No version-census row; basis: `escher-0.2.0/census.md` is absent.
- **Counts / qualifiers moved:** each fact below names its pattern, the `standing` figure (from
  `cascade.py sweep --pre-pass`, trail `.andromeda/runs/2026-10-20T07-57-08-wrap/cascade-2026-10-19-visual-lint-pre-pass.json`)
  and a per-row or per-class disposition.
  - **The verb count, eleven → twelve** (`verbs12`, standing 36 rows over the seven masters and one key file). The
    pattern is `ignore_case`, so its `\bVERBS\b` alternative matched every word «verbs». Its rows are every line naming
    the driver's verbs, dispositioned per row as 26 stale + 10 no-change = 36.
    - Stale, 26 rows, each stating the count of verbs or tools (nineteen) or of instance verbs (seven, the same sites as
      `instance8`):
      - architecture.md:196 (@c9111 «`VERBS` lists eleven verbs», @c14768 «11 verbs», @c44821/44832/44842 «`tools/list`
        answers the eleven verbs»; @c18605 is under `lintverb`) · :200 (@c967/2196 «the seven instance verbs») · :202
        («`VERBS` stays eleven») · :219 (@c2256 «the verbs are the ten of `VERBS`», which also lists no `screenshot` —
        stale before this chunk) · :222 (@c10057, @c10443 «the eleven `mcp__escher__{verb}` tools»).
      - security-plan.md:77 («`tool` one of the eleven verbs») · :80 · :81 («the seven instance verbs») · :83 (@c204
        «table of eleven verbs») · :85 (@c1482 «`tools/list` the eleven verbs») · :125 (@c10479 «`VERBS` staying
        eleven») · :214 («eleven since 2026-10-12-…») · :223 (@c1337 «its seven instance-level verbs») · :410 (@c7002
        «the verb table holds seven instance-level verbs»; its @c1829 and @c4345 are dated history).
      - layout-templates.md:74 (@c256, @c3663/3674 «the eleven verbs of the verb table»).
      - test-plan.md:10 (@c5000) · :22 (@c7062/7073 «the listing is the eleven verbs of the table»; @c3454 «seven
        instance-level») · :126 · :127 · :130 · :146 (@c2480 «eleven verbs in a stated order», @c2663) · :151 («`FAKE`,
        which speaks the eleven tools») · :204 (@c692 «each of the seven instance-level verbs») · :205 (@c330).
      - registries/contracts/test-plan/session-lifecycle.md:5 (@c4313 «the seven instance verbs — `tool` …») · :9
        (@c119 «eleven verbs»).
    - No-change, 10 rows:
      - «five acting verbs»: architecture.md:203.
      - The verb table named as a module: architecture.md:354.
      - Other verb sets — agent-run's `boot · run · …` and cold-agent's `run · status · …`: layout-templates.md:72 · :73.
      - Dated history «the six instance-level verbs … as that chunk measured»: test-plan.md:27 @c29961.
      - «the eleven readers at …», another set: test-plan.md:103.
      - Session-level verbs: obs-plan.md:122.
      - «built from `VERBS`»: obs-plan.md:190.
      - Dated history «six instance-level verbs of that day»: obs-plan.md:311.
      - The key file's :11 (@c3039 a description, @c9807 «the eleventh driver-action file», another count).
    - One site outside the pattern's wording, found by the window read: obs-plan.md:195, «the verb table's eleven
      words» (stale; its row stands under `instance8`).
    - The disposition is per row; a detector's own sweep stays the control for reach.
  - **The instance-verb count, seven → eight** (`instance8`, standing 11).
    - Stale, 10 rows:
      - architecture.md:200 (@c952/2181 «the seven instance verbs», the recorded check's `tool`).
      - security-plan.md:80 · :81 · :223 (@c1316 «none of its seven instance-level verbs has an argument or a result
        field of a wait» — the count; the claim holds for `lint`, which has no argument and no wait) · :410 (@c6981
        «the verb table holds seven instance-level verbs and eight causes»).
      - test-plan.md:22 (@c3500) · :146 (@c2642) · :204 (@c671 «each of the seven instance-level verbs is accepted»).
      - registries/contracts/test-plan/session-lifecycle.md:5 (@c4298) · :9 (@c138).
    - No-change, 1 row: obs-plan.md:195's «(seven instance-level; …», inside the dated history of its eleven-word
      clause. The clause's current count is stale under `verbs12`.
  - **The lint's carriage, in process only → a verb on every carrier, recorded** (`lintverb`, standing 14).
    - Stale, 11 rows:
      - architecture.md:196 (@c18681 «from `screenshot` alone among the verbs … and from the in-process
        `Session::lint`»: the `lint` verb now also returns `fonts-unavailable`; @c33714 «the in-process
        `Session::lint`») · :202 (the whole bullet: «in process only — its `lint` verb, on every carrier and
        recorded, owed to «Visual lint»'s gated part»).
      - security-plan.md:125 (@c9112 «meaningful in process only», @c10052/10107 «a fifth in-process reader … (in
        process only)», @c14546) · :137 («its one non-test caller is the in-process `Session::lint`»: the caller
        stands, its in-process qualifier is stale).
      - layout-templates.md:46 («in process o…»).
      - test-plan.md:27 (@c50762/50778 «the in-process `Session::lint` (in process only)»).
      - obs-plan.md:121 (@c5941 «the in-process `Session::lint` — no verb, never handed to `Session::run`»).
      - a11y-plan.md:80 («in process only») · :236 («in process only, no verb carries it»).
    - No-change, 3 rows:
      - architecture.md:169, which names `text_extent`'s non-test caller, `Session::lint` (it stands).
      - architecture.md:354, the lint module in the crate's module list.
      - test-plan.md:22 @c10204 («`Session::lint`'s readings», a unit/integration tier note).
    - Also no-change: obs-plan.md:80, which the `title` pattern reads, and security-plan.md:364 @c6550, «held in
      process only», another refusal's history.
  - **The `Outcome` variants** (`outcome`, standing 6).
    - Stale, 3 rows:
      - architecture.md:202 @c2608 and security-plan.md:125 @c10372: «no verb, `Command`, `Outcome` variant …».
      - security-plan.md:83 @c7093: an `Outcome` holds ids, names, values and a shot's PNG bytes, and now a lint's
        findings — ids, closed words and numbers.
    - No-change, 3 rows: architecture.md:354 @c3292, obs-plan.md:70 @c3743 and the key file's :5 @c2306, each naming
      `Outcome::Shot`.
  - **`run stand`, 190 · 0 · 7 → 192 · 0 · 7** (`runstand`, standing 2). Stale 2: test-plan.md:119 @c10865 and :132.
  - **The workspace tally, 204 · 1083 · 0 · 12 → 204 · 1087 · 0 · 12** (`tally`, standing 3).
    - Stale 2: test-plan.md:334 @c4968 (the per-OS record) and :340 @c19517 (the local tally).
    - No-change 1: a11y-plan.md:268, a `default.css:1087` line citation.
  - **The per-OS CI tallies, CI#39371468367 → CI#39387656399** (`citallies`, standing 2). Stale 2:
    - test-plan.md:334 (@c4895–5106): linux 204 · 1083 → 204 · 1087; macos and coverage 185 · 1080 → 185 · 1084;
      windows 185 · 1038 → 185 · 1041; a11y 6 · 43 unmoved; stand repeat 165 · 660 → 165 · 672.
    - test-plan.md:353: the stand repeat, 165 · 660 → 165 · 672. Basis: `evidence/ci.md`, entry 19 on
      CI#39387656399.
  - **`stand_act_spans`' calls per mode, 32 → 34**, with eight verbs read back where it read seven (`spancalls`,
    standing 22).
    - Stale 1: test-plan.md:27 @c31178 «32 calls per mode».
    - No-change 21: the name token in citations and in descriptions of what the check covers. security-plan.md:410
      @c7065, «`stand_act_spans` also reads a refused shot's line», stays true; that the check also reads a lint's line
      and a refused no-font lint's line is new fact a detector may add.
  - **`cli_commands`' test count, 12 → 13** (`clicmds`, standing 20).
    - Stale 4: test-plan.md:10 @c3102 «(12 tests» · :146 @c9628 «(12 since 2026-10-14-quality-gates» · :204 @c193
      «(12, 11 …» · registries/contracts/test-plan/session-lifecycle.md:11 @c2511.
    - No-change 16: path citations, and dated tallies at test-plan.md:340.
  - **`mcp_server`' test count, 5 → 6, and `stand_mcp_tools`', 5 → 6** (`mcpsrv`, standing 25).
    - Stale 4: test-plan.md:10 @c4782 «`tests/mcp_server.rs` (5 te…» · :27 @c38114 «`stand_mcp_tools` 5» · :205 (@c158
      «`mcp_server` (5,», @c1037 «`stand_mcp_tools` (5,») · the key file's :11 (@c2717, @c2928).
    - No-change 21: citations, dated tallies, and coverage descriptions; obs-plan.md:122 describes the `tool_call`
      spans measured for `screenshot` and `help`, and adding the lint's is new fact.
  - **`stand_lint`'s test count, 5 → 6** (`standlint`, standing 14).
    - Stale 2:
      - test-plan.md:27 @c50653 «`stand_lint` 5».
      - test-plan.md:103 @c3133: `stand_lint` now also reads `lint` and `linted`, two `session_common` helpers this
        chunk added. The reader count, 28, is unmoved.
    - No-change 12: dated tallies (test-plan.md:119, :334, :340, :353), citations and file lists.
  - **The cold-agent pipe's tools, eleven → twelve** (`coldagent`, standing 11).
    - Stale 2: test-plan.md:127 and :130 «the eleven `mcp__escher__…` tools», the same sites as `verbs12`.
    - No-change 9: flags, and line citations of `cold-agent.sh`. cold-agent.sh changed line 48 in place and moved no
      line. test_cold_agent.py gained 2 lines at 100-101, so a citation past line 99 is the citation sweep's.
  - **`host_log` and `mcp_log` each drive one `lint`** (`logflows`, standing 30).
    - No-change 30 as claims: the tests' counts (2 each) are unmoved. Descriptions of their flows — e.g. test-plan.md:187
      @c2077, «also drive shots» — may gain the lint; that is new fact.
    - Line citations of `host_log.rs` and `mcp_log.rs` past the inserted lines (host_log.rs:97, :124-127, :152,
      :165-168; mcp_log.rs:107, :110, :153-156) are the citation sweep's.
  - **The two renamed tests** (`renamed`): 0 master sites. Basis: the sweep's refusal «control: renamed, newtests never
    fired over the seven masters' pre-pass text (ea3153f3)».
    - `schema::tests::the_eleven_verbs_are_named_in_order_each_at_its_level` → `…the_twelve_verbs…`.
    - `mcp::tests::the_listing_is_the_eleven_verbs_of_the_table` → `…the_twelve_verbs…`.
  - **The four new tests** (`newtests`): 0 master sites, basis the same refusal line.
  - **The chunk's title** (`title`, standing 6).
    - Stale 5, the «in process only» qualifier: architecture.md:202 · :354 @c3921 · layout-templates.md:46 ·
      a11y-plan.md:80 · :236.
    - No-change 1: obs-plan.md:80, «The visual lint is silent …». It holds: the module still prints, logs and fields
      nothing, and a finding leaves only as an answer's written form.
  - **The generic owner phrase** (`owner`, standing 2). No-change 2: architecture.md:90, the select model's CARRY, and
    security-plan.md:125 @c9272, another entry's ownership.
  - Sweep reach: 15 patterns · 17 moved facts, 15 with a pattern (the report's own count: the two zero-site facts
    rest on the refusal line) · owner pattern 6 standing · by line 39 lines · 221 offsets.
- **Dev-tool versions:** none. The Andromeda pipeline tools moved mid-session (`gate.py` v1.21 → v1.22, skills
  rewritten at 2026-10-20T07:56Z on the dev host); they are not a project host tool.
- **Harness / gate surface:**
  - The cold-agent pipe's `VERBS` gains `lint` (scripts/cold-agent.sh:48), and its contract test's fake answers it
    (test_cold_agent.py:100-101). The pipe's `isolated` reading now requires twelve tools.
  - Plan gate entries edited by the operator (inputs#I8; `evidence/gate-amend-gated.md`):
    - Entry 7, the guard against a tracked file outside the in-process six, takes
      `defer = "voided — the founder's F1 word (inputs#I7) rules the crossing it forbade; its base also reads the two
      later chunks"`.
    - Entry 10's atom becomes `"passed": 192, "failed": 0, "ignored": 7`.
    - Entry 14's atom becomes `lines 204 passed 1087 failed 0 ignored 12`.
  - The stand-repeat leg was read in this chunk: it runs every `stand_*` file plus seven_guis' `cli_commands`,
    `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and `recording`, three rounds
    (.github/scripts/ci-leg.sh stand-repeat). Its +12 is four new tests × three rounds.
- **Cross-project / external claims:**
  - CI#39387656399 on `ea3153f34cd7` read `verdict: green · checks 17/17 · wall 1087 s` (`ci.py conclusion`, entry 18;
    `evidence/ci.md`). This wrap's commit adds to that tree.
  - `inputs.py verify`, as printed: `inputs: 9 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 8 · uncited 1 · unparsed 0`. No DRIFT.
    - I1 · I2 · I3 · I4 · I5 · I6 · copy message · n/a — a message has no live source. These are the take-up's: I1–I5
      are cited (scope, research, plan, `report.md`). **I6 reads UNCITED**: the operator's P2 answer «Record as built»,
      at the take-up's wrap (2026-10-20T01:08Z). It is named here and carried as cited by this bullet: inputs#I6.
    - I7 · copy no-repo · unchanged — the founder's word, cited by this report. It is a copy of the overseer's relay
      `~/dev/projects/additional/escher-overseer/relays/morning-founder-answers-2026-10-20.md`: «Visual lint» F1
      «Новая команда lint (Рекомендую)» = option 1 «New verb, all carriers, recorded»; F2 «Без поля (Рекомендую)» =
      option 1 «No new field». It was given by dialog in the overseer session at about 04:35Z, relayed verbatim by the
      overseer.
    - I8 is the operator's word on the plan edit and the operator pass, cited by this report.
    - I9 is the operator's word that this wrap runs P1 only, cited by this report.
  - inputs#I3 names the P5 review card `.andromeda/runs/2026-10-19T23-47-25-phase/p5-card-1.md`. Its `## Beyond the
    plan` row, verbatim, with the word's authority (the operator's own «yes», inputs#I3):

    | # | Proposal | Owner |
    |---|---|---|
    | 1 | F1 and F2, answered by the operator as delegate, stay PROVISIONAL for the founder's own word | the wrap that closes Epoch 4 lists them, with the carried `:focus-within` take, for the founder's boundary batch |

    Discharged: the founder's own word on F1 and F2 is inputs#I7. The select-by-id wrap dropped every standing
    PROVISIONAL mark on «Drop all now» (handoff).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - research.md §Sweeps said `stand_act_scroll.rs` «reads the table and pins no count: no change». Measured false:
    tests/blitz-tests/tests/stand_act_scroll.rs:498 asserted `VERBS.len() == 11`. It is fixed as a companion edit
    (scope record). This is a research claim, not a master's; `no master site — the claim stands in research.md alone`.
  - The plan's gate entries 7, 10 and 14 could not read green in the gated pass, as written at the take-up. Entry 7's
    premise fell with the founder's word, and entries 10 and 14 hold counts two later chunks moved. Disposed by the
    operator's plan edit (inputs#I8). `no master site — plan text`.
- **Expected amendments (from plan):** the take-up's report (`report.md`) carried and dispositioned every entry; this
  gated part moves each one again.
  - architecture §Standard Contracts → Driver session (`Session::lint`, `Finding`, `Defect`, the six readings, in
    process; F1 and F2 PROVISIONAL): carried. The carriage moves to the twelfth verb on every carrier, recorded, and
    the span keeps `verb` and `cause` alone on the founder's word. Patterns `lintverb` · `title` · `verbs12` ·
    `instance8` · `outcome`, with hits in architecture.md (rows 196, 200, 202, 219, 222). A boundary widening, now
    ratified by the founder's own word (inputs#I7).
  - architecture §Standard Contracts → Scrolling, selection, tree (`BaseDocument::text_extent`, read-only): carried.
    Unmoved but its caller qualifier: architecture.md:169 is no-change; security-plan.md:137 is stale (`lintverb`).
  - security-plan §Input Validation (the lint's findings crossing nothing): carried. The findings now cross every
    ratified carrier — the socket, the command line, MCP and a kept check — as ids, closed words and numbers. Patterns
    `lintverb` (security-plan.md:125, :137) · `verbs12` (:77, :83, :85, :125, :214) · `instance8` (:80, :81, :223, :410)
    · `outcome` (:83, :125). The founder's ratification is inputs#I7, F1 option 1.
  - test-plan §1 · §2 · §3 · §9 (the counts): carried. Patterns `runstand` · `tally` · `citallies` · `spancalls` ·
    `clicmds` · `mcpsrv` · `standlint` · `verbs12` · `instance8`, with hits in test-plan.md and the key file
    `session-lifecycle.md`.
  - obs-plan §3 (silence) and §4 (no span): carried. §4 moves: the `lint` verb's `command` span records `verb` and,
    refused, `cause` only, «Без поля (Рекомендую)» (inputs#I7). Patterns `lintverb` (obs-plan.md:121) and `verbs12`
    (:195).
  - a11y-plan §6 (`Harness::text_contrast`'s first non-test caller): carried. The «in process only» qualifier is stale
    (`lintverb`, `title`: a11y-plan.md:80, :236).
  - layout-templates §Surface: desktop-native → IA notes: carried. The qualifier is stale (layout-templates.md:46,
    `lintverb` · `title`), and so is §Surface: cli's verb count (layout-templates.md:74, `verbs12`).
  - matrix#v020-05: `no master site — ledger`. Ref written at implement with `--cleared-file`; P7.3 flips it.
- **Coverage of new surfaces:**
  - `lint` verb (socket · command line · MCP · help · recorded check) → validation `validate`✓ (no argument; an
    argument refused `malformed`) · instrumentation the `command` span✓ (`verb` and `cause` only, F2) and the
    `tool_call` span✓ (`tool` and `ending`) · PII redacted✓ (findings are ids, closed words and numbers; a text control's
    value never measured; `host_log` and `mcp_log` read 0 needles with a lint on their flows) · tests unit (json,
    command, schema, cli, mcp, recording) / integ (`stand_lint`, `stand_mcp_tools`, `stand_act_spans`, `cli_commands`,
    `mcp_server`, `host_log`, `mcp_log`)✓ · a11y n/a (no UI) · tokens n/a.

## Deviations from intent
- `tests/blitz-tests/tests/stand_act_scroll.rs` was edited outside research's lists: a companion of the verb table
  (its `VERBS.len()` pin, 11 → 12).
- Two test names were renamed eleven → twelve, as 2026-10-12-headless-screenshot renamed ten → eleven:
  `schema::tests::the_twelve_verbs_are_named_in_order_each_at_its_level` and
  `mcp::tests::the_listing_is_the_twelve_verbs_of_the_table`. The plan named neither, and its gated list includes both
  files.
- Two tests were added beyond the plan's three named ones, in files research lists for the MCP carriage:
  `stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command` (tests/blitz-tests/tests/stand_mcp_tools.rs:706-766)
  and `mcp_server::a_lint_tool_call_answers_its_line_of_findings` (examples/seven_guis/tests/mcp_server.rs:340-358).
- The plan's smoke section says there is no boot path. The binary gained a verb, so implement smoked it by hand:
  `start counter`, `lint` → `{"findings":[]}`, `help` → twelve verbs, `stop`, each exit 0.
- The plan's gate entries 7, 10 and 14 were edited by the operator (inputs#I8), each basis named in
  `evidence/gate-amend-gated.md`.
- The scope record, P1's `gate.py scope` (`scope: clean — changed 22 · listed 21 · recorded 1 (companion 1 · mechanical
  0 · in-intent 0 · widening 0) · absorbed 0 · excluded 19`). Its one line, of class companion:
  - `` `tests/blitz-tests/tests/stand_act_scroll.rs` · companion · serves `packages/escher-driver/src/schema.rs` · self ``

## Decisions & corrections
- **The founder's word, as given**, recorded verbatim in its own script in `evidence/founder-word.md`: F1 «Новая
  команда lint (Рекомендую)» = option 1; F2 «Без поля (Рекомендую)» = option 1. It was given by dialog on 2026-10-20 at
  about 04:35Z and relayed verbatim by the overseer (inputs#I7). It ratifies the lint's carriage, a boundary widening.
  P2's Validate still stops on one card citing it (the 2026-10-10 learning).
- **The operator's word** (inputs#I8): «On my word, an operator edit of plan.md … entries 10 and 14 re-expect HEAD
  537a7ced8's measured counts plus this chunk's own … entry 7 takes defer …», then the operator pass in plan order,
  «Stop if red». Done: entries 10 and 14 re-fired green; hygiene was clean; commit `ea3153f34`; the push moved
  `origin/build/escher-0.2.0 537a7ced→ea3153f3`; CI#39387656399 green, 17/17; the tallies were recorded.
- **The operator's word at invocation** (inputs#I9): «run P1 only and stop». This wrap is the two-window form.
- **A gated take-up re-fires the block P5 froze at the first take-up.** Its exact-count atoms (183, 1055) were moved by
  the two chunks committed in between. Its pre-word guard (entry 7) forbade exactly the crossing the founder's word
  set. All three read red on correct code, and the plan offered no `defer` on entry 7. Resolved only by an operator
  plan edit. Evidence: the gate trail `.andromeda/runs/2026-10-20T07-11-30-implement/gate-2026-10-19-visual-lint.json`
  (firing 1: `red 3 (7,10,14)`), and friction records 2026-10-20T07:26:35Z-b and -c. A curation candidate:
  pipeline-scope.
- **A standing learning held:** 2026-10-10, «search `tests/` and `examples/*/tests/` for the table's name … before
  calling a standing check unaffected». A `git grep` for `VERBS` found the `stand_act_scroll.rs:498` pin, which
  research's sweep had read as «pins no count».
- **A sweep hazard:** predicting the stand-repeat CI figure from the `stand_*` files alone undercounts. The leg also
  runs eight seven_guis targets; measured +12 where `stand_*` alone predicts +6
  (.github/scripts/ci-leg.sh stand-repeat).
- **A sweep hazard in the site sweep's patterns:** an `ignore_case` pattern holding a code token
  (`\bVERBS\b`) matches its plain-word homonym. Here every «verbs» matched, which turned a table-name alternative into
  a line-per-verb-mention sweep. Measured: `verbs12` stood on 36 rows, about a third of them no `VERBS` token. Put a
  code token in a case-sensitive pattern of its own.
- **Guard refusals:**
  - A `cd` into a non-root directory was refused once in implement, re-issued with absolute paths, and refused once in
    this wrap, re-issued with `K=` absolute paths.
  - A pipeline tool's output piped into `tail` (`cascade.py window … | tail -n +2`) was refused once in this wrap and
    re-issued bare. This is a recurrence of a standing rule: host-linux Session Addition 2026-10-13, «never piped into
    head, tail, grep …».
- **The skills were upgraded mid-session** (gate v1.21 → v1.22 at 07:56Z). Implement's gate trail is stamped v1.21 and
  this wrap's v1.22.
- **A verified binary smoke beside the agent harness:** `escher-session start counter / lint / help / stop` in
  `target/tmp/vl-smoke`, each exit 0, the binary built at 07:25Z, after the edits.

## Outcome
- **Acceptance criteria**, against the diff:
  - **(tests, v020-05 gated), MET.** One command an agent can run names, on the planted page and the four lean tasks
    at boot, the findings `Session::lint` reads:
    - In process through `Session::run`:
      `stand_lint::the_lint_verb_answers_what_the_reader_reads` (tests/blitz-tests/tests/stand_lint.rs:308-330) reads
      7 findings on the page, 0 on each task, equal to the reader's, in both modes.
    - Through the binary: `cli_commands::lint_answers_one_line_of_findings` (examples/seven_guis/tests/cli_commands.rs:364-385)
      reads exit 0 and `{"findings":[]}`.
    - Over MCP: `stand_mcp_tools::a_lint_tool_call_…`.
    - The carriage is the founder's own word on F1, option 1 (inputs#I7).
  - **(tests, v020-05 in process), MET as at the take-up.** `stand_lint`'s five in-process tests stay green: 6 passed
    with the gated test.
  - **(obs), MET.** escher-driver holds its 2 `info_span!` sites (entry 5 green), and the four new files of the take-up
    stay silent (entry 6 green). The lint verb adds no span or field.
  - **(security), MET under the founder's word.** The criterion «Nothing crosses a carrier before the founder's word»
    is discharged: his word came, and the crossing is the one he chose. A finding holds ids, closed words and numbers
    only.
    - Entry 7 now reads `not run — defer`, deferred on the operator's edit (inputs#I8). The entry was red by the plan's
      wording: entry 7 against plan step 12's nineteen gated files and the scope record's line, word: «entry 7 takes
      defer = "voided — the founder's F1 word (inputs#I7) rules the crossing it forbade; its base also reads the two
      later chunks"» — the operator, 2026-10-20.
    - Entry 8, no new untracked code file, is green.
  - **(a11y), MET.** The a11y leg is green and its six files unmoved.
  - **(tests), MET.** fast, doc, `run stand` 192 · 0 · 7, `run text_extent` 3 · 0 · 0, and the tally 204 · 1087 · 0 ·
    12, as re-expected by the operator's edit with each basis named. CI is green on the pushed sha.
- **Gates:** implement's firing (`.andromeda/runs/2026-10-20T07-11-30-implement/`), the re-fires, then the operator
  pass. Each entry by its `run`:
  - `cargo test -p blitz-tests --locked --test text_extent 2>&1` — green.
  - `cargo test -p escher-driver --locked lint 2>&1` — green.
  - `cargo test -p blitz-tests --locked --test stand_lint 2>&1` — green.
  - `cargo check --tests --locked --target x86_64-pc-windows-gnu -p blitz-tests` — green.
  - the `info_span!` census — green, last line 2.
  - the silence census — green, exit 1, last line 0.
  - `git diff --name-only e97a7f41b… | grep -c -v -x …` — red in firing 1 (40), now `not run — defer (key)` on the
    operator's edit (inputs#I8; evidence/gate-amend-gated.md).
  - `git ls-files --others … | grep -c -v -x …` — green, exit 1, last line 0.
  - `bash scripts/agent-run.sh boot` — green.
  - `bash scripts/agent-run.sh run stand` — red in firing 1 against the stale atom 183; re-fired by `--entry 10` after
    the edit: green (`10.2.log`), `"passed": 192, "failed": 0, "ignored": 7`.
  - `bash scripts/agent-run.sh run text_extent` — green.
  - `bash .github/scripts/ci-leg.sh a11y` — green.
  - `bash .github/scripts/ci-leg.sh fast` — green.
  - the workspace-tally probe — red in firing 1 against the stale atom; re-fired by `--entry 14`: green (`14.2.log`),
    `lines 204 passed 1087 failed 0 ignored 12`.
  - `bash .github/scripts/ci-leg.sh doc` — green.
  - The `operator` entries, results in `evidence/ci.md`:
    - `gate.py hygiene` — `hygiene: clean`.
    - the push entry — green, `history moved: refs/remotes/origin/build/escher-0.2.0 537a7ced→ea3153f3`.
    - `ci.py conclusion --sha HEAD --wait 1800` — `verdict: green · checks 17/17`, CI#39387656399.
    - the per-OS tallies — `recorded`, exit 0, history unmoved; its output is in `evidence/ci.md`, every move
      explained by name.
  - The three `gated:` entries — each run `--void 20/21/22:block cleared — the founder's own word on F1 «Новая команда
    lint (Рекомендую)» = option 1 (inputs#I7 · evidence/founder-word.md)`, green:
    - the founder-word row probe: last line 1;
    - `stand_lint` with `the_lint_verb_answers_what_the_reader_reads ... ok`;
    - `cli_commands` with `lint_answers_one_line_of_findings ... ok`.
  - Smoke: fired by hand, green (above).
- **Watches:** none folded.
- **Outcome basis:** the operator pass's final state.
  - Setup 4's commit list is `ea3153f34`, and the final HEAD's CI is CI#39387656399, recorded in `evidence/ci.md`.
  - The operator directive between implement and this report (inputs#I8) changed plan entries 7, 10 and 14.
  - Implement's P4 report is in this window, and persisted in
    `.andromeda/runs/2026-10-20T07-11-30-implement/outcome-2026-10-19-visual-lint.md`. Its reading of entries 7, 10 and
    14 as red is superseded by the operator's edit and the re-fires.
  - Post-implement artifacts: `evidence/gate-amend-gated.md` and `evidence/ci.md`'s gated section.
- **Process hygiene:**
  - Implement's census: `none running`; the smoke host stopped (`{"stopped":true}`).
  - Re-measured here: `census · ps · comm escher-session, seven_guis_native · args vl-smoke, agent-run · own tree
    excluded: 8` → `census: none running`.
## Addendum — resumed 2026-10-20
- inputs#I10 · message: the operator, at the wrap resume, 2026-10-20T08:09Z · copy message · n/a — the operator's word
  at the resume: the gated wrap resumes from this report; the lint's carriage is the founder's own word (inputs#I7,
  «Новая команда lint (Рекомендую)»), relayed by the overseer from his dialog in the overseer session; a P2 card citing
  it the operator brings to the founder.
  - bears on **The founder's word, as given**: adds to it — the card P2's Validate stops on is carried to the founder by
    the operator.
  - bears on **The operator's word at invocation**: supersedes «run P1 only and stop» — the wrap resumes at P2.
- inputs.py verify, re-fired: `inputs: 10 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 9 · uncited 1 · unparsed 0` (the one uncited entry I10, cited by this addendum); moved since the
  report: I6 uncited → cited (report-gated.md:234), I10 added.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 537a7ced (gated: the parent of the oldest pre-CI commit ea3153f3 after the gated flip 165cfe44) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/test_cold_agent.py — added 4 line(s) in 3 range(s)
added: 23 · 60 · 100-101
  - 100-101 «if name == "lint":»
### examples/seven_guis/tests/cli_commands.rs — added 27 line(s) in 4 range(s)
added: 2 · 5-6 · 128 · 364-386
- 364-385 @365 «fn lint_answers_one_line_of_findings() {»
  - 369-372 «assert_eq!(»
  - 374-378 «assert_eq!(»
  - 380-383 «assert!(»
### examples/seven_guis/tests/host_log.rs — added 12 line(s) in 5 range(s)
added: 3-4 · 97 · 124-127 · 152 · 165-168
  - 165-168 «assert!(»
### examples/seven_guis/tests/mcp_log.rs — added 8 line(s) in 4 range(s)
added: 3-4 · 107 · 110 · 153-156
  - 153-156 «assert!(»
### examples/seven_guis/tests/mcp_server.rs — added 29 line(s) in 5 range(s)
added: 3-5 · 126 · 129 · 133-136 · 340-359
- 340-358 @341 «fn a_lint_tool_call_answers_its_line_of_findings() {»
  - 345-348 «assert_eq!(»
  - 350-353 «assert!(»
### packages/escher-driver/src/cli_tests.rs — added 8 line(s) in 4 range(s)
added: 143 · 296-299 · 420 · 536-537
### packages/escher-driver/src/command.rs — added 11 line(s) in 10 range(s)
added: 175-176 · 190 · 267 · 500 · 528 · 624 · 688 · 788 · 860 · 870
### packages/escher-driver/src/describe.rs — added 1 line(s) in 1 range(s)
added: 145
### packages/escher-driver/src/execute.rs — added 13 line(s) in 5 range(s)
added: 14 · 75-79 · 121-123 · 261-263 · 276
  - 75-79 @76 «Linted {»
### packages/escher-driver/src/json.rs — added 160 line(s) in 7 range(s)
added: 13 · 255-309 · 360-364 · 435 · 625-697 · 765-788 · 790
- 255-269 @256 «fn bounds(rect: Bounds) -> String {»
  - 258-266 «for (index, bound) in [rect.x, rect.y, rect.width, rect.height]»
- 271-278 @272 «fn ratio(out: &mut String, value: f32) {»
  - 273-277 «if value.is_finite() {»
- 280-308 @281 «fn finding(finding: &Finding) -> String {»
  - 286-306 «match &finding.reason {»
  - 625-632 «fn rect(x: f64, y: f64, width: f64, height: f64) -> Bounds {»
  - 634-696 @635 «fn every_reason() -> Vec<Finding> {»
### packages/escher-driver/src/lib.rs — added 9 line(s) in 2 range(s)
added: 31-33 · 82-87
### packages/escher-driver/src/mcp.rs — added 1 line(s) in 1 range(s)
added: 58
### packages/escher-driver/src/mcp_tests.rs — added 7 line(s) in 4 range(s)
added: 399 · 408 · 499-501 · 512-513
### packages/escher-driver/src/recording_tests.rs — added 22 line(s) in 4 range(s)
added: 13 · 133-149 · 156 · 184-186
### packages/escher-driver/src/schema.rs — added 29 line(s) in 3 range(s)
added: 310-336 · 458 · 461
- 310-335 «pub(crate) const LINT: VerbSpec = VerbSpec {»
  - 313-320 «help: "reads the screen for six defects, each found on an element by its id with the \»
  - 322-334 «fields: &[FieldSpec {»
### packages/escher-driver/src/schema_tests.rs — added 10 line(s) in 9 range(s)
added: 10 · 18 · 26-27 · 32 · 51 · 163 · 191 · 250 · 259
### scripts/cold-agent.sh — added 1 line(s) in 1 range(s)
added: 48
### tests/blitz-tests/tests/session_common/mod.rs — added 17 line(s) in 4 range(s)
added: 20 · 115-128 · 196 · 199
- 115-118 @116 «pub fn lint() -> Call {»
- 120-127 @122 «pub fn linted(session: &mut Session, call: &Call) -> Option<Vec<Finding>> {»
  - 123-126 «match session.run(call).ok()? {»
### tests/blitz-tests/tests/stand_act_scroll.rs — added 1 line(s) in 1 range(s)
added: 498
### tests/blitz-tests/tests/stand_act_spans.rs — added 24 line(s) in 11 range(s)
added: 3 · 15-16 · 41 · 183 · 193 · 358-364 · 437 · 466-467 · 601 · 687-692 · 704
### tests/blitz-tests/tests/stand_lint.rs — added 28 line(s) in 3 range(s)
added: 7-9 · 18 · 307-330
- 308-330 @309 «fn the_lint_verb_answers_what_the_reader_reads() {»
  - 310-329 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_mcp_tools.rs — added 65 line(s) in 2 range(s)
added: 21-23 · 706-767
- 706-766 @707 «fn a_lint_tool_call_answers_the_readers_findings_and_parents_its_command() {»
  - 708-765 «for incremental in [false, true] {»
