# Classified — arch · established-decisions.b2

## Answers
established-decisions#14.1 · no · the replaced-element aspect-ratio rule lives in blitz-dom's replaced.rs, which the chunk does not reach, and no scope term or named check witnesses it
established-decisions#14.2 · yes · the classify_flow_children vote sits in construct.rs and the pseudo_element_flow_classification.rs test, both conflict surfaces the resolution must keep
established-decisions#14.3 · yes · the IME enable in create_text_editor sits in construct.rs (conflict surface), held by dioxus_document.rs's input_focused_on_mount_enables_ime (conflict surface)
established-decisions#15.1 · yes · the removed-node mapping clear at dioxus_document.rs:514-548 (conflict surface) and the hit canonicalization and scrollable_overflow rules in node.rs and scrolling.rs (conflict surfaces) bind
established-decisions#16.1 · yes · inert's Node::is_focussable, the hit_inner no-hit and the accessibility subtree rule sit in node.rs (conflict surface), and inert_subtree.rs is a re-read check
established-decisions#16.2 · yes · the focus fixup at mutator.rs:1051-1063 is a conflict surface, held by the re-read inert_subtree check
established-decisions#16.3 · yes · the FOCUS_WITHIN and changed-set rules of add_children_to_parent sit in mutator.rs (conflict surface), which the changed-set contract keeps
established-decisions#16.4 · yes · the select choice rules and the mutator's select arms (mutator.rs:1041-1295, conflict surface, named as the select model) and the default.css select rows bind
established-decisions#16.5 · yes · the optgroup rules sit in mutator.rs's select arm (1253-1257), kept unchanged by the scope, and the open list's group header
established-decisions#16.6 · yes · the group header's synthesized bold is in blitz-paint text.rs (conflict surface, the named stroke_text_as seam), and select_popup.rs (edited) holds its rows
established-decisions#16.7 · yes · the optgroup and multiple rows are in default.css lines 161-199 (conflict surface)
established-decisions#16.8 · yes · the size hint in stylo.rs (conflict surface, named seam) and its default.css rows (183-193, conflict surface) bind
established-decisions#17.1 · yes · the dropped-node hit skip is hit-test behaviour in hit_inner and node.rs (conflict surface), which the scope keeps, and the paint walk shares the rule
established-decisions#17.2 · yes · the :open state rules sit in mutator.rs (conflict surface, attribute writes) and stylo.rs:510 (conflict surface), and the a11y leg is re-asserted
established-decisions#18.1 · no · HTML parsing and the ua_stylesheets wiring live in blitz-html and config.rs, none reached by the chunk, and no named check or scope term witnesses them
established-decisions#19.1 · no · windowing rules live in blitz-shell and blitz-traits, not reached; the winit pin is taken as one set with upstream's shell, which carries these workarounds
established-decisions#20.1 · yes · the redraw path names focus_redraw and the seven_guis_native smoke boot as witnesses, which the chunk re-asserts through the fast, doc, a11y and audit legs
established-decisions#21.1 · yes · the scope's boundary reads keep a password input masked through paint changes; the mask sits in blitz-paint render.rs and blitz-dom document.rs, the named merge surface
established-decisions#22.1 · yes · the change-event rule in pointer.rs is an engine-debt surface, and the document_may_load gate, bootstrap globals and report_js_error sit in the bootstrap.rs and events.rs conflict surfaces
established-decisions#23.1 · yes · DioxusDocument::new's no-stylesheet rule and base_url sit in dioxus_document.rs (conflict surface), so a resolution there must not re-add DEFAULT_CSS
established-decisions#24.1 · yes · the synchronous Cells handle calls sit in examples/seven_guis/src/tasks/cells.rs (conflict surface), and the events.rs:162 hazard is the resolution's to keep clear
established-decisions#25.1 · no · the browser app's history store lies outside the chunk, the scope names no change in apps/ and lists the browser app as not needed
established-decisions#26.1 · no · tab lenses are in apps/browser, which the scope lists as not needed
established-decisions#27.1 · no · WPT runner behaviour is outside the chunk, the scope excludes the runner's execution, outputs and budgets
established-decisions#28.1 · yes · the no-new-port, no-listener and dependency-surface values bind through the Cargo.lock and Cargo.toml conflict surfaces (the escher-driver lock entry and workspace lines); the standing S8 line itself is not the basis
