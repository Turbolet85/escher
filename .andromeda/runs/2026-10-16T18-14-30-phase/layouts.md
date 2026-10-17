# layouts extract

## Relevance
partial — the chunk adds tests only and creates or modifies no surface, but its tests drive the Timer's headless TaskShell mount, the dioxus document's mount and the `escher-session mcp` role's stdio, all three described by the plan, so the extract is limited to keeping those as they are.

## Constraints
- The Timer's controls and value displays carry the author `id`s `timer-progress` · `timer-elapsed` · `timer-duration` · `timer-duration-value` · `timer-reset`, which are their stable element ids, and ids add no class, style, wrapper or order (per layout-templates §Surface: desktop-native → Primary screens). Any change made to reach `TimerTicks::next` has to leave those ids and the Timer markup's order as they are.
- `timer-duration` is named by `for` on its existing label alone, adding no element, class, style or order (per layout-templates §Surface: desktop-native → Primary screens). A test-only helper must not add markup to `examples/seven_guis/src/tasks/timer.rs`.
- The headless stand mounts one lean task in TaskShell through `task_in_shell`: `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans (per layout-templates §Surface: desktop-native → Primary screens). A Timer test that boots through `seven_guis::stand` reads that mount, and the chunk must not move it.
- The plan requires a held instance to read as a fresh boot, with the TaskShell mount and the pinned viewport holding after commands in both layout modes (per layout-templates §Surface: desktop-native → Primary screens). A fast-failing check that drives the Timer through a session or the `mcp` role keeps to that.
- Each document starts with the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, and the app mounts into `main` (per layout-templates §Surface: desktop-native → IA notes). This is the tree that `MutationWriter::create_text_node` writes into, and the chunk must not change it.
- The `escher-session mcp` role reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured, with no ESC byte (per layout-templates §Surface: cli → Primary screens). A test of `read_request` framing must not change that output structure.
- On the command line, `mcp` alone is the role, and any other shape after it is usage, exit 2 (per layout-templates §Surface: cli → Primary screens). A framing test must leave the role's argv grammar unchanged.

## Patterns to follow
- Boot the Timer headlessly through `seven_guis::stand` in TaskShell, fresh per boot, at the pinned viewport, which is how the stand's checks and the `escher-session` host role both consume the same mount (per layout-templates §Surface: desktop-native → Primary screens).
- Address Timer elements by their author `id`s (`timer-elapsed`, `timer-progress`, `timer-reset` and so on) as stable element ids, and never by a structural path (per layout-templates §Surface: desktop-native → Primary screens).
- A headless test harness defaults to an 800x600 viewport at scale 1 in light mode (per layout-templates §Surface: desktop-native → IA notes). A minimal fixture for the mutation writer can rely on that default and the fixed `main#main` mount (per layout-templates §Surface: desktop-native → IA notes).

## Anti-patterns to avoid
- Do not add an element, class, style, wrapper, id or order to the Timer's markup, or a CSS selector on its ids, to make a mutant observable. The plan's stand entries each state "adding no element, class, style, wrapper or order" (per layout-templates §Surface: desktop-native → Primary screens).
- Do not write anything but protocol lines to the `mcp` role's stdout from a test hook or a watchdog (per layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts ↔ tests: the `escher-session` session contract that the host and `mcp` roles serve lives in test-plan §3 → Session lifecycle (per layout-templates §Surface: cli → Primary screens). A framing or Timer check run through the binary binds there.
- layouts ↔ a11y (focus order, SC 2.4.3): the Timer's controls are Tab stops in a lean task's TaskShell. The chunk adds no focusable element, so no focus-order position moves. Whether `stand_keyboard`'s Timer row is untouched is the a11y and test extractors' question.

## Acceptance criteria contributions
- (layouts) After the chunk, the Timer's five author ids and the TaskShell mount (`main#main > #task-shell`, `#task-header` above `#task-body`) read unchanged at the pinned 800 × 600 boot, and the chunk's diff adds no element, class, style, wrapper, id or order to the Timer markup in `examples/seven_guis/src/tasks/timer.rs` (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The document skeleton the dioxus document mounts into (`<main id="main">` under `<body>`) is unchanged by any test or fixture added for `create_text_node` (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) `escher-session mcp` still writes protocol lines only on stdout, with no ESC byte, and `mcp` followed by anything else still ends as usage, exit 2, after the framing tests land (per layout-templates §Surface: cli → Primary screens).
