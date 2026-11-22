@task_prose part=session
A behaviour-preserving refactor: escher's own additions in three upstream engine functions were moved out into private functions of escher's own, with one call left in upstream's body.
The accessibility node builder now reads aria-expanded, aria-selected, aria-label, toggle and invalid states through escher's own writer functions, each ARIA attribute still read once.
The keyboard handler's control-key arms (range, single list box, drop-down, multiple select, Enter activation) and the close-the-open-list-before-Tab step moved into two private functions, in their former order and gates; the control-key function stands before upstream's text-input arm and returns early on an element holding text-input data.
The list-box presentational hints (a select's size height, a single list box's overflow-y auto, the list-box row blocks) moved into one private function.
The driver's command validation tests moved whole into their own file; the url and form_urlencoded lock versions were bumped, closing an audit advisory; all a11y, fast, coverage and audit legs and CI green, behaviour unchanged.
@task_file n=1 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder — the node builder that sets each element's role, name and ARIA states (expanded, selected, pressed, checked, toggled, invalid) on the AccessKit node
@task_file n=2 path=packages/blitz-dom/src/events/keyboard.rs
what: the engine's keyboard event handling — Tab and Shift+Tab focus moves, closing an open select list before Tab, range input keys, single list box, drop-down and multiple select keys, Enter and Space activation
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the presentational hints computation — a select's size attribute height, a single list box's overflow-y auto and the list-box row blocks
@task_file n=4 path=packages/escher-driver/src/command.rs
what: the driver's command validation — the verb table, a call checked at its own level, a malformed call refused with the rule it broke
@task_file n=5 path=packages/escher-driver/src/command_tests.rs
what: the driver command validation's unit tests, moved whole out of the validation module
@task_file n=6 path=Cargo.lock
what: the workspace dependency lock — url and form_urlencoded versions, the dependency audit's input
@task_item n=1 part=touches
The one-read-per-ARIA-attribute decision: the reads of aria-expanded, aria-selected and aria-label now stand in escher's own writer functions called once from the node builder; each attribute is still read once across the builder and those functions.
@task_item n=2 part=touches
The decisions that ARIA states are written on every element and that the accessibility tree is built through the document's own method: the node builder keeps its signature and single caller, its states now written through one call right after the role is set.
@task_item n=3 part=touches
The ARIA property read contract item aria-expanded and the single-select-expanded-from-its-list decisions: the expanded write moved into its own writer function, unchanged in value.
@task_item n=4 part=touches
The accessibility tree states contract items aria-selected, select option, aria-pressed, aria-checked, native checkbox or radio and aria-invalid, and the toggle-state-read-by-role and native-checkable-toggle decisions: the selected, toggled and invalid reads moved into escher's own functions, values and closed lower-case matches unchanged.
@task_item n=5 part=touches
The accessible name contract item aria-label and the accessible-name-precedence and aria-labelledby-outranks-label decisions: the aria-label name write moved into its own function, precedence unchanged.
@task_item n=6 part=touches
The keyboard activation contract items Enter, Space and handler order, and the Enter-and-Space-activate-as-a-click decision: Enter activation is now one arm of a private control-key function called before upstream's text-input arm, which returns early on a text-input element so exactly one of them acts, as before.
@task_item n=7 part=touches
The range input key contract items and the range-input-keys decision: the range arm moved into the control-key function, its disabled gate and keys unchanged.
@task_item n=8 part=touches
The single-choice select list closed and list open key contract items, the single list box keys item and the multiple select keys item: their arms moved into the control-key function in their former order and gates.
@task_item n=9 part=touches
The Tab-chooses-the-row-and-closes-the-list-first decision and the single-choice select list open Tab contract item: the close-before-Tab step is now its own private function called in the Tab arm.
@task_item n=10 part=touches
The select key change fires one input then one change decision and the select list choices share one write path decision: the key arms that make those writes moved, the writes unchanged.
@task_item n=11 part=touches
The open select list rows take one height and select list coverage headless decisions, as they rest on the list-box presentational hints: the size height, overflow-y auto and row blocks are now pushed by one private function, which reads every size attribute of the select as the loop did.
@task_item n=12 part=touches
The a11y CI leg and accessibility-test-leg decisions: the leg was fired as the behaviour guard of the moved bodies and stayed green; its rows hold no changed value.
@task_item n=13 part=touches
The accessibility tree contract items for a single select's expanded and select role, and the snapshot rows that take readings from the tree: their source writes moved functions only, the readings unchanged.
@task_item n=1 part=untouched
Contrast and visual lint rows — the WCAG contrast ratio, thresholds, lint findings, stand accent colour and scrollbar thumb stroke: no colour, ratio or lint reading moved.
@task_item n=2 part=untouched
Snapshot text form, actions answer, handler coverage, UI graph, MCP results and node id rows: no snapshot field, driver answer shape or served id changed.
@task_item n=3 part=untouched
Platform adapter and feature-gate rows — AccessKit adapter creation order, accesskit_xplat, accessibility default-on, no adapter on the stand binaries: no feature, adapter or binary changed.
@task_item n=4 part=untouched
Pointer and click rows — synthetic click, double click threshold, range pointerdown and click, checkable click toggles, label click, driver click refusals and hit-testing: no pointer path moved.
@task_item n=5 part=untouched
Focusability and focus fixup rows — what is focusable, focus-visible, focus-within, removed focused node, inert focus fixup: no focus rule changed.
@task_item n=6 part=untouched
The seven_guis Cells grid and CRUD list contract items and the Dioxus document reached-handler record: no app or Dioxus document code changed.
@task_item n=7 part=untouched
Cognitive accessibility, stand tab order table, stand checks repeated three rounds and the not-yet-measured overview notes of every area: nothing measured or added there.
