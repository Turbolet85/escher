proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-proof gains, after `stand_load_wait` 2, «since 2026-10-20-large-screen-in-parts `stand_snapshot_parts` 3
      (`Session::run` in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: `snapshot` with no
      argument answers part 1 of `parts` ≥ 2, each part within the budget, the parts concatenating to `to_text()` with every
      node id on exactly one line, `parts + 1` answering the empty text with the same `parts` and the screen unchanged,
      `item-400`, read only in the last part, taking `scroll` then `click`; a password reading `MASKED_VALUE` in every part,
      the typed sentinel 0 times; the same walk through `serve_mcp` answering the JSON lines `Session::run` writes; a screen
      inside the budget read whole as before)»
    sidecar: >-
      test-plan §3 Session lifecycle session-proof: + `stand_snapshot_parts` 3, the paged snapshot's walk (2026-10-20-large-screen-in-parts)
    rationale: >-
      The report adds the blitz-tests target `stand_snapshot_parts` (Harness / gate surface; Outcome (tests), 3 passed) and its
      Expected amendments name test-plan §3 → Session lifecycle, `session-proof`, for «the new check»; the key's proof list
      names every session check and holds none for the part read.
    basis: tests/blitz-tests/tests/stand_snapshot_parts.rs:82-333
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-proof: seven_guis `cli_commands` 13 → 14 (since 2026-10-20-large-screen-in-parts a part read by number accepted,
      and `--part` 0, 1001 and `x` refused, status 1, with a line equal to `Refusal::malformed(fault).to_json()`)
    sidecar: >-
      test-plan §3 Session lifecycle session-proof: `cli_commands` 13 → 14
    rationale: >-
      Report Counts / qualifiers moved, `clicount`: «seven_guis `cli_commands`: 13 → 14», changed at
      registries/contracts/test-plan/session-lifecycle.md:11 @c2511; Outcome (security) states the refusal line.
    basis: examples/seven_guis/tests/cli_commands.rs:294-339
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-proof: «escher-driver's 99 unit tests (… mcp_tests.rs 14 …» → «escher-driver's 100 unit tests (… mcp_tests.rs 15 …»,
      with «99 until 2026-10-20-large-screen-in-parts added mcp_tests.rs's optional-`part` listing test, as measured at
      escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md» at the head of its history
    sidecar: >-
      test-plan §3 Session lifecycle session-proof: escher-driver unit tests 99 → 100 (mcp_tests.rs 14 → 15)
    rationale: >-
      Report Counts / qualifiers moved: «escher-driver unit tests 99 → 100» (gate entry 2, 100 passed); the one new escher-driver
      test head in New text, by line is mcp_tests.rs 481-498 `the_snapshot_tool_takes_an_optional_part_and_names_its_two_counts`.
    basis: packages/escher-driver/src/mcp_tests.rs:481-498
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-binary: the usage line reads `snapshot [--part <part>]` beside `start <app> [--record <dir>]`, `host role: …` and
      `mcp role: …` (since 2026-10-20-large-screen-in-parts), built from the table
    sidecar: >-
      test-plan §3 Session lifecycle session-binary: usage line's first segment `snapshot [--part <part>]`
    rationale: >-
      Report Counts / qualifiers moved, `usage`: «The usage line's first segment `snapshot` → `snapshot [--part <part>]`», changed at
      session-lifecycle.md:9 @c1859; Symbols / APIs: «The usage line now spells `snapshot [--part <part>]`, built from the table».
    basis: packages/escher-driver/src/cli.rs:253-257
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle
    change: >-
      session-binary: the instance verbs' flag list gains `--part` with one value on `snapshot` (a whole number 1 to 1000, read
      as a number as `--ms` is), a paged answer one line `{"text":…,"part":n,"parts":m}`, status `0`, a whole screen's still
      `{"text":…}` alone
    sidecar: >-
      test-plan §3 Session lifecycle session-binary: `snapshot --part` flag and its paged answer line
    rationale: >-
      Report Symbols / APIs: «The command line reads a `Part` flag's value as a number, as `Milliseconds` (`cli.rs:253-257`)» and
      Schema / config: «Paged answer line: `{"text":…,"part":n,"parts":m}`. A whole screen's line is still `{"text":…}` alone»;
      the row's list of flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms`, `--shift`) omits it.
    basis: packages/escher-driver/src/cli.rs:253-257
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      the blitz-tests row gains, after `stand_act_select` 5, «and, since 2026-10-20-large-screen-in-parts, `stand_snapshot_parts` 3,
      declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page over `SNAPSHOT_TEXT_BUDGET`:
      the walk in parts through `Session::run` (default paging from part 1, each part within the budget, the parts concatenating
      to `to_text()`, the empty part past the last, `item-400` addressable from the last part, the password masked in every part,
      each part's lines equal to the whole text's), the same walk through `serve_mcp`, and a screen inside the budget read whole
      (tests/blitz-tests/tests/stand_snapshot_parts.rs:82-333); picked up by `run stand` and the stand-repeat leg by its prefix —
      as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
    sidecar: >-
      test-plan §1 blitz-tests: + `stand_snapshot_parts` 3
    rationale: >-
      Report Harness / gate surface: new blitz-tests target `stand_snapshot_parts`, joining `run stand` and stand-repeat by its
      prefix; Outcome (tests, security, a11y) states what it asserts. §1's blitz-tests row lists each new stand check with its count
      (`stand_load_wait` 2, `stand_act_select` 5) and has none for it.
    basis: tests/blitz-tests/tests/stand_snapshot_parts.rs:82-333
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      the escher-driver row: «99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files» → «100 … 64 inline and
      36 in four», mcp_tests.rs 14 → 15 (since 2026-10-20-large-screen-in-parts `snapshot`'s tool takes an optional `part` and names
      its two counts), with «99 in sixteen, 64 inline and 35 in four `#[path]` files, until 2026-10-20-large-screen-in-parts» at the
      head of its history
    sidecar: >-
      test-plan §1 escher-driver: 99 → 100 unit tests (mcp_tests.rs 14 → 15)
    rationale: >-
      Report Counts / qualifiers moved: «escher-driver unit tests 99 → 100» (gate entry 2); the one new test head is mcp_tests.rs
      481-498. The pre-pass `unitcount` pattern fired 0 (its control's refusal), so the sweep did not carry this site.
    basis: packages/escher-driver/src/mcp_tests.rs:481-498
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      the dioxus-native-dom row: «75 unit tests in nine files» → «81», adding «six on the snapshot text's parts reader
      `Snapshot::text_parts` — the parts concatenating to the whole text each within the budget, each node id in exactly one part in
      pre-order, a line longer than the budget a part alone, an empty snapshot one empty part, a budget holding the whole text one
      part, a part keeping each line's absolute indent (packages/dioxus-native-dom/src/snapshot_text.rs:493-609; 75 until
      2026-10-20-large-screen-in-parts)»
    sidecar: >-
      test-plan §1 dioxus-native-dom: 75 → 81 unit tests (+6 `Snapshot::text_parts`)
    rationale: >-
      Report Counts / qualifiers moved: «dioxus-native-dom lib tests 75 → 81» (gate entry 1, 81 passed); Symbols / APIs: «Six unit
      tests: 493-609», whose six heads New text, by line lists. The `unitcount` pattern fired 0, so the sweep did not carry this site.
    basis: packages/dioxus-native-dom/src/snapshot_text.rs:493-609
    dependent-of: D-tests-coverage
  - detector: D-tests-obs-harness
    severity: warning
    section: §1 Test Scope Summary
    change: >-
      `stand_act_spans`: «34 calls per mode» → «35 calls per mode», adding that since 2026-10-20-large-screen-in-parts one `snapshot`
      with `part` 1 per mode reads a `command` line holding `verb="snapshot"` alone
    sidecar: >-
      test-plan §1 stand_act_spans: 34 → 35 calls per mode (a part read records `verb` alone)
    rationale: >-
      Report Counts / qualifiers moved, `spancalls`: «`stand_act_spans` calls per mode: 34 → 35», changed at test-plan.md:27 @c31178
      and obs-plan.md:311 @c2334 — both sides move together, so test-plan and obs-plan §3/§4 stay in agreement only if both apply;
      Outcome (obs).
    basis: tests/blitz-tests/tests/stand_act_spans.rs:356-362
  - detector: D-tests-coverage
    severity: warning
    section: §2 Test Strategy
    change: >-
      Directory pattern: `tests/fixtures/mod.rs` read through `mod fixtures;` by 24 checks — 22 `stand_*.rs` (`stand_snapshot_parts`
      added to the list) — 23 before `stand_snapshot_parts`
    sidecar: >-
      test-plan §2 Directory pattern: fixtures readers 23 → 24 (`stand_*` 21 → 22)
    rationale: >-
      Report Counts / qualifiers moved, `readers`: «`tests/fixtures/mod.rs` readers 23 → 24 (`stand_*` 21 → 22), since
      `stand_snapshot_parts` declares both», changed at test-plan.md:68 @c2549.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §2 Test Strategy
    change: >-
      Directory pattern: `tests/session_common/mod.rs` read through `mod session_common;` by 29 checks (28 before
      `stand_snapshot_parts.rs`, …)
    sidecar: >-
      test-plan §2 Directory pattern: session_common readers 28 → 29
    rationale: >-
      Report Counts / qualifiers moved, `readers`: «`tests/session_common/mod.rs` readers go 28 → 29»; line 68 states the 28 a second
      time beside the fixtures count the sweep carried at @c2549.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      Crate-local test helpers, blitz-tests (session checks): the builders gain, since 2026-10-20-large-screen-in-parts,
      `snapshot_part(n)` and the part reader `part(outcome)` (text, part, parts; panicking with a fixed message on any other outcome),
      `act` returning `None` for a part read too; readers «29 with `stand_snapshot_parts`; 28 before it, with `stand_act_select`; …»
    sidecar: >-
      test-plan §3 session checks: + `snapshot_part`, `part`; readers 28 → 29
    rationale: >-
      Report Symbols / APIs, Test helpers: `snapshot_part` (68-71), `part` (73-80), `act` gains `Outcome::Part` (213-216); Counts /
      qualifiers moved, `readers`: 28 → 29, changed at test-plan.md:103 @c3021, «The call builders' list gains `snapshot_part` and
      the part reader `part`».
    basis: tests/blitz-tests/tests/session_common/mod.rs:68-80
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      Crate-local test helpers, blitz-tests (fixtures): «24 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_act_select`;
      `stand_load_wait`, reading `HeldNet`; `stand_lint`; `stand_snapshot_parts`)»
    sidecar: >-
      test-plan §3 fixtures: 23 → 24 readers, 21 → 22 `stand_*`
    rationale: >-
      Report Counts / qualifiers moved, `readers`: fixtures readers 23 → 24 (`stand_*` 21 → 22), changed at test-plan.md:104 ×2.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      Agent-run contract, Proof: append «re-counted with the paged snapshot, `scripts/agent-run.sh` unchanged: `run stand`'s
      `run.end` reads passed 195 · failed 0 · ignored 7 over 48 files — +3 passed (`stand_snapshot_parts` 3, a new file picked up by
      its `stand_` prefix with no script change); `run all` not re-counted — as measured at
      escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
    sidecar: >-
      test-plan §3 agent-run Proof: `run stand` 48 files · 195 · 0 · 7
    rationale: >-
      Report Counts / qualifiers moved, `standrun`: «47 files · 192 passed · 0 · 7 → 48 files · 195 passed · 0 failed · 7 ignored»
      (gate entry 13), changed at test-plan.md:119 @c10904/@c11286/@c11325.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      Test-data bootstrap: «`run stand` 192 · 0 · 7, as measured at …2026-10-19-visual-lint/report-gated.md» → «`run stand`
      195 · 0 · 7 over 48 files, as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
    sidecar: >-
      test-plan §3 test-data bootstrap: `run stand` 195 · 0 · 7
    rationale: >-
      Report `standrun` lists test-plan.md:132 among its four standing sites, all changed.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract
    change: >-
      Test-data bootstrap: «`run stand` runs the 21 `stand_*` readers of the shared module» → «the 22 `stand_*` readers»
    sidecar: >-
      test-plan §3 test-data bootstrap: shared-module `stand_*` readers 21 → 22
    rationale: >-
      Report Counts / qualifiers moved, `readers`: `tests/fixtures/mod.rs` `stand_*` readers 21 → 22; line 132 restates that count,
      a site the `readers` sweep did not list.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy
    change: >-
      escher-driver: «the six argument kinds with their bounds (the sixth, `ArgKind::Name`, …)» → «the seven argument kinds with their
      bounds (the sixth, `ArgKind::Name`, …; the seventh, `ArgKind::Part`, a whole number 1 to 1000 (`MAX_PART`), since
      2026-10-20-large-screen-in-parts)»
    sidecar: >-
      test-plan §4 escher-driver: argument kinds six → seven (`ArgKind::Part`)
    rationale: >-
      Report Counts / qualifiers moved, `kinds`: «Argument kinds: six → seven (`ArgKind::Part`, 1 to `MAX_PART` = 1000)», changed at
      test-plan.md:146 @c3029 and @c3079.
    basis: packages/escher-driver/src/schema/spec.rs:19-21
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy
    change: >-
      escher-driver: «seven_guis' `cli_commands` (13)» → «`cli_commands` (14)»
    sidecar: >-
      test-plan §4 escher-driver: `cli_commands` 13 → 14
    rationale: >-
      Report `clicount`: changed at test-plan.md:146 @c9655 «`cli_commands` (13)».
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy
    change: >-
      Session host ↔ what it writes: «each also drives one `lint`» → «each also drives one `lint` and, since
      2026-10-20-large-screen-in-parts, one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool
      calls and 8 `command` lines — still reading 0 needles on stderr at `trace`, its count (2 each) unmoved»
    sidecar: >-
      test-plan §5 host_log / mcp_log: each drives one part read, 0 needles
    rationale: >-
      Report Counts / qualifiers moved, `logflows`: «`host_log` sends seven calls, not six, and reads 7 command-span lines; `mcp_log`
      sends ten tool calls, not nine, with 8 `command` lines. Each file's test count is unchanged at 2», changed at test-plan.md:187
      @c2326; Outcome (obs).
    basis: examples/seven_guis/tests/host_log.rs:173-177
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy
    change: >-
      Command line: «`cli_commands` (13)» → «`cli_commands` (14)», adding «since 2026-10-20-large-screen-in-parts `snapshot --part`
      read by number answers one uncoloured JSON line, status 0, and `--part` 0, 1001 and `x` exit 1 with a line equal to
      `Refusal::malformed(fault).to_json()`, nothing typed written back (examples/seven_guis/tests/cli_commands.rs:294-339)»
    sidecar: >-
      test-plan §6 command line: `cli_commands` 13 → 14, the part read and its refusals
    rationale: >-
      Report `clicount`: changed at test-plan.md:204 @c193; Outcome (security) and (design) state what the new test asserts.
    basis: examples/seven_guis/tests/cli_commands.rs:294-339
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration
    change: >-
      stand-repeat leg: «47 files, … `stand_load_wait` since 2026-10-20-load-wait and `stand_act_select`» → «48 files, … and
      `stand_snapshot_parts` since 2026-10-20-large-screen-in-parts»
    sidecar: >-
      test-plan §9 stand-repeat: 47 → 48 `stand_*` files
    rationale: >-
      Report `standrun`: changed at test-plan.md:324 @c1299 («47 files»); Harness / gate surface: `stand_snapshot_parts` joins
      `ci-leg.sh stand-repeat` by its prefix.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration
    change: >-
      per-OS tallies: append «re-read with the paged snapshot on CI#39410865604, the pre-CI commit `191e66fdb` (green 17/17): linux
      205 · 1098; macos and coverage 186 · 1095; windows 186 · 1051; the a11y job 6 · 43, unmoved; the stand repeat 168 · 684 · 0 · 21
      — +1 result line and +11 passed on linux, macos and coverage over CI#39387656399, +10 on windows (`cli_commands` unix-only),
      +3 · +12 on the stand repeat (`stand_snapshot_parts` 0 → 9, `cli_commands` 39 → 42) — as measured at
      escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/evidence/ci.md»
    sidecar: >-
      test-plan §9 per-OS tallies: CI#39410865604 read
    rationale: >-
      Report Counts / qualifiers moved, `peros`: «Per-OS CI tallies, CI#39387656399 → CI#39410865604», changed at test-plan.md:334
      @c5564/@c5593/@c5668.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration
    change: >-
      workspace tally: append «re-counted with the paged snapshot: 205 result lines, 1098 passed · 0 failed · 12 ignored, from
      204 · 1087 — +1 result line (`stand_snapshot_parts`) and +11 passed (dioxus-native-dom 6, escher-driver 1,
      `stand_snapshot_parts` 3, `cli_commands` 1), timings not re-measured — as measured at
      escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
    sidecar: >-
      test-plan §9 workspace tally: 205 · 1098 · 0 · 12
    rationale: >-
      Report Counts / qualifiers moved, `tally`: «lines 204 · passed 1087 · 0 · 12 → lines 205 · passed 1098 · failed 0 · ignored 12»
      (gate entry 16), changed at test-plan.md:334 @c5524 and test-plan.md:340 @c19974.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §10 Quality Gates & Coverage Targets
    change: >-
      Flakiness budget: «run the 47 `stand_*.rs` files» → «run the 48», the parenthesis led by «with `stand_snapshot_parts`
      CI#39410865604 read 168 · 684 · 0 · 21, +3 · +12 (`stand_snapshot_parts` 0 → 9, `cli_commands` 39 → 42); 47 before
      `stand_snapshot_parts`, with the `lint` verb's four tests CI#39387656399 read …»
    sidecar: >-
      test-plan §10 flakiness budget: 47 → 48 files, stand repeat 168 · 684 · 0 · 21
    rationale: >-
      Report `standrun` lists test-plan.md:353 («the 47 `stand_*.rs` files») as changed; `peros` lists :353 too; the stand repeat
      «165 · 672 → 168 · 684 · 0 · 21» by a per-target diff of the two job logs.
    dependent-of: D-tests-coverage
note: >-
  D-tests-framework holds (cargo test, agent-run and the stand-repeat leg unchanged; Harness / gate surface «No agent-run script,
  CI step, leg or status shape changed»). The two unit-count sites (test-plan §1 escher-driver and dioxus-native-dom rows, and
  session-proof's escher-driver count) were not carried by the pre-pass sweep — its `unitcount` pattern fired 0 — and are proposed
  here from the report's own «99 → 100» and «75 → 81». The §1 span-count proposal moves with obs-plan.md:311 @c2334.
