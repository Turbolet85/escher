# layouts extract

## Relevance
partial — the chunk adds no rendered surface, wireframe, focusable element, modal or breakpoint. Its only layout-domain touchpoint is the agent-facing CLI output structure (layout-templates §Surface: cli §Primary screens). Only if P4 chooses a stand-backed stub task does it also touch §Surface: desktop-native §Primary screens. The header notes of both surfaces (`NOT YET MEASURED`: tooling context, expression level, signature placement) and the §Decisions Log (`NO RECORDED INTENT`) are not cited.

## Constraints
- If the pipe joins `scripts/agent-run.sh` as a further verb, layout-templates §Surface: cli §Primary screens requires the script's usage line `agent-run.sh <verb> [selection]` and its verb enumeration (`boot · run · status · cleanup · logs`) to name it. A usage error prints the usage to stderr and exits 2 (per layout-templates §Surface: cli §Primary screens).
- layout-templates §Surface: cli §Primary screens requires every verb's stdout to be JSON lines only, one object per event, with raw tool output kept in a file under `target/agent-run/`. The pipe's transcript and the agent client's raw output belong in files under `target/`, never on stdout. Only the verdict and progress events go on stdout (per layout-templates §Surface: cli §Primary screens).
- The CLI contract runs from the repository root (per layout-templates §Surface: cli §Primary screens). The pipe's invocation and any paths it writes resolve from the root.
- layout-templates §Surface: cli §Primary screens names `scripts/agent-run.ps1` as the Windows pass-through. Research has to answer whether a new verb reaches the client through `agent-run.ps1` unchanged or needs an edit there. The same section defers the exit grammar and event schema to test-plan §3, so this domain sets neither.
- If P4 has the stub task boot the stand, layout-templates §Surface: desktop-native §Primary screens fixes its layout: one lean task mounted in TaskShell through `task_in_shell` (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. The stub reads that structure as built and does not reshape it. Stand content is out of scope (per layout-templates §Surface: desktop-native §Primary screens).

## Patterns to follow
- The stand test contract's CLI shape: a verb plus an optional selection, a stderr usage line, JSON-line stdout, and raw output in a file under `target/agent-run/` (layout-templates §Surface: cli §Primary screens). This is the precedent the scope names for the pipe's invocation surface.
- `bump`'s single-line output for a single-result command (layout-templates §Surface: cli §Primary screens). It is a model for one verdict record per run.
- The headless stand's boot path through `seven_guis::stand` into TaskShell at the harness-default viewport (layout-templates §Surface: desktop-native §Primary screens; §IA notes, which give the test harness default of 800x600, scale 1, light). Applies only if the stub task uses the stand.

## Anti-patterns to avoid
- The wpt runner's human-terminal output on the pipe's stdout: in-place ANSI cursor rewriting, `[done/count]` progress lines, an "Ordered Results" heading or a right-aligned summary block (layout-templates §Surface: cli §Output structure — wpt runner). It conflicts with the JSON-lines-only stdout that §Surface: cli §Primary screens requires.
- Writing the transcript or the agent client's raw stream to stdout or interleaving it with the JSON events (per layout-templates §Surface: cli §Primary screens: raw output stays in a file).

## Contract bindings
- cli output structure ↔ tests: the exit grammar (0/1/2/3) and the JSON-line event schema the pipe emits are owned by test-plan §3 (cross-referenced from layout-templates §Surface: cli §Primary screens). The pipe's verdict event fields bind there, not here.
- cli files under `target/` ↔ arch §Occupied Resources → Filesystem: the transcript and run-state paths that the CLI output structure points to are a registration at wrap.
- stand layout ↔ tests (conditional): a stand-backed stub task binds to the stand's TaskShell structure and pinned viewport that `tests/blitz-tests/tests/stand_boot.rs` checks (layout-templates §Surface: desktop-native §Primary screens).
- focus order ↔ a11y SC 2.4.3: (none). The chunk adds no focusable element.

## Acceptance criteria contributions
- (layouts) Every line the pipe's invocation writes to stdout parses as exactly one JSON object. The transcript and the client's raw output exist as files under `target/` and do not appear on stdout (per layout-templates §Surface: cli §Primary screens).
- (layouts) A malformed invocation of the pipe prints its usage line to stderr, writes nothing to stdout and exits 2 (per layout-templates §Surface: cli §Primary screens).
- (layouts) If the pipe is an `agent-run.sh` verb, the script's usage line names it and `scripts/agent-run.ps1` forwards it. If it is its own script, its usage line follows the same `<script> <verb> [selection]` shape (per layout-templates §Surface: cli §Primary screens).
- (layouts) If the stub task boots the stand, the booted document has `main#main > #task-shell` at the 800 × 600, scale 1.0, Light viewport, unchanged by the pipe (per layout-templates §Surface: desktop-native §Primary screens).
