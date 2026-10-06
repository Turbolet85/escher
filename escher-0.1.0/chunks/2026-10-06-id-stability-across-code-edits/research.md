# Codebase Research — 2026-10-06-id-stability-across-code-edits

## Scope
- **Depth:** deep on the id grammar and its pins, moderate on the stand · **Reads:** 14 · **Globs/Greps:** 12 (python sweeps over `git ls-files`; five code-graph queries, trace `tree-query-2026-10-06-id-stability-across-code-edits.json` in the run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (a `timeout N` smoke's exit 124 reads as the gate's own bound; not applicable — this chunk drives no stay-up boot) · `.claude/rules/testing.md` — 1 Session Addition (a `test_` helper prefix in `.github/scripts`; not applicable — no script test is added) · `.claude/rules/a11y.md` — 1 Session Addition (the AT-SPI bus flag; not applicable — the proof is headless)
- **External inputs (continued):** `inputs#I3` — the founder's P5 review: the actionable check is a public function of `dioxus-native-dom` (a contract addition, recorded for the wrap); Home's seven task cards are keyed in this chunk; the three non-lean tasks are measured with the same function, their counts reported and a CARRY pinned
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the founder's P1 answer: paths anchor at the nearest keyed ancestor; every element an agent can act on carries an author key, with a failing check that names the element and the remedy; v010-16's `observed_gap` is corrected before the claim
- **External inputs (continued):** `inputs#I2` — the founder's answers to the three open questions below, given at P4's fork round: the anchor acts inside one component only (a component's template root still restarts at its component chain); an element carrying an event listener counts as actionable, and CRUD's row gains an author key from the person's model id; an anchored path is spelled `key//segment`

## Files inspected
- `packages/dioxus-native-dom/src/element_id.rs` (full, 410 lines) — the one place ids are computed. `element_ids` (153-181) walks the DOM in pre-order from `place_children`; the author-key predicate is one inline chain at 170-174 (HTML `id`, non-empty, no `/`, first claim through `claimed_keys`); a child is placed from its parent's PATH even when the parent reads its key (175-176: `place_children(…, &placed.path)` runs before the key replaces the path). `place_children` (184-228) counts `(owner, tag)` per parent (206-208), takes a Dioxus-keyed root's `{tag}[{key}]` once per `(owner, segment)` (210-215), and restarts the path at the owning component's chain when the child is a template root of another owner (217-220). `component_name` (38-41) keeps only the last `::` segment of the type path. Six unit tests (268-409).
- `packages/dioxus-native-dom/src/dioxus_document.rs` (80-305) — the skeleton built outside the VirtualDom: `html`, `head`, `body` and a `main` element carrying `id="main"` (112-131); `element_id` / `element_ids` (185-208) with the grammar restated in the `element_id` doc comment (185-199); the `accessibility_tree` override (288-304) takes each `author_id` from `element_ids()` — one computation, no second edit needed for a grammar change.
- `packages/dioxus-native-dom/src/snapshot.rs` (1-195, 326-347) — `SnapshotNode.id` is the tree's `author_id` verbatim (117-130); the unit test at 326-346 compares snapshot ids with `element_ids()` and pins only `/html:0`, so it follows the grammar with no edit.
- `packages/dioxus-native-dom/src/lib.rs` (10-30) — `snapshot` and its re-exports are `#[cfg(feature = "accessibility")]`; `element_id` is a private module. Every module is private and every export is listed, so a new public item needs its own `pub use` here.
- `packages/dioxus-native/src/lib.rs` (27, by sweep) — `pub use dioxus_native_dom::*;`: a new public item of the bridge is re-exported with no edit to this crate.
- `packages/blitz-dom/src/node/node.rs` (544-552) — `Node::is_focussable()` is public: the cached focusability bit, false for a non-element.
- `packages/blitz-dom/src/accessibility.rs` (127-263, by grep) — `role_from_name` / `role_from_element_data`: a `div` maps to `Role::GenericContainer`, a `button` to `Role::Button`.
- `packages/dioxus-native-dom/src/mutation_writer.rs` (316-323, by grep) — registering a listener writes `data-dioxus-id` on the element: a present attribute means the element carries at least one Dioxus event listener.
- `examples/seven_guis/src/stand.rs` (full, 103 lines) — `boot` / `boot_timer` take a `LeanTask` only and mount through `task_in_shell`; `options(incremental)` is the public pinned-options builder (58-69); `font_ctx()` is public.
- `examples/seven_guis/src/app.rs` (1-60, 84-181) — `TaskShell` is private (153-172) and takes its task as `children`; the task component is rendered by `task_in_shell`'s owner inside `main#task-body`; Home's seven task cards are `button.task-card` with `onclick` and no `id` (135-144). Twelve `#id` rules style the Home and shell chrome (184-324, by sweep).
- `examples/seven_guis/src/tasks/crud.rs` (1-146) — the person row is a `div` with `key: "{person.id}"`, a `class` and an `onclick`, no `id` (67-78), inside `div#crud-list` (57). No task CSS selects by `#id` (sweep below).
- `tests/blitz-tests/tests/stand_element_ids.rs` (full, 271 lines) — v010-01's witness; pins `PINNED_PATHS` (22-27), the `TaskShell/{Component}/` prefix for every unkeyed element under `#task-body` (190-207), two Counter paths (224-225) and the first row's full path (231). Its `author_keys` helper (98-113) re-implements the author-key predicate.
- `tests/blitz-tests/tests/stand_id_persistence.rs` (1-70, 176-200, 296-321) — v010-02's witness; `ROW_PATH` (15) and `row_path` (58-60) spell the row id; `SKELETON` (186); the `#[ignore]` child `ids_in_a_child_process` (312-321) prints `stand-id {task} {mode} {id}` lines its parent parses by whitespace.
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` (1-60, 255-285) — the 15 controls with roles (15-44); one full row path (271).
- `tests/blitz-tests/tests/stand_snapshot.rs` (1-40, 245-300, 410-430) — rows found under `crud-list` by `id.contains("/div[")` and `ends_with("div[n]")` (286-294, 421).

## Graph impact (from the code-graph query; rust plane)
- **element_ids** — 13 call sites: `DioxusDocument::element_ids` @ `packages/dioxus-native-dom/src/dioxus_document.rs:207`, the `accessibility_tree` override @ `dioxus_document.rs:294`, `element_id` @ `element_id.rs:238`, 4 unit-test sites in `element_id.rs`, 1 in `snapshot.rs:338`, and 5 stand-check sites (`stand_accessibility_ids.rs:126`, `stand_element_ids.rs:132 · 220 · 266`, `stand_id_persistence.rs:19`). No signature changes: the grammar is internal to `element_id.rs`, so no caller needs threading.
- **element_id** — 19 call sites, all in `dioxus_document.rs`, `element_id.rs`'s tests and four stand checks. Same reading: no signature change.
- **snapshot / accessibility_tree / place_children / template_roots** — 38 rows (the trace's `rows`); every consumer of an id reads it through `element_ids` or the tree's `author_id`. `place_children` is called only from `element_ids` (`element_id.rs:163 · 175`): the anchor is one change at one call site plus the path rule inside it.
- **crate_edges for dioxus-native-dom** — inbound from `dioxus-native`, `blitz-test-harness`, `seven_guis`, `blitz-tests`, `blitz-examples`, `browser`, `wgpu_texture`; outbound to `blitz-dom`, `blitz-traits`. No new edge is needed.
- **Name check** — `unkeyed_actionable`, `UnkeyedActionable`, `actionable`, `remedy`, `slug`, `stand_id_edits`, `stand_actionable_keys`: 0 symbols (the one `anchor` hit is `TextSelection::anchor` in blitz-dom, unrelated).

## Patterns detected
- **Measured ids at HEAD** (`cargo test -p blitz-tests --locked --test stand_id_persistence -- --ignored --exact ids_in_a_child_process --nocapture`, 178 `stand-id` lines, both modes equal per task): every task shares the 11-id shell prefix `/html:0`, `/html:0/head:0`, `/html:0/body:0`, `main`, `TaskShell/style:0`, `task-shell`, `task-header`, `back-btn`, `task-title`, `task-header-spacer`, `task-body`; then Counter 5 ids (3 paths), FlightBooker 10 (5 paths), Timer 11 (5 paths), Crud 19 (12 paths). Every task-internal path starts `TaskShell/{Component}/`.
- **The CARRY's mechanism, measured** (`cargo test -p dioxus-native-dom --locked element_id`: 7 passed): the index counts same-owner, same-tag siblings of one parent in document order (`element_id.rs:206-208`), pinned by `a_duplicate_html_id_reads_a_path` (the second `div` reads `/div:1`) and `an_empty_or_slashed_html_id_reads_a_path` (`/div:0`, `/div:1`, `/div:2`); a level adds a segment (`element_id.rs:219`), pinned by `keyed_list_rows_read_their_key_segment` (`/ul:0/li[ada]`). So an earlier same-owner, same-tag sibling under the same parent raises an unkeyed element's index and every unkeyed descendant's path below it, and a wrapper adds a segment. The shift stops at a component boundary: a template root of another owner restarts at its component chain (`element_id.rs:217-220`).
- **A keyed parent does not anchor its children today** (`element_id.rs:170-176`): CRUD's rows read `TaskShell/Crud/div:0/div:1/div:0/div[n]` although their parent reads `crud-list`.
- **`main` is a keyed ancestor of everything** (`dioxus_document.rs:126-131`; measured id `main`): the skeleton's mount element reads an author key and is never rendered by a component, so a literal "nearest keyed ancestor" is never empty for a vdom-rendered element.
- **The component boundary is already an anchor** (`element_id.rs:217-220`): `TaskShell/Counter/div:0` does not depend on any DOM level above the Counter component, only on component names.
- **Stand listeners** (sweep `\bon[a-z]+:` over `app.rs` and the four lean-task files: 17 hits): 15 sit on elements with an author key; 2 do not — Home's task-card `button` (`app.rs:135`, focusable, `Role::Button`) and CRUD's person row (`crud.rs:67`, a `div`: `Role::GenericContainer`, not focusable). The 15 keyed controls are exactly the list `stand_accessibility_ids.rs:15-44` pins.
- **No id-styled task** (sweep for a line-leading `#name` selector over `examples/seven_guis`: 12 hits, all in `app.rs`'s Home and shell CSS, 0 in `tasks/`): a new `id` on a task element gains or loses no declaration.
- **No log site in the id files** (sweep `tracing::|log::|println!|eprintln!|cfg(feature = "tracing")` over `element_id.rs`, `snapshot.rs`, `dioxus_document.rs`: 0 hits).
- **No CI leg builds dioxus-native-dom with `accessibility` off by flag** (sweep `no-default-features|--features|--all-features` over `ci-leg.sh` and `ci.yml`: 2 hits, both the `wasm` leg's `seven_guis` / `todomvc` `--no-default-features --features hybrid` builds). Under the workspace's `default-features = false` pin a crate gets the feature only by naming it: `tests/blitz-tests/Cargo.toml:21` does, `examples/seven_guis/Cargo.toml` does not (0 hits for `accessibility`).
- **The `stand` selection is a glob** (`scripts/agent-run.sh:182`: `"$TESTS_DIR"/stand_*.rs`): a new `stand_*.rs` file joins `run stand` with no script change.

## Conventions to follow
- **One integration file per behaviour, `//!` head, both layout modes, fixture condition asserted first** (`tests/blitz-tests/tests/stand_element_ids.rs:1-3`, `:127`, `:130`).
- **Unit tests at the end of the source file, an app as a plain `fn() -> Element`** (`packages/dioxus-native-dom/src/element_id.rs:243-266`).
- **Stand boots use `stand::options(incremental)`** (`examples/seven_guis/src/stand.rs:58-69`); a fixture that is not a `LeanTask` goes through `Harness::from_vdom(vdom, stand::options(incremental))`, the constructor `boot_with_ticks` itself uses (`stand.rs:90-91`).
- **An edited variant keeps its component name**: two components named alike in two modules read the same chain segment (`element_id.rs:38-41`), so a before/after pair does not confound the edit with a rename.
- **Keys on rows come from the model** (`crud.rs:68`, `{person.id}`); an added row `id` is built from the same value.
- **The persistence child keeps its name and its line shape** (`stand_id_persistence.rs:312-321`; its parent runs it with `--exact`).

## New files to create
- `packages/dioxus-native-dom/src/actionable.rs` — the public actionable-key check and its unit tests
- `tests/blitz-tests/tests/stand_id_edits.rs` — the edit proof: before/after variants booted with the stand's options, ids of the elements left in place compared
- `tests/blitz-tests/tests/stand_actionable_keys.rs` — the actionable-element check over the four lean tasks, and its failing case on a fixture

## Files to modify
- `packages/dioxus-native-dom/src/element_id.rs` — the anchored path rule and its unit tests
- `packages/dioxus-native-dom/src/dioxus_document.rs` — the grammar restated in the `element_id` doc comment
- `packages/dioxus-native-dom/src/lib.rs` — the gated module and its gated re-export
- `examples/seven_guis/src/tasks/crud.rs` — an author key on each person row
- `examples/seven_guis/src/app.rs` — an author key on each Home task card and the slug field behind it
- `tests/blitz-tests/tests/stand_element_ids.rs` — the row pin and the unkeyed-path assertions re-pinned
- `tests/blitz-tests/tests/stand_id_persistence.rs` — the row pin re-pinned
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` — the row pin re-pinned
- `tests/blitz-tests/tests/stand_snapshot.rs` — the row lookups re-pinned

## Open questions
- none — the three plan-decision questions research raised (how the anchor composes with a component boundary; whether a listener-bearing element is actionable; how an anchored path is spelled) were answered by the founder at P4 (inputs#I2). The `key//segment` spelling was then checked for collisions as the founder asked: no author key holds a `/` (`element_id.rs:173`), no unanchored path holds an empty segment (`element_id.rs:103-114`, `206-215`), a document or app-root path starts with `/` (`element_id.rs:163`), and a tag cannot hold a `/` — measured on dioxus-rsx 0.7.10: `ElementName::parse` accepts only identifiers joined by `-` (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dioxus-rsx-0.7.10/src/element.rs:349-365`). No collision case found.
