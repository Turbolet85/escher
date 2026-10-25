# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · no · dependency pins are Not needed (no manifest or toolchain change) and no Cargo.toml is reached
established-decisions#2.1 · no · build profiles are Not needed (no manifest or toolchain change)
established-decisions#3.1 · no · release versioning is Not needed (no manifest or release change)
established-decisions#4.1 · no · the chunk's goals and checks take no shot, so the renderer backends are not reached
established-decisions#5.1 · yes · the stand's dioxus-native-dom default `accessibility` gates the snapshot modules the re-pinned stand checks read
established-decisions#6.1 · yes · the pinned 800x600 Light viewport and incremental layout defaulting on bind the stand's headless boot and the both-mode re-pinned checks
established-decisions#7.1 · yes · a `<head>` stylesheet is a render-blocking pending resource that settle reports, the mechanism the stand's held load needs
established-decisions#8.1 · no · CSS approximations are Not needed (no engine layout change) and the task names no approximated property
established-decisions#9.1 · no · unsupported CSS features are Not needed (no engine change), and its one supported reading, `:focus-within`, binds through established-decisions#15.2
established-decisions#10.1 · yes · keyboard activation, focusability, the aria state reads and `scroll_into_view` are read by the re-pinned Tab, Enter/Space, snapshot, diff and act checks
established-decisions#11.1 · yes · `inert`, `disabled` on a form control alone, the per-role toggle and `absolute_position` less its containing blocks' scroll are read by the re-pinned Tab, snapshot and act checks
established-decisions#11.2 · yes · the document's clock is the one virtual clock `advance` moves, and the 500 ms double-click window reads it
established-decisions#11.3 · yes · the single-choice select model is the flight booker's new select under stand_act_select, flight.sh and the agent's select goal
established-decisions#12.1 · yes · the damage bits and order sort behind the both-mode layout identity every re-pinned stand check asserts, and the control layout the Tab and act checks read
established-decisions#13.1 · no · a replaced element's aspect-ratio height is Not needed (replaced elements and inline SVG, no engine layout change)
established-decisions#14.1 · yes · `disabled` by presence on a form control alone and the Dioxus removal clearing `element_id_mapping` bind the re-pinned Tab, element-id and contrast-row checks
established-decisions#15.1 · yes · `inert` skipped by Tab and out of the accessibility tree is what the re-pinned Tab tables and actionable-key checks read
established-decisions#15.2 · yes · `set_focus_to` and FOCUS_WITHIN, with a focus move marking exactly two nodes, are what the re-pinned Tab walk, focused readings and diff checks rely on
established-decisions#15.3 · yes · a select's choice held as CHECKED, its options as snapshot children and a controlled app's revert bind the flight booker's select and stand_act_select
established-decisions#16.1 · no · hit-testing is named Not needed except where stand checks read it, and no re-pinned check hits a node dropped since the last resolve
established-decisions#17.1 · no · HTML parsing is not the stand's path (its DOM is Dioxus-built) and no re-pinned check reads a fixture page
established-decisions#18.1 · no · the shell loop, windowing and the accesskit_xplat adapter are Not needed, and the stand and driver run headless
established-decisions#19.1 · no · painting internals and the password mask are Not needed (no paint change)
