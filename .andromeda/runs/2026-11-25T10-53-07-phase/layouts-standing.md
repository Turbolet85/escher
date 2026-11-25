# layouts · standing conventions

- S1 · each new test that drives a session answer (graph, actions, coverage, layout, snapshot) asserts the answer's shape as its contract states it · surface-cli
- S2 · each new or changed check on the stand keeps the stand's boot readings (ids as attributes, lint clean at boot, the pinned headless viewport) · surface-desktop-native
  decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
  decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
  decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
  decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
  decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD, the load page, the order page and Circle Drawer read zero visual-lint findings, in both layout modes, and Circle Drawer reads no canvas finding after a click or at 12 and 100 circles either. · guard: stand_lint
