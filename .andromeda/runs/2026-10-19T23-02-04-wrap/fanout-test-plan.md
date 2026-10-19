proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "blitz-tests holds three shared modules that are no target — `tests/common/mod.rs` (the stand checks' tables and helpers, each stated once), `tests/session_common/mod.rs` (what the checks that hold a session share) and, since 2026-10-19-shared-fixtures, `tests/fixtures/mod.rs` (the fixture boot and hold, three fixture pages and the held network stand-in `HeldNet`, each stated once, read by 19 checks) (§3 Crate-local test helpers) — citing tests/blitz-tests/tests/fixtures/mod.rs:17-107 beside the two existing module citations"
    sidecar: "test-plan §1: blitz-tests' shared modules that are no target 2 → 3, `fixtures/mod.rs` added (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Counts «shared modules that are no target: 2 → 3 (`common`, `session_common`, `fixtures`)», dispositions «`test-plan.md:27` (@c116–466, «two shared modules that are no target») stale»; Expected amendments «test-plan §1 — blitz-tests holds three shared modules that are no target». A new test-only module the plan's scope summary does not state is a coverage-record gap."
    basis: "test-plan.md:27; tests/blitz-tests/tests/fixtures/mod.rs:17-107"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "`tests/blitz-tests` holds three shared modules that are no target — `common` (21 readers) and `session_common` (25 readers) as now, and since 2026-10-19-shared-fixtures `tests/fixtures/mod.rs`, read through `mod fixtures;` by 19 checks: the 18 `stand_*` files `stand_settle`, `stand_boot`, `stand_act_diff`, `stand_diff`, `stand_snapshot_text`, `stand_mcp_masked`, `stand_session_recording`, `stand_act_spans`, `stand_mcp_tools`, `stand_act_filled`, `stand_act_screenshot`, `stand_act_obstructed`, `stand_act_scroll`, `stand_act_transformed`, `stand_act_timer`, `stand_replay`, `stand_keyboard`, `stand_snapshot_state`, and `font_face_sniffed_source` — and `tests/all.rs` becomes the fourth file that is no built target (was «a third file»); the `declare none` notes about `mod common;` and `mod session_common;` stay as written"
    sidecar: "test-plan §2 Directory pattern: blitz-tests' shared modules 2 → 3, `fixtures` with its 19 readers; `tests/all.rs` the fourth file that is no built target (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Counts «`test-plan.md:68` (@c913–2325, «two shared modules that are no target» with their reader counts) stale», «Readers of the new module: 0 → 19 … it lands where the third module is stated», «`common` stays at 21 readers and `session_common` at 25»; Symbols / APIs «19 files declare `mod fixtures;` (the nineteen modified files)»; declnone at :68 @c1974 no-change."
    basis: "test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers"
    change: "add a third blitz-tests bullet after `blitz-tests (session checks)`: «blitz-tests (fixtures): `tests/fixtures/mod.rs` (since 2026-10-19-shared-fixtures, `#![allow(dead_code)]`, no test in it and no target of its own) is the one statement of the fixture boot and hold and the fixtures several checks share — `boot(app, incremental)`, a `Harness<DioxusDocument>` from `VirtualDom::new(app)` under `stand::options(incremental)` (tests/blitz-tests/tests/fixtures/mod.rs:17-20); `hold(label, app, incremental)`, a `Session::start(label, …)` over that boot, a refused label panicking (tests/blitz-tests/tests/fixtures/mod.rs:22-25); the pages `password_labelled`, `password_named` and `covered` (§7); and `HeldNet`, the held network stand-in (§8) (tests/blitz-tests/tests/fixtures/mod.rs:64-107); read through `mod fixtures;` by 19 checks — 18 `stand_*` files and `font_face_sniffed_source`; the MCP boot closures of `stand_mcp_masked` and `stand_mcp_tools` keep `Session::start(app, …)` with its `Result` (the label is the MCP caller's) and build through `fixtures::boot`; a page with one reader, or whose ids differ, stays in its file (`stand_act_obstructed`'s covered page, `stand_act_filled`'s and `stand_act_screenshot`'s password pages, both `restless_fixture`s); no print, `tracing` call, env read, file, socket, listener or `blitz_net`»"
    sidecar: "test-plan §3 Crate-local test helpers: `blitz-tests (fixtures)` bullet added — boot, hold, three pages, `HeldNet`, 19 readers (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Counts «`test-plan.md:102`/`:103` (§3 Crate-local test helpers: `common` and `session_common` described, no `fixtures` entry)» stale; Symbols / APIs (boot :17-20, hold :22-25, pages :27-62, HeldNet :64-107; remaining callers kept; single-reader pages stay); Deviation 1 (the MCP boot closures); Expected amendments «test-plan §3 Crate-local test helpers — `fixtures/mod.rs` and what it states». Lines :102 and :103 themselves stay true of `common` and `session_common`; the drift is the missing entry, so one addition covers both."
    basis: "test-plan.md:102-103; tests/blitz-tests/tests/fixtures/mod.rs:17-107"
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract (the NOT YET MEASURED test-data bootstrap line)"
    change: "replace «NOT YET MEASURED — the test-data bootstrap mechanism of the 5-command contract: no reading gathered one» with the measured reading: no data is seeded — the fixtures are code compiled into each target, built by `agent-run.sh boot` (`cargo test -p blitz-tests --locked --no-run`), so `boot` is the bootstrap; `run stand` runs the 18 fixture-reading `stand_*` checks and `run font_face_sniffed_source` (or `run all`) the one non-stand reader, which holds a load; no `run fixtures` selection (the operator's «Existing selections (Recommended)») — boot `ready`, `run stand` 178 · 0 · 7, `run font_face_sniffed_source` 2 · 0 · 0, as measured at escher-0.2.0/chunks/2026-10-19-shared-fixtures/report.md"
    sidecar: "test-plan §3: the agent-run test-data bootstrap NOT YET MEASURED → measured — fixtures compiled into each target by `boot`, nothing seeded (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Counts «The test-data bootstrap of the agent-run contract: NOT YET MEASURED → measured — pattern `bootstrap`: standing 1 (`test-plan.md:131` …) → stale» with its reading; Harness / gate surface (agent-run unchanged, no `run fixtures` selection, the 18 + 1 readers); Outcome gates `agent-run.sh boot` ready, `run stand` 178 · 0 · 7, `run font_face_sniffed_source` 2 · 0 · 0."
    basis: "test-plan.md:131"
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures → Fixture sources"
    change: "add a bullet: «tests/blitz-tests/tests/fixtures: since 2026-10-19-shared-fixtures the fixture pages two or more checks read, stated once as rsx! components — `password_labelled` (`fx-root` holding the `<label>` `Passphrase` for the password input `fx-secret`; tests/blitz-tests/tests/fixtures/mod.rs:27-35), `password_named` (`fx-root` holding `fx-secret`, a password input named `Secret` by `aria-label`; tests/blitz-tests/tests/fixtures/mod.rs:37-44) and `covered` (`fx-under` under the `z-index: 5` overlay `fx-cover`; tests/blitz-tests/tests/fixtures/mod.rs:46-62), booted with the stand's options through `fixtures::boot`; single-reader pages stay in their files»"
    sidecar: "test-plan §7 Fixture sources: the shared fixture pages `password_labelled`, `password_named`, `covered` in `tests/fixtures/mod.rs` (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Symbols / APIs «three pages: `password_labelled` (`:27-35` …), `password_named` (`:37-44` …), `covered` (`:46-62` …)» and «Pages with one reader, or whose ids differ, stay in their files»; Expected amendments «test-plan §7 Test Data & Fixtures … the shared pages»."
    basis: "tests/blitz-tests/tests/fixtures/mod.rs:27-62"
  - detector: D-tests-coverage
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Hand-written fakes and stubs (Network rows)"
    change: "the `ManualNetProvider` row names upstream's file-private one alone — `render_blocking_stylesheet`'s (its two citations kept, `stand_settle.rs:182-214` dropped) — and a new Network row follows it: «`HeldNet` (tests/blitz-tests/tests/fixtures/mod.rs:64-107), escher's one held network stand-in since 2026-10-19-shared-fixtures, in place of `stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider` and `stand_boot`'s `RecordingNetProvider`: `fetch` keeps each request's url and handler, `held()` and `urls()` (request order) read them, and `answer(body)` answers the last held request through `NetHandler::bytes` and returns its url; it does not override `is_noop`, so the harness counts its loads (`stand_settle` reads `NotSettled { busy: Loads }` through it, answering a head stylesheet and then its `@import` on `.test` URLs with no real fetch)»; upstream's `RecordingNetProvider` row (link_rel_attribute.rs:14-24) unchanged"
    sidecar: "test-plan §8 Network: escher's three file-private held network stand-ins → one shared `fixtures::HeldNet`; the `ManualNetProvider` row now upstream's alone (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Counts «escher's held network stand-ins: 3 file-private copies → 1 shared (`HeldNet`) … `test-plan.md:278` (the §8 Network row: «two file-private ones: `render_blocking_stylesheet`'s, and `stand_settle`'s», citing `stand_settle.rs:182-214`, lines that no longer hold the stand-in) → stale; `test-plan.md:277` … → no-change»; Symbols / APIs (HeldNet :64-107, its readers and writer, no `is_noop` override); Expected amendments «§8 Hand-written fakes and stubs (Network rows)»."
    basis: "test-plan.md:277-278; tests/blitz-tests/tests/fixtures/mod.rs:64-107"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-dom covered behaviours"
    change: "`font_face_sniffed_source`: «every request held by a hand `HeldNetProvider`» → «every request held by the shared stand-in `fixtures::HeldNet` (§8)»; its file citation re-pointed by the citation sweep"
    sidecar: "test-plan §1: `font_face_sniffed_source`'s stand-in renamed `HeldNetProvider` → the shared `fixtures::HeldNet` (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Counts heldnet «`test-plan.md:12` («every request held by a hand `HeldNetProvider`») → stale (the name)»; Symbols / APIs «Removed file-private symbols … `font_face_sniffed_source`'s `HeldNetProvider`»."
    basis: "test-plan.md:12"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests (`stand_snapshot_text`'s masked reading)"
    change: "«on two in-file fixtures booted with the stand's options, a typed password and a file input's authored path» → «on a shared `fixtures` password page and an in-file file-input page, both booted with the stand's options, a typed password and a file input's authored path»"
    sidecar: "test-plan §1: `stand_snapshot_text`'s password page is the shared `fixtures` one, no longer in-file (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Symbols / APIs «Removed file-private symbols … `password_fixture` in … `stand_snapshot_text` …; `boot_fixture` in `stand_snapshot_text`»; Deviation 4 «`stand_snapshot_text`'s page gained the root id `fx-root` … so the file's own page was not kept». The passage restates the retired claim that the page is file-private."
    basis: "test-plan.md:27"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests (`stand_diff`'s fixture steps)"
    change: "«on three in-file fixtures booted with the stand's options, an element hidden and shown …, a typed password named once …, and a rewritten label …» → «on fixtures booted with the stand's options — two in-file (the hidden-and-shown element, the rewritten label) and the shared `fixtures` password page (the typed password named once and only by the fixed mask)»"
    sidecar: "test-plan §1: `stand_diff`'s password page is the shared `fixtures` one, two of its three fixtures still in-file (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Symbols / APIs «`password_fixture` in `stand_act_diff`, `stand_diff`, …» removed; «`stand_diff`'s `Screen::fixture` keeps its name and signature (body `Self::of(fixtures::boot(fixture, incremental))`)». The passage restates the retired file-private page."
    basis: "test-plan.md:27"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests (`stand_mcp_tools`' causes)"
    change: "«`covered` read on an in-file overlay fixture» → «`covered` read on the shared overlay page `fixtures::covered`»"
    sidecar: "test-plan §1: `stand_mcp_tools`' `covered` is read on the shared `fixtures::covered` page (2026-10-19-shared-fixtures)"
    rationale: "Report Changes → Symbols / APIs «`covered_fixture` in `stand_act_spans`, `stand_mcp_tools`» removed, the page `covered` (`:46-62`) stated once; Deviation 1 (`stand_mcp_tools`' boot closure builds through `fixtures::boot(fixtures::covered, …)`). The passage restates the retired file-private page."
    basis: "test-plan.md:27; tests/blitz-tests/tests/fixtures/mod.rs:46-62"
    dependent-of: D-tests-coverage
note: "D-tests-framework and D-tests-obs-harness otherwise hold (runner, agent-run script, status shape and log format unchanged; Harness / gate surface none); no Retired item. The three §1 :27 in-file-fixture occurrences (stand_snapshot_text, stand_diff, stand_mcp_tools) are not in the report's sweep classes — found by reading for the moved file-private pages; the other :27 «in-file fixture» passages (stand_snapshot_state, stand_settle, stand_act_obstructed, stand_keyboard, stand_act_spans' restless_fixture) still hold, their pages left in-file."
