# design · Motion — category read

## Nothing in scope
Nearest: (.andromeda/design-system-detail/motion.md:27) the virtual clock the harness and the driver's `advance` move, with `click`, `type` and `press` leaving it where it was. This chunk only refactors `lint.rs::findings` and two table-asserting tests under the complexity ceiling, with no change to behaviour, verbs, fields or spans. The `lint` verb runs no step and moves no clock, so no duration, easing, animation, settle or clock rule in Motion is reached.
