# Classified — arch · established-decisions.b2

## Answers
established-decisions#14.1 · no · the replaced-element aspect-ratio rule lives in replaced.rs, which no reached file or scope item names
established-decisions#14.2 · yes · the pseudo-element flow vote lives in construct.rs:860 and 867-872, a conflict surface, and in the conflict-surface test pseudo_element_flow_classification.rs
established-decisions#14.3 · yes · the IME enable in create_text_editor (construct.rs:313-317) sits in a conflict surface, and its witness input_focused_on_mount_enables_ime sits in dioxus_document.rs, also a conflict surface
established-decisions#15.1 · yes · the disabled, lang, dir and bridge rules run through unkeyed_actionable in dioxus_document.rs, a conflict surface, and the Enter and Space gates are witnessed by the re-read check stand_keyboard.rs:889-930
established-decisions#16.1 · yes · inert's hit_inner, focus-walk and accessibility rules sit in node.rs (a conflict surface) and are witnessed by the re-read check inert_subtree
established-decisions#16.2 · yes · the focus fixup under inert lives in mutator.rs:1051-1063, a conflict surface, and the scope names it as an engine-debts merge row
established-decisions#16.3 · yes · the FOCUS_WITHIN carry rules at mutator.rs:775-877 sit in the conflict surface mutator.rs, and the changed-set contract they keep is named by the scope
established-decisions#16.4 · yes · the select choice rules and the mutator.rs select arms at 1041-1295 are the select model the scope names, and mutator.rs and default.css are conflict surfaces
established-decisions#16.5 · yes · the optgroup arm at mutator.rs:1253-1257 sits in a conflict surface, and option_disabled is the select-model rule the scope names
established-decisions#16.6 · yes · the group header's synthesized bold through stroke_text_as in text.rs:586-689 is a seam the scope names, and text.rs is a conflict surface
established-decisions#16.7 · yes · the optgroup and multiple rows of default.css:161-199 sit in a conflict surface, and the select list's merge surface is named by the scope
established-decisions#16.8 · yes · the multiple select's size hint in stylo.rs:1124-1139, a seam the scope names, and default.css:183-193 sit in conflict surfaces
established-decisions#17.1 · yes · the dropped-node hit skip is part of the hit test in node.rs, a conflict surface, and the scope names hit_inner and the hit stops
established-decisions#17.2 · yes · the OPEN state writes at mutator.rs:372-375 and 528-531 and the :open match at stylo.rs:510 sit in conflict surfaces
established-decisions#18.1 · no · the HTML parsing and sink rules live in blitz-html and config.rs, none reached and none named by the scope
established-decisions#19.1 · no · the windowing and winit workarounds live in blitz-shell, not reached, and the scope names no shell behaviour
established-decisions#20.1 · no · the redraw request path lives in blitz-shell, not reached, and the scope names no redraw behaviour
established-decisions#21.1 · yes · the password input masking, which the scope names as held through paint changes, sits in blitz-paint and in the merge surfaces render.rs and document.rs
established-decisions#22.1 · yes · the single-frame rule at bootstrap.rs:48-52 sits in a conflict surface, and the bootstrap globals and script gate it shares with events.rs are named by the scope
established-decisions#23.1 · yes · the base_url default at dioxus_document.rs:106-110 sits in a conflict surface, and the bridge's default-sheet hand-off is named by the scope
established-decisions#24.1 · yes · the Cells handle rule sits in examples/seven_guis/src/tasks/cells.rs, a conflict surface, and the RefCell hazard it avoids is what the take must keep
established-decisions#25.1 · no · the browser app's history store and about pages are Not needed, the scope names no change in apps/
established-decisions#26.1 · no · the tab lens rule lives in the upstream-only browser app, which the scope does not reach
established-decisions#27.1 · no · the WPT runner is Not needed, its execution, buckets and budgets are out of scope
established-decisions#28.1 · yes · the escher-driver dependency set and its no-new-package, no-env-var and no-accesskit-adapter invariants are the dependency surface the scope names, through Cargo.lock and Cargo.toml, both conflict surfaces
