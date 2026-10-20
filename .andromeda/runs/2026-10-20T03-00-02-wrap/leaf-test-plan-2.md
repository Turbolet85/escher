group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «since 2026-10-19-visual-lint, `stand_lint` (26 readers;»
  source: .andromeda/test-plan.md:103 «(27 readers since 2026-10-20-load-wait added `stand_load_wait`, which reads `Acted`, `act`, `click` and `snapshot`; 26 before `stand_load_wait`,»
  replace-with: «since 2026-10-19-visual-lint, `stand_lint` and, since 2026-10-20-load-wait, `stand_load_wait` (27 readers;»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «what 20 checks — 19 `stand_*` files (`stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»
  source: .andromeda/test-plan.md:104 «21 checks declare `mod fixtures;` — 20 `stand_*` files (`stand_load_wait` since 2026-10-20-load-wait, reading `HeldNet`; `stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»
  replace-with: «what 21 checks — 20 `stand_*` files (`stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «escher's one held network stand-in `fixtures::HeldNet`;»
  source: .andromeda/test-plan.md:282 «| Network | `LateNet`, `stand_load_wait`'s in-file self-answering stand-in since 2026-10-20-load-wait: `fetch` answers each request from its own thread after a 200 ms sleep»
  replace-with: «escher's one held network stand-in `fixtures::HeldNet`; since 2026-10-20-load-wait `stand_load_wait`'s in-file self-answering `LateNet`, whose `fetch` answers each request from its own thread after a 200 ms sleep;»

- leaf: .claude/rules/testing.md:30   class: stale
  leaf-text: «the one exception `stand_virtual_clock`'s recorded 800 ms sleep, taken so a shot after a scroll is read to repeat across wall time until `advance` moves the fade)»
  source: .andromeda/test-plan.md:185 «bar two recorded exceptions whose subject is wall time: `stand_virtual_clock`'s one 800 ms sleep, taken so a shot after a scroll is read to repeat across wall time until `advance` moves the fade, and»
  replace-with: «two recorded exceptions whose subject is wall time: `stand_virtual_clock`'s 800 ms sleep, taken so a shot after a scroll is read to repeat across wall time until `advance` moves the fade, and, since 2026-10-20-load-wait, `stand_load_wait`, which reads `Instant::now()` around its call to measure the load wait against `LOAD_WAIT` and whose in-file `LateNet` answers from its own thread after a 200 ms sleep)»

- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «and since 2026-10-19-visual-lint, which added lint.rs, it reads 4634/4732 = 97.93 %»
  source: .andromeda/test-plan.md:352 «since 2026-10-19-visual-lint, which added lint.rs, 4634/4732 = 97.93 %, and since 2026-10-20-load-wait it reads 4695/4794 = 97.93 %, the 97 % floor holding»
  replace-with: «since 2026-10-19-visual-lint, which added lint.rs, 4634/4732 = 97.93 %, and since 2026-10-20-load-wait it reads 4695/4794 = 97.93 %»

- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «`run stand` runs the 19 `stand_*` readers of the shared module»
  source: .andromeda/test-plan.md:132 «`run stand` runs the 20 `stand_*` readers of the shared module»
  replace-with: «`run stand` runs the 20 `stand_*` readers of the shared module»

- leaf: .claude/rules/verification-harness.md:32   class: omission
  leaf-text: «`escher_driver::Session::act(step)` runs a step on a held instance and settles it,»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5 «the harness is settled, a load in flight waited for up to `LOAD_WAIT` (5 s of wall time, settled again every 10 ms, no time moving on the held instance) since 2026-10-20-load-wait»
  replace-with: «`escher_driver::Session::act(step)` runs a step on a held instance and settles it — since 2026-10-20-load-wait waiting for a load in flight up to `LOAD_WAIT` (5 s of wall time, settled again every 10 ms, no time moving), one still loading at the bound `NotSettled(Loads)` —»
