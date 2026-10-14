proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "Replace the actionable-key pin «with the three tasks not yet keyed measured and pinned (Temperature Converter 2 · Circle Drawer 3 · Cells 676)» with: since 2026-10-14-stand-requirement-sweep every actionable element of all seven tasks and of Home reads an author key — `unkeyed_actionable()` 0 · 0 · 0 on Temperature Converter · Circle Drawer · Cells in both layout modes (2 · 3 · 676 until then), Circle Drawer's dialog open included — the three tasks keyed by added `id:` lines alone (`temp-celsius`, `temp-fahrenheit`, `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter`, `circle-close`, `cell-{A..Z}{0..25}` on the 676 data cells, `cell-input`), no CSS selecting by them and no other markup line moved (tests/blitz-tests/tests/stand_actionable_keys.rs:142-144; tests/blitz-tests/tests/stand_actionable_keys.rs:153-155)."
    sidecar: "§1 tests/blitz-tests: the non-lean actionable-key pin 2 · 3 · 676 → 0 · 0 · 0 (both modes, dialog open too) — the three tasks keyed by nine new author ids (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «The non-lean pin»: `unkeyed_actionable()` on the three non-lean tasks goes from 2 · 3 · 676 to 0 · 0 · 0, both modes, and 0 with the dialog open; `nonlean-pin` names test-plan.md:26 a change row. Symbols → the stand's author ids lists the nine ids."
    basis: ".andromeda/test-plan.md:26"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "Add, after the `stand_keyboard` / `focusability_updates` clause: since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` 2, a stand check `run stand` picks up by its prefix, declaring `mod common;` and `mod session_common;` — through `Session::run` alone, in both layout modes, on each lean task: the screen inside `SNAPSHOT_TEXT_BUDGET`, every node line carrying one id, pairwise distinct, each an accessibility node's `author_id`; one id-addressed acting call (`click counter-increment` · `click flight-return` · `advance 300` · `click crud-person-0`) returning settled with a non-empty diff equal to the check's own; `click task-title` an empty diff; `press tab` reading `tab_order(task)[0]` as the one focused node; two screenshots byte-equal 800 × 600 PNGs; an unread id refused `not-found`, the instance unchanged, its remedy in `Refusal::to_json` (`every_requirement_holds_through_the_driver_on_every_lean_task`) — and `WITNESSES`, the requirement-to-check record: 16 rows in requirement order, each naming at least one witness test found as `fn {test}(` through `include_str!`, the every-screen requirements naming the sweep for all four lean tasks (`every_requirement_names_its_witness`) (tests/blitz-tests/tests/stand_requirement_sweep.rs:85-211; tests/blitz-tests/tests/stand_requirement_sweep.rs:233-528; tests/blitz-tests/tests/stand_requirement_sweep.rs:578-613)."
    sidecar: "§1 tests/blitz-tests: `stand_requirement_sweep` 2 — the driver sweep on each lean task through `Session::run` and `WITNESSES`, the 16-row requirement-to-check record (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Files: `stand_requirement_sweep.rs` new (613 lines); Outcome → «Met — the driver sweep» and «Met — `WITNESSES`»; Expected amendments → test-plan §1, §3, §9 name the three new targets and `WITNESSES` as the requirement-to-check record, which no master site names yet."
    basis: "tests/blitz-tests/tests/stand_requirement_sweep.rs:233-528"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "Add, beside the sweep: since 2026-10-14-stand-requirement-sweep `stand_non_lean_tasks` 3, a stand check declaring `mod common;` and not `mod session_common;` — each in both layout modes: the elements that react to a click and state no role counted against a stated 7-row table (0 on the four lean tasks and Home, Circle Drawer 1, Cells 676 — measured and pinned, the snapshot field for it deferred, `NODE_FIELDS` still thirteen), Circle Drawer's dialog rule applying (its range input 240 wide), and the open dialog's controls each reading an author key (tests/blitz-tests/tests/stand_non_lean_tasks.rs:92-120; tests/blitz-tests/tests/stand_non_lean_tasks.rs:122-132; tests/blitz-tests/tests/stand_non_lean_tasks.rs:134-147)."
    sidecar: "§1 tests/blitz-tests: `stand_non_lean_tasks` 3 — the C3 react-without-role count pinned, the dialog rule and the open dialog's keys (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Files: `stand_non_lean_tasks.rs` new (147); Outcome → «Met — C3» (0 · 0 · 0 · 0 · 0 · Circle Drawer 1 · Cells 676, no snapshot field added), «Met — C12» (240 wide) and «Met — the non-lean keys» (the dialog's controls read their keys with it open)."
    basis: "tests/blitz-tests/tests/stand_non_lean_tasks.rs:92-120"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "Restate the per-file counts and cases: `stand_act_obstructed` 6 (5 until 2026-10-14-stand-requirement-sweep, which added a target beside a box clipped on one axis refused `covered`, the instance unchanged, both modes — tests/blitz-tests/tests/stand_act_obstructed.rs:567-612); `range_input_interaction` 8 (6 until that chunk, which added removing `value` re-seeding and removing `step` / `min` / `max` re-sanitizing, and a type change clearing the range data and a change back re-seeding — tests/blitz-tests/tests/range_input_interaction.rs:302-407); `scrolled_box_client_rect` 7 (6 until then, adding a `writing-mode: vertical-lr` box scrolled (70, 120) reading its unscrolled rect (40, 50, 300, 200), green under the per-package and the workspace build — tests/blitz-tests/tests/scrolled_box_client_rect.rs:163-185); `hit_clipped_at_scrolling_box` 13 (9 until then, adding a hit at an `img` holding injected raster data answering the `img`, a hit inside a block of an `iframe`'s `srcdoc` sub-document answering the `iframe` — the parent's walk does not enter the sub-document, pinned as measured — a hit inside a filled text `input` answering the `input`, and a box under `translate(50px, 30px)` scrolled 280 hit at its ninth row where that row shows and answering the box where its first row lies unscrolled beside the narrower row showing there; that test's fixture condition pins `BaseDocument::get_client_bounding_rect` reading the transformed box at its layout position (0, 40, 300, 140), not where the transform moves it — an engine divergence from CSSOM View, measured, not fixed, owned by a `.andromeda/residuals.md` line (target: next) — tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:432-507; tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:537-595)."
    sidecar: "§1 tests/blitz-tests: per-file counts obstructed 5 → 6, range 6 → 8, client rect 6 → 7, hit 9 → 13, with their new cases and the transformed-box client-rect reading (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «Per-file test counts» (hit 9 → 13, client rect 6 → 7, range 6 → 8, obstructed 5 → 6; `file-counts` change row test-plan.md:26); Spec claims disproved → C4, C5, C6(1), C6(2) and the client-rect-ignores-`transform` reading; Outcome → «Met — C13»; Addendum inputs#I5 places that reading's owner on a residuals line, target next."
    basis: ".andromeda/test-plan.md:26"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "Delete the qualifier «no check reads a box that is both transformed and scrolled or a scrolled box under a vertical writing mode, and a hit at an image's, a sub-document's or a text input's own box is not measured» — since 2026-10-14-stand-requirement-sweep each is read (`hit_clipped_at_scrolling_box`, `scrolled_box_client_rect`, above); what stays measured and not fixed is the `contain: paint` hit and `Node::absolute_position` of a scrolled box."
    sidecar: "§1 tests/blitz-tests: the transformed-and-scrolled, vertical-writing-mode and own-box-hit «not measured» qualifier retired — each now read (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Spec claims disproved → C4 (img / iframe / input hits measured), C6(1) (a check reads a box both transformed and scrolled) and C6(2) (a fixture of `scrolled_box_client_rect` sets a vertical `writing-mode`); Counts → «The C4 / C5 / C6 qualifiers» says all three are now measured."
    basis: ".andromeda/test-plan.md:26"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-dom (layout)"
    change: "Replace «on horizontal pages only: no check reads a scrolled box under a vertical writing mode, the path of that body the chunk also edited — … and owned by a route entry» with: on horizontal pages until 2026-10-14-stand-requirement-sweep, which added a `writing-mode: vertical-lr` box scrolled (70, 120) reading its unscrolled rect (40, 50, 300, 200), in both layout modes, green under `-p blitz-tests` and under `--workspace`, where `writing-mode` compiles `layout/writing_mode.rs`' bounds body (tests/blitz-tests/tests/scrolled_box_client_rect.rs:163-185)."
    sidecar: "§1 blitz-dom (layout): the vertical-writing-mode bounds body is read by a check — the «no check reads» gap and its route owner retired (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Spec claims disproved → C6(2): the vertical-lr scrolled box reads its rect, green under `-p blitz-tests` and `--workspace`; Outcome → «Met — C6(2)». The same claim the §1 tests/blitz-tests qualifier carries."
    basis: "tests/blitz-tests/tests/scrolled_box_client_rect.rs:163-185"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-paint"
    change: "Replace «and the caret a click places inside a password field holding a value is read by no check» with: and, since 2026-10-14-stand-requirement-sweep, the caret a click places inside a password field holding a value is read by blitz-tests' `password_caret_click` 1, an engine check on a parsed page with no `stand_` prefix, so not selected by `run stand`, declaring no shared module: in both layout modes, with the bundled font at 20 px, a click 30 px into the content box places the caret at index 5 after eight narrow characters and at 2 after eight wide ones — the click caret placed by the real characters' widths, not the bullets', measured and pinned, not fixed, the fix owned by a `.andromeda/residuals.md` line (target: next) on the founder's C11 «Замерить сейчас, исправить потом (Рекомендую)» (tests/blitz-tests/tests/password_caret_click.rs:47-95)."
    sidecar: "§1 blitz-paint: the password click caret measured by `password_caret_click` (index 5 vs 2), its fix owned by a residuals line (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Files: `password_caret_click.rs` new (95); Spec claims disproved → C11 (caret index 5 vs 2, both modes, the bundled font, 20 px); Counts → «The password caret qualifier» (now measured, owner a residuals line, card row 6, label «Замерить сейчас, исправить потом (Рекомендую)»)."
    basis: "tests/blitz-tests/tests/password_caret_click.rs:47-95"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → apps (rdme, bump, examples, accesskit_xplat)"
    change: "In `tests/cli_commands.rs`' description, after «a state directory of 94 bytes holding a session where one of 95 is refused `state-dir-too-long` with its bound», add: and, since 2026-10-14-stand-requirement-sweep, on every target but Linux (`cfg(not(target_os = \"linux\"))`) one of 90 bytes holding a session where one of 91 ends `state-dir-too-long`, status 3, its message holding no path — the two through one shared helper that asserts the message holds not the given directory, so the 94 case asserts it too; the file counts 11 on Linux and 11 on macOS, the 90 case measured on the macOS matrix leg (CI#38642493451, job 116123587286) (examples/seven_guis/tests/cli_commands.rs:547-551)."
    sidecar: "§1 apps: `cli_commands` gains the off-Linux 90/91 state-directory test, measured on the macOS leg CI#38642493451 — still 11 per OS (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «The state-directory bound off Linux» (now measured on the macOS leg, CI#38642493451, job 116123587286; `state-dir-bound` change rows test-plan.md:10 and :200); Per-file counts: `cli_commands` 11 unchanged on Linux and macOS; Deviations → `cli_commands`' helper."
    basis: ".andromeda/test-plan.md:10"
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Command line (`escher-session`)"
    change: "After «a state directory of 94 bytes as given starts and stops while one of 95 ends `state-dir-too-long`, status `3`, its message naming 94 and its remedy the table's», add: and, since 2026-10-14-stand-requirement-sweep, off Linux one of 90 bytes starts and stops while one of 91 ends `state-dir-too-long`, status `3`, with no path in the message (the shared helper asserting the given directory absent from the message for both bounds); and extend the macOS witness clause: the fork's CI run on the macOS leg is also the one witness of that 90/91 test — CI#38642493451, job 116123587286, `cli_commands` 11 passed on `aarch64-apple-darwin` (examples/seven_guis/tests/cli_commands.rs:547-551)."
    sidecar: "§6 Command line: the off-Linux 90/91 bound test and its macOS witness CI#38642493451 (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «The state-directory bound off Linux» (change row test-plan.md:200); Cross-project → the CI run the gates read: `Test (macos)` job 116123587286 ran `a_state_directory_of_90_bytes_holds_a_session_and_one_of_91_is_refused_with_its_bound ... ok` (11 passed)."
    basis: ".andromeda/test-plan.md:200"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "In session-proof: add `stand_requirement_sweep` 2 (since 2026-10-14-stand-requirement-sweep — `Session::run` alone on each lean task, both layout modes: one id per node line, an acting call's diff, an empty diff, the first Tab stop, two byte-equal shots, a `not-found` refusal, and the `WITNESSES` record); restate `stand_act_obstructed` 6 and the thirteen driver-action files' sum 48 (47 until that chunk added the one-axis `covered` test); and add to the witnesses the fork's CI run 38642493451 on `55a25638` (16 of 16 checks green, wall 886 s; the macOS job 116123587286 running `cli_commands` 11 with the off-Linux 90/91 state-directory test `ok`)."
    sidecar: "§3 Session lifecycle (session-proof): the sweep joins the proof, `stand_act_obstructed` 6 (sum 48), and CI#38642493451's macOS witness of the 90/91 bound (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Files (`stand_requirement_sweep.rs` new, holding a session through `Session::run`, a `session_common` reader per Counts → shared-module readers); Per-file counts `stand_act_obstructed` 5 → 6; Cross-project → CI#38642493451, sha 55a256384, 16/16, wall 886 s, `Test (macos)` job 116123587286. The report lists session-lifecycle.md:11 among dated file-count rows; its proof list is the contract's current statement, so the new session check and the count are owed there."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Directory pattern"
    change: "Restate: `tests/common/mod.rs` read through `mod common;` by 21 `stand_*.rs` checks (19 until 2026-10-14-stand-requirement-sweep added `stand_requirement_sweep.rs` and `stand_non_lean_tasks.rs`), `tests/session_common/mod.rs` by 21 checks (20 until that chunk added `stand_requirement_sweep.rs`, which is no `stand_act_*` file), seven of which declare both (`stand_requirement_sweep` the seventh), `password_caret_click.rs` declaring none; and the file count: 124 `.rs` files, 123 test targets and the unbuilt `all.rs` (121 and 120 until that chunk added `stand_requirement_sweep.rs`, `stand_non_lean_tasks.rs` and `password_caret_click.rs`, each its own auto-discovered target with no manifest line, `tests/blitz-tests/Cargo.toml` untouched)."
    sidecar: "§2 Directory pattern: shared-module readers 19 → 21 and 20 → 21, seven declaring both; 124 files · 123 targets with the three new auto-discovered targets (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «Shared-module readers» (`tests/common/mod.rs` 19 → 21, both new stand files declaring `mod common;`, `password_caret_click` none; `session_common` 20 → 21 by `stand_requirement_sweep`; change row test-plan.md:66); Crates / modules: the three test targets new and auto-discovered, no `[[test]]` table. 124 / 123 is 121 / 120 plus the report's three new files."
    basis: ".andromeda/test-plan.md:66"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: "Extend the reader list «… and, since 2026-10-13-stand-keyboard-harness, `stand_keyboard` — do, 19 in all» with: and, since 2026-10-14-stand-requirement-sweep, `stand_requirement_sweep` and `stand_non_lean_tasks` — do, 21 in all (`password_caret_click` declares none)."
    sidecar: "§3 stand-check helpers: `tests/common/mod.rs` readers 19 → 21 (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «Shared-module readers»: `tests/common/mod.rs` readers 19 → 21, both new stand files declaring `mod common;`. The same count the §2 Directory pattern states."
    basis: "tests/blitz-tests/tests/stand_requirement_sweep.rs:23-25"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: "Extend the reader list «… and, since 2026-10-12-headless-screenshot, `stand_screenshot_repeat`, which is no `stand_act_*` file (20 readers, …)» with: and, since 2026-10-14-stand-requirement-sweep, `stand_requirement_sweep`, also no `stand_act_*` file (21 readers; `stand_non_lean_tasks` and `password_caret_click` declare none)."
    sidecar: "§3 session-check helpers: `tests/session_common/mod.rs` readers 20 → 21 (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «Shared-module readers»: `tests/session_common/mod.rs` readers 20 → 21 (`stand_requirement_sweep`). The report's no-change for test-plan.md:101 covers its citation into the module; the row's «20 readers» is the count that moves."
    basis: ".andromeda/test-plan.md:101"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "Append: re-counted at 2026-10-14-stand-requirement-sweep, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 168 · failed 0 · ignored 7 over 40 files — +6 passed (`stand_requirement_sweep` 2 and `stand_non_lean_tasks` 3, two new files picked up by their `stand_` prefix with no script change, and `stand_act_obstructed` +1) — `password_caret_click` carries no `stand_` prefix and is not selected, and `run all` was not re-read — as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md."
    sidecar: "§3 Agent-run Proof: `run stand` re-counted 168 · 0 · 7 over 40 files (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «`run stand`» (162 · 0 · 7 over 38 files → 168 · 0 · 7 over 40, +6; `stand-run-count` change row test-plan.md:116, a re-count to append); Harness / gate surface: the two new `stand_` files picked up by prefix, no script change."
    basis: ".andromeda/test-plan.md:116"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Local baseline"
    change: "Append to the `cargo test --workspace` row: re-counted at 2026-10-14-stand-requirement-sweep: 185 result lines, 951 passed · 0 failed · 12 ignored (+14 passed, +3 result lines: blitz-tests' `stand_requirement_sweep` 2, `stand_non_lean_tasks` 3 and `password_caret_click` 1 in three new result lines; `stand_act_obstructed` +1, `hit_clipped_at_scrolling_box` +4, `scrolled_box_client_rect` +1, `range_input_interaction` +2), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md)."
    sidecar: "§9 Local baseline: workspace re-counted 185 · 951 · 0 · 12 (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «Workspace totals» (182 · 937 · 0 · 12 → 185 · 951 · 0 · 12, +3 result lines, +14 tests; `ws-totals` change row test-plan.md:330, a re-count to append); Outcome → Gates: `ci-leg.sh fast` green, `target/ci-logs/test.log` 185 · 951 · 0 · 12."
    basis: ".andromeda/test-plan.md:330"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session ↔ held instance"
    change: "Replace «one reading stays as measured: a hit still reaches content clipped by `contain: paint` beside a `visible` `overflow`, so `covered` can read where nothing shows» with: two readings stay as measured, each stated in `covered`'s fixed meaning — a hit still reaches content clipped by `contain: paint` beside a `visible` `overflow`, so `covered` can read where nothing shows; and, since 2026-10-14-stand-requirement-sweep, a box clipped on one axis stops a hit at its whole padding box, so a target lying outside it along the axis it does not clip is refused `covered` though it shows, the instance unchanged, in both layout modes (`stand_act_obstructed`'s sixth test; tests/blitz-tests/tests/stand_act_obstructed.rs:567-612; packages/escher-driver/src/refusal.rs:80-82)."
    sidecar: "§5 Session ↔ held instance: `covered`'s measured limits one → two — the one-axis clip's target refused `covered` (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Symbols → «`Cause::Covered`'s fixed meaning» (a second measured limit after the `contain: paint` one); Counts → «`covered`'s limits» one → two, `covered-limit` change row test-plan.md:181 («one reading stays as measured»); Spec claims disproved → C5 (refused `covered`, the instance unchanged)."
    basis: ".andromeda/test-plan.md:181"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: "Replace «over one CRUD flow at `info` it writes 9 stderr lines built alone and 10 built with the workspace — the tenth `INFO blitz_dom::document … message=[redacted]` —» with: over one CRUD flow at `info` it writes 9 stderr lines built alone and, since 2026-10-14-stand-requirement-sweep, 11 built with the workspace — the `blitz_dom::document` `focussed node` INFO line, `message=[redacted]`, printing twice where it printed once (10 until then) —, needles 0 under both builds (escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md)."
    sidecar: "§3 Stand log format: the `mcp` role's workspace-built CRUD flow 10 → 11 stderr lines, the engine's `focussed node` line printed twice; built alone 9 (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «The host's and the `mcp` role's log counts»: workspace build, the `focussed node` INFO line prints twice, the `mcp` role 10 → 11 lines, built alone unchanged at 9, needles 0; `mcp-lines` change rows obs-plan.md:181 and test-plan.md:103 — both must move together for test-plan §3 and obs-plan to agree."
    basis: ".andromeda/test-plan.md:103"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: "After «and the workspace-built host one engine line more, `blitz_dom::document` at INFO, with no further line at `debug` or `trace` on a snapshot · click · type flow (as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/by-level.md)», add: re-read at 2026-10-14-stand-requirement-sweep — the workspace-built host now prints that `focussed node` line twice, 6 lines / 978 B where it read 5 / 860 B, the host built alone unchanged at 4 / 742 B, needles 0 (escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md)."
    sidecar: "§3 Stand log format: the workspace-built host's engine line now printed twice, 6 lines / 978 B; built alone 4 / 742 B (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «The host's and the `mcp` role's log counts»: the host goes from 5 lines / 860 B to 6 / 978 B under the workspace build, alone 4 / 742 B; `log-counts` change row obs-plan.md:296 («one `blitz_dom::document` line») — the same claim in test-plan §3."
    basis: ".andromeda/test-plan.md:103"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§2 Test Strategy → Process-lifecycle checks"
    change: "Replace «plus, since 2026-10-10-driver-cli, one `command` span line per call it runs and, built by the workspace, one engine line» with: plus, since 2026-10-10-driver-cli, one `command` span line per call it runs and, built by the workspace, the engine's `blitz_dom::document` line — one until 2026-10-14-stand-requirement-sweep, two since (the `focussed node` line printed twice)."
    sidecar: "§2 Process-lifecycle checks: the workspace-built host's engine lines one → two (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «The host's and the `mcp` role's log counts»: the `blitz_dom::document` `focussed node` INFO line now prints twice where it printed once under the workspace build. This row states the single engine line in the present tense."
    basis: ".andromeda/test-plan.md:49"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration → Engine features by runner"
    change: "After «Measured at 2026-10-10-driver-cli for the session host: on a snapshot · click · type flow the workspace-built `escher-session` host prints one engine line, `blitz_dom::document` at INFO, … (as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/by-level.md)», add: re-read at 2026-10-14-stand-requirement-sweep, the workspace-built host prints that `focussed node` line twice (6 lines / 978 B, the `mcp` role 11 lines) and the host built alone none beyond its own (4 / 742 B, the `mcp` role 9), `host_log` and `mcp_log` green under both builds (escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md)."
    sidecar: "§9 Engine features by runner: the workspace-built host's engine line re-read, now printed twice (2026-10-14-stand-requirement-sweep)."
    rationale: "Report Changes → Counts → «The host's and the `mcp` role's log counts» (workspace 6 / 978 B and 11, alone 4 / 742 B and 9, needles 0); Outcome → «Met — C16» (`host_log` and `mcp_log` green under both builds). The row is dated, so a re-read is appended, not a rewrite."
    basis: ".andromeda/test-plan.md:324"
    dependent-of: D-tests-obs-harness
note: "No test-plan detector covers a route-owner pointer, so two are raised here and not proposed: §3 Inspection helpers says the fix of `Node::absolute_position` for a self-scrolled box is «owned by a route entry», and §9 Engine features by runner says moving the overlay scrollbar fade onto the harness clock is «owned by a `CARRY:` on the route entry 'Stand requirement sweep'». The report's P5 card moves both owners to `.andromeda/residuals.md` lines, target next: C14 «Отложить на следующую версию (Рекомендую)» and C10 «Отложить на следующую версию (Рекомендую)». D-tests-framework found no drift: the chunk runs cargo test, agent-run.sh and ci-leg.sh as §2 and §4 state. D-tests-coverage found the chunk's one code change, `Cause::Covered`'s meaning text, covered by the refusal.rs table row, json.rs's pinned sentence and the `help`-bytes checks."
