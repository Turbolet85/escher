# arch extract

## Relevance
relevant — the chunk moves time reads in blitz-dom (an engine crate every document runs), blitz-test-harness's clock contract, the stand's time step and escher-driver's `advance` and help text.

## Constraints
- Embedder services reach the DOM only through what `DocumentConfig` carries, each with a default when the embedder supplies none (per architecture §Design Philosophy, "Embedder services are injected, with no-op defaults"). A clock the document reads outside `resolve(time)` follows that shape. Whether the default stays wall time for windowed documents is the P4 founder fork the scope names.
- Configuration lives on `DocumentConfig`, and optional capabilities are compile-time Cargo features (per architecture §Cross-cutting Patterns → Config management). Note that blitz-paint ships `scrollbars` off by default (per architecture §Established Decisions → [Default features]). The harness takes blitz-paint with `default-features = false`, so a per-package build paints no overlay scrollbars (per architecture §Standard Contracts → Test harness). Whether the stand's headless build paints them is research's question. Without them the matrix proof passes vacuously.
- `Harness::screenshot` / `screenshot_rect` resolve at the harness's own time, with no tick and no pump. `Harness::settle` reads no clock but the harness's own, and it neither waits on nor advances a source that only time moves (per architecture §Standard Contracts → Test harness). The one clock has to keep both of these true.
- The stand contract says `timer_step` delivers whole 100 ms ticks and that "the harness's animation clock stays where it was" (per architecture §Standard Contracts → Headless stand). If `advance` moves the instance's clock, this clause changes and is amended at wrap.
- escher-driver names no app. The time step is the caller's, supplied through `Session::with_time`, and a session with none refuses `advance` as `time-unavailable`. The executor, session, schema and JSON writer read no clock. The cause set is closed at nine. The `command` span reads no clock and never fields `advanced_ms` (per architecture §Standard Contracts → Driver session). Answering `advance` on every task moves this contract. That is the P4 founder fork, not a silent change.
- Incremental and non-incremental layout run one pipeline and must produce identical layouts (per architecture §Design Philosophy, "One incremental pipeline, safe identities"). Animations, transitions and smooth scrolls advance through `resolve(time)` and `is_animating()` (per architecture §Cross-cutting Patterns → Invalidation and state integrity). Inside `resolve`, the scroll animation runs third, after messages and the critical-resource gate (per architecture §Infrastructure Patterns → Runtime threading and event flow).
- There is no new env var, crate, listener or name. The env-var list is `RUST_LOG`, and crate names are registered (per architecture §Occupied Resources → Environment variables and → Names). Anything new is a registration first.

## Patterns to follow
- Precedent for a switchable virtual clock: blitz-vibey-script's `ScriptDocument` has `with_virtual_time`, `clock_now` and `advance_clock_to` (per architecture §Standard Contracts → JavaScript). The WPT runner runs JS timers on virtual time with a real-time backstop (per architecture §Design Philosophy, "Headless, measurable, testable").
- Keep the timing values and change only where time comes from. The overlay scrollbar fade uses Chromium's timings (per architecture §Design Philosophy, "Behave like a browser, and cite the spec"). Smooth scrolls last 300 ms with cubic ease-in-out, and the fling decelerates 0.95 per 60 fps frame (per architecture §Established Decisions → [Scrolling and input]).
- `scroll_into_view` writes every nested box as an instant `scroll_to`, and the requested behaviour applies only to the viewport. The harness helper passes `ScrollBehavior::Instant` (per architecture §Established Decisions → [Scrolling and input]; §Standard Contracts → Test harness). Whether a driver `scroll` ever reaches the smooth path is research's question.
- Write tests the established way: in the blitz-tests crate, one file and one target per behaviour, with shared stand and session helpers in `common/` and `session_common/` (per architecture §Conventions → Tests).

## Anti-patterns to avoid
- Do not add a second clock that only the harness or the driver reads alongside the engine's (per architecture §Standard Contracts → Test harness: settle "reads no clock but the harness's own"). Do not add a wall-clock read to escher-driver (per architecture §Standard Contracts → Driver session: the executor and session "read no clock").
- Do not reach through `DocumentMutator::doc` for DOM writes. Mutations go through `doc.mutate()` (per architecture §Conventions → DOM API patterns).
- Do not land an env var, crate or listener without an §Occupied Resources registration (per architecture §Occupied Resources → Environment variables).

## Contract bindings
- arch ↔ tests: the harness's clock (`tick`, `pump`, `resolve` at the harness's time, settle's `animating`) is the shared surface for the repeat-shot proof in `stand_screenshot_repeat` and the both-layout-modes rule (per architecture §Standard Contracts → Test harness; §Conventions → Tests).
- arch ↔ security / obs: the `screenshot` verb's help states the wall-clock fade limit in schema.rs's fixed `'static` text, which `describe()` serves over the command line and MCP. The `command` span's field set leaves out `advanced_ms` and any clock read (per architecture §Standard Contracts → Driver session). Rewording the help moves the served self-description. A new cause or field would be a crossing question.
- arch ↔ stand: `Session::with_time` and `stand::timer_step` are where the caller supplies time (per architecture §Standard Contracts → Headless stand; §Standard Contracts → Driver session).

## Acceptance criteria contributions
- (arch) Every engine time read the chunk moves reaches blitz-dom through `resolve(time)` or a `DocumentConfig`-carried source with a default. No new crate, env var or listener is added (per architecture §Design Philosophy, "Embedder services are injected, with no-op defaults"; §Occupied Resources → Names).
- (arch) `Harness::screenshot` still resolves at the harness's own time with no tick or pump. `Harness::settle` still advances no time (per architecture §Standard Contracts → Test harness).
- (arch) The driver's verb table, its nine causes and the `command` / `tool_call` span fields are unchanged unless a P4 fork rules otherwise. The `screenshot` help's wall-clock fade sentence is rewritten or removed to match the built behaviour (per architecture §Standard Contracts → Driver session).
- (arch) The incremental and non-incremental runs of the new pipeline tests read identical layouts and pixels (per architecture §Design Philosophy, "One incremental pipeline, safe identities").
