# tests · Test Data & Fixtures — category read

## Carried
- (.andromeda/test-plan.md:146) The re-asserted Tab sequences must be read against the shared tables `tab_order` and `ENABLED_TAB_ORDER` in `tests/common/mod.rs`, and a sequence 0.2.0 moved is re-recorded in those tables.
  > `tab_order(task)` (each lean task's Tab sequence, CRUD six stops with `crud-list` third, moved in from `stand_accessibility_ids`), and, since 2026-10-14-stand-a11y-assertions, `ENABLED_TAB_ORDER` (each lean task's Tab sequence in an enabled state — the flight booker after `flight-return`, six stops, and CRUD after a row is selected, eight stops, `crud-update` and `crud-delete` among them — read by `stand_keyboard`)
- (.andromeda/test-plan.md:147) Any row added for the select's keyboard model, and any table re-recorded, must stay stated expectation in the shared module, never a second mapping derived from the code.
  > the tables are stated expectation, never a second role or name mapping
- (.andromeda/test-plan.md:151) `select_interaction` and `stand_act_select`, both candidates for the widened `a11y` leg, read the shared fixtures module, so the leg must build them with it; a new fixture page follows the module's placement rule and its no-I/O rule.
  > 24 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source` and `select_interaction`
- (.andromeda/test-plan.md:151) A page that a new a11y check alone reads stays in that check's file, and the shared module stays free of I/O.
  > a page with one reader, or whose ids differ, stays in its file (`stand_act_obstructed`'s covered page, `stand_act_filled`'s and `stand_act_screenshot`'s password pages, both `restless_fixture`s); the module holds no print, `tracing` call, env read, file, socket, listener or `blitz_net`
- (.andromeda/test-plan.md:189) `select_interaction` must stay reachable through the agent harness by name (no `stand_` prefix), and the fixtures stay code that `boot` compiles, with no `run fixtures` selection.
  > nothing is seeded — the fixtures are code, the pages, the fixture boot and hold and the held network stand-in stated in `tests/blitz-tests/tests/fixtures/mod.rs` (§3 Crate-local test helpers) and the one-reader pages in their checks' own files, compiled into each target by `agent-run.sh boot` (`cargo test --no-run`), so `boot` is the bootstrap; `run stand` runs the 22 `stand_*` readers of the shared module, `run {file}` the two non-stand readers (`run font_face_sniffed_source`, which holds a load, and `run select_interaction`) and `run all` the others; no `run fixtures` selection
- (.andromeda/test-plan.md:320) A fixture page a new a11y row needs goes into the shared module only when more than one check reads it.
  > a page with one reader stays in its check's file
- (.andromeda/test-plan.md:324) Data for any new a11y row or check must be stated in code — no seeded or loaded data file.
  > test data is stated in code — inline HTML, literals and byte arrays, images injected directly, fixed viewports, the embedded fonts and pages generated in the test
- (.andromeda/test-plan.md:334) The re-read stand checks (Tab order, Enter/Space, contrast) must boot with the stand's pinned options, so their readings match the recorded tables.
  > `seven_guis::stand::options(incremental)` builds the stand's full `HarnessOptions` literal: 800×600, scale 1.0, Light, the bundled font, `net_provider: None`, `base_url: None`
- (.andromeda/test-plan-detail/test-data-fixtures.md:22) A new non-stand a11y check must state its page as inline HTML in the test file, the strategy the engine checks use.
  > | Inline HTML | inline HTML string constants or `format!`-built pages |
- (.andromeda/test-plan-detail/test-data-fixtures.md:30) Contrast and Tab-order readings must be taken at the fixed viewport the stand pins.
  > | Viewports | fixed sizes (800x600 and 400x300, scale 1.0, Light) |
- (.andromeda/test-plan-detail/test-data-fixtures.md:31) `stand_contrast`'s re-read (font size and weight decide the large-text threshold) must keep the bundled DejaVu Sans registered for every generic as its one font payload.
  > | Font payload | the embedded bullet font; the bundled DejaVu Sans woff2 `seven_guis::DEJAVU_SANS`, which the headless stand registers for every generic |

## Bindings
- **Shared Tab-order tables ↔ the plan body's a11y wiring**: `tab_order` and `ENABLED_TAB_ORDER` are the data `stand_keyboard` asserts. A re-recorded sequence moves the table and the check's reading together, and the tables stay stated expectation (.andromeda/test-plan.md:146; .andromeda/test-plan.md:147).
- **Fixtures module ↔ CI Integration (§9 a11y leg)**: widening the `a11y` leg to `select_interaction` / `stand_act_select` pulls in the targets that declare `mod fixtures;`. The fixtures stay code that `boot` compiles, and `select_interaction` stays reachable by `run {name}` (.andromeda/test-plan.md:151; .andromeda/test-plan.md:189).
- **Stand pinned options and font ↔ Mocking (§8 real font kept) and CI (§9 Fonts)**: the stand's options literal and the bundled DejaVu Sans payload are the data under the contrast and keyboard readings, and they hold on any runner (.andromeda/test-plan.md:334; .andromeda/test-plan-detail/test-data-fixtures.md:31).
- **Seeds stated in code ↔ any new a11y check**: a new row's page is inline HTML or an `rsx!` page in its own file, never seeded (.andromeda/test-plan.md:324; .andromeda/test-plan-detail/test-data-fixtures.md:22).
