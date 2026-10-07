# layouts extract

## Relevance
partial — the chunk creates and modifies no surface element (no wireframe, component placement, focus order, breakpoint, modal, navigation or empty state applies); it touches the plan only where the plan fixes the output shape of the two sink-installing binaries: the `escher-session` entry of layout-templates §Surface: cli → Primary screens and the stand mount of layout-templates §Surface: desktop-native → Primary screens. The surface-level `NOT YET MEASURED` markers of both surfaces name facets (tooling context, expression level, signature placement, hero/signature section) that nothing below relies on; §Decisions Log reads `NO RECORDED INTENT` and is not cited.

## Constraints
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires that the host write nothing to stdout on any path — the drop rule and the by-level measurement must leave stdout empty at every level `RUST_LOG` can name, `trace` included.
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires that the host's stderr log lines carry `service.name=seven_guis` — every line that still prints after the drop (engine-target and escher-target records) must keep that identity; whether any line survives at each level after the drop is research's question.
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires a closed argv of exactly two arguments, with any other argv printing the usage line to stderr and exiting 2 having booted nothing and created no state — the chunk adds no flag, verb or argument for selecting log targets, and the usage path must not be reshaped by the sink change.
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires exit 0 after the session is stopped and exit 1 with an error's fixed message on stderr — the drop must not swallow that fixed message; whether the message reaches stderr through the sink (and under which target) or past it is research's question.
- layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) requires that the `escher-session` host be a consumer of the stand mount that adds no markup, id, class, style, wrapper or order, with a held instance reading as a fresh boot — the chunk's change stays in the sink; the host's mount, the TaskShell structure and the pinned viewport are not to move.
- layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) fixes the author-id vocabulary of the stand — Home's task-card ids and each lean task's control, display and CRUD-row ids — which is the set of stable element ids the "no id at `trace`" check and the by-level measurement search stderr for; the windowed binary also renders Home, which the headless stand skips, so its measurement's id set includes the Home ids. Whether any of these ids reaches stderr at HEAD is research's question.

## Patterns to follow
- The cli entries share one output grammar — usage to stderr with exit 2 on a usage error, stdout reserved (JSON lines for the scripts, empty for the host), raw detail kept off stdout — and the new check reads the host inside that grammar rather than adding a channel (per layout-templates §Surface: cli → Primary screens, entries `scripts/agent-run.sh`, `scripts/cold-agent.sh`, `escher-session`).
- The host boots one lean task through `seven_guis::stand` only; a check of the sink-installing host spawns that real binary rather than building a second mount or a new fixture surface (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell).
- Element ids are author `id`s that add no class, style, wrapper or order and that no CSS selects by — the check's id needles are read from the stand's own ids, never from ids minted for the test (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell).

## Anti-patterns to avoid
- Printing anything to the host's stdout — a diagnostic, a drop counter or a by-level summary included (per layout-templates §Surface: cli → Primary screens, `escher-session`).
- Widening the host's closed two-argument argv, or adding a usage variant, to control the allowlist (per layout-templates §Surface: cli → Primary screens, `escher-session`).
- Adding markup, an id, a class, a style, a wrapper or an order change to the stand or the host to make the log check observable (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell).

## Contract bindings
- layouts ↔ obs: the `service.name=seven_guis` identity on the host's stderr lines (layout-templates §Surface: cli → Primary screens, `escher-session`) is produced by the sink the obs plan owns; the allowlist and the drop are obs's statement, the line shape the host shows is this plan's.
- layouts ↔ tests: the session contract the host serves is deferred by layout-templates §Surface: cli → Primary screens (`escher-session`) to test-plan §3 → Session lifecycle; the new host-spawning check sits beside that contract and must not contradict its exit and stdout grammar.
- layouts ↔ a11y: none — the chunk adds no focusable element, so no focus-order or modal binding arises.
- Wrap note: the scope's expected amendments name no layout-templates row; whether the `escher-session` entry's stderr sentence needs restating after the by-level measurement is the wrap cascade's question, not this chunk's edit.

## Acceptance criteria contributions
- (layouts) `escher-session` run with `RUST_LOG=trace` writes zero bytes to stdout, before and after the drop (per layout-templates §Surface: cli → Primary screens, `escher-session`)
- (layouts) Every stderr log line the host still prints after the drop carries `service.name=seven_guis` (per layout-templates §Surface: cli → Primary screens, `escher-session`)
- (layouts) The host's usage-error path (usage line on stderr, exit 2, nothing booted, no state created) and its exit codes 0 and 1 are unchanged by the sink change (per layout-templates §Surface: cli → Primary screens, `escher-session`)
- (layouts) A held instance still reads as a fresh boot with the TaskShell mount and the pinned viewport, in both layout modes, after the sink change (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell)
