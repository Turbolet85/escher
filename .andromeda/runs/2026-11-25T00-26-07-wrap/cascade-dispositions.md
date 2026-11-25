# Cascade dispositions — 2026-11-24-canvas-lint

## Step 1 — apply
Six TXs: architecture 3722 (6 operations), security-plan 3723 (3), layout-templates 3724 (2), test-plan 3725 (2), obs-plan 3726 (1), a11y-plan 3727 (1). design-system: its one operation rejected (check 2); its dry run would land 0, so no call fired.

## Step 2 — the sweep
Skipped: every `rows-land` dry run printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file` (architecture 3, security-plan 1, layout-templates 2, test-plan 2, obs-plan 1, a11y-plan 1; design-system `patterns: none — the call sets and retires no row`), and this wrap wrote no `cascade-patterns.toml` of its own. No file holds a pattern, so step 3's leaves read each changed row's delta.
Lateral and cross-master citations of the amended claims: Phase 1's pre-pass site sweep (`sweep-pre-pass-1.1.txt`, 10 patterns, 22 standing master rows) dispositioned every master row. The two rows stating «reach no … lint» (a11y-plan `decision/circle-drawer-canvas-pixels-only`, layout-templates `decision/circle-drawer-circles-painted-on-canvas`) are re-stated in this pass. Two architecture rows the plan names as anchors were read whole and stand true: `contract/driver-session/lint` («Session::lint reads the screen and changes nothing») and `contract/lint-reason-keys/truncation` («text then visible, visible left out when no part can be seen»). No step-2 operation was owed. Curation homes and bases: the pre-pass sweep read `curation 0/0 · base 0/0`.

## Leaves
`cascade.py leaves` (pass mode): `epoch · «Epoch 4 — Inside the canvas» · markerless 5 · gated 0 · open` · `sweep leaves · none — no pass sweep of this marker`, so no group carries a `swept` row · `total · pass · 16 group(s) · 25 leaves · 790089 B · unmapped 1`.
- unmapped `.claude/rules/host-linux.md` (host): the host leaf, no comparer — its statements are the host's (`amendment-flow.md` §Cascade).
- Comparers: 16, one per group, all passing `fanout.py check` (`16 of 16 pass`). Every one answered by findings or by reading its leaves clean; none wrote a `@leaf_stands` record. With no sweep lines, none was owed one.
- `leaves --apply` (all files): `total · blocks 27 · applied 27`. By hand 0 · declined 0.
  - a11y-plan-1 (pass): 1 finding, applied (a11y-summary.md:24, stale «reach no … lint»).
  - a11y-plan-2 (pass): 0.
  - architecture-1 (pass): 0.
  - architecture-2 (pass): 1, applied (stack.md:18, skrifa's new role).
  - architecture+layout-templates (pass): 2, applied (seven_guis.md:32 stale; :38 omission).
  - architecture+obs-plan (pass): 1, applied (escher-telemetry.md:32).
  - architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan (pass): 3, applied (CLAUDE.md:122 ×3).
  - architecture+security-plan+test-plan-1 (pass): 8, applied (escher-driver.md :6, :11, :17 ×3, :24, :32, :60).
  - architecture+test-plan (pass): 1, applied (commands.md:60).
  - layout-templates (pass): 2, applied (design-summary.md:27 ×2).
  - obs-plan-1 (pass): 1, applied (obs-summary.md:49).
  - obs-plan-2 (pass): 1, applied (observability.md:24).
  - security-plan-1 (pass): 1, applied (security-summary.md:23).
  - security-plan-2 (pass): 3, applied (security.md :21, :22, :28).
  - test-plan-1 (pass): 2, applied (tests-summary.md:24 ×2).
  - test-plan-2 (pass): 0.
- `.claude/docs/security-summary.md`: reached by group security-plan-1. Its comparer wrote one finding, on the lint's closed content. It wrote none on the two gaps the last 0-pending wrap declined: OS drag-and-drop as an untrusted input source (`dec-5vvljfpf`) and the dropped-file reader with no path restriction (`dec-fjqcv7ai`). Neither was in this pass's delta. They are named here and carried, not written by hand (inputs#I5 points 7–8).
