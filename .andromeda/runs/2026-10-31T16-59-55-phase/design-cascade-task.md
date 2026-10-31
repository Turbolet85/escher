# Task — design · 2026-10-31-select-popup

## The chunk
A closed single `select` opens a painted list of its options, in every document and renderer, as a new overlay in paint, hit testing and keys that lives in the engine and not in the shell.
A person or an agent chooses an option from the open list by pointer and by key, through the one write path `apply_select_choice`; the driver can act on an open option; the list closes without a choice.
It restates what open options change: their `bounds` in the snapshot, the lint's `overlaps` exclusion, and the `covered` refusal's reading and schema text; a group's label and `size` are painted or read here.
The focus and key contract, reduced motion and the list's engine colours are the founder's forks at P4; the colours are a new engine default.
It adds no verb, node key, cause, session error, exit status, carrier, listener, port, env var or workspace crate; it adds `escher-0.4.0-incubator/` to `.gitignore` and re-reads the watch on the free-disk step in `Stand checks repeated`.

## Reached files
- packages/blitz-paint/src
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/events/keyboard.rs · edits the single select's keys (Enter, Space, arrows, typeahead)
- packages/blitz-dom/src/events/pointer.rs · edits the clicks on a select and on an open list's options
- packages/blitz-dom/assets/default.css · edits the group label's and the open list's defaults
- packages/blitz-dom/src/accessibility.rs · edits the select's `expanded` state
- packages/dioxus-native-dom/src/snapshot.rs · edits `snapshot`, an option's `bounds`
- packages/escher-driver/src/execute.rs · edits `execute`, the `covered` reading of an option
- packages/escher-driver/src/lint.rs · edits `lint`, the `overlaps` exclusion
- packages/escher-driver/src/schema.rs · edits the schema help
- packages/escher-driver/src/schema/spec.rs · edits the schema help
- packages/escher-driver/src/refusal.rs · edits the `covered` refusal's text
- tests/blitz-tests/tests/stand_contrast.rs · edits its table, with any new row
- tests/blitz-tests/tests/stand_keyboard.rs · edits its table, with any new row
- tests/blitz-tests/tests/stand_act_select.rs · edits its table, with any new row
- tests/blitz-tests/tests/stand_lint.rs · edits its table, with any new row
- .gitignore · edits the incubator lines
- .github/workflows/ci.yml · re-reads the free-disk step in `Stand checks repeated`

## Needs
- The closed single select as a painted form control, and what its open list paints from it: the width, the anchor and the label it shows.
- The list's place in the paint order: an overlay above the page, read against what the plan records of hoisted children, layers and the engine's depth behaviour.
- Hit testing and paint clipping over the list: the rule that stops hits at the padding box of a node paint clips binds a select inside an `overflow` or `contain: paint` ancestor, and the overlay must answer on its rows there or say why not.
- The list's own surface values, none of them yet set: its padding, row height, gaps and border, read against what the plan records of engine spacing.
- The list's corners, read against what the plan records of engine radius handling.
- The list's colours as a new engine default, read against the colour model and scheme and the engine-painted defaults, since the engine holds no tokens; the values are a P4 fork.
- The states a list row paints: the chosen option, the row under the pointer, the row the keys are on, and a disabled option or group.
- Contrast of the list's text and row states under SC 1.4.3; the plan's reading that a closed select's contrast pair is unmeasured moves if the list adds one.
- The row text: font family, size and weight through the engine's text mapping, and how the plan records text measuring, loading and absent families for a row and a closed select's label.
- A group's label, named in the tree and unpainted today with `content: attr()` unbuilt, is this chunk's to paint or read.
- `size`, unread on a select today, is this chunk's to read, and how it bears on the single select and on `multiple`.
- `multiple` select rows stay as built by 2026-10-31-engine-debts: the list's values sit beside theirs and must not move what they paint, their `size` apart.
- The focus and key contract with a list open, and whether a closed select's arrows and typeahead keep moving the choice, under SC 2.1.1 and SC 2.4.3; the contract is a P4 fork.
- Focus order and any focus indication between the select and the open list's rows.
- Reduced motion and any motion on opening or closing: the plan records reduced-motion handling as absent and the engine's motion values; the answer is a P4 fork.
- A long list that scrolls is a new nested scrolling box: the plan's rule for a programmatic into-view scroll, written at once innermost first, and the driver's instant `scroll` reach it.
- The cursor over the list's rows and past its edge, through the engine's cursor rule.
- An open option's real box in the snapshot, against a closed option's `bounds` equal to its select's client rect.
- The lint's `overlaps` reading of an open list's options against the page beneath them, and the F9 exclusion restated from today's closed options, those under an `optgroup` included.
- The `covered` refusal's closed-option reading and its schema text, restated here, as the driver's agent-facing surface carries them.
- The driver's `screenshot` of an open list: a validated surface admits new input and pixels, so the list's pixels are read through it.
- `dioxus-native`'s windowed path shows the same list on the desktop surface; a separate OS window is not assumed, and the plan's window and stack notes say what that path holds.
- The list on the WASM canvas surface and on the other renderers the plan records, since it paints in every document and renderer.
- The stand's popup, the flight booker's `select#flight-type`, and the rows of `stand_contrast`, `stand_keyboard`, `stand_act_select` and `stand_lint` that read it.
- The accessibility tree says whether the select is open, through `aria-expanded`'s `expanded`.

## Standing conventions
- S1 · A new test that measures or asserts on text runs in the blitz-tests crate with `blitz-dom`'s `system-fonts` feature, or it passes vacuously on text of 0x0 · Typography section of the body
- S2 · A new pixel check through the driver's `screenshot` owes a font context that resolves `sans-serif`, or it is refused `fonts-unavailable` with nothing painted · Typography section of the body
- S3 · A new test's time runs through the harness's `VirtualClock`: settle never advances it, `advance` moves it by its `ms`, and `click`, `type` and `press` leave it · Motion section of the body
- S4 · New driver output on `escher-session` or its `mcp` role stays JSON on stdout, one line per command or JSON-RPC line, with no colour or ESC byte and no terminal test · Surface: cli, Component Patterns, in the body

## Not needed
- The browser app's toolbar icon assets, `IconButton`, tab strip glyphs and bundle icons: a select's list draws none of them.
- The browser app's titlebar, and on mobile its urlbar, back button and capture: browser chrome the list does not touch.
- `apps/readme`'s GitHub markdown styling and its octicons: an app's own stylesheet, no engine default.
- The WPT runner's terminal colours and the terminal printing of the `paint_bench` and `screenshot` examples: output of those binaries, apart from the driver's.
- The todomvc page's colour around its canvas: host page styling, outside what the engine paints.
- The own values of example fixtures other than the stand's flight booker: demonstrations, which a new engine default does not read.
