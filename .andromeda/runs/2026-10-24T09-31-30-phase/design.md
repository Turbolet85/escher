# design extract

## Relevance
partial — the chunk adds no surface and no token, but an upstream take that moves Stylo, Taffy, inline layout, hit testing, `scrolling.rs` or Dioxus reaches the stand's painted colours, its text, the hit rule, the motion rules and the driver's CLI output, which the plan records as target state.

## Constraints
- The seven_guis accent stays `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it. Its non-text uses move with it, and the plan requires every measured stand colour to equal its declared value (per design-system §Color Palette › Core Colors). The take must not edit stand task CSS. Upstream's `286874cc1` edits only `examples/seven_guis/src/tasks/cells.rs`, a non-lean task. Whether a Stylo 0.23 move changes colour resolution or conversion is research's question.
- Stand text is the bundled DejaVu Sans, registered for every generic through `build_single_font_ctx` with system fonts off. blitz-tests is built with blitz-dom's `system-fonts` feature, because without it text measures 0x0 and font-dependent assertions pass vacuously (per design-system §Typography). A take that changes the manifest or features (Stylo, Dioxus 0.8) must keep both. Whether `3954ecb85` (`text-indent` keywords, `word-break: break-word`) or `28f848d3b` (the inline layout split) moves text metrics is research's question.
- A driver `screenshot` of an app whose font context resolves no `sans-serif` family stays refused `fonts-unavailable` with nothing painted (per design-system §Typography).
- A hit stops at the padding box of a node that paint clips, by `overflow` or by a computed `contain` that includes `paint`, the root element excepted. The node itself is still answered inside its border box (per design-system §Depth Strategy). `f102663b8` (#1244) changes hit testing (`hit_inner`) to skip dropped nodes. The rule must still hold on the take's tree, and whether upstream's hunk touches the clip stop is research's question.
- A programmatic into-view scroll never animates a nested scrolling box. Every box holding the target is written at once, innermost first, and `Smooth` applies to the viewport alone. The driver's `scroll` stays instant (per design-system §Motion). `scrolling.rs` is a predicted conflict, and its resolution must keep this rule.
- A harness settle answers every member of the animating set the same way: none holds it open, and none is waited on or advanced. Animation time is read only through the harness's `VirtualClock`. `advance` moves it by exactly `ms`, and the load wait moves no clock (per design-system §Motion). `1c5265a8b` (animation-set lookups skipped) and the `scrolling.rs` and `config.rs` hunks reach this ground. Whether they change the animating flag's reading is research's question.
- The `escher-session` stdout and its `mcp` role hold only JSON: one line per command, or newline-delimited JSON-RPC 2.0 lines. They never hold colour or an ESC byte, and nothing tests for a terminal. The exit status carries the class `0`/`1`/`2`/`3` (per design-system §Surface: cli › Component Patterns). A Dioxus 0.8 take rebuilds that binary, and this must still hold.

## Patterns to follow
- A moved number is re-measured and recorded, never loosened. `stand_contrast`'s `TEXT_ROWS` and `STATE_ROWS` are the record of each stand colour pair, at rest, on hover and when active. A take that moves Stylo re-reads them and records any change (per design-system §Color Palette › Core Colors).
- The headless stand keeps its font path: `HarnessOptions.font_ctx` gets `build_single_font_ctx(DEJAVU_SANS)`, decoded through seven_guis' native `woff` feature, so stand text does not depend on host fonts (per design-system §Typography).
- The hit rule's standing readings are `hit_clipped_at_scrolling_box`, in both layout modes. They cover the `img`, `srcdoc` iframe and filled-input answers, and a box both transformed and scrolled hit where its rows show. Upstream's new `hit_test_dropped_nodes.rs` runs beside them, not in place of them (per design-system §Depth Strategy).
- The motion rules' standing readings are `scroll_into_view_nested`, `stand_settle`, `stand_virtual_clock` and `stand_act_timer` (per design-system §Motion).

## Anti-patterns to avoid
- Loosening a contrast row, a declared stand colour or a measured ratio so the take passes. The record says every measured stand colour equals its declared value (per design-system §Color Palette › Core Colors).
- Building a blitz-tests feature set without `system-fonts`, where font-dependent assertions would pass vacuously (per design-system §Typography).
- Letting colour, an ESC byte or a terminal test into `escher-session` stdout through any upstream or dependency change (per design-system §Surface: cli › Component Patterns).

## Contract bindings
- Stand accent and text colours ↔ a11y §Contrast (SC 1.4.3). White on `#3a5ce5` reads 5.47, on hover 6.83 and when active 8.27, each against 4.5, pinned by `stand_contrast` (design-system §Color Palette › Core Colors).
- The hit stop at a paint-clipping box ↔ the driver's `covered` and `off-screen` refusals (escher-driver executor), and CARRY 3's `contain: paint` row (design-system §Depth Strategy).
- Motion and settle time read through the `VirtualClock` ↔ CARRY 4's clock invariant (no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs`) and the test harness's settle contract (design-system §Motion).
- CLI stdout as JSON only ↔ the obs and security rule that stdout is the answer and never a log line (design-system §Surface: cli › Component Patterns).

## Acceptance criteria contributions
- On the take's tree, `stand_contrast` passes with every row's measured colour equal to its declared value, and the accent pairs still read 5.47, 6.83 and 8.27. If a ratio moved, it is re-measured and recorded, never loosened (per design-system §Color Palette › Core Colors).
- `hit_clipped_at_scrolling_box` passes in both layout modes on the take's tree (per design-system §Depth Strategy).
- `scroll_into_view_nested` and `stand_settle` pass in both layout modes once `scrolling.rs` is resolved. A nested box is still written at once, and settle still neither waits on nor advances an animation (per design-system §Motion).
- The `escher-session` command line and `mcp` checks (`cli_commands`, `cli_flow`, `mcp_server`) still read stdout as JSON lines with no ESC byte after the take (per design-system §Surface: cli › Component Patterns).
