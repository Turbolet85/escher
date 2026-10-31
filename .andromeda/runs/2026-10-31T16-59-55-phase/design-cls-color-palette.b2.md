# Classified — design · color-palette.b2

## Answers
color-palette#4.4 · no · the todomvc example's own body and heading colours, host page styling outside what the engine paints and an example's own value
color-palette#4.5 · no · the wasm_hello example's heading and code colours, an example fixture's own value that a new engine default does not read
color-palette#5.1 · yes · the disabled form-control value 209,209,209 is the engine-painted disabled state that a list's disabled option or group colour is read against, in blitz-paint/src
color-palette#6.1 · no · WPT runner terminal colour for PASS, output of the WPT runner and not engine paint
color-palette#6.2 · no · WPT runner terminal colour for FAIL with some passing subtests, output of the WPT runner and not engine paint
color-palette#6.3 · no · WPT runner terminal colour for FAIL, output of the WPT runner and not engine paint
color-palette#6.4 · no · WPT runner terminal colour for TIMEOUT, output of the WPT runner and not engine paint
color-palette#6.5 · no · WPT runner terminal colour for SKIP, output of the WPT runner and not engine paint
color-palette#6.6 · no · WPT runner terminal colour for CRASH, output of the WPT runner and not engine paint
color-palette#7.1 · no · browser urlbar border, browser chrome the list does not touch
color-palette#7.2 · no · browser chrome focus colour, browser chrome the list does not touch
color-palette#7.3 · yes · the blitz-dom default input focus outline is the engine's focus paint that a focus indication between the select and the open list's rows is read against, and default.css is edited for the open list's defaults
color-palette#8.1 · yes · the engine's default scrollbar thumb palette paints the scrolling box of a long open list, a new nested scrolling box, and is read against the list's colours in blitz-paint/src
color-palette#8.2 · no · author scrollbar-color resolution and author thumb blending, which the chunk neither sets nor names
color-palette#8.3 · no · unchecked radio ring and checkbox tick colours, and the list draws no radio or checkbox
color-palette#8.4 · no · debug overlay colours, a devtools paint the list does not reach
color-palette#8.5 · no · devtools layout stroke colours by display type, devtools output the list does not reach
color-palette#8.6 · no · SVG currentColor serialization, not the list's paint
color-palette#9.1 · no · rdme GitHub markdown tokens, an app's own stylesheet with no engine default
color-palette#10.1 · yes · the plan's record that the engine holds no colour tokens or custom properties is the ground the list's new engine colour default is read against
color-palette#11.1 · no · clip-path example fixture palette, a demonstration the new engine default does not read
color-palette#11.2 · no · border-styles example fixture colours, a demonstration the new engine default does not read
color-palette#11.3 · no · github_profile_reduced example fixture's borderColor token, a demonstration the new engine default does not read
color-palette#11.4 · no · BBC example fixture's relative colour syntax, a demonstration the new engine default does not read
color-palette#11.5 · no · graphite example fixture's custom properties, a demonstration the new engine default does not read
