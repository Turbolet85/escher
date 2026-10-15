# design extract

## Relevance
partial — the chunk renders nothing and adds no tokens. Its design reach is the agent-read output contract of the `escher-session mcp` role that the swapped pipe consumes, plus the stand's bundled font and its recorded colours, which the live task must leave as they are.

## Constraints
- design-system §Surface: cli → Component Patterns requires the `mcp` role's stdout to hold newline-delimited JSON-RPC 2.0 protocol lines only: one reply per request that is not a notification, every reply one line, uncoloured, with no ESC byte, and no code asking whether a stream is a terminal. The swapped `scripts/cold-agent.sh` gives the fresh session this surface as it is specified and must not wrap, recolour or reframe it.
- design-system §Surface: cli → Component Patterns fixes a tool result's shape by verb: `snapshot` returns the screen text, `help` the self-description's line, and an accepted `screenshot` an `image` block (`image/png`, base64) then a text block that is the shot's line without `png`. Every other verb returns its JSON line, and a refusal or session error is `isError`. Any completion oracle or call count that reads the tool's answers (scope items 2–3) has to read each verb's shape, never one shape for all.
- design-system §Surface: cli → Component Patterns sends the end of the `mcp` role to stdout as nothing at all, and an `Err` ending's fixed message to stderr with status `3`. Stdout is protocol only, so a second call count drawn from the role's output belongs on stderr, never mixed into the protocol stream. Whether escher's sink writes the `tool_call` span there for each call is research's question (see Contract bindings).
- design-system §Typography → Loading requires the headless stand to register the bundled DejaVu Sans for every generic family, with system fonts off. The same section requires a driver `screenshot` to be refused `fonts-unavailable` when the app resolves no `sans-serif` family. A stand task booted for the cold agent has to come through that bundled-font boot, or a `screenshot` call becomes a refusal the pipe would count as a wrong call. Whether the `mcp` role boots the four tasks through the same stand boot is research's question.
- design-system §Color Palette → Core Colors records the seven_guis accent (`#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`) and the page and text values, each as measured by `stand_contrast`. The chunk's boundaries name no stand change, so the live task and any fixture it adds must leave every stand colour declaration as recorded.
- design-system §Motion → Animation runtime requires a driver `advance` to move only the app's own time, through the step the session's caller hands over (whole Timer ticks on the stand), and to leave the harness animation clock where it was. A `click`, `type` or `press` returns settled without waiting on a CSS animation. This applies if the live run's stand task is the Timer: scope item 9 lists `advance` and the `timer` app as not yet measured live.

## Patterns to follow
- The agent-read surface pattern: one line per answer, keyed by the driver schema's own words, the exit status carrying the class (`0` · `1` · `2` · `3`, read from `EXIT_STATUSES`), with the same output whether piped or not (design-system §Surface: cli → Component Patterns). It is the form the fresh session sees, so any verdict evidence the pipe reads from answers keeps these classes.
- The in-band screenshot: the image as an MCP `image` block followed by a text block, never a separate carrier (design-system §Surface: cli → Component Patterns). If the cold agent takes a shot, the transcript holds it in that form.
- The stand's headless boot: pinned bundled font, system fonts off, through `HarnessOptions.font_ctx` (design-system §Typography → Loading). It is the boot path a cold-agent stand task reuses rather than duplicates.

## Anti-patterns to avoid
- Colour, ESC bytes or a terminal test on any line the escher server or the pipe hands an agent (design-system §Surface: cli → Component Patterns).
- Reading verdict evidence from a line that is not protocol: a log or stderr record mixed into stdout, or the agent's own reply standing in for the tool's answer. Stdout is protocol lines only (design-system §Surface: cli → Component Patterns).

## Contract bindings
- design-system §Surface: cli ↔ test-plan §3: the shared `Speaker` asserts no ESC byte on every reply. The contract tests in `.github/scripts/test_cold_agent.py` (shim-only) should read the swapped server's replies under the same single-line, uncoloured shape.
- design-system §Surface: cli ↔ obs-plan §3 (the `tool_call` span): stdout carries protocol and stderr carries the sink's records. The independent call count of scope item 2 sits on the obs side of that split.
- design-system §Color Palette → Core Colors ↔ a11y-plan §1 (SC 1.4.3): the binding is live only if a stand colour moves. `stand_contrast` holds each recorded pair, and the chunk is expected to move none.

## Acceptance criteria contributions
- Every reply line the swapped pipe's escher server writes on stdout is a single JSON-RPC line with no ESC byte, and nothing else reaches stdout (per design-system §Surface: cli → Component Patterns).
- The pipe's tool-answer readers read `snapshot` as screen text, `help` as the self-description line, an accepted `screenshot` as an `image` block followed by a text block without `png`, and every other verb as its JSON line (per design-system §Surface: cli → Component Patterns).
- In the live run and the shim tests, a `screenshot` on the cold agent's stand task is never refused `fonts-unavailable`: the task boots with the bundled sans-serif font (per design-system §Typography → Loading).
- No stand colour declaration changes in the chunk's diff. If one does, its `stand_contrast` row moves with it (per design-system §Color Palette → Core Colors).
