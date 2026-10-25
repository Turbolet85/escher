# tests · Integration Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/integration-test-strategy.md:14) The statement's acting-verb capability must name the verbs and their addressing as the session checks prove them, by stable id alone for `click`, `type` and `scroll`.
  > a caller-built `Call` run through `Session::run` on a held session — `snapshot`, and `click` · `type` · `press` · `advance` · `scroll`, the first two and the last addressed by stable id alone — returns after settle with the diff of the snapshots taken before and after it
- (.andromeda/test-plan-detail/integration-test-strategy.md:14) The load wait must be stated with its proven bound and behaviour: 5 s of wall time, no time moved, and an unanswered load ending unsettled with busy class `Loads`.
  > since 2026-10-20-load-wait an acting step that meets a load in flight waits for it up to `LOAD_WAIT` (5 s of wall time), settling again every 10 ms and moving no time — `stand_load_wait` 2 through `Session::run` in both layout modes: a load answered at 200 ms ends the `click` settled with its content on screen, a load never answered ends it at the bound `settled` false with `busy` `Loads`
- (.andromeda/test-plan-detail/integration-test-strategy.md:14) The headless screenshot capability must be stated as proven: the whole screen or one element as a PNG equal to the harness's raster, changing nothing.
  > since 2026-10-12-headless-screenshot a `screenshot` returns the screen, or the element its optional id names, as a PNG equal to the harness's raster and changes nothing
- (.andromeda/test-plan-detail/integration-test-strategy.md:15) The statement's masking claim for shots must match what is proven: a password typed through the driver never leaves as pixels, and a shot cropping to no pixel is refused `off-screen`.
  > an element shot's id refused `not-found` or `stale` and one cropping to no pixel `off-screen` with the instance unchanged, and a password typed through the driver never leaving as pixels (`stand_act_screenshot`)
