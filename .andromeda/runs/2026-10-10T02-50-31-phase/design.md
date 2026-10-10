# design extract

## Relevance
partial — the chunk touches no markup, style or colour token; the plan's measured content that reaches it is §Surface: cli (Platform · Toolkit · Tokens · Component Patterns), §Typography → Loading and §Motion (the driver's scroll and time rows). §Surface: cli → Navigation Pattern and → Platform-Specific Notes are marked NOT YET MEASURED, and Brand Identity, Anti-Patterns, Self-Validation Protocol and Design Decisions Log read NO RECORDED INTENT: nothing is extracted from any of them, so decisions 2 and 3 of the scope (how a session is addressed, command and argument names) get no input from this domain.

## Constraints
- The only terminal styling the plan records for a command-line surface belongs to the WPT runner — result colours and bright-black markers (per design-system §Surface: cli → Tokens (platform-specific)). The plan records no colour token for a driver CLI and requires none; the chunk's uncoloured stdout takes no token from this section and adds none.
- The plan records a second class of escape sequence on this surface beside colour: hyperlinks emitted only when stdout is a terminal (per design-system §Surface: cli → Tokens (platform-specific)). The chunk's stdout carries no escape sequence in a terminal or out of one, so "uncoloured" has to be read as "no escape sequence of either class", and no output may vary with whether stdout is a terminal.
- The colour toolkit the plan names is a dependency of the WPT runner alone (per design-system §Surface: cli, the Toolkit / Framework line). Whether the driver CLI's stderr diagnostics take any styling is not recorded in the plan; a styled diagnostic would be a new entry on this surface, not a reuse of the runner's result colours.
- The headless stand's text is measured with one bundled font registered for every generic, system fonts off (per design-system §Typography → Loading). The bounds a hosted `snapshot` writes to stdout are font-dependent, so the instance a CLI command reaches has to be booted with that same font context. Whether the host binary's boot path already keeps it is research's question.
- The driver's `scroll` is instant in every scrolling box and in the viewport, made inside one settled step, whatever behaviour a document asks (per design-system §Motion → This project's values, the into-view scroll row). A CLI `scroll` exposes no behaviour or duration argument and its one answer already holds the in-view reading.
- Settle neither waits on nor advances any member of the animating set, and `advance` moves the app's own time through the caller's step while the animation clock stays where it was, as it does for `click`, `type` and `press` (per design-system §Motion → Animation runtime). A CLI command adds no wait for an animation or a transition to finish, and a hosted `advance` moves time only through the step the host's session carries.
- The plan's cli surface enumerates the repository's command-line binaries and what each prints (per design-system §Surface: cli, the Platform line). The chunk adds one more, so the scope's "design-system — none expected" holds for markup, style and tokens only; whether that record gains the new binary and its output form is the wrap's reconciliation, named here so it is not missed.

## Patterns to follow
- Success output on stdout and errors on stderr, as the plan records for the release tool (per design-system §Surface: cli → Component Patterns, the bump row) — the split the chunk's "diagnostics on stderr" extends. The other recorded binaries print human-readable status lines to the terminal (per design-system §Surface: cli → Component Patterns, the paint_bench and screenshot rows); that form is not carried to the driver CLI's stdout, which holds one JSON answer.
- One bundled font for every generic through the harness options' font context, system fonts off (per design-system §Typography → Loading) — the stand's determinism pattern, to be inherited by the hosted instance rather than rebuilt.
- One instant, settled step per acting command, with time moved only by the caller-supplied step (per design-system §Motion → This project's values and → Animation runtime) — the CLI carries that step out of the process unchanged.

## Anti-patterns to avoid
- The plan's own ban list holds no recorded intent, so no plan ban is extracted. The two entries below are cautions read from measured sections against the chunk's scope, not bans the plan states.
- Borrowing the WPT runner's terminal-conditional output for the driver CLI — colour or hyperlinks switched on a terminal check (per design-system §Surface: cli → Tokens (platform-specific)): the same command would then answer differently to an agent and to a person.
- Giving a CLI `scroll` or any acting command a smooth or timed variant (per design-system §Motion → This project's values, the into-view scroll row): the document holds one scroll animation at a time and the driver's scroll is defined as instant.

## Contract bindings
- design §Surface: cli (stdout answer, stderr errors) ↔ obs: the telemetry sink's lines share stderr with the CLI's diagnostics; what that stream carries is the obs plan's, the stream split is the pattern recorded here.
- design §Typography → Loading ↔ tests: a check that compares bounds read through the CLI with the in-process reading depends on the pinned font context, and on the layout mode the host boots in.
- design §Motion (instant scroll · settle and the animating set · `advance`) ↔ architecture §Standard Contracts (Driver session) and tests (no sleep in a check): the CLI is a new caller of the same contract, not a new timing surface.
- design §Surface: cli → Tokens ↔ security: an escape sequence in stdout would sit beside ids, names and values in the one stream an agent parses; the no-escape reading above is also what keeps that stream inert.

## Acceptance criteria contributions
- Every command's stdout holds zero ESC bytes (0x1b), read both with stdout a pipe and with stdout a terminal — one count that covers both escape classes the plan records on this surface (per design-system §Surface: cli → Tokens (platform-specific)).
- A command's stdout is byte-identical whether stdout is a terminal or a pipe, for the same session state (per design-system §Surface: cli → Tokens (platform-specific)).
- A hosted `snapshot` of a stand task reads the same bounds as the in-process snapshot of that task in the same layout mode (per design-system §Typography → Loading).
- A hosted `scroll` returns its in-view reading in its own answer, and the `snapshot` that follows it reads the same position — no intermediate scroll position is observable from the command line (per design-system §Motion → This project's values).
