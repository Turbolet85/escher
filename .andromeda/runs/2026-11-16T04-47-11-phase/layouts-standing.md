# layouts · standing conventions

- S1 · each new test target is reachable through the agent-run.sh test contract — its usage, stdout, run log, exit 2 and the Windows pass-through · surface-cli
  contract/agent-run-cli/exit-2 · agent-run.sh test contract · exit 2 — A usage error prints the usage to stderr and exits 2.
  contract/agent-run-cli/run-log · agent-run.sh test contract · target/agent-run/run.log — The raw cargo output stays in this file, never on stdout.
  contract/agent-run-cli/stdout · agent-run.sh test contract · stdout — Every verb's stdout is JSON lines only, one object per event.
  contract/agent-run-cli/usage · agent-run.sh test contract · usage — agent-run.sh <verb> [selection], run from the repository root, with verbs boot, run, status, cleanup, logs and run selections stand, all, session or a blitz-tests file name; session is a closed word matched before a file name and runs the escher-session binary's flows in seven_guis.
  contract/agent-run-cli/windows-pass-through · agent-run.sh test contract · agent-run.ps1 — The Windows pass-through to agent-run.sh.
- S2 · each new fixture app's controls are keyed, their stable ids carried as attributes alone and their names, roles and states by attributes alone, and the fixture lints clean at boot · surface-desktop-native
  decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
  decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
  decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
  decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes. · guard: stand_lint
- S3 · each new headless test of a session boots on the pinned headless viewport · surface-desktop-native
  decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
