# design extract

## Relevance
partial: the chunk adds no painted UI, colour, type or token. Its design reach is the agent-read CLI and MCP surface that carries the wait's answer and the stated bound, plus the virtual-time motion rules the wall-time wait must leave alone.

## Constraints
- An acting command's answer at the end of a wait, in either arm (`settled` with the loaded content, or `settled: false` · `busy: "loads"` at the bound), must still be one uncoloured line of JSON on stdout, keyed by the schema's own words, with nothing else on stdout. It writes no ESC byte and makes no terminal test (per design-system §Surface: cli §Component Patterns).
- The stated bound, when it joins the schema's help and `help`'s self-description, must ride as fixed `'static` table text. `escher-session help` must stay one uncoloured line of JSON, status `0`, reaching no session (per design-system §Surface: cli §Component Patterns).
- The exit status keeps carrying the class from the schema's `EXIT_STATUSES` table (`0` · `1` · `2` · `3`). A bound that passes with a load in flight adds no exit and no status meaning. Which status a `NotSettled` answer exits with today is research's question (per design-system §Surface: cli §Component Patterns).
- Over the `mcp` role, a waited command's reply must stay one newline-delimited JSON-RPC line on stdout, uncoloured and with no ESC byte. The tool result's text keeps its current shape: the JSON line for the acting verbs (per design-system §Surface: cli §Component Patterns).
- The harness drives animation from the `VirtualClock` its documents read. That clock moves only by `tick`, `advance_time` and the driver's `advance`, and settle never advances it. `click`, `type` and `press` leave it where it was, so a wall-time wait must not move it, and a timer or CSS animation must not run during a wait (per design-system §Motion §Animation runtime).
- The overlay scrollbar fade (`FADE_DELAY` 500 ms, then `FADE_DURATION` 200 ms) is read off the document's clock. After a driver `scroll`, settle's `animating` reads true until 700 ms of that clock pass. A `scroll` that waits out a load must not age the fade by the wall time it waited (per design-system §Motion §This project's values).
- A fixture whose arm asserts loaded content "on screen" by its bounds needs text that measures. blitz-tests builds blitz-dom with `system-fonts`, and without it text measures 0x0 and font-dependent assertions pass vacuously. The stand's own boot registers the bundled DejaVu Sans with system fonts off (per design-system §Typography §Loading).

## Patterns to follow
- Help and exit-status text is read from the schema's own tables (`EXIT_STATUSES`, served by `help`). State the bound the same way, as one table entry in `schema.rs` / `describe.rs`, never as a literal repeated per carrier (per design-system §Surface: cli §Component Patterns).
- The no-ESC reading is asserted on every answer line by the seven_guis checks' shared reader, and on every MCP reply by the shared `Speaker`. Any binary-run arm of the new check should go through those readers (per design-system §Surface: cli §Component Patterns).
- Precedent for "returns without waiting and without moving time": a driver click beside a running CSS animation returns settled without waiting on it, pinned by `stand_act_timer` and `stand_virtual_clock`. Check the wait's clock-neutrality in the same style (per design-system §Motion §Animation runtime).

## Anti-patterns to avoid
- Do not add colour, a progress or "waiting…" line, or any interim output on stdout or stderr while a command waits. The surface prints its one answer line and nothing else there (per design-system §Surface: cli §Component Patterns).
- Do not advance the session's `VirtualClock`, or hand the app's time step, to make a load "finish" or to pace the wait. `advance` stays the one mover of the session's clock (per design-system §Motion §Animation runtime).

## Contract bindings
- design-system §Surface: cli ↔ security-plan §API Security / §Input Validation: the bound's help text is the program's own fixed table text on the ratified carriers (command line stdout, `mcp` stdio). The answer shape (`settled` / `busy`) is closed words inside the ratified crossing.
- design-system §Motion §Animation runtime ↔ arch §Standard Contracts (Driver session; Harness settle): the `VirtualClock` and `advance` as the one time mover, and settle reading no wall clock.
- design-system §Typography §Loading ↔ test-plan §3/§8 (network stand-in rows, `HeldNet` and any self-answering stand-in): the fixture's font context determines whether "loaded content on screen" is a real reading.
- No a11y colour or contrast binding: the chunk paints and colours nothing.

## Acceptance criteria contributions
- Through `escher-session`, both arms' acting-command answers are exactly one line on stdout, with no ESC byte. The `help` self-description that states the bound is one uncoloured line, status `0` (per design-system §Surface: cli §Component Patterns).
- Over the `mcp` role, a waited acting call's reply is one JSON-RPC line with no ESC byte (per design-system §Surface: cli §Component Patterns).
- The harness's `time()` reads the same immediately before and after an acting command that waits to the bound with a load in flight, and the same after one whose load is answered inside the bound (per design-system §Motion §Animation runtime).
