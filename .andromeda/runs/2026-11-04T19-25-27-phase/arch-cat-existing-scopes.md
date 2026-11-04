@carried n=1 label=architecture/existing-scopes fact=fct-kaaobbkf
why: The probe must read links (1) to (4) through the bridge's existing modules (dioxus_document, element_id, events) with no new crate; a probe check placed inside the bridge must sit in the `bridge_tests` child file and must update its stated count of seven unit tests.
quote: dioxus_document (with one `cfg(test)` child module, `bridge_tests`, in its own file `dioxus_document_tests.rs` — seven unit tests of the bridge (six until 2026-10-16-fast-failing-mutants, which added a created text node's mapping and stack read), declared at the end of `dioxus_document.rs`, not in `lib.rs`: packages/dioxus-native-dom/src/dioxus_document.rs:838-840), element_id (crate-private: the stable element id derivation behind `DioxusDocument::element_id` / `element_ids`), events, mutation_writer,
@carried n=2 label=architecture/existing-scopes fact=fct-dac2ih77
why: The windowed path of the MountedData hazard is read, not run, and that reading must go through dioxus-native's dependency on the bridge, the one crate where the windowed shell meets `dioxus_document.rs`'s `poll`.
quote: depends on dioxus-native-dom (packages/dioxus-native/Cargo.toml:92)
@carried n=3 label=architecture/existing-scopes fact=fct-qkp2micb
why: Every verdict must rest on a run on this stand: a lean task (Counter or CRUD) and a served task (Cells or the load page) booted by the native-only `stand` module, every boot offline except the load page's.
quote: Tasks cells, circle_drawer, counter, crud, flight_booker, temp_converter, timer (examples/seven_guis/src/tasks/mod.rs:1-7); native-only module `stand`, the headless stand booting the lean four, Cells and the load page in TaskShell, every boot offline but the load page's (examples/seven_guis/src/lib.rs:2-3; examples/seven_guis/src/stand.rs:1-5);
@carried n=4 label=architecture/existing-scopes fact=fct-4zmppehn
why: Any probe check must live in `tests/blitz-tests` beside the existing Dioxus integration and stand checks, and must not restate what they already pin (stable element ids, one driven interaction per lean task).
quote: Dioxus integration (tests/blitz-tests/tests/stale_node_mapping.rs:18-23; tests/blitz-tests/tests/harness_smoke.rs:102-113);
@carried n=4 run=2 label=architecture/existing-scopes fact=fct-4zmppehn
quote: the seven_guis headless stand — mount, bundled-font text, fresh-boot and incremental identity, offline, one driven interaction per lean task, timer ticks, one stable element id per element (tests/blitz-tests/tests/stand_boot.rs:1-3; tests/blitz-tests/tests/stand_counter.rs:1-2; tests/blitz-tests/tests/stand_flight_booker.rs:1-3; tests/blitz-tests/tests/stand_timer.rs:1-3; tests/blitz-tests/tests/stand_crud.rs:1-3; tests/blitz-tests/tests/stand_element_ids.rs:1-5)
@carried n=5 label=architecture/existing-scopes fact=fct-4zmppehn
why: The "reacts to a click" answer for link (2) must agree with these pinned per-task counts of elements that react to input (Cells 677, with `cells-grid`'s `onkeydown`), or it must name and measure the difference.
quote: the per-task count of elements that react to input and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, its 676 data cells and `cells-grid` with its `onkeydown`, measured and pinned)
@carried n=6 label=architecture/existing-scopes fact=fct-okim3jaw
why: The served task the probe runs beside the lean one must boot the way these served-page checks boot it, through `stand::boot_cells` or `stand::boot_load`, held as a session.
quote: blitz-tests' served-page checks: `stand_cells_served`, 5 tests in both layout modes — Cells booted through `stand::boot_cells` and held as a session, its screen 47,089 bytes in 740 lines walked in 5 parts of `SNAPSHOT_TEXT_BUDGET` through the driver,
@carried n=7 label=architecture/existing-scopes fact=fct-o6vmxcuj
why: A probe check that boots or holds the stand must reuse these shared non-target modules rather than restate their helpers, and if a kept probe check reads one, the counts 21, 32 and 31 must be updated at wrap.
quote: blitz-tests holds three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 21 checks, `tests/session_common/mod.rs`, by 32, and `tests/fixtures/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet`, by 31
@binding n=1
**dioxus-native-dom scope ↔ the chunk's Surfaces and "No fork-level change" boundary**: the probe must read the component, listener, handler and state links through the bridge's existing modules and the Dioxus 0.7.3 crates, read only; an "exposed with a cost" verdict must name its change inside this crate's module set, and a check kept in the bridge must move the `bridge_tests` count (architecture/existing-scopes fct-kaaobbkf); the windowed reading must go through dioxus-native's dependency on it (architecture/existing-scopes fct-dac2ih77)
@binding n=2
**seven_guis stand and served-page checks ↔ the chunk's "Measured, not read"**: each verdict must run on a lean task booted by `stand` and on a served task booted through `stand::boot_cells` or `stand::boot_load` (architecture/existing-scopes fct-qkp2micb) (architecture/existing-scopes fct-okim3jaw)
@binding n=3
**blitz-tests scope ↔ the "reacts to a click" reading (deferred-from-0.2.0 L23)**: link (2)'s `click` answer must be checked against the pinned per-task counts of elements that react to input (architecture/existing-scopes fct-4zmppehn)
@binding n=4
**blitz-tests shared modules ↔ test-plan §3**: test-plan §3 owns what each shared module holds; a probe check must reuse them, and if a probe check is kept, its reader counts recorded here must change with it (architecture/existing-scopes fct-o6vmxcuj)
