@task_prose part=chunk
Syncs this fork with upstream through one real two-parent merge of upstream/main at 29a439e05 into the 0.4.0 build branch; it is never a rebase, a squash or a cherry-pick series. The take carries all six upstream commits: a br kept in its anonymous flex item, Taffy 0.16 from the registry, one-axis overflow clip in paint, drops from other applications (DataTransfer, drag events, a winit drop source, a dropped file's reader), preventDefault ignored on a non-cancelable event, and two root examples.
It resolves the one conflict, the event conversion's import blocks, and makes two adaptations of ours: the layout reason's calc resolution under Taffy's CalcHandle, and the drag event's modifiers through convert_modifiers under the Dioxus 0.7.3 hold.
It measures the merged lock whole, builds the workspace with --locked, and runs the audit and the MSRV leg. It re-reads the left-out set, the Dioxus hold, eight merge-surface residuals, the one-axis-clip hit limit and the sink allowlist.
It re-runs every escher proof on the merged tree: the fast and doc CI legs, the stand checks in both layout modes, the incremental oracle, the a11y leg, the audit, the embolden witness, and the screenshot and cross-OS readings, where a reading that moved is re-measured and recorded, never loosened.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus event conversion, which carries keyboard, pointer, form and now drag events into the Dioxus document's handlers. Its two import blocks conflict. The drag event's modifiers go through convert_modifiers under the Dioxus hold.
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: escher's layout reason, the reading behind Session::layout and the layout verb's answer — edits: its call to the calc-value resolver, which now takes Taffy's CalcHandle
@task_file n=3 path=Cargo.toml mark=merge
what: the root workspace manifest: the workspace members, the Taffy pin moved from a git rev to registry 0.16.0, and the dioxus-native dev features gaining net and html
@task_file n=4 path=Cargo.lock mark=merge
what: the workspace lock: Taffy moves from git 0.15.0 to registry 0.16.0 with its checksum, and url joins blitz-shell's dependency list
@task_file n=5 path=packages/blitz-shell/Cargo.toml mark=merge
what: the windowed shell's manifest, gaining the url dependency for the drop source
@task_file n=6 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the windowed shell's OS drop source, which turns drops from other applications into drag events
@task_file n=7 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine's event driver, which dispatches DOM events, handles default actions and gains drag events; preventDefault is now ignored on a non-cancelable event
@task_file n=8 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout tree construction, where a br after text stays in the text's anonymous flex item
@task_file n=9 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-layout conversion, whose calc-value resolver now takes Taffy's CalcHandle
@task_file n=10 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer, which now clips only the clipped axis of a one-axis overflow clip
@task_file n=11 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests module list, gaining upstream's new mod lines
@task_file n=12 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: upstream's new test of a br inside an anonymous flex item
@task_file n=13 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: upstream's new test of drag-and-drop events
@task_file n=14 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the pointer-leave test, extended upstream for preventDefault on a non-cancelable event
@task_file n=15 path=examples/dnd.rs mark=merge
what: upstream's root drag-and-drop example
@task_file n=16 path=examples/file_drop.rs mark=merge
what: upstream's root file-drop example, which previews dropped content
@task_file n=17 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner, re-run on the merged tree for its fast and doc legs
@task_file n=18 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check (one file and one target per behaviour), re-run over upstream's new test files
@task_item n=1 part=needs
New behaviour the take brings: drag data from other applications reaches every windowed document's page as four DOM events, and a Dioxus app's handler can read a dropped file's bytes through std::fs. No harness input, driver verb, MCP tool or stand task reaches this; the founder ratified it as a boundary widening.
@task_item n=2 part=needs
New behaviour the take brings: preventDefault is ignored on a non-cancelable event in the engine's event dispatch. Keyboard activation by Enter and Space, handler order, the click walk and the default actions of select, range and checkable controls must keep their intent through it.
@task_item n=3 part=needs
Resolving the event conversion's conflict keeps both sides' intent: the keyboard and range models, the select model and its list keys, the pointer frame and click synthesis, and the Dioxus document's reached-handler record. Upstream's drag events are added beside these.
@task_item n=4 part=needs
The drag event's modifiers go through convert_modifiers under the Dioxus 0.7.3 hold. Dioxus features stay taken by name only, with no move to 0.8, and the keyboard_compat coverage exclusion reads standing.
@task_item n=5 part=needs
The layout reason surface: escher's layout reason resolves calc values under Taffy 0.16's CalcHandle. Session::layout, the layout verb's answer and stand_layout must read as they did.
@task_item n=6 part=needs
Taffy moves from git 0.15 to registry 0.16. The bounds the snapshot reports come through layout and are re-read on the merged tree: control nodes, a closed select's options, an open list's rows, a multiple select's rows and groups, and open select list rows of one height. The stand checks run in both layout modes, and the incremental/non-incremental oracle runs too.
@task_item n=7 part=needs
New layout behaviour: a br after text stays in the text's anonymous flex item. Text runs still label their parent element, and accessible names drawn from text stay as they were.
@task_item n=8 part=needs
New paint behaviour: a one-axis overflow clip now clips only the clipped axis. The hit still stops at the whole padding box as a stated limit. Hit-testing (the inert element takes no hit, and the driver click's covered refusal), the screenshot readings and the contrast readings taken from paint are re-measured on the merged tree, and a moved reading is recorded, never loosened.
@task_item n=9 part=needs
Conflicts keep both sides' intent over the accessibility reads. These are the accessibility tree built through the document's method, the ARIA state and property reads, the accessible name precedence, hidden and inert content kept out of the tree, the stable ids and the snapshot, the changed set's contract and its clock invariant, and the custom-widget paint.
@task_item n=10 part=needs
The accessibility feature stays on by default and the session library still enables it. The stand binaries still build no platform adapter, even though the windowed shell gains a winit drop source and the url dependency.
@task_item n=11 part=needs
The lock move enters whole, through the cargo build --workspace --locked and the dependency audit. No dependency, the AccessKit crates and accesskit_xplat among them, moves beyond upstream's own set.
@task_item n=12 part=needs
Sink allowlist: every new tracing or log call site, target, print and env read in the merged delta is read against the allowlist. Records from accesskit_xplat and the blitz targets print only their admitted fields, and the standing capture checks stay at 0 needles.
@task_item n=13 part=needs
Upstream's new test files and module lines run under the fork's blitz-tests rule: one file and one target per behaviour.
@task_item n=14 part=needs
No new crate, port, env var or listener of escher's own comes with the take. The escher crates and the seven_guis members stay in the workspace.
@task_standing n=1
convention: the accessibility test leg runs in the slow tier, and its stand checks run in three rounds. It gates CI on the pushed commit, and every job must settle green.
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Accessibility tier and reach rows: the tier-0 rows, the absence of reach for assistive technology, and the capability statement's reach edited by hand. The merge changes no reach tier and no capability statement.
@task_item n=2 part=not_needed
Role-mapping tables (HTML-AAM native roles, the role attribute's precedence, role="switch") as new work. The take adds no element, no role and no ARIA attribute read.
@task_item n=3 part=not_needed
UI graph state registration rows (use_graph_state, undeclared, unconfirmed, no registered state). The take brings no Dioxus state hook, and Dioxus stays held.
@task_item n=4 part=not_needed
Driver schema, MCP result, actions-answer and snapshot-diff field contracts as new items. No driver verb or MCP tool reaches the drop input, so no answer gains a field.
@task_item n=5 part=not_needed
Viewport scheme and zoom rows, hidpi or zoom reshaping text, and the :open and :focus-visible pseudo-class rows. The take touches no viewport, device or selector-matching code.
@task_item n=6 part=not_needed
Password masking and caret rows, and IME enabling for a prefocused input. The take changes no text-input editing or value reading.
@task_item n=7 part=not_needed
The sections with no recorded intent: SLO invariants and a11y budgets, a11y anti-patterns, and the a11y decisions log.
@task_item n=8 part=not_needed
Cognitive-accessibility rows on stating the invalid date state beyond colour or class. The take touches no date input and no invalid-state styling.
