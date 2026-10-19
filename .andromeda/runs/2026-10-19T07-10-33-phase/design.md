# design extract

## Relevance
partial — no painted UI, tokens or layout change; the chunk records and replays sessions through `escher-session`'s two agent-read surfaces (the command line and its `mcp` role) and keeps their answer lines at rest, and its replays depend on the stand's pinned font and virtual clock.

## Constraints
- design-system §Surface: cli (Component Patterns) requires `escher-session` to print one line of JSON on stdout per command (the answer, the refusal or a session error's object, keyed by the driver schema's own words) and nothing else there. Any replay entry point this chunk adds to the binary, if it adds one, must keep that form. Whether a replay surface is added at all is P4's.
- design-system §Surface: cli (Component Patterns) requires no colour, no colour dependency, no ESC byte on any answer line, and no terminal test, so output is the same piped or not. An `answer` kept "as the driver wrote it" is that uncoloured line. Whether the kept line is exactly `Outcome::to_json` / `Refusal::to_json`'s output is research's question.
- design-system §Surface: cli (Component Patterns) requires the exit status to carry the class (`0` accepted · `1` refused · `2` usage · `3` session error), read from the schema's `EXIT_STATUSES` table. `record-dir-not-private` and `recording-failed` are session errors with no new exit. A replay that reads red must not add an exit status to the binary's table unless that is ruled as a new exit.
- design-system §Surface: cli (Component Patterns) requires an accepted `screenshot` over the `mcp` role to give an `image` block, then a text block that is "the shot's line without `png`". The scope's F3 B form, a screenshot step keeping its answer without `png`, matches that existing text-block shape. Whether the digest still hashes the whole line, PNG included, is P4's.
- design-system §Typography (Loading) requires the stand's headless boot to register the bundled DejaVu Sans for every generic family with system fonts off, and a `screenshot` of an app with no `sans-serif` family to be refused `fonts-unavailable`. A replay on a fresh boot depends on that pinned font for a stable screenshot answer and digest across hosts.
- design-system §Motion (Animation runtime) requires the driver's `advance` to move the harness's `VirtualClock` by exactly its `ms`, and the app's own time through the caller's step (the Timer's whole ticks). It also requires `click`, `type` and `press` to leave the clock where it was. A replayed `advance` step can reproduce its answer only through the same `with_time` step the recording session had. Whether the replay runner hands it that step is research's question.

## Patterns to follow
- One answer form per carrier, per design-system §Surface: cli (Component Patterns): the command line's single JSON line; over MCP, the screen text for `snapshot`, the self-description line for `help`, an image block then a text block for `screenshot`, and the JSON line otherwise. A kept `answer` follows the driver's JSON line, not the MCP screen text (scope: a `snapshot`'s answer is the JSON line).
- The stand's headless boot, per design-system §Typography (Loading): pinned viewport, bundled font through `HarnessOptions.font_ctx`, system fonts off, fresh per boot. A replay target boots each session this way.
- Time moved only by `advance`, per design-system §Motion (Animation runtime): settle never advances the clock, and a step beside a running CSS animation returns settled without waiting on it. A replay therefore needs no waits or sleeps.

## Anti-patterns to avoid
- Colour, ESC bytes or a terminal test in any replay or recording output on the command line or the `mcp` role's stdout (per design-system §Surface: cli (Component Patterns)).
- A replay that boots with system fonts or the wall clock instead of the stand's bundled font and the harness's `VirtualClock`. Its screenshot and `advance` answers would then differ by host and time, not by app behaviour (per design-system §Typography (Loading) and §Motion (Animation runtime)).

## Contract bindings
- design ↔ tests: design-system §Surface: cli (Component Patterns) records that the no-ESC reading is asserted on every answer line by the seven_guis checks' shared reader, and on every MCP reply by the shared `Speaker` (`examples/seven_guis/tests/common/mod.rs`). `examples/seven_guis/tests/recording.rs` and any new replay check that drives the binary bind to those readers.
- design ↔ security: the kept `answer` line (ids, names, values, snapshot text and diff at rest, a shot's pixels excepted) is the same uncoloured line the cli surface writes. Its at-rest ruling belongs to the security plan's §Surfaces (the F3 → B widening), not to design.

## Acceptance criteria contributions
- Every stdout line `escher-session` writes in the chunk's recording and replay runs (command line and `mcp` role) is one uncoloured line with no ESC byte, and each exit status is one of `EXIT_STATUSES`' four (per design-system §Surface: cli (Component Patterns)).
- A kept `screenshot` step's `answer` holds no `png` key and equals the shot's line without `png` that the MCP text block carries (per design-system §Surface: cli (Component Patterns)).
- A committed check holding a `screenshot` step replays green on two fresh boots in separate processes, which needs the bundled-font stand boot (per design-system §Typography (Loading)).
- A committed check holding an `advance` step on the Timer replays green on the unchanged stand, which needs the same time step as the recording (per design-system §Motion (Animation runtime)).
