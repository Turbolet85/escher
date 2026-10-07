# design extract

## Relevance
partial — the chunk changes no screen, so no colour, type, spacing, radius, depth or icon value is bound; the one part of the plan that bears on it is design-system §Motion (the recorded animation runtime and motion sources, which are exactly the "never goes quiet on its own" sources the scope's timers decision has to answer for), with a narrow tie to §Typography (Loading) for the new loads fixture.

## Constraints
- design-system §Motion (Animation runtime) records a set of sources that keep a document animating, wider than CSS animations and transitions alone (it also names canvases, animating sub-documents, custom widgets, scroll animations and scrollbar fades). The settle rule the chunk states for "a running animation" must give an answer for every member of that set, not for `has_active_animations`' CSS case only. Whether the code's animating reader still enumerates exactly that set — and at which line, since the plan's citation and the scope's differ — is research's question.
- design-system §Motion (Animation runtime) records two clocks: the shell's, wall-time since the first animation-time query, and the test harness's, a controlled clock advanced by tick. A headless settle must read animation time only through the harness's controlled clock; the shell's clock and its redraw-while-animating behaviour are not this chunk's to change (the scope's "windowed path is not changed" boundary).
- design-system §Motion (This project's values) records finite-duration engine motions (smooth scroll, touch fling deceleration, the overlay scrollbar's hold-then-fade). Each ends only as the clock advances, so each is "pending but not due" in the scope's sense: the chunk must state whether settle advances the clock through them, reports them as the busy source, or excludes them — one stated rule, not a per-case accident. Whether the stand's pinned viewport ever shows an overlay scrollbar (and so starts a fade) is research's question.
- design-system §Motion (Animation runtime) records two sources that never go quiet by themselves: a custom widget that asks for continuous redraw, and `requestAnimationFrame`, approximated as a short repeating timer. The bounded wait's typed "still busy" outcome must be able to name these, or the rule must exclude them explicitly.
- design-system §Motion (This project's values) records transitions and animations as observed absent in the seven_guis styles. Under that record the stand proof on the Timer task exercises no CSS-animation leg; if the chunk claims an answer for running animations, that leg needs a fixture of its own. Whether the absence still holds in `examples/seven_guis/src/` is research's question.
- design-system §Typography (Loading) records that the headless stand registers one bundled font for every generic with system fonts off, that blitz-tests text measures 0x0 without the `system-fonts` feature (so font-dependent assertions pass vacuously), and that `document.fonts` is a stub reporting every font loaded. The loads fixture beside the stand must boot with a deterministic font context, and a font may not be assumed observable as a pending load through `document.fonts`.
- design-system §Color Palette (Color model and scheme) records that a colour-scheme change on the viewport triggers a full recascade. A device change of that kind is outstanding style and layout work for settle's layout source — relevant only if research finds a resolve can queue it (`pending_device_changes`, named in the scope).

## Patterns to follow
- Drive time the way design-system §Motion (Animation runtime) records the harness doing it — a controlled clock advanced by tick — rather than introducing a second time source for settle.
- Treat the recorded keeps-animating set in design-system §Motion (Animation runtime) as the checklist for the "periodic source" decision: one line of the stated rule per member.
- Boot the loads fixture the way design-system §Typography (Loading) records the stand booting — `build_single_font_ctx` over the bundled font, system fonts off — so its layout is the same on every host.
- Leave every style value untouched: design-system §Color Palette (Core Colors, the seven_guis row) and §Border Radius (the seven_guis row) record the stand's values as untokenized literals in the task sources; a chunk that edits `tasks/timer.rs` for the tick seam changes behaviour there, not appearance.

## Anti-patterns to avoid
- (none recorded — design-system §Anti-Patterns reads `> NO RECORDED INTENT`; nothing is extracted from it)

## Contract bindings
- design-system §Motion (Animation runtime) ↔ architecture §Standard Contracts (the `Document` trait's animating reader, the headless `Harness` clock): the settle rule for running animations is written against the same reader and clock the motion section records.
- design-system §Motion (Animation runtime, This project's values) ↔ test-plan §3 (the no-sleep rule): motions that end only with time are advanced through the harness clock, never a sleep or a wall-clock read.
- design-system §Typography (Loading) ↔ test-plan §3 (determinism — no font skip): the loads fixture's font context.
- design-system §Motion (Reduced motion) ↔ a11y-plan: the section records reduced-motion handling as observed absent in the engine. The chunk adds no animation, so the reduce-motion override binding is not triggered; settle must not rely on a reduced-motion preference to quiet a source.
- Token contrast ↔ a11y (SC 1.4.3): not triggered — no painted colour changes.

## Acceptance criteria contributions
- (design) The chunk's stated settle rule names its answer for each member of the recorded keeps-animating set — CSS animations and transitions, canvases, animating sub-documents, custom widgets, scroll animations, scrollbar fades (per design-system §Motion — Animation runtime)
- (design) No check and no settle path in the chunk reads animation time other than through the harness's controlled clock; no wall-clock read and no sleep stands in for a motion's duration (per design-system §Motion — Animation runtime)
- (design) The loads fixture boots with a deterministic bundled font context, and its text measures non-zero in both layout modes (per design-system §Typography — Loading)
- (design) The chunk's diff adds or changes no colour, font, spacing, radius or shadow value in the stand's styles — the Timer task's painted appearance is unchanged (per design-system §Color Palette — Core Colors)