- (.andromeda/test-plan-detail/integration-test-strategy.md:15) The refusal set the statement names must be exactly these eight causes, each leaving the instance unchanged and its text holding nothing the call supplied.
  > a refused call — any of the eight causes (ratified by the founder, 2026-10-18, `advance` accepted on every session): `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `off-screen`, `covered` and, since 2026-10-12-headless-screenshot, `fonts-unavailable` — leaves the instance unchanged and reads as a text holding none of the bytes the call supplied
- (.andromeda/test-plan-detail/integration-test-strategy.md:15) `scroll` must be described as the statement's way to bring a target into view, never refused by the three click-side checks, reporting whether the target is then in view.
  > a `scroll` is refused for none of the three checks that stand in front of a click and says in its result whether its element is then in view
- (.andromeda/test-plan-detail/integration-test-strategy.md:15) The control-value masking the statement says an MCP client receives must match the proven reading: a password typed through the driver never appears in an answer and reads `MASKED_VALUE`.
  > a password typed through the driver occurs 0 times in what the call returns and reads `MASKED_VALUE`
- (.andromeda/test-plan-detail/integration-test-strategy.md:16) `type`'s replace-and-clear behaviour and its masked written form must be stated as proven.
  > a `type` replaces what a filled control holds and an empty text clears it, a masked control's written form holds the mask and never the typed text (`stand_act_filled`)
- (.andromeda/test-plan-detail/integration-test-strategy.md:16) Two of the three measured-off readings the statement lists as limits must be stated in `covered`'s fixed meaning, worded to match the check and the schema's own help text (refusal.rs:74-79).
  > two readings stay as measured, each stated in `covered`'s fixed meaning: a box with no width or no height is hit at its centre on the element that holds it, so `covered` can read where nothing covers it (`stand_act_ids`), and a box clipped on one axis stops a hit at its whole padding box, so a target lying outside it along the axis it does not clip is refused `covered` though it shows, the instance unchanged, in both layout modes
- (.andromeda/test-plan-detail/integration-test-strategy.md:17) The statement's select capability must match the proven `type` on a select, and the transform reach must match the translated-button check.
  > a `click` on a translated button is accepted and lands, and its snapshot `bounds` read where it shows, in both layout modes (`stand_act_transformed`; tests/blitz-tests/tests/stand_act_transformed.rs:76-116); a `type` on a single-choice select chooses the first option whose label starts with the text — accepted and settled, its diff naming the select with `value` in `differs` and the two options whose `selected` moved, the select's options read as its children named by their labels, a disabled select skipped by the Tab walk and refused, in both layout modes
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) The statement must state the session socket as unix only, with non-unix arms returning `Unsupported`.
  > the non-unix arms, which return `Unsupported`, are compiled and run by the fork's windows CI leg only
- (.andromeda/test-plan-detail/integration-test-strategy.md:20) What escher hands an MCP client onward must be stated as carried on stdout alone, with the `mcp` role's stderr holding no id, name or typed text.
  > the binary's `mcp` role is covered by seven_guis' `mcp_log` — at `RUST_LOG=trace`, under the `-p seven_guis` and the workspace build, 0 typed-sentinel occurrences, 0 id needles and 0 name needles on stderr, one `tool_call` line per call with only its stated field names, and protocol lines only on stdout
- (.andromeda/test-plan-detail/integration-test-strategy.md:21) The statement's limits must include the unmeasured paged `part` read through the host's own sink, an owed residual.
  > A paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next)
- (.andromeda/test-plan-detail/integration-test-strategy.md:22) A claim about the windowed stand's log silence must say it is read by an evidence instrument, not a standing check.
  > By level, both sink-installing binaries read 0 ids and 0 names at the default level, `info`, `debug` and `trace` — the windowed stand by the evidence instrument on the dev host, not by a standing check
- (.andromeda/test-plan-detail/integration-test-strategy.md:24) The statement must not claim typed text measured on the windowed binary's log.
  > typed text is measured on the session host's log since 2026-10-10-driver-cli — 0 occurrences of a typed sentinel at `trace`, as `host_log` above — and stays unmeasured on the windowed binary's
- (.andromeda/test-plan.md:19) The select's stated limits must be stated as asserted by no check, unlike the text extent's.
  > the text extent's stated limits are asserted by blitz-tests' `text_extent`, while the select's are asserted by no check
- (.andromeda/test-plan.md:44) The boolean model's two limits the statement lists must be worded as pinned.
  > the two limits of the boolean model — a mixed toggle reads unpressed, a grammar or spelling error reads invalid — are pinned here and at the engine only
- (.andromeda/test-plan.md:48) The a11y reach must state as unmeasured, and owed, the reading of accessibility states as an assistive technology receives them.
  > no check reads the accessibility states as the platform adapter hands them to an assistive technology — owed, owner: its `.andromeda/residuals.md` line (target: next)
- (.andromeda/test-plan.md:89) The shot's cross-OS limit must be stated as the pixel tests hold it.
  > a shot's PNG bytes differ between OSes, so a check compared byte for byte across OSes holds no shot
- (.andromeda/test-plan.md:94) `fonts-unavailable` must be stated as a refusal with nothing painted, never a skip.
  > a shot of a document whose font context resolves no `sans-serif` family is refused, `fonts-unavailable` at the driver, with nothing painted and no skip path
- (.andromeda/test-plan.md:187) The cold-agent pipe, cited as the measured instance of what an MCP client carries onward, must be stated with its one live run on eleven tools.
  > fired once: `passed`, `isolated` (the eleven `mcp__escher__*` tools, one server `escher`)
- (.andromeda/test-plan.md:187) The statement must not claim a live run on twelve tools: the pipe requires twelve and the live run is not re-fired.
  > the pipe requires twelve tools with the `lint` verb, its contract tests' fake answering `lint`, and the live run is not re-fired
- (.andromeda/test-plan.md:286) Session recording and replay must be stated as proven, with the committed checks holding no `screenshot` step because a shot's bytes differ by OS.
  > a recorded session run as a process keeps its check, and the committed checks are read byte-equal to what the binary records, the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS
- (.andromeda/test-plan.md:288) The MCP surface's real-client reach must be stated as one drive in the operator pass, not in CI.
  > a real client drove it once, in the operator pass, not in CI
- (.andromeda/test-plan.md:301) The statement must not claim windowed or browser-driven end-to-end coverage.
  > NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read
- (.andromeda/test-plan.md:352) The one virtual clock must be stated as `advance` moving it by exactly `ms`.
  > the driver's `advance` moves the session's clock by exactly `ms` inside one settled step, and `advanced_ms` is exactly `ms`, the clock's move, not what the Timer's counter gained

## Bindings
- **Measured-off `covered` readings ↔ the schema's help text (escher-driver `refusal.rs`)**: the statement's limit wording matches the check and the fixed meaning the tool itself states, (.andromeda/test-plan-detail/integration-test-strategy.md:16)
- **Unmeasured paged `part` read ↔ `.andromeda/residuals.md`**: the limit is an owed residual (target: next), stated as owed and not as measured, (.andromeda/test-plan-detail/integration-test-strategy.md:21)
- **Assistive-technology reading ↔ a11y-plan and `.andromeda/residuals.md`**: the a11y reach's unmeasured windowed witness is the residual the scope's a11y section names, (.andromeda/test-plan.md:48)
- **MCP onward content ↔ security-plan §Surfaces (MCP crossing, cold-agent crossing)**: what reaches the client is stdout protocol lines with masked values; the measured instance is the eleven-tool live run, (.andromeda/test-plan-detail/integration-test-strategy.md:20), (.andromeda/test-plan.md:187)
- **Cross-OS shot limit ↔ the committed checks**: the shot's per-OS bytes are why the committed Counter checks hold no `screenshot` step, (.andromeda/test-plan.md:89), (.andromeda/test-plan.md:286)
- **Load wait and refusal set ↔ the body §5 broad line**: the session ↔ held instance detail serves the body's broad line on settle, refusal and masking, (.andromeda/test-plan-detail/integration-test-strategy.md:14), (.andromeda/test-plan.md:257)
