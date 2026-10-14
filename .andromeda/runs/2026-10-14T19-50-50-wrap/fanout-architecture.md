proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → CI contracts
    change: 'The `a11y` leg of `.github/scripts/ci-leg.sh` runs `cargo test --workspace --locked` over six named integration tests — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` (.github/scripts/ci-leg.sh:34; its pin `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`, .github/scripts/test_ci_workflows.py:289-290) — widened at 2026-10-14-stand-a11y-assertions on the founder''s F4 «Расширить задание a11y (Рекомендую)», with no new job, no required status check and no `ci.yml` change: the `a11y` job runs the leg script, so it now runs six files.'
    sidecar: 'CI contracts: the `a11y` leg list widened from three files to six (stand_keyboard, stand_contrast, stand_accessibility_ids added), F4, ci-leg.sh:34.'
    rationale: 'Report Changes → Schema / config: "The `a11y` CI leg list (`.github/scripts/ci-leg.sh:34`) is now … --test stand_keyboard --test stand_contrast --test stand_accessibility_ids"; Harness / gate surface: the leg runs six files; Expected amendments: "architecture §Standard Contracts → CI contracts. The leg''s list … Sites: `a11y-leg` architecture.md:141 ×2". architecture.md:141 still lists only the three files.'
    basis: .github/scripts/ci-leg.sh:34
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The `accessibility_states` clause reads "run by the workspace test leg and per package, and not by the `a11y` CI leg, which runs six named files since 2026-10-14-stand-a11y-assertions (accessibility_states not among them)" in place of "while the `a11y` CI leg stays its three named files".'
    sidecar: 'Existing Scopes (blitz-tests): the `a11y` leg no longer "stays its three named files"; it runs six.'
    rationale: 'Same retired claim as the CI contracts leg list; report `a11y-leg` sweep names architecture.md:263 among the masters, and Changes → Schema / config gives the six-file list (ci-leg.sh:34).'
    basis: .github/scripts/ci-leg.sh:34
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The stand contrast check reads: `stand_contrast`, 5 tests in both layout modes — the 18 `controls(task)` rows at boot against a stated record of pass 14 · fail 0 · exempt-inactive 3 · no-text 1 (the five former accent fails `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create` now `#ffffff` on `#3a5ce5` at 5.47, since the stand''s accent moved one step darker at 2026-10-14-stand-a11y-assertions), the list rows and the text outside the controls (`TEXT_ROWS`, 14, all pass) and the reached colour states, hover and active included (`STATE_ROWS`, 19: 18 pass, 1 exempt), the exempt class exactly `enabled == Some(false)`, and the painted background matching the measured one within 1 per channel; it writes no record and, since 2026-10-14-stand-a11y-assertions, runs in the `a11y` CI leg (tests/blitz-tests/tests/stand_contrast.rs:577-605; tests/blitz-tests/tests/stand_contrast.rs:607-635; tests/blitz-tests/tests/stand_contrast.rs:637-661) — retiring "2 tests", "pass 9 · fail 5" and "it records and gates nothing".'
    sidecar: 'Existing Scopes (blitz-tests): stand_contrast restated at 5 tests, record 14 · 0 · 3 · 1, TEXT_ROWS 14 and STATE_ROWS 19, in the a11y leg.'
    rationale: 'Report Counts → `stand_contrast`''s control record moves from pass 9 · fail 5 · exempt-inactive 3 · no-text 1 to pass 14 · fail 0 · exempt-inactive 3 · no-text 1 (`record-tally` architecture.md:263 ×3, "all 3 change"); per-file counts `stand_contrast` 2 → 5; Symbols → the three new tests and TEXT_ROWS / STATE_ROWS; Expected amendments: "Also architecture.md:263 (`record-tally` ×3, `stand-checks` ×15 …) … hits, so a detector reads it".'
    basis: tests/blitz-tests/tests/stand_contrast.rs:577-661
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The stand keyboard check reads `stand_keyboard`, 14 tests in both layout modes, adding to the boot walk: the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight stops) walked in `ENABLED_TAB_ORDER`, each state reached by keyboard alone; every `controls(task)` id reached by Tab (an empty set difference against `tab_order` and `ENABLED_TAB_ORDER` over every lean task); and Enter and Space on the enabled `crud-update` and `crud-delete` each read as its click (tests/blitz-tests/tests/stand_keyboard.rs:237-268; tests/blitz-tests/tests/stand_keyboard.rs:291-322; tests/blitz-tests/tests/stand_keyboard.rs:324-342) — retiring "11 tests"; it runs in the `a11y` CI leg.'
    sidecar: 'Existing Scopes (blitz-tests): stand_keyboard restated at 14 tests — enabled-state walks, every control reached by Tab, Enter/Space on the later-enabled CRUD buttons.'
    rationale: 'Report Counts → per-file test counts `stand_keyboard` 11 → 14 (+3); SC 2.1.1 / 2.4.3 in the enabled states; Symbols → the three new stand_keyboard tests; `stand-checks` sweep lists architecture.md:263 ×15 as coverage statements for the detectors to read.'
    basis: tests/blitz-tests/tests/stand_keyboard.rs:237-342
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The shared module `tests/common/mod.rs` is the one statement of the per-task tables `controls`, `tab_order`, `ENABLED_TAB_ORDER` (the flight booker after `flight-return`, CRUD after a row is selected), `INPUT_NAMES`, `REGION_NAMES` (`crud-list` → `People`) and `rendered` (tests/blitz-tests/tests/common/mod.rs:72-99; tests/blitz-tests/tests/common/mod.rs:133-136), since 2026-10-14-stand-a11y-assertions — adding the two new tables to the list that names `controls`, `tab_order`, `INPUT_NAMES` and `rendered`.'
    sidecar: 'Existing Scopes (blitz-tests): common/mod.rs tables gain ENABLED_TAB_ORDER and REGION_NAMES.'
    rationale: 'Report Symbols → common/mod.rs: `pub const ENABLED_TAB_ORDER` (72-99) and `pub const REGION_NAMES` (133-136) are new shared tables; Counts → `common-tables` standing at architecture.md:263, "`REGION_NAMES` and `ENABLED_TAB_ORDER` join them".'
    basis: tests/blitz-tests/tests/common/mod.rs:72-99
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Unsupported features]
    change: 'The `inert` clause''s owner reads "a fix is an engine change for every document, owned by a `.andromeda/residuals.md` line (target: next), re-homed by the founder''s F3 «Оставить, перенести (Рекомендую)» at 2026-10-14-stand-a11y-assertions, `stand_keyboard::the_engine_honours_no_inert` staying the pin and whether an inert subtree takes a hit staying unmeasured" in place of "owned by a CARRY on the route entry \"Stand a11y assertions\"".'
    sidecar: '[Unsupported features]: the `inert` gap''s owner moves from the completed route entry "Stand a11y assertions" to a residuals line (target: next), F3.'
    rationale: 'Report Counts → `aria-activedescendant` and `inert`: unchanged facts, owners moving (F3); `route-entry` sweep names architecture.md:86 — "its owner moves when the chunk completes: … the residuals lines P5 writes"; P5 card row 3 (inputs#I5) places it as a `.andromeda/residuals.md` line, `(target: next)`. Apply after P5 lands that line.'
    basis: escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Mutation, query and CSSOM
    change: 'The assistive-technology witness clause reads "and that witness — the stand''s platform adapter and a reading at an active AT — owned by a `.andromeda/residuals.md` line (target: next), re-homed by the founder''s F1 «Пока нет, перенести (Рекомендую)» at 2026-10-14-stand-a11y-assertions, whose headless halves (the tree the shell hands the adapter through `&dyn Document`, and the refresh on a Tab and an Enter) are proven on the stand" in place of "and that witness owned by a route entry".'
    sidecar: 'Mutation, query and CSSOM: the windowed AT witness''s owner moves from a route entry to a residuals line (target: next), F1.'
    rationale: 'Same retired claim (the a11y gap owned by the route entry) restated at architecture.md:122: report Counts → platform adapter not built on the stand, "only their owner moves (route: re-homed, P5)"; P5 card row 1 (inputs#I5) places the adapter and its windowed AT witness as a `.andromeda/residuals.md` line, `(target: next)`; the refresh''s two halves proven headlessly, the platform witness still owed (F1, inputs#I2). Apply after P5 lands that line.'
    basis: escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md
    dependent-of: D-arch-decisions
note: 'No new IPC method, endpoint, socket, port, env var, crate or dependency (report Symbols, Crates, Dependencies), so §Occupied Resources and §Stack need nothing. The two owner re-homes (arch :86, :122) depend on P5 writing the residuals lines, which .andromeda/residuals.md does not hold yet; architecture.md:136 (no platform adapter, `stand_accessibility_ids.rs` override fact) and the refresh mechanism rows stay, per the report''s sweep.'
