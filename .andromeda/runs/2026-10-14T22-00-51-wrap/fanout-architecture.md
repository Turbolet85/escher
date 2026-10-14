proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session
    change: '`covered`''s quoted meaning reads, since 2026-10-14-stand-requirement-sweep, "another element is hit at the point the action would land; a hit reaches content clipped by `contain: paint`, so an element lying where such content extends can read `covered` though nothing shows over it" followed by a second measured limit, "a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows" — carried by `describe()` on the CLI and MCP `help` and in every `covered` refusal''s JSON line, its remedy, the nine causes and their names unchanged (packages/escher-driver/src/refusal.rs:80-82; packages/escher-driver/src/refusal.rs:22-24; packages/escher-driver/src/json.rs:679-681).'
    sidecar: 'Driver session: `covered`''s meaning gains its second measured limit (the one-axis clip), fixed table text in a ratified carrier; remedy and the nine causes unchanged.'
    rationale: 'Report Symbols / APIs → `Cause::Covered`''s fixed meaning (refusal.rs:80-82) "now carries a second measured limit after the `contain: paint` one"; Counts → `covered`''s limits go from one to two, change site architecture.md:133 @18340 "the quoted meaning".'
    basis: packages/escher-driver/src/refusal.rs:80-82
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session
    change: 'The crate docs clause reads "the crate''s docs state that an element''s `bounds` are where it stands, that a hit stops where a box clips by `overflow`, two readings as off — the `contain: paint` one and, since 2026-10-14-stand-requirement-sweep, a box clipped on one axis stopping a hit at its whole padding box, so an element beside it along its unclipped axis can read `covered` though it shows" (packages/escher-driver/src/lib.rs:68-73), in place of "one reading as off, the `contain: paint` one".'
    sidecar: 'Driver session: the crate docs now state two readings as off (contain: paint, the one-axis clip), lib.rs:68-73.'
    rationale: 'Report Symbols → `Cause::Covered`: carried in "the crate doc (`lib.rs:68-73`: \"Two readings are known to be off\")"; Counts → `covered-limit` change architecture.md:133 @19772 "the crate docs'' \"one reading as off\"".'
    basis: packages/escher-driver/src/lib.rs:68-73
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session
    change: 'Replace "the one reading still off is content clipped by `contain: paint` beside a `visible` `overflow`, which a hit still reaches" with: two readings are off — content clipped by `contain: paint` beside a `visible` `overflow`, which a hit still reaches, and, measured at 2026-10-14-stand-requirement-sweep, a target lying beside a box clipped on one axis, along the axis it does not clip, which reads in view and is refused `covered` with the instance unchanged, in both layout modes (tests/blitz-tests/tests/stand_act_obstructed.rs:567-612); `stand_act_obstructed` holds both readings.'
    sidecar: 'Driver session: the obstructed-click clause names two off readings, the one-axis clip refused `covered` measured by `stand_act_obstructed`.'
    rationale: 'Report Spec claims disproved → C5: `stand_act_obstructed::a_target_beside_a_box_clipped_on_one_axis_is_refused_covered`, both modes, "refused `covered`, the instance unchanged"; Counts → `covered-limit` change architecture.md:133 @20965 "the one reading still off".'
    basis: tests/blitz-tests/tests/stand_act_obstructed.rs:567-612
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: 'The one-axis clause reads "so a target lying outside the box along its visible axis reads in view and is not reached by a hit, and a driver `click` on it is refused `covered` with the instance unchanged — measured at 2026-10-14-stand-requirement-sweep in both layout modes and stated in `covered`''s meaning (tests/blitz-tests/tests/stand_act_obstructed.rs:567-612; packages/escher-driver/src/refusal.rs:80-82)".'
    sidecar: 'Scrolling, selection, tree: the one-axis clip''s driver reading measured — refused `covered`.'
    rationale: 'Report Counts → C4 / C5 / C6 qualifiers: architecture.md:124 reads the one-axis target "reads in view and is not reached by a hit" with no driver reading; `one-axis` row architecture.md:124 change; Spec claims disproved → C5 measured refused `covered`.'
    basis: tests/blitz-tests/tests/stand_act_obstructed.rs:567-612
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: 'Replace "and whether it stops at an image''s, a sub-document''s or a text input''s own box is not measured" with: and its stop at an element''s own box is measured since 2026-10-14-stand-requirement-sweep, in both layout modes and both builds — a hit at an `img` holding raster data answers the `img`, a hit inside a block of an `iframe`''s `srcdoc` sub-document answers the `iframe`, since the parent''s walk does not enter the sub-document (pinned as it reads), and a hit inside a filled text `input` answers the `input` (tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:432-507).'
    sidecar: 'Scrolling, selection, tree: the hit at an img / iframe / text input own box measured (C4); an iframe answers for its sub-document.'
    rationale: 'Report Spec claims disproved → C4: architecture.md:124 "whether it stops at an image''s, a sub-document''s or a text input''s own box is not measured" — now measured by `hit_clipped_at_scrolling_box`, three hits pinned; `depth-hits` row architecture.md:124 change.'
    basis: tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:432-507
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: 'Replace "no check reads a box that is both transformed and scrolled" with: a box both transformed (`translate(50px, 30px)`) and scrolled (280) is hit at the row showing where that row shows, and a point inside the box where its first row lies unscrolled, beside the narrower row showing there, answers the box, in both layout modes, since 2026-10-14-stand-requirement-sweep (tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:512-595); `BaseDocument::get_client_bounding_rect` reads that box at its layout position (0, 40, 300, 140), not where its transform moves it — a divergence from CSSOM View''s `getBoundingClientRect`, which includes transforms, pinned as the fixture''s condition and owned by a `.andromeda/residuals.md` line (target: next) on the overseer''s directive (inputs#I5).'
    sidecar: 'Scrolling, selection, tree: the transformed-and-scrolled hit measured (C6(1)); the client rect ignoring `transform` recorded, owned by a residuals line.'
    rationale: 'Report Spec claims disproved → C6(1): architecture.md:124 "no check reads a box that is both transformed and scrolled" — now `a_box_both_transformed_and_scrolled_is_hit_where_its_rows_show`; and "A reading measured beside C6(1), stated by no master": the client rect reads (0, 40, 300, 140), an engine divergence; the Addendum (inputs#I5) gives it a residuals line, target next. Apply after P5 lands that line.'
    basis: tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:512-595
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: 'Replace "each of these four readings is owned by a route entry" with: the `contain: paint` reading and `Node::absolute_position` are each owned by a `.andromeda/residuals.md` line (target: next) — widening the stop to a `contain: paint` box on the founder''s «Отложить (Рекомендую)» (C4), `hit_clipped_at_scrolling_box` and `covered`''s text standing, and fixing `Node::absolute_position` for a self-scrolled box with its seven readers on his «Отложить на следующую версию (Рекомендую)» (C14), `scrolled_box_absolute_position` its pin (inputs#I3) — while the own-box, one-axis and transformed-and-scrolled readings are measured since 2026-10-14-stand-requirement-sweep.'
    sidecar: 'Scrolling, selection, tree: the four hit readings'' route-entry owner retired — two measured, contain: paint and absolute_position moved to residuals lines (C4, C14).'
    rationale: 'Report P5 card rows 1 (C4) and 2 (C14), each a residuals line (target: next) naming the founder''s label (inputs#I3, operator''s apply inputs#I4); Outcome → "this entry''s sixteen CARRYs are each discharged here, or moved to a residuals line by the card"; C4/C5/C6 measured (Spec claims disproved). Apply after P5 lands those lines.'
    basis: escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: 'Replace "and reads Temperature Converter 2 · Circle Drawer 3 · Cells 676 on the three tasks not yet keyed" with: and, since 2026-10-14-stand-requirement-sweep, on Temperature Converter, Circle Drawer and Cells too (2 · 3 · 676 until then), Circle Drawer''s dialog open included — keyed by nine author ids added as `id:` lines alone: `temp-celsius`, `temp-fahrenheit`; `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter` and `circle-close` (the last two only while the dialog is open); `cell-{A..Z}{0..25}` on the 676 data cells through `col_label(c)` and `cell-input` (examples/seven_guis/src/tasks/temp_converter.rs:13; examples/seven_guis/src/tasks/temp_converter.rs:26; examples/seven_guis/src/tasks/cells.rs:375; examples/seven_guis/src/tasks/cells.rs:384; tests/blitz-tests/tests/stand_actionable_keys.rs:142-144; tests/blitz-tests/tests/stand_actionable_keys.rs:153-155; tests/blitz-tests/tests/stand_non_lean_tasks.rs:134-147).'
    sidecar: 'Dioxus DOM bridge: `unkeyed_actionable()` reads 0 on all seven stand tasks; nine non-lean author ids named.'
    rationale: 'Report Counts → the non-lean pin: `unkeyed_actionable()` on Temperature Converter · Circle Drawer · Cells goes from 2 · 3 · 676 to 0 · 0 · 0, both modes, dialog open too; `nonlean-pin` change architecture.md:136; Symbols → the stand''s author ids, nine new.'
    basis: tests/blitz-tests/tests/stand_non_lean_tasks.rs:134-147
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Paint and shell
    change: 'Replace "— not measured by any check, owned by a `CARRY:` on the route entry \"Stand requirement sweep\"" with: — measured since 2026-10-14-stand-requirement-sweep by `password_caret_click` (both layout modes, the bundled font at 20 px): a click 30 px into the content box places the caret at index 5 after eight narrow characters and at index 2 after eight wide ones, a known 0.1.0 limit whose fix — placing a click caret by the bullet layout — is owned by a `.andromeda/residuals.md` line (target: next) on the founder''s «Замерить сейчас, исправить потом (Рекомендую)» (C11, inputs#I3) (tests/blitz-tests/tests/password_caret_click.rs:47-95).'
    sidecar: 'Paint and shell: the password click-caret dependence measured (index 5 vs 2), a known 0.1.0 limit; its fix owned by a residuals line (C11).'
    rationale: 'Report Spec claims disproved → C11: architecture.md:129 "not measured by any check" — now measured, 5 vs 2; Counts → `pw-caret` change architecture.md:129, owner moving to a residuals line (P5 card row 6); Addendum (inputs#I5) states it as a known 0.1.0 limit. Apply after P5 lands that line.'
    basis: tests/blitz-tests/tests/password_caret_click.rs:47-95
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'Add three test targets, each auto-discovered (no `[[test]]` table): since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep`, 2 tests in both layout modes — every requirement held through `Session::run` alone on each lean task (the screen under `SNAPSHOT_TEXT_BUDGET`, one distinct id per node line, each an `author_id`, one id-addressed acting call settled with its diff, an empty diff for `click task-title`, `press tab` focusing the first Tab stop, two byte-equal 800 × 600 shots, an unread id refused `not-found`), and `WITNESSES`, 16 rows in requirement order naming each requirement''s witness test, read for existence (tests/blitz-tests/tests/stand_requirement_sweep.rs:85-211; tests/blitz-tests/tests/stand_requirement_sweep.rs:233-613); `stand_non_lean_tasks`, 3 tests — the per-task count of elements that react to a click and state no role (0 on the four lean tasks and Home and Temperature Converter, Circle Drawer 1, Cells 676), the Circle Drawer dialog rule, and its open dialog read keyed (tests/blitz-tests/tests/stand_non_lean_tasks.rs:92-147); and `password_caret_click`, 1 test, a password''s click-placed caret pinned at index 5 vs 2 (tests/blitz-tests/tests/password_caret_click.rs:47-95).'
    sidecar: 'Existing Scopes (blitz-tests): three new targets — stand_requirement_sweep (2, WITNESSES 16 rows), stand_non_lean_tasks (3), password_caret_click (1).'
    rationale: 'Report Changes → Files: three new test files; Crates / modules: "The three test targets are new and auto-discovered"; Symbols → new test helpers incl. `WITNESSES` (16 rows); Counts → workspace totals +14 (the sweep 2 · non-lean 3 · password caret 1); Outcome → C3 measured 0 · 0 · 0 · 0 · 0 · Circle Drawer 1 · Cells 676; Expected amendments: `session-checks` architecture.md:263, where `stand_requirement_sweep` joins.'
    basis: tests/blitz-tests/tests/stand_requirement_sweep.rs:85-211
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The range input clause reads `range_input_interaction`, 8 tests (6 until 2026-10-14-stand-requirement-sweep), adding: removing `value` re-seeds the value and removing `step`, `min` or `max` re-sanitizes it, and a type change clears the range data and a change back re-seeds it, both layout modes (tests/blitz-tests/tests/range_input_interaction.rs:302-407).'
    sidecar: 'Existing Scopes (blitz-tests): range_input_interaction 6 → 8 tests (attribute removals; the type change).'
    rationale: 'Report Counts → per-file test counts `range_input_interaction` 6 → 8; `file-counts` change architecture.md:263 (range 6); Outcome → C13 met.'
    basis: tests/blitz-tests/tests/range_input_interaction.rs:302-407
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The scrolled-box and hit clause reads `scrolled_box_client_rect`, 7 tests (adding, since 2026-10-14-stand-requirement-sweep, a `writing-mode: vertical-lr` box scrolled (70, 120) reading its unscrolled rect, green under both builds) and `hit_clipped_at_scrolling_box`, 13 tests (adding the hits at an `img`, inside an `iframe`''s sub-document and inside a filled text `input`, and a box both transformed and scrolled hit where its rows show) (tests/blitz-tests/tests/scrolled_box_client_rect.rs:163-185; tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:432-595), in place of 6 and 9.'
    sidecar: 'Existing Scopes (blitz-tests): scrolled_box_client_rect 6 → 7 (vertical-lr), hit_clipped_at_scrolling_box 9 → 13 (own boxes, transformed-and-scrolled).'
    rationale: 'Report Counts → per-file test counts `hit_clipped_at_scrolling_box` 9 → 13, `scrolled_box_client_rect` 6 → 7; `file-counts` change architecture.md:263 (hit 9, client rect 6); Spec claims disproved → C4, C6(1), C6(2).'
    basis: tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:432-595
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The `tests/common/mod.rs` clause reads "read through `mod common;` by 21 stand checks (`stand_contrast` the eighteenth, `stand_keyboard` the nineteenth, and `stand_requirement_sweep` and `stand_non_lean_tasks` the twentieth and twenty-first, since 2026-10-14-stand-requirement-sweep)" in place of "by 19 stand checks".'
    sidecar: 'Existing Scopes (blitz-tests): tests/common/mod.rs readers 19 → 21.'
    rationale: 'Report Counts → shared-module readers: `tests/common/mod.rs` readers go from 19 to 21 (both new stand files declare `mod common;`, `password_caret_click` none); `common-readers` change architecture.md:263 ("by 19 stand checks … the nineteenth").'
    basis: escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The `tests/session_common/mod.rs` clause adds "and, since 2026-10-14-stand-requirement-sweep, by `stand_requirement_sweep`, twenty-one readers" in place of "twenty readers (measured at 2026-10-12-headless-screenshot, where eighteen stood)".'
    sidecar: 'Existing Scopes (blitz-tests): tests/session_common/mod.rs readers 20 → 21 (stand_requirement_sweep).'
    rationale: 'Report Counts → shared-module readers: `tests/session_common/mod.rs` readers go from 20 to 21 (`stand_requirement_sweep`); `common-readers` change rows include architecture.md:263.'
    basis: tests/blitz-tests/tests/stand_requirement_sweep.rs:23-25
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Tests
    change: 'The list of checks that hold a `Session` and read `tests/blitz-tests/tests/session_common/mod.rs` adds "and, since 2026-10-14-stand-requirement-sweep, the driver sweep `stand_requirement_sweep`" after `stand_screenshot_repeat`.'
    sidecar: 'Conventions (Tests): the session_common reader list names stand_requirement_sweep.'
    rationale: 'Same retired reader set as the Existing Scopes session_common clause; report Counts → shared-module readers names architecture.md:115 "(the `session_common` reader list)" among the change rows.'
    basis: tests/blitz-tests/tests/stand_requirement_sweep.rs:23-25
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: 'The driver-action clause''s case list adds, after "a covered and an out-of-view target refused on in-file fixtures": "and, since 2026-10-14-stand-requirement-sweep, a target beside a box clipped on one axis refused `covered`, the instance unchanged (tests/blitz-tests/tests/stand_act_obstructed.rs:567-612)" — `stand_act_obstructed` 5 → 6 tests, the stated total of the twelve listed `stand_act_*` files moving by that one test.'
    sidecar: 'Existing Scopes (blitz-tests): stand_act_obstructed gains the one-axis case (5 → 6 tests).'
    rationale: 'Report Counts → per-file test counts `stand_act_obstructed` 5 → 6; `run stand` +6 includes `stand_act_obstructed` +1; Spec claims disproved → C5.'
    basis: tests/blitz-tests/tests/stand_act_obstructed.rs:567-612
note: 'No new IPC method, endpoint, socket, port, env var, crate or dependency (report Symbols "No public API, no wire field, no verb, no node key, no log field and no span changed", Crates none, Dependencies none), so §Occupied Resources and §Stack need nothing; the nine author ids are stand fixtures, not registry resources. The `stand_act_*` aggregate "41 tests" at architecture.md:263 is not re-counted by the report — re-count it before applying the last proposal. The client-rect-ignores-`transform` reading also reaches the snapshot `bounds` (Dioxus DOM bridge, which reads `get_client_bounding_rect`); no proposal states it there beyond the Scrolling contract. The owner re-homes (C4, C14, C11, the transform rect) depend on P5 writing their residuals lines. architecture.md:87 and the bound rows at :133 are no-change per the report.'
