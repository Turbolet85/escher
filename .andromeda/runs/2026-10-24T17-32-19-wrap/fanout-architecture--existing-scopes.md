proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: '## Existing Scopes'
    change: 'In the blitz-tests unit, the `accessibility_states` runner clause now reads: "5 tests in both layout modes, run by the workspace test leg and per package and, since 2026-10-24-stand-requirement-sweep, by the `stand-repeat` leg as one of the ten blitz-tests targets it names by `--test` after its `stand_*.rs` glob (the founder''s «Список в скрипте (Рекомендую)», 2026-10-24; .github/scripts/ci-leg.sh:54-58), and not by the `a11y` CI leg, which runs six named files since 2026-10-14-stand-a11y-assertions, this one not among them (tests/blitz-tests/tests/accessibility_states.rs:134-316)" — the rest of the line unchanged.'
    sidecar: 'existing-scopes: accessibility_states also run by stand-repeat, one of its ten named targets (2026-10-24-stand-requirement-sweep, Count 1).'
    rationale: 'The report''s Count 1 moves this row (`architecture-detail/existing-scopes.md:102`, accessibility_states "run by the workspace test leg and per package") — stand-repeat now names ten blitz-tests targets by `--test`, accessibility_states among them (Schema / config; Expected amendments, §Existing Scopes).'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:66
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 113
    section: '## Existing Scopes'
    change: 'In the blitz-tests unit, the `stand_requirement_sweep` clause now reads: "`stand_requirement_sweep`, 3 tests (2 until 2026-10-24-stand-requirement-sweep): every requirement held through `Session::run` alone on each lean task, in both layout modes (…as now…), `WITNESSES`, 16 rows in requirement order naming each 0.1.0 requirement''s witness tests, each found in its file through `include_str!`, and, since 2026-10-24-stand-requirement-sweep, a second table `WITNESSES_020` — rows v020-01…09, 46 witnesses, each with its `Subject`, `Subject::Unit` a crate''s unit test run by the workspace `test` leg and outside `stand-repeat` by construction — read by `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`, which finds each named test in its file, holds v020-01/02 to `Unit` witnesses alone, and reads from `.github/scripts/ci-leg.sh` that every other integration witness is inside `stand-repeat` by the `stand_*.rs` glob or by the `--test` words of `STAND_REPEAT_NAMED` (10) and `STAND_REPEAT_SEVEN_GUIS` (8) (tests/blitz-tests/tests/stand_requirement_sweep.rs:538-818; tests/blitz-tests/tests/stand_requirement_sweep.rs:820-844; tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069)" — the rest of the line unchanged.'
    sidecar: 'existing-scopes: stand_requirement_sweep 2 → 3 tests, WITNESSES_020 (9 rows, 46 witnesses), Subject::Unit and the stand-repeat leg check (2026-10-24-stand-requirement-sweep, Count 3).'
    rationale: 'The report''s Count 3 moves this row (`architecture-detail/existing-scopes.md:113`, "`WITNESSES`, 16 rows"): `stand_requirement_sweep` 2 → 3 with a second table `WITNESSES_020`, `Subject::Unit` and the leg check; `WITNESSES` and the 0.1.0 test byte-unchanged (Symbols / APIs; Expected amendments, §Existing Scopes).'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:28
    fact: F3
note: 'No detector fires (no new crate, target, resource or dependency); `stand_lint`, `text_extent`''s tests and `host_log`''s count are not described in this file, so Counts 4, 5, 8 leave it untouched; the fixtures reader count at :133 is a dated reading (2026-10-20) and stands; the old cites `stand_requirement_sweep.rs:90-216` and `:241-992` at :113 are pre-chunk coordinates the cites tooling should re-point.'
