# layouts extract

## Relevance
partial — the chunk creates and modifies no surface, region or focusable element; it touches the layouts plan only where its checks and CI legs run against recorded surfaces: the headless stand mount that repeat runs boot, and the `agent-run.sh` and `escher-session` command-line surfaces (CARRY 10, and any repeat runner the flakiness budget adds).

## Constraints
- The headless stand mounts one lean task in TaskShell (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. Every consumer of that boot adds no markup, id, class, style, wrapper or order, and a held instance must read as a fresh boot (per layout-templates §Surface: desktop-native → Primary screens). A flakiness budget's repeat runs depend on that fresh-per-boot mount. Whether repeated stand runs read identical screens is research's question.
- The stand's layout facts are recorded as stated: each control's author `id`, CRUD's six-stop Tab sequence `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`, and the attribute-only roles, states and names. A gate or a check added by this chunk must not move any of them (per layout-templates §Surface: desktop-native → Primary screens).
- The stand's nested-scroll geometry is a recorded reading. `crud-list` reads (24, 112.796875, 369 × 419.203125) at boot, `task-body` reads (0, 46, 800 × 554), and the pinned viewport is scrolled only on an in-file fixture taller than it (per layout-templates §Surface: desktop-native → IA notes). If a flake definition compares readings across repeats, these are the figures a repeat is expected to reproduce. Whether they hold run to run is research's question.
- `scripts/agent-run.sh` keeps its surface: usage `agent-run.sh <verb> [selection]`, the verbs `boot · run · status · cleanup · logs`, the selections `stand · all · <blitz-tests file name>`, stdout JSON lines only, a usage error on stderr with exit 2, and raw cargo output in `target/agent-run/run.log` (per layout-templates §Surface: cli → Primary screens). If the repeat mechanism adds a verb or a selection there, it must keep that grammar. Whether the chunk touches `agent-run.sh` at all is P4's decision.
- `escher-session` has four endings. Accepted writes one uncoloured JSON line on stdout and exits 0. Refused writes the refusal's JSON on stdout and exits 1. Usage writes one fixed line on stderr, leaves stdout empty and exits 2. A session error writes its object on stdout and its fixed message on stderr, and exits 3. No ending writes back anything the caller typed (per layout-templates §Surface: cli → Primary screens). CARRY 10's checks (a closed-pipe write dropped rather than a panic, and the `ANSWER_BOUND` read) must assert against these endings and leave them unchanged.
- The `escher-session mcp` role writes protocol lines only on stdout, uncoloured with no ESC byte. Its end writes nothing on stdout. The host's stderr log lines carry `service.name=seven_guis` (per layout-templates §Surface: cli → Primary screens). CARRY 10's untaken stderr-by-level readings are taken on stderr alone and must not add output on stdout.

## Patterns to follow
- A case the stand lacks gets an in-file fixture beside the stand, with no stand markup, id, class or style moved, as the viewport-scroll case was proven (per layout-templates §Surface: desktop-native → IA notes). Fit CARRY 4's not-settled `busy` reading and CARRY 9's hint-less font check to this pattern.
- Each process-level consumer boots through the one `seven_guis::stand` boot and adds nothing to the mount: the `escher-session` host role and its `mcp` role (per layout-templates §Surface: desktop-native → Primary screens). A repeat runner over the process-level checks (`cli_*`, `host_*`, `mcp_*`) inherits this.
- An agent-run script keeps JSON lines on stdout and raw output in a log under `target/`, as `scripts/agent-run.sh` and `scripts/cold-agent.sh` do (per layout-templates §Surface: cli → Primary screens). A repeat runner or flake report built as a script follows that shape.

## Anti-patterns to avoid
- Making a stand check deterministic by changing the stand's markup, ids, classes, styles, wrappers, order or Tab sequence (per layout-templates §Surface: desktop-native → Primary screens).
- Putting anything but the command's answer on `escher-session`'s stdout, such as a log line, a diagnostic from a new check path or caller-typed text echoed back (per layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts §Surface: cli → `scripts/agent-run.sh` ↔ tests: its exit grammar and event schema live in test-plan §3. A repeat verb or selection is a test-plan §3 contract change as well as a cli-surface change.
- layouts §Surface: cli → `escher-session` ↔ tests: the session contract it serves is test-plan §3 → Session lifecycle. CARRY 10's checks bind to both.
- layouts §Surface: desktop-native → Primary screens (CRUD's Tab sequence) ↔ a11y: SC 2.4.3 focus order, asserted by `stand_keyboard`'s tables. If the flakiness budget covers `stand_keyboard`, a flake there reads against this recorded order.

## Acceptance criteria contributions
- (layouts) The chunk's diff moves no stand markup, id, class, style, wrapper or order under `examples/seven_guis/src/app.rs` and `examples/seven_guis/src/tasks/`, and CRUD's Tab sequence keeps its six stops (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) A case added for CARRY 4 or CARRY 9 that the stand lacks lives in an in-file fixture, not in stand markup (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) If `scripts/agent-run.sh` gains a verb or a selection, every verb's stdout is still JSON lines only, and a usage error still prints on stderr with empty stdout and exit 2 (per layout-templates §Surface: cli → Primary screens).
- (layouts) CARRY 10's closed-pipe and `ANSWER_BOUND` checks each observe one of the four `escher-session` endings unchanged (stdout one line or empty, exit 0 · 1 · 2 · 3), and nothing typed by the caller is written back (per layout-templates §Surface: cli → Primary screens).
