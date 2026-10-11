# design extract

## Relevance
partial — the chunk paints nothing and touches no markup, style or token (scope §Surfaces and contracts touched says so itself), so no colour, type-scale, spacing, radius, depth or icon token applies; what binds is the plan's record of the driver's agent-read output surface (design-system §Surface: cli → Component Patterns), the stand's font loading (design-system §Typography → Loading) and the driver's motion readings (design-system §Motion), each of which a second driver surface either inherits or must not disturb.

## Constraints
- design-system §Surface: cli → Component Patterns records the driver's command line as a surface an agent reads, not a person: stdout holds the answer and nothing else, keyed by the driver schema's own words. The MCP server's stdout is a second agent-read stream of the same driver; the plan requires that discipline of `escher-session` and names no MCP surface — whether the MCP entry point lives in that binary (and so falls under this entry as written) is the operator's fork, and the entry itself needs an as-built line for MCP at wrap either way.
- design-system §Surface: cli → Component Patterns requires no colour and no colour dependency on the driver's command line, and no ESC byte on any answer line. A tool result that carries the driver's written forms (scope "A tool result's form") must keep that reading; whether the reused or replaced writer still emits no ESC byte is research's question.
- design-system §Surface: cli → Component Patterns requires that no code asks whether a stream is a terminal, so output reads the same piped or not. An MCP stdio server is always piped; the requirement is that it adds no terminal test to the crate, not merely that it happens to run without one.
- design-system §Surface: cli → Component Patterns fixes where each class of the command line's output goes — the answer on stdout, a usage error as one fixed line on stderr with stdout empty, a session error's fixed message on stderr — and the four exit-status classes. If the MCP server enters as a role of the same binary, those readings of the existing verbs must stand unchanged; whether a new role's argv misuse is a usage error of that same class is research's question.
- design-system §Typography → Loading requires the seven_guis headless stand to register its one bundled font for every generic with system fonts off. A stand the MCP server boots must come up through that same boot, so the bounds a tool result carries do not depend on the host's fonts; whether the server's boot path is the stand's own is research's question.
- design-system §Motion → This project's values requires the driver's `scroll` to be instant in every scrolling box and in the viewport, made inside one settled step. A `scroll` tool exposes that verb as it is: no smooth behaviour, no animation, no second step.
- design-system §Motion → Animation runtime requires that a settle neither waits on nor advances an animation, and that the driver's `advance` moves the app's own time while the harness's animation clock stays where it was. The tools inherit both readings; the MCP layer adds no wait, tick or clock of its own.

## Patterns to follow
- The `escher-session` output discipline as the model for the second agent-read surface: one written form per answer, schema words as keys, nothing decorative, same bytes on a pipe as anywhere (per design-system §Surface: cli → Component Patterns).
- The measurement pair the plan records for that surface — the no-ESC reading asserted on every answer line by the seven_guis checks' shared reader, the no-terminal-test reading by a search of the crate (per design-system §Surface: cli → Component Patterns). Whether that shared reader can read an MCP result line as it stands is research's question.
- The stand's headless boot with its bundled font through the harness options, rather than a second boot path with a font context of its own (per design-system §Typography → Loading).
- Driver verbs as instant, settled steps with time moved only by `advance` (per design-system §Motion → This project's values; design-system §Motion → Animation runtime).

## Anti-patterns to avoid
- Nothing is extracted from design-system §Anti-Patterns, which reads no recorded intent; the bans below come from the sections cited on each.
- No terminal styling on the protocol stream or on the server's stderr lines: the result colours and the conditional hyperlinks of design-system §Surface: cli → Tokens are the WPT runner's, a person-read surface, and are not a pattern for a driver surface, which design-system §Surface: cli → Component Patterns requires to be uncoloured with no colour dependency.
- No branch on whether a stream is a terminal — not for formatting, buffering or a banner (per design-system §Surface: cli → Component Patterns).
- No animated or smooth `scroll` and no wait on an animation behind a tool call (per design-system §Motion → This project's values; design-system §Motion → Animation runtime).

## Contract bindings
- design ↔ security / obs: "stdout holds the answer and nothing else" (design-system §Surface: cli → Component Patterns) is the same line the scope draws as "the server's stdout is the protocol, never a log"; the stderr side — what a telemetry line may hold — is security-plan's and obs-plan's, not this plan's.
- design ↔ tests: the no-ESC and no-terminal-test readings are recorded in design-system §Surface: cli → Component Patterns as measured by checks; extending them to the MCP stream is a test-plan contribution, placed by tests.
- design ↔ layouts: the bounds a tool result carries depend on the stand's pinned viewport (layouts) and its bundled font (design-system §Typography → Loading); design owns the font half only.
- design ↔ a11y: none of the token bindings fires — no painted colour, no motion token, no state colour is added; the three state readings cross as words, which is a11y's and security's subject.
- design ↔ wrap: design-system §Surface: cli → Component Patterns has no entry for an MCP surface; an as-built entry is owed at this chunk's wrap. design-system §Surface: cli → Navigation Pattern is unmeasured and nothing is taken from it.

## Acceptance criteria contributions
- (design) Every line the MCP server writes on stdout holds no ESC byte, asserted on every result line a check reads, a refusal's and a protocol error's included (per design-system §Surface: cli → Component Patterns).
- (design) After the chunk, a search of the driver crate reads no terminal test and its manifest names no colour dependency (per design-system §Surface: cli → Component Patterns).
- (design) The existing command line reads as before with the MCP entry point present: one line of JSON on stdout per command, a usage error's stdout empty, the four exit-status classes unchanged — the existing seven_guis command-line checks pass unedited (per design-system §Surface: cli → Component Patterns).
- (design) A `snapshot` tool result on a stand task reads the same bounds as the command line's `snapshot` of the same freshly booted task, in both layout modes — the MCP-booted stand uses the bundled font with system fonts off (per design-system §Typography → Loading).
