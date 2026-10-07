# layouts extract

## Relevance
partial — the chunk renders nothing and adds no markup, id, class, style, wrapper or focusable element; the layout plan binds it only as surfaces it must leave as they read: the headless stand mount its check boots on, and the output streams of the CLI surfaces a span record could reach.

## Constraints
- layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) requires the headless stand to mount one lean task in TaskShell at the pinned viewport with the bundled font; the chunk's check boots a stand task through that mount and changes nothing of it.
- layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) requires a held instance to read as a fresh boot, and the TaskShell mount and pinned viewport to hold after commands, in both layout modes; a command that leaves a span must leave the same screen as one that leaves none. Whether the code already holds this with a `tracing` call site inside `Session::run` is research's question.
- layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) requires the lean tasks' author ids to add no class, style, wrapper or order, and no task CSS to select by them; the chunk adds no id and no markup to make a span observable.
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires the session host to write nothing to stdout on any path; a span record, in whichever form the plan chooses, goes to stderr through the sink only.
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires the host's stderr log lines to carry its service identity; if the plan has the sink print a span's record (an edit in `escher-telemetry`), that line shape is bound by the same clause. Whether the sink's formatter already puts the service identity on every line it writes, a span's included, is research's question.
- layout-templates §Surface: cli → Primary screens (`escher-session`) requires a closed argv of exactly two arguments, one usage line and three exit codes; the chunk adds no argument, flag or exit path to the host — the level a span prints at is reached through the existing `RUST_LOG` read, not through argv.
- layout-templates §Surface: cli → Primary screens (`scripts/agent-run.sh`) requires every verb's stdout to be JSON lines only, raw cargo output staying in the run log; the new `stand_*` check is run through that contract, so the span text it reads stays in its captured writer and never reaches the script's stdout.

## Patterns to follow
- Boot the check's instance the way the stand's other consumers do — one lean task through the stand mount, no Home — per layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell).
- Take the ids, names and values the check asserts absent from the captured text from the task it drives; the plan lists each lean task's author ids per layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell). Whether that list matches the ids the code reads today is research's question.
- Where the check needs a refused `off-screen` call or a `scroll` on the stand, the stand's one nested scroller is the case the plan records, per layout-templates §Surface: desktop-native → IA notes (the into-view scroll bullet); no fixture markup is added to the stand for it.
- Keep stdout and stderr apart as the plan's CLI surfaces do — machine-read output on stdout, log lines on stderr — per layout-templates §Surface: cli → Primary screens (`escher-session`).

## Anti-patterns to avoid
- Adding or moving stand markup, an id, a class, a wrapper or an order to give a span something to read — banned by layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell).
- Printing a span record to stdout, or adding a host argument to turn spans on — banned by layout-templates §Surface: cli → Primary screens (`escher-session`).
- Letting a check's log text reach the agent-run contract's stdout as a non-JSON line — banned by layout-templates §Surface: cli → Primary screens (`scripts/agent-run.sh`).

## Contract bindings
- layouts ↔ obs: the layout plan states only which stream the host's log lines use and that they carry the service identity (layout-templates §Surface: cli → Primary screens, `escher-session`); what a span's record holds and how its fields are judged is obs-plan §4 and §8 → Scrubbing.
- layouts ↔ tests: the host's session contract and the agent-run exit grammar and event schema are test-plan §3 (→ Session lifecycle for the host), as layout-templates §Surface: cli → Primary screens points to them; the new check joins the `stand` selection there.
- layouts ↔ a11y: not engaged — no focusable element, focus order or modal is added, so the SC 2.4.3 binding has nothing to bind.
- Wrap note: no layout-templates amendment is expected unless the plan changes what a sink-installing binary's stderr line reads; then the `escher-session` bullet of layout-templates §Surface: cli → Primary screens is the one clause to re-derive.

## Acceptance criteria contributions
- (layouts) The chunk's diff touches no stand markup: no change under the seven_guis task, app or stand sources adds or moves an element, id, class, style or wrapper (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell).
- (layouts) A stand task driven by commands that leave spans still reads the TaskShell mount at the pinned viewport afterwards, in both layout modes (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell).
- (layouts) `escher-session` still writes nothing to stdout on any path, with its usage line, argv and exit codes unchanged (per layout-templates §Surface: cli → Primary screens, `escher-session`).
- (layouts) Running the new check through the agent-run contract leaves that script's stdout JSON lines only (per layout-templates §Surface: cli → Primary screens, `scripts/agent-run.sh`).
