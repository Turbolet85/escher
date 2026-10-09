# layouts extract

## Relevance
partial — the chunk creates and modifies no surface; layout-templates binds it only as a guard: Item 1's textarea fixture stands beside the stand's mount and must leave it unmoved, and Item 2 touches none of the registered CLI screens.

## Constraints
- The headless stand's mount is fixed: one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell under `main#main`, through `task_in_shell`. Item 1's fixture is a separate document beside that mount, never a fifth task and never a child of TaskShell (per layout-templates §Surface: desktop-native → Primary screens).
- The lean tasks' controls and value displays are an enumerated set of author ids, and none of them is a textarea. A textarea added to a stand task would change that set and is a layout-templates amendment the chunk's scope excludes (per layout-templates §Surface: desktop-native → Primary screens).
- The stand's screen is pinned: 800 x 600, scale 1.0, Light, the bundled DejaVu Sans. A fixture booted with the stand's options is required to read at that same viewport; the fixture must not carry a viewport or font of its own (per layout-templates §Surface: desktop-native → Primary screens).
- The test harness's own default viewport is 800 x 600 at scale 1 in light mode, so a fixture booted without the stand's options reads the same size but not necessarily the same font; which boot the fixture takes is the plan's, and whether the two read identically for a textarea is research's question (per layout-templates §Surface: desktop-native → IA notes).
- Every Dioxus document starts from the fixed skeleton whose `<main id="main">` the app mounts into, and arbitrary `index.html` templates are not supported. A Dioxus-built fixture's textarea therefore sits under `main#main`; whether the fixture is Dioxus-built or parsed HTML is research's question (per layout-templates §Surface: desktop-native → IA notes).
- A fixture beside the stand moves no stand markup, id, class or style: the plan records the in-file fixture as the place a case the stand's screens lack is exercised, with the stand untouched (per layout-templates §Surface: desktop-native → IA notes).
- The registered command surfaces are `paint_bench`, `bump`, `scripts/agent-run.sh`, `scripts/cold-agent.sh` and `escher-session`, each with its usage line and output grammar; no `.github/scripts/` file is among them. Item 2 must change none of those five, and a CI install helper stated once under `.github/` is not a registered screen — whether it earns a row is the wrap's question, not this extract's (per layout-templates §Surface: cli → Primary screens).

## Patterns to follow
- An element an agent can act on is keyed by an author `id` that adds no class, style, wrapper or order, and no CSS selects by it; the fixture's textarea takes its key the same way (per layout-templates §Surface: desktop-native → Primary screens).
- An input is named by an attribute alone — `for` on a label that already exists, or `aria-label` — adding no element, class, style or order; the fixture's textarea, if it is named, is named that way (per layout-templates §Surface: desktop-native → Primary screens).
- A consumer of the stand's mount adds nothing to it: the plan records the session host as a consumer that adds no markup, id, class, style, wrapper or order, and a fixture file that reuses the stand's boot options is held to the same reading (per layout-templates §Surface: desktop-native → Primary screens).
- The minimal in-file fixture is the standing form for a case the lean tasks lack; the stand's own measured boxes stay as recorded while the fixture carries the new case (per layout-templates §Surface: desktop-native → IA notes).

## Anti-patterns to avoid
- Placing the textarea in a stand task or in TaskShell to give the check a screen — it changes the enumerated id set and the mount the plan records (per layout-templates §Surface: desktop-native → Primary screens).
- Giving the fixture's textarea a wrapper, a class or a style that exists only to carry its key or its name (per layout-templates §Surface: desktop-native → Primary screens).
- Changing a registered command's usage line, stdout or exit grammar as a side effect of restating the CI install step (per layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts ↔ a11y: the fixture's textarea is a focusable, actionable element. layout-templates holds no focus-order section, so its key, its accessible name and SC 2.4.3 are a11y-plan's to state; layouts contributes only that the stand's markup, and so the stand's focus order, is not moved.
- layouts ↔ tests: the pinned viewport and the TaskShell mount are read by the stand's boot check the plan cites; test-plan owns that check and the new fixture's place among the stand checks. Whether the new check asserts the viewport itself is the plan's.
- layouts ↔ architecture: a script or a local action added under `.github/` is a directory-tree fact of architecture §Infrastructure Patterns, not a layout-templates screen.

## Acceptance criteria contributions
- (layouts) After the chunk, the four lean tasks read the id sets the plan enumerates and no task holds a textarea: the diff over `examples/seven_guis/src/` is empty (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The textarea fixture, if booted with the stand's options, reads the pinned 800 x 600, scale 1.0, Light viewport; it sets none of its own (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The fixture's textarea carries an author `id` and adds no wrapper, class or style for it (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The usage lines and output grammar of the five registered commands are unchanged: the diff over `scripts/`, `apps/bump/`, `examples/paint_bench.rs` and `examples/seven_guis/src/session_host.rs` is empty (per layout-templates §Surface: cli → Primary screens).
