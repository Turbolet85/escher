# P5 review — 2026-11-14-bundled-fonts «Bundled fonts»

**Plan:** `doc:2026-11-14-bundled-fonts/plan` rev 4, 278 lines. Open its render at
`.andromeda/cache/docs/2026-11-14-bundled-fonts/plan.md`. No plan stands in a chunk's folder. Writing in that file
changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-bundled-fonts`
diffs the chunk's records.

## Goal
Research found that a run's fonts are already bundled. Every run path goes through `stand::options`, whose font is DejaVu
Sans with system fonts off. That covers the session host's `start`, `mcp` and `coverage` roles, a kept check's replay
and the fixtures (94 call sites, graph query 1). The scope's premise was corrected at P3. The chunk changes no
production code. It adds the cross-OS measurement v030-14 asks for, which nothing holds today: only the committed
Counter checks are read on all three OSes.

## Top acceptance criteria
1. `matrix#v030-14`: `stand_cross_os_readings` reads green on the final HEAD's CI run on the `test` leg (linux x86_64),
   «Test (windows)» (x86_64) and «Test (macos)» (aarch64). The check covers the six served tasks in both layout modes,
   at boot and after one step reached by `click`, `type` or `advance`. Each `snapshot` (every part), `lint` and
   `actions` answer's `answer_digest` must equal one committed table. No shot is compared.
2. `stand_bundled_font_families`: `serif`, `monospace`, `system-ui` and an unavailable family each read `sans-serif`'s
   text extent on the stand's options. `cursive` and `fantasy` are asserted as measured.
3. `fonts-unavailable` is kept: its four witness checks read green unchanged.
4. No production file, manifest, workflow, committed check or dependency changes (the guard reads exit 0).

## Touchpoints
- New: `tests/blitz-tests/tests/stand_cross_os_readings.rs`, `tests/blitz-tests/tests/stand_bundled_font_families.rs`.
- Modified: none.

## Test commands (10 entries)
- Entries 1–3: the two new checks, under the per-package and workspace builds. Baseline red: no test target yet.
  Their control is implement's red-before-green run (step 4: a flipped digest, and the stand's font swapped for
  `FontContext::default()`).
- Entry 4: `stand_replay` and the `fonts-unavailable` witnesses. Baseline green: 6 + 8 + 9 + 8 + 4 passed.
- Entry 5: a silence census of the two new files. Must-pass reads 0 on clean stubs; the control reads 1 on a
  `println!` (`p5-controls.md`).
- Entry 6: a no-production-change guard against `557f1c98d`. Baseline green; the control reads red on a
  `packages/` edit.
- Entry 7: `ci-leg.sh fast`, seen green at the last wrap.
- Entries 8–10: the operator's hygiene, push and CI read. `verdict: green` there is the cross-OS reading itself.

## Concretization preview — v030-14
- **Now:** «Hosts of different operating systems read one state of a run the same way, compared by its readings, not by
  image bytes.»
- **Proposed:** «On the final HEAD's CI run, `stand_cross_os_readings` reads green on linux x86_64 (the `test` leg),
  windows x86_64 and macOS aarch64 (`matrix_test`). Each of the six served tasks is booted as the session host boots
  it, on the stand's bundled DejaVu Sans with system fonts off, in both layout modes. At boot and after one acting step
  reached by `click`, `type` or `advance`, its `snapshot` (every part), `lint` and `actions` answers have an
  `answer_digest` equal to one committed table. That is one state read the same way on three OSes, compared by its
  readings and never by a shot's bytes; the run id is named from the operator's CI read. The bundled set is the
  document's own font context, the one every glyph run in the document is shaped from, so canvas text (`GlyphRun`), a
  0.4.0 reader, reads the same set.» The outcome is the same one, made concrete, never weakened. It is claimable: the
  refs come from /implement's green and the operator pass's CI read, not from a wrap-only artifact.

## Forks
None. No `founder-forks.md` was written. The plan changes no font loading and no ratified row's meaning, and adds no
boundary crossing. I leaned on `(architecture · decision/stand-typography-dejavu-sans)` and on the playbook's
boundary-widening pattern `rul-izni57qv`, which a test constant of digests does not touch. A third committed kept check
and a CI upload were both rejected for that reason.

## Coverage-sanity
0 of 7 masters picked 0 rows. No areas were flagged.

## Beyond the plan
1. **A deferral that applies only if the measurement calls for it.** If `stand_bundled_font_families` reads `cursive`
   or `fantasy` differently from `sans-serif` on the bundled set, that family becomes a stated limit. Its deferral
   would be a residual (target next) plus its line in `escher-0.4.0-incubator/deferred-from-0.3.0.md` («Bundled set,
   `cursive` / `fantasy` not registered»), written at the wrap per the take-up directive (inputs#I1). If both read
   `sans-serif`'s extent, nothing is deferred.
