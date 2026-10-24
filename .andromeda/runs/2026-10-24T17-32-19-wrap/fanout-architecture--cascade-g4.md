proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 25
    section: "## Occupied Resources"
    change: "the session checks' own state directories are `ss-life`, `ss-idle` and `ss-quiet` (blitz-tests) and `hb-*`, `hl-trace`, `cc-*`, `cf-*` and `ht-timer` (seven_guis: `host_binary`, `host_log`, `cli_commands`, `cli_flow`, `host_timer`) and, since 2026-10-24-stand-requirement-sweep, `hl-client` and `hl-client-rec` (`host_log`'s unix-only `start --record` clients' check) under `CARGO_TARGET_TMPDIR`, i.e. `target/tmp/`, …; add the citation examples/seven_guis/tests/host_log.rs:40-130 beside the list's, and re-point examples/seven_guis/tests/host_log.rs:156 past the 97 lines added before it (11-15 · 40-131)"
    sidecar: "occupied-resources Filesystem: the session checks' state directories gain host_log's `hl-client` and `hl-client-rec` (2026-10-24-stand-requirement-sweep); host_log.rs:156 re-pointed"
    rationale: "Report Changes › Counts 9 names occupied-resources.md:25 moved (the list gains `hl-client`, `hl-client-rec`) and Outcome calls it «an arch §Occupied Resources filesystem line owed at P2»; Symbols › host_log.rs places the two directories in the new test at 40-130; New text, by line lists host_log.rs added 11-15 · 40-131, all before the old :156."
    basis: .andromeda/architecture-detail/occupied-resources.md:25
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 23
    section: "## Occupied Resources"
    change: "re-point the CI leg logs' citation .github/scripts/ci-leg.sh:88-89 five lines later, past the five lines added at 54-58 (its `target/ci-logs/` prose unchanged); the coverage leg's .github/scripts/ci-leg.sh:40-47 stands, before the insertion"
    sidecar: "occupied-resources Filesystem: the ci-leg.sh log citation re-pointed past stand-repeat's five added lines"
    rationale: "ci-leg.sh is a reached file; New text, by line lists it added 54-58 in one range, so every line past 53 moved by five — the logs' :88-89 among them; the report's Schema / config changes only stand-repeat's selection, so the claim itself holds."
    basis: .andromeda/architecture-detail/occupied-resources.md:23
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: "## Occupied Resources"
    change: "after «`cli_commands` runs the binary once per command as a client, its sessions started by the binary's own `start` command» add: «and, since 2026-10-24-stand-requirement-sweep, `host_log`'s unix-only `a_recording_sessions_clients_log_nothing_of_it_at_trace` does so too — `start crud --record`, `type`, `snapshot`, `snapshot --part 1`, `lint` and `stop`, each through `common::run` with `RUST_LOG=trace` on the client, in `hl-client` and `hl-client-rec`, a `common::Started` guard stopping the started session (examples/seven_guis/tests/host_log.rs:40-130)»"
    sidecar: "occupied-resources Process-wide state: host_log now also runs escher-session as a client under start --record at trace (2026-10-24-stand-requirement-sweep)"
    rationale: "Report Symbols › host_log.rs: the new test runs each client command through `common::run` with `RUST_LOG=trace`, `start crud --record` first, guarded by `common::Started { dir, pid }`; the row now says only that host_log spawns the host role itself. RUST_LOG is the registered read (Outcome), so no env var is new."
    basis: .andromeda/architecture-detail/occupied-resources.md:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 41
    section: "## Occupied Resources"
    change: "re-point tests/blitz-tests/tests/stand_act_spans.rs:569-577 (its children's re-execution) four lines later, past the lines added at 6-8 and 243; the clause's prose unchanged"
    sidecar: "occupied-resources Process-wide state: the stand_act_spans re-execution citation re-pointed"
    rationale: "stand_act_spans.rs is a reached file; New text, by line lists added 6-8 · 243 · 685-693 · 727, so the four lines at 6-8 and 243 sit before :569; the report changes no child, argv or env of the re-run (Symbols › stand_act_spans.rs: one more call in `child_records_each_command_at_info`)."
    basis: .andromeda/architecture-detail/occupied-resources.md:41
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 43
    section: "## Occupied Resources"
    change: "re-point examples/seven_guis/tests/host_log.rs:22 five lines later (past the lines added at 11-15) and examples/seven_guis/tests/host_log.rs:156-175 ninety-seven lines later (past 11-15 and 40-131); the prose unchanged"
    sidecar: "occupied-resources Process-wide state: the host_log spawn citations re-pointed past the added test"
    rationale: "host_log.rs is a reached file; New text, by line lists added 11-15 · 40-131 (5 + 92 lines), both before :156 and the first before :22; the host-role spawn those lines cite is unchanged by the report."
    basis: .andromeda/architecture-detail/occupied-resources.md:43
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/inherited-defaults.md
    near: 14
    section: "## Inherited Defaults"
    change: "re-point the range .github/scripts/ci-leg.sh:37-64 so its end falls five lines later, taking in the five lines added at 54-58 inside it; .github/scripts/ci-leg.sh:20-25 and the row's prose unchanged"
    sidecar: "inherited-defaults Code quality: the ci-leg.sh legs range widened past stand-repeat's five added lines"
    rationale: "Report Counts 1 rules inherited-defaults.md:14 no-change for its prose (the leg named alone), but ci-leg.sh is reached and New text, by line lists added 54-58, inside the cited 37-64; the task's Touches note the same five-line move for the body's range."
    basis: .andromeda/architecture-detail/inherited-defaults.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "accessibility_states' runner clause: «run by the workspace test leg, per package and, since 2026-10-24-stand-requirement-sweep, by the `stand-repeat` leg — a named `--test` word in `ci-leg.sh` on the founder's «Список в скрипте (Рекомендую)» (2026-10-24) — and not by the `a11y` CI leg, which runs six named files since 2026-10-14-stand-a11y-assertions, this one not among them»"
    sidecar: "existing-scopes blitz-tests: accessibility_states also runs in stand-repeat (2026-10-24-stand-requirement-sweep)"
    rationale: "Report Counts 1 lists existing-scopes.md:102 moved (accessibility_states «run by the workspace test leg and per package» gains stand-repeat); Schema / config names `--test accessibility_states` among the ten added to stand-repeat at ci-leg.sh 54-58; Decisions & corrections gives the founder's word."
    basis: .andromeda/architecture-detail/existing-scopes.md:102
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 110
    section: "## Existing Scopes"
    change: "after «and no id, name or typed text anywhere in the capture» add «— since 2026-10-24-stand-requirement-sweep a paged read among the calls, part 2 of the shared `many_items` page read as `Outcome::Part`, its item ids and button names joining the needles, 36 calls per mode»; re-point tests/blitz-tests/tests/stand_act_spans.rs:1-24 past the three lines added at 6-8 inside it"
    sidecar: "existing-scopes blitz-tests: stand_act_spans' sink capture holds a paged part read (2026-10-24-stand-requirement-sweep)"
    rationale: "Report Symbols › stand_act_spans.rs: `fixtures::hold(\"paged\", fixtures::many_items, …)` reads `snapshot_part(2)` as `Outcome::Part { part: 2, parts ≥ 2 }` (685-693), `Needles::read` takes the `#items > button` text and 400 `item-NNN` ids (243), 35 → 36 calls (727); the row states what that capture reads; New text lists 6-8 added inside the cited 1-24."
    basis: .andromeda/architecture-detail/existing-scopes.md:110
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 113
    section: "## Existing Scopes"
    change: "`stand_requirement_sweep`, 2 tests → 3: after «… each found in its file through `include_str!`» add «and, since 2026-10-24-stand-requirement-sweep, a second record `WITNESSES_020` — rows v020-01 to v020-09, 46 witnesses — with a `Subject::Unit` witness kind for a crate's unit test, run by the workspace `test` leg and outside `stand-repeat` by construction, and a third test, `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`, reading the nine rows in order, every named test found in its file, v020-01 and v020-02 `Unit` alone, and every integration witness inside `stand-repeat` by the `stand_` prefix or by the named `--test` list read from `ci-leg.sh`'s own text (tests/blitz-tests/tests/stand_requirement_sweep.rs:227-229; tests/blitz-tests/tests/stand_requirement_sweep.rs:538-818; tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069)»; re-point the row's :90-216 past the six lines added at 10-15, and :241-992 for the ranges added inside it (538-847 · 892-952)"
    sidecar: "existing-scopes blitz-tests: stand_requirement_sweep 3 tests, WITNESSES_020 and Subject::Unit (2026-10-24-stand-requirement-sweep)"
    rationale: "Report Counts 3 lists existing-scopes.md:113 moved («`WITNESSES`, 16 rows»; the file 2 → 3 tests); Symbols › stand_requirement_sweep.rs gives `Subject::Unit` (227-229), `WITNESSES_020` (538-818) and the test (994-1069) and its assertions; New text lists added 10-15 · 227-229 · 232 · 538-847 · 892-952 · 993-1069."
    basis: .andromeda/architecture-detail/existing-scopes.md:113
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 117
    section: "## Existing Scopes"
    change: "re-point examples/seven_guis/tests/host_log.rs:1-15 so it ends five lines later, the five lines added at 11-15 now inside it; the row's claim (eight targets, all spawning the built binary) unchanged"
    sidecar: "existing-scopes seven_guis: the host_log header citation re-pointed"
    rationale: "host_log.rs is a reached file; New text, by line lists added 11-15 · 40-131, so the old lines 11-15 the citation took in now sit five lines later; the new test still spawns the binary (Symbols › host_log.rs), so no target or role clause moves."
    basis: .andromeda/architecture-detail/existing-scopes.md:117
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 133
    section: "## Existing Scopes"
    change: "re-point tests/blitz-tests/tests/fixtures/mod.rs:17-130 so it ends twenty-three lines later, taking in `ITEMS` and `many_items` (tests/blitz-tests/tests/fixtures/mod.rs:64-86), moved in unchanged from `stand_snapshot_parts`; the reader count stands unless measured otherwise"
    sidecar: "existing-scopes shared modules: the fixtures/mod.rs citation re-pointed past many_items"
    rationale: "Report Symbols › fixtures/mod.rs: `pub const ITEMS` and `pub fn many_items` (64-86) moved in; Counts 10 rules the generic «shared fixture pages» rows no-change, but New text lists fixtures/mod.rs added 64-86, inside the cited 17-130."
    basis: .andromeda/architecture-detail/existing-scopes.md:133
note: occupied-resources.md:25 also omits `hl-rec` and `hl-rec-kept`, a pre-existing gap the report names at Counts 9 with no source line, not proposed; and existing-scopes.md:133's «by 24 (22 `stand_*` files …)» is unconfirmed now that `stand_act_spans` calls `fixtures::hold` with three lines added at 6-8 — measure whether it newly declares `mod fixtures;` (24 → 25, 22 → 23).
