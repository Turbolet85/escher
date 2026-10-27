# layouts extract

## Relevance
partial. The chunk refactors code and creates or changes no surface, region, component or focus stop. Two measured layout readings depend on what it refactors: the stand's lint readings, which `lint.rs::findings` produces, and the `escher-session mcp` tool listing, which `mcp_tests.rs` asserts.

## Constraints
- The stand's layout as the visual lint reads it must stay as the plan records it (per layout-templates §Surface: desktop-native → IA notes). At boot the counter, timer, CRUD and the load page read 0 findings. The flight booker reads one `overlap`. After twelve Creates, the last CRUD row reads `clipped` inside `crud-list`. Refactoring `findings` must leave each of these readings unchanged in both layout modes, through the verb and through the reader alike. Whether the refactored walk keeps them is research's and the tests' question.
- The flight booker's `overlap` (`flight-type-one-way` × `flight-type-return`) is the snapshot giving each option its select's client rect. It is not a layout defect, and a `.andromeda/residuals.md` line owns it (per layout-templates §Surface: desktop-native → IA notes). The factoring must keep it, neither suppressing nor "fixing" it.
- The `clipped` reading's reason numbers must stay identical: the visible part of `crud-person-14` inside `crud-list`, which agrees with the driver's `off-screen` refusal of its `click` (per layout-templates §Surface: desktop-native → IA notes). The lint's `crud-list` clip reading depends on the list being the nested scroller that clips its rows and on `task-body` not being that box (per layout-templates §Surface: desktop-native → IA notes).
- `escher-session mcp` lists the twelve verbs of the verb table as its tools in order, `lint` after `screenshot` and `help` last. Each tool's description is the verb's help followed by `. Returns: ` and its result fields (per layout-templates §Surface: cli → Primary screens). The factored `the_listing_is_the_twelve_verbs_of_the_table` must still assert exactly that order and that description form.
- The `lint` verb's answer stays one uncoloured line `{"findings":[…]}`. Each finding is keyed `id` · `defect` · `bounds`, then its reason's keys, and holds ids, closed words and numbers only (per layout-templates §Surface: cli → Primary screens). Moving the findings walk into helpers or a table must not change the key order or the finding order.
- The stand's served screens keep their markup, author ids, classes, styles and order: TaskShell mount, the lean four, Cells and the load page (per layout-templates §Surface: desktop-native → Primary screens). This chunk touches no `examples/seven_guis/src/tasks/` markup.

## Patterns to follow
- Lint readings are pinned in both layout modes, and through the verb as through the in-process reader, at boot and after a scripted step such as twelve Creates (per layout-templates §Surface: desktop-native → IA notes). A table walk in `findings` should be checked against both paths.
- A held instance reads as a fresh boot, and the TaskShell mount and pinned 800 × 600 viewport hold after commands (per layout-templates §Surface: desktop-native → Primary screens). The lint's per-node walk reads geometry at that pinned viewport, and the refactor relies on it unchanged.
- The tool listing and `help` read the same with and without `--record` (per layout-templates §Surface: cli → Primary screens). A table-driven listing assertion can share one expected table across both.

## Anti-patterns to avoid
- Moving any stand markup, id, class or style, or adding a wrapper, to change a lint reading. Each measured reading in the plan states that no stand markup, id, class or style moved (per layout-templates §Surface: desktop-native → IA notes).
- Reordering the tool listing or changing the description form while turning the listing test into a table walk (per layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts §Surface: desktop-native → IA notes (stand lint readings) ↔ test-plan stand checks: `tests/blitz-tests/tests/stand_lint.rs` and `tests/blitz-tests/tests/stand_load_page.rs` pin the readings the refactored `findings` must keep.
- layouts §Surface: desktop-native → IA notes (select-option `overlap`) ↔ the `.andromeda/residuals.md` line that owns the lint's reading of a select's options.
- layouts §Surface: cli → Primary screens (`mcp` tool listing order and description form) ↔ escher-driver's schema `VERBS` table (`packages/escher-driver/src/schema.rs`, `packages/escher-driver/src/mcp/listing.rs`), the source the listing test walks.

## Acceptance criteria contributions
- (layouts) `stand_lint` passes unchanged in both layout modes on the refactored `findings`. At boot, counter/timer/CRUD/load read 0 findings and the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return`). After twelve Creates, `crud-person-14` reads `clipped` inside `crud-list` with the same visible-part numbers (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) The refactored `the_listing_is_the_twelve_verbs_of_the_table` still asserts twelve tools in `VERBS` order with `lint` after `screenshot` and `help` last, each described by its help, `. Returns: ` and its result fields. Dropping or reordering a row turns it red (per layout-templates §Surface: cli → Primary screens).
- (layouts) The chunk's diff touches no file under `examples/seven_guis/src/tasks/` and no stand CSS, so no served screen's markup, id, class, style or order moves (per layout-templates §Surface: desktop-native → Primary screens).
