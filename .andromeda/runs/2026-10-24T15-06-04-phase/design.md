# design extract

## Relevance
partial — the chunk builds checks and fixture pages, not UI; design binds only where a fixture renders text, the stand's painted colours must stay put, a lint or hit reading leans on the plan's clip and motion rules, and the driver's agent-read stdout is measured.

## Constraints
- A fixture check that asserts text geometry (C1's `truncated` reading, `text_extent`'s RTL, vertical and transformed-box limits) needs a font context that resolves text: design-system §Typography (detail `.andromeda/design-system-detail/typography.md`, "Loading") requires blitz-dom built with `system-fonts` for blitz-tests, or the stand's bundled DejaVu Sans through `build_single_font_ctx`; without either, text measures 0x0 and font-dependent assertions pass vacuously. Whether each existing 0.2.0 fixture witness already runs under such a context is research's question.
- A `screenshot` or `lint` witness on an app whose font context resolves no `sans-serif` family must read the refusal `fonts-unavailable` with nothing painted or read; design-system §Typography ("Loading") requires that a shot never paints with text skipped.
- The sweep re-proves and must not repaint the stand: design-system §Color Palette › Core Colors fixes the seven_guis accent at `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, white button text on it passing SC 1.4.3 at rest, on hover and when active. A new fixture beside the stand takes no stand colour as a token.
- design-system §Color Palette › Semantic Colors sets the stand's invalid (`#e53e3e` / `#fff5f5` / `#c53030`) and success (`#ebf8ee` / `#68d391` / `#276749`) values in the flight booker. A sweep row that witnesses `invalid` or `selected` reads them from the markup's `aria-*` state, not from colour.
- C1's clipped/off-screen readings follow design-system §Depth Strategy ("Engine depth behavior"): a hit stops at the padding box of a node paint clips (by `overflow`, or a computed `contain` that includes `paint`), the root excepted, and the node is still answered inside its border box. A fixture planting a clipped text node is built on that rule. Whether `lint`'s `clipped` and `truncated` readings agree with it on such a node is research's question.
- A time-dependent witness moves time only through the harness's `VirtualClock`: design-system §Motion ("Animation runtime") requires that settle never advances it, `advance` moves it by exactly `ms`, `click`/`type`/`press` leave it where it was, and the load wait moves no clock. design-system §Motion ("This project's values") makes the driver's `scroll` instant in every box and in the viewport.
- design-system §Surface: cli › Component Patterns requires the stdout of `escher-session` and its `mcp` role to be JSON only, one line per command or JSON-RPC reply, with no colour, no ESC byte and no terminal test. A usage or session error's fixed message goes to stderr, and the exit status carries the class (`0` · `1` · `2` · `3`). This holds for the 0.2.0 additions the sweep measures: `lint`'s `{"findings":[…]}`, `snapshot --part`'s `{"text","part","parts"}` and a `--record` session's `record-dir-not-private` and `recording-failed`.

## Patterns to follow
- 0.1.0's sweep pinned hit readings at an element's own box in both layout modes and both builds (`img`, `iframe` `srcdoc`, filled text `input`, a transformed-and-scrolled box hit where its rows show), per design-system §Depth Strategy (detail `.andromeda/design-system-detail/depth-strategy.md`). A 0.2.0 fixture row follows that shape: one behaviour per fixture, `for incremental in [false, true]`.
- The stand's headless boot registers bundled DejaVu Sans for every generic, with system fonts off (`build_single_font_ctx` through `HarnessOptions.font_ctx`), per design-system §Typography (detail, "Loading"). That is the font precedent for a fixture whose text geometry must be the same on every host.
- The seven_guis checks' shared answer reader and the shared `Speaker` assert no ESC byte on every answer line and every MCP reply, per design-system §Surface: cli › Component Patterns (detail `.andromeda/design-system-detail/surface.md`, cli-output). New verb witnesses reuse those readers rather than a parallel one.
- Settle reports `animating` from the document's flag and holds open on no animation (for example, the scrollbar fade after a `scroll` reads true until 700 ms of the clock pass), per design-system §Motion (detail `.andromeda/design-system-detail/motion.md`). A witness asserts against that reading rather than waiting on wall time.

## Anti-patterns to avoid
- A font-dependent assertion on a fixture with no resolvable font context, which passes vacuously at 0x0 text (design-system §Typography, "Loading").
- Colour, an ESC byte or a terminal-dependent branch on the driver's stdout, or a log line there (design-system §Surface: cli › Component Patterns).
- A witness that waits on wall time or expects settle to advance an animation or timer (design-system §Motion, "Animation runtime").

## Contract bindings
- design-system §Color Palette › Core Colors ↔ a11y §Contrast: `stand_contrast`'s `TEXT_ROWS` / `STATE_ROWS` record the accent's pairs (5.47 / 6.83 / 8.27). Re-asserting them after 0.2.0 belongs to «A11y re-read», not this chunk.
- design-system §Color Palette › Semantic Colors ↔ a11y §Use of Color (SC 1.4.1): invalid and success state are carried by `aria-*` and text, not colour alone.
- design-system §Typography ("Loading") ↔ the driver schema's refusal cause `fonts-unavailable` (the eighth `Cause`) and the test harness's `ScreenshotError::FontsUnavailable`.
- design-system §Depth Strategy ↔ the driver's `covered` refusal and `lint`'s `clipped` / `off-screen` readings. The same padding-box rule produces the stated `covered` limits.
- design-system §Motion ↔ the test-plan harness contract (`VirtualClock`, settle, `advance`) and the driver's `LOAD_WAIT`.
- design-system §Surface: cli › Component Patterns ↔ obs-plan's stderr sink. The answer is on stdout and is never a log line, so the stderr id-free measurement in `host_log` / `mcp_log` reads stderr alone.

## Acceptance criteria contributions
- Every new or named fixture witness that asserts text extent, `truncated` or a text-bearing lint finding runs under a font context that resolves `sans-serif` (blitz-tests' `system-fonts`, or bundled DejaVu via `build_single_font_ctx`), and its text box reads a non-zero size in both layout modes (per design-system §Typography).
- Every `lint`, `snapshot --part` and `--record` answer line the sweep's witnesses read on stdout, and every `mcp` reply, holds one JSON line with no ESC byte; usage and session-error text appears on stderr only, with the exit status in `0`–`3` (per design-system §Surface: cli › Component Patterns).
- No seven_guis task CSS colour moves in this chunk: the accent stays `#3a5ce5` / `#2a4cd3` / `#1f3fc1` and `stand_contrast` stays green unchanged (per design-system §Color Palette › Core Colors).
- No witness the sweep names advances time except through `advance` / the harness's `VirtualClock`, and none waits on wall time beyond `LOAD_WAIT` (per design-system §Motion).
