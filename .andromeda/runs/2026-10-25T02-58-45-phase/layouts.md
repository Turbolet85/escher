# layouts extract

## Relevance
partial — the chunk writes a document and adds or changes no surface, but the statement describes two surfaces the plan records: the driver's command line and MCP role (cli) and the headless stand's layout that the driver reads (desktop-native).

## Constraints
- What the statement says about the command line (the twelve verbs in three levels, each verb's flags, the four endings with their exit statuses, one uncoloured JSON line on stdout, nothing the caller typed written back) must match per layout-templates §Surface: cli → Primary screens (the `escher-session` entry). The statement points at that entry or at the schema and does not restate it in new words.
- What the statement says about the MCP role (the tools listed in verb-table order, `lint` after `screenshot`, `help` last, an accepted `screenshot` sent as an image block then a text block, a paged `snapshot` sent as its JSON line) must match per layout-templates §Surface: cli → Primary screens (the `escher-session` entry, its `mcp` role). CARRY 1's README count of «eleven» goes against this.
- What the statement says about recording (`--record <dir>` on `start`, the host role and the `mcp` role, the owner-only `check-{n}.json`, the session errors `record-dir-not-private` and `recording-failed`) must match per layout-templates §Surface: cli → Primary screens (the `escher-session` entry, its `--record` clause). That entry also says «unix only in 0.1.0». Whether 0.2.0 still says unix only is research's question.
- The stand reach the statement claims (the four lean tasks booted headless in TaskShell at the pinned 800 × 600, scale 1.0, Light viewport with the bundled font; author ids on the controls of all seven tasks) must match per layout-templates §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell entry). That entry records the windowed stand as not measured and not booted, so the statement must not claim a windowed reach.
- What the statement says about scrolling and clipping (`scroll_into_view` moves every scrolling box that holds the target, then the viewport; `visible_region`; the CRUD list as the stand's nested scroller that clips its rows) must match per layout-templates §Surface: desktop-native → IA notes (the scrolling-box note).
- What the statement says about the visual lint on the stand (0 findings at boot on each lean task in both layout modes; after twelve Creates the last CRUD row reads `clipped` inside `crud-list`) must match per layout-templates §Surface: desktop-native → IA notes (the visual-lint note).
- The chunk moves no stand markup, id, class, style, wrapper or order, the same rule every ratified stand change kept per layout-templates §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell entry).

## Patterns to follow
- Each claim carries its own anchor: a `file:line` or an «as measured at {chunk report}» citation, as every entry in layout-templates §Surface: desktop-native → Primary screens does. The statement's capability and limit lines should point at where they are proven in the same way.
- Describe one binary by its roles (the client command, the host role `serve`, the `mcp` role), each with its own grammar and endings, as layout-templates §Surface: cli → Primary screens does for `escher-session`.
- State an unmeasured reach as unmeasured in the sentence that makes the claim, the way layout-templates §Surface: desktop-native → Primary screens says «the windowed stand not measured» next to the headless reading.

## Anti-patterns to avoid
- Claiming any layout reach on the mobile-native or web-spa surfaces, or the windowed stand, from this plan. Their surface headers are NOT YET MEASURED, and per layout-templates §Surface: desktop-native → Primary screens the windowed stand was not booted. Whether any later chunk measured a windowed run is research's question.
- Carrying the stale «eleven driver commands» count into the statement or leaving it in the README. Per layout-templates §Surface: cli → Primary screens the MCP role lists twelve.
- Citing the Decisions Log for any choice of location or layout. It reads NO RECORDED INTENT.

## Contract bindings
- layouts §Surface: cli → Primary screens (`escher-session`) ↔ test-plan §3 Session lifecycle. The plan entry says the session contract it serves lives there. The statement's lifecycle and recording claims are judged against both.
- layouts §Surface: desktop-native → Primary screens (CRUD's six-stop Tab sequence `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`) ↔ a11y §Focus Order SC 2.4.3 and `stand_keyboard`'s tab-order tables. The statement's a11y-reach section uses this binding.
- layouts §Surface: desktop-native → IA notes (the stand reads clean to the lint) ↔ test-plan's `stand_lint` check. Any lint-reach claim in the statement cites that check.

## Acceptance criteria contributions
- The statement and README.md both give the MCP role twelve tools, `lint` after `screenshot` and `help` last. No tracked line still reads «eleven driver commands» (per layout-templates §Surface: cli → Primary screens).
- The statement places its stand reach headless, at the pinned 800 × 600, scale 1.0, Light viewport, and lists the windowed stand as not measured among its stated limits, unless research finds a later measurement (per layout-templates §Surface: desktop-native → Primary screens).
- The statement's lint and scrolling reach matches the recorded stand readings: 0 findings at boot on each lean task, `crud-person-14` `clipped` inside `crud-list` after twelve Creates, and `scroll` bringing a clipped row inside the list (per layout-templates §Surface: desktop-native → IA notes).
- The chunk's diff touches no file under `examples/seven_guis/src/`, so no stand markup, id, class, style or order moves (per layout-templates §Surface: desktop-native → Primary screens).
