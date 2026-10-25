# design · Motion — category read

## Carried
- (.andromeda/design-system-detail/motion.md:11) Where the capability statement describes the driver's `scroll`, it must state it as instant in every box and the viewport, inside one settled step, never as an animated scroll.
  > A programmatic into-view scroll never animates a nested scrolling box: `BaseDocument::scroll_into_view` writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked — the document holds one scroll animation at a time — and the requested behaviour, `Smooth` included, applies to the viewport alone; the driver's `scroll` is instant in every box and in the viewport, made inside one settled step
- (.andromeda/design-system-detail/motion.md:20) The statement's settle and acting-verb claims must match this: settle neither waits on nor advances an animation, and it reads `animating` from the document's flag, so the statement must not claim that a step waits for animations to finish.
  > a harness settle answers for every member of that set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag, the scrollbar fade's membership read off the document's clock — after a driver `scroll` it reads true until 700 ms of that clock pass
- (.andromeda/design-system-detail/motion.md:26) The statement's «one virtual clock» capability must say that `advance` alone moves time, by exactly its `ms` (and the app's own time through a caller-supplied step), while the other acting verbs leave it where it was.
  > the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was: a driver click beside a running CSS animation returns settled without waiting on it
- (.andromeda/design-system-detail/motion.md:32) The statement's load-wait capability must state that the wait is bounded at `LOAD_WAIT` of wall time and moves no clock, so no timer or animation runs during it.
  > The driver's load wait moves no clock: while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait
- (.andromeda/design-system-detail/motion.md:34) This is a limit the master already states, so the limits list must weigh it at P3's completeness close: a Dioxus app on escher gets no animation or transition event data.
  > Animation and transition event data are `unimplemented!()` in dioxus-native-dom
- (.andromeda/design-system-detail/motion.md:46) The statement's a11y reach must not claim any reduced-motion support. It must either state the absence or stay silent on motion.
  > Observed absent — reduced-motion handling
- (.andromeda/design-system.md:129) Same as the virtual-clock item, at the body's broad line: the statement's wording must not contradict it.
  > The test harness drives animation from the `VirtualClock` its documents read, and settle reads animation time only through it, never advancing it; the driver's `advance` moves it by exactly its `ms` and, through the step a session's caller hands it, the app's own time together, while `click`, `type` and `press` leave it where it was.

## Bindings
- **reduced motion ↔ a11y-plan**: the a11y target is SC 2.1.1 · 1.4.3 · 2.4.3, not SC 2.3.3, and reduced-motion handling is observed absent. The statement's a11y reach therefore names no reduced-motion proof (.andromeda/design-system-detail/motion.md:46), (.andromeda/design-system.md:135)
- **the one virtual clock and the load wait ↔ the body's Animation runtime**: the detail lines and the body's broad lines state the same clock rule. The statement matches both (.andromeda/design-system-detail/motion.md:26), (.andromeda/design-system.md:131)
- **expression level ↔ Brand Identity**: no expression level is recorded, so the statement makes no claim about escher's motion design (.andromeda/design-system.md:119)
