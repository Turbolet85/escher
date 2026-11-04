@task_prose part=session
The session added one test target, tests/blitz-tests/tests/dioxus_runtime_probe.rs (10 tests, both layout modes), that reads through public Dioxus 0.7.10 API what each node's component, listeners, written state and readers are; no crate, symbol, package, example or Cargo.toml changed.
It measured the MountedData hazard at HEAD: a task spawned from a key handler that wrote a signal and calls set_focus panics RefCell already borrowed (DioxusDocument::poll holds the borrow across render_immediate); the same set_focus called at once in the handler moves focus. The fix is routed to a later port chunk, not made here.
It measured that the bridge's listener attributes beside the data-dioxus-id mark over-report (a removed listener's attribute stays; a mounted listener writes none), while the vdom walk's dynamic attributes read current listeners exactly; no snapshot field was added (NODE_FIELDS thirteen).
An operator fix commit bumped pulldown-cmark 0.13.5 to 0.13.6 in Cargo.lock (RUSTSEC-2026-0433, reached only through rdme) and added RUSTSEC-2026-0429 (zip 2.4.2) to the deny.toml ignore list, now eight IDs.
Counts moved: blitz-tests 158 .rs files and 157 test targets; run all 743 passed, 0 failed, 12 ignored; run stand unmoved at 237 over 51 files. Nothing retired.
@task_file n=1 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_item n=1 part=touches
Motion's claim that a programmatic into-view scroll (MountedData scroll_to, called synchronously) writes nested scrolling boxes at once: re-read against the MountedData hazard and found unmoved; the panic is measured only for set_focus from a spawned task, and a synchronous MountedData call acts.
@task_item n=2 part=touches
Any design-system text naming MountedData focus or scroll calls from Dioxus handlers: the hazard is now measured at packages/dioxus-native-dom/src/dioxus_document.rs:295 and :297, events.rs:295 (set_focus) and events.rs:162 (doc_mut); get_scroll_offset and get_scroll_size borrow shared through doc at events.rs:152, read and not run.
@task_item n=3 part=touches
Any count of blitz-tests files or targets carried in an example-fixture or test-inventory line: now 158 .rs files and 157 test targets (was 157 and 156).
@task_item n=4 part=touches
Any run all count carried in a fixture or test line: now 743 passed, 0 failed, 12 ignored (was 733); run stand unmoved at 237 over 51 files.
@task_item n=5 part=touches
Text on rdme (the readme app styling GitHub markdown and octicons, apps/readme/assets/github-markdown.css): its markdown parser pulldown-cmark resolves 0.13.6 now (was 0.13.5); no rdme source, asset or option changed, ENABLE_WIKILINKS never set.
@task_item n=6 part=touches
Desktop-native surface text on Dioxus apps opening through dioxus_native::launch: Dioxus stays at the 0.7.3 requirement (dioxus-core and dioxus-signals 0.7.10 in the lock); re-read, unmoved.
@task_item n=7 part=touches
Any claim about the data-dioxus-id listener mark or a node's listener attributes marking an element as reacting to click: the mark now measured to over-include an element whose listener was removed; on Cells 677 marked elements stating no role are 676 cells reacting to click and cells-grid to keydown; on Counter counter-increment carries click and counter-value none.
@task_item n=8 part=touches
Motion's harness lines (VirtualClock, settle, click, type and press leaving the clock): the probe drives Harness from_vdom, press, click, pump and focused unchanged; re-read, unmoved.
@task_item n=9 part=touches
The browser capture feature (mobile-native capture, apps/browser): its anyrender_serialize 0.8.0 dependency pulls zip 2.4.2, whose RUSTSEC-2026-0429 is now ignored by ID beside three others; deserialize reads entries by_name and never calls extract; no capture code changed.
@task_item n=10 part=touches
Any line citation into Cargo.lock or deny.toml: Cargo.lock lines 6505 and 6507 changed (pulldown-cmark version and checksum); deny.toml gained line 26, the ignore list at deny.toml:20-30.
@task_item n=1 part=untouched
Color palette, core colors, surface scale, text hierarchy, semantic colors, border progression and the SelectListView engine constants: no colour, token, stylesheet or engine paint code changed.
@task_item n=2 part=untouched
Typography, font loading, system-fonts for blitz-tests and build_single_font_ctx: no font or text-measuring code changed; the new test target does not change the blitz-tests font feature.
@task_item n=3 part=untouched
Spacing, depth strategy, hit-testing and paint clipping, and border radius: no layout, paint or hit-test code moved.
@task_item n=4 part=untouched
Iconography and cursors: browser icons, IconButton, favicons and cursor selection are untouched; no apps/browser file changed.
@task_item n=5 part=untouched
CLI surface: escher-session and its mcp role, stdout JSON-only and the exit-status classes 0 to 3 are unchanged; VERBS twelve, CAUSES eight and SessionError fifteen unmoved.
@task_item n=6 part=untouched
Web-spa and mobile-native platform and toolkit lines (canvas surface, IS_MOBILE, Android and iOS builds, publish workflow): no workflow, CI step or platform code changed.
@task_item n=7 part=untouched
The NO RECORDED INTENT and NOT YET MEASURED markers (brand identity, anti-patterns, self-validation, design decisions log, project-wide scales, reduced motion absent): the session records no design intent and measures no token scale.
