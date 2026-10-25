# Task — tests · 2026-10-25-capability-statement

## The chunk
Writes one capability statement, current with 0.2.0. It is a document and not code: what escher can do, its stated limits, its a11y reach and what an MCP client carries onward, placed where Viola's frontend planning reads it. The location is an open P4 fork: escher's README, a dedicated tracked escher document the README points to, or a file in Viola's incubator.
Every claim is read from what already stands: the driver's twelve verbs on their carriers, stable ids, the snapshot with its text, parts and diff, the screenshot, the lint's six defects, recording and replay, the load wait, the select and range models, keyboard activation and the virtual clock. The limits and the a11y claims are checked at P3 against the schema's help texts, the masters and the residuals.
Folds two CARRYs. README's MCP-role line moves from eleven driver commands to twelve. The unpaged diff and findings are stated as a limit: on a large screen either can pass 10,000 bytes, bounded only by the 1,048,576-byte answer bound. The chunk does not page them.
No code, engine, schema, verb, carrier, log field, span or test changes. A defect the statement finds is a wrap finding, not a fix. Acceptance is manual: the operator confirms the statement stands where the planning reads it.

## Reached files
- README.md · edits the MCP-role line's verb count (line 48) and the 0.2.0 stage list's mention of the statement

## Needs
- What the tests witness for each stated capability, so every claim cites a standing check or a measured reading and claims nothing unwitnessed: the stand checks, the session and MCP checks, the command-line process checks and flows, `stand_replay` and the committed Counter checks
- The pixel-test fact that a shot's PNG bytes differ between OSes, so a check compared byte for byte across OSes holds no shot. This is stated as a limit, together with the committed checks holding no `screenshot` step
- The boolean model's two limits (a mixed toggle reads unpressed; a grammar or spelling error reads invalid), pinned in dioxus-native-dom's unit tests and at the engine only
- The select's unmodelled parts (`multiple`, `optgroup`, no popup), which no check asserts. The statement must say these limits are stated but not asserted
- `text_extent`'s stated limits (RTL and vertical text read as LTR horizontal lines; a transformed box read at its client rect), asserted by the blitz-tests `text_extent` check
- The three measured-off readings (`off-screen` under a transformed scrolling box; `covered` on a zero-size box and on a box clipped on one axis) as the session and refusal checks measure them
- The large-screen-in-parts reading behind CARRY 2: `snapshot` pages over the budget (`stand_snapshot_parts`, the `many_items` fixture), while an acting verb's diff and a lint's findings are unpaged
- The a11y reach as tests witness it: `stand_keyboard`'s Tab sequences and Enter/Space, `stand_contrast`'s rows, the fourteen-file `a11y` CI leg, `inert_subtree`, and the accessibility-tree checks
- What is not measured, from the test scope: no check reads the accessibility states as the platform adapter hands them to an assistive technology (a residuals line, target next), and windowed end-to-end runs are not yet measured
- The masking witnesses behind the MCP-carries-onward section: a typed password never leaves in an answer or as pixels (session checks, `stand_mcp_masked`, `password_painted_masked`, `stand_act_screenshot`), and a password or file input reads the fixed mask
- The cold-agent pipe as the measured instance of a client carrying content onward: its one live run's verdict, the isolated twelve-tool requirement, its contract tests, and the fact that the live run was not re-fired after `lint` arrived
- The session host's write discipline: nothing of the screen reaches stdout, stderr or the state directory, while a paged `part` read through the host's own sink is unmeasured (a residuals line). This feeds the stated limits
- The MCP and command-line drivers as tested surfaces: the `mcp_server`, `mcp_log`, `stand_mcp_tools`, `cli_commands`, `cli_flow` and `host_timer` checks, plus the one real-client trial outside CI
- Indirect, through the README edit: the project's done-gate (`bash .github/scripts/ci-leg.sh fast` and `doc`) still runs on the tree that carries the edited README, and the edit must break no CI-script or workflow invariant
- Indirect, through the acting verbs' stated diff: the 1,048,576-byte answer bound and the `oversize` ending are the only bound on an unpaged diff or findings

## Not needed
- Coverage floors and the coverage leg: no crate's code changes, so no per-crate line reading moves
- The mutation tier: no unit of code is written or changed
- Unit-test conventions, assertion helpers and test-file placement: no test is written
- The agent-run contract's counts and the stand-repeat leg's target list: no test target is added, removed or renamed, so no tally is re-counted
- Fixture pages, seeds and the shared fixtures module: no fixture is added or edited
- Mocking and stubbing (stand-ins, `HeldNet`, `TimerTicks`): no stand-in changes
- WPT runner, WPT workflows and WPT scoring: upstream-only conformance, not part of the statement's capabilities
- apps/browser, blitz-vibey-script, the style-bridge, layout-unit and paint-unit tests: engine and upstream surfaces the statement does not describe
- The CI workflow invariants, caches, package installs and per-OS tallies: ci.yml and the leg script are not edited
- The incremental oracle, randomized stress and pure-restyle test methods: no layout or pipeline change
