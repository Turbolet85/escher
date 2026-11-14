# Cascade dispositions — 2026-11-14-bundled-fonts (P2)

## Step 1 — operations landed
- architecture tx 862 (ops-architecture.md, 2) · tx 865 (ops-main-architecture.2.md, 1) · security-plan tx 863 (1) · test-plan tx 864 (2).
- Pattern files from the dry runs: none — each dry run printed `patterns: none: 1 set(s) and retire(s) changed prose alone — no file` (architecture ×2, security-plan, test-plan); no dry run was superseded.

## Step 2 — lateral binds and the sweep
- Binds: no row of test-plan §3, obs-plan §3/§6 or the a11y violation schema changed — nothing to keep in step.
- The search: main's own `cascade-patterns.toml`, two patterns over the whole of every master, its key files, the leaves, the curation homes and the bases, at baseline 557f1c98:
  - `registered-all` — the retired mechanism «registered for every generic family» (its verbs, the DejaVu-Sans-for-every-generic phrasing, the row's handle); control fired at architecture.md:654.
  - `unix-only` — the retired «the driver session … runs on unix only» (unix only, unix-only, the row's handle, a session said unix/linux-and-macOS only); control fired at architecture.md:329.
- Listing: `total (2 patterns) · 32 rows over 15 files` · `per class · new 0/0 · standing 11/4 · leaf 21/11 · curation 0/0 · base 0/0` (cascade-2026-11-14-bundled-fonts.json; part sweep-pass-1.1.txt, read whole).
- `registered-all` · standing 1 · leaf 1:
  - architecture.md:655 dec-pgrj4xim — amended (tx 862: the rule now states the fallback; `edited`, no duplicate of the retired wording stands on the line).
  - .claude/docs/design-summary.md:24 @c606 «bundle DejaVu Sans for every generic family» — leaf, re-derived at step 3 (the report's named leaf).
- `unix-only` · standing 10 · leaf 20 — no change, each a true claim sharing the token: every hit places the socket, the socket-backed command-line targets and checks, or the recording directory on unix, all still true; none says the in-process session is unix only:
  - architecture.md:329 dec-rpkdliry, :1343 dec-wx3orv7r, :1382 ctr-wo64vdv7 (recording) — no change.
  - security-plan.md:13 dec-m7xw3ssy — amended (tx 863; `edited ×2`: «exists on unix only» on the socket, the handle's own token — no duplicate of the retired clause); :172 dec-lv2rmxco (recording), :207 dec-j43eng4m (the socket) — no change.
  - test-plan.md:86 dec-dy73ren6 (CLI targets), :90 dec-e4mrafg3 (socket session checks), :611 ctr-biula2nf (recording directory) — no change.
  - obs-plan.md:293 dec-5jcc2wd6 (recording) — no change.
  - Leaves (20 rows over 10 files: rules/observability.md, rules/security.md, rules/testing.md, docs/commands.md, docs/obs-summary.md, docs/security-summary.md, docs/services/escher-driver.md, docs/services/escher-telemetry.md, docs/services/seven_guis.md, docs/tests-summary.md) — each names a unix-only test, the socket session, recording or the CLI targets: true, no change; `cascade.py leaves` takes them into step 3's set by its own read, where the comparers re-compute them against the changed rows.
- Not looked for: wording that states the generic-family registration without «every/each/all» and «registered» together, or the session's platform without «unix»; the comparers' recompute of each changed row's delta is the control for that reach.

## Leaves
Set: `cascade.py leaves` (pass mode, baseline 557f1c98) — `total · pass · 12 group(s) · 23 leaves · 617578 B · unmapped 1`; `.claude/rules/host-linux.md` unmapped (the host leaf — no master states it, no step-3 edit). Apply: `total · blocks 11 · applied 10 · held 1`.
- added-1 (design-summary.md, obs-summary.md) · pass · findings 0 · applied 0 · by hand 0 · declined 0 — design-summary.md:24 «WASM builds bundle DejaVu Sans for every generic family» re-read by hand: a WASM statement from design-system decision/wasm-bundled-font-context (unchanged), true by layout, the stand's typography pointed to architecture — no change.
- added-2 (rules/observability.md) · pass · findings 0.
- architecture-1 · pass · findings 1 · applied 0 · declined 1 — gotchas.md:52 «registers the bundled DejaVu Sans with system fonts off» stays true; the fallback detail stays in its master (held).
- architecture-2 · pass · findings 0.
- architecture-3 · pass · findings 2 · applied 2 (services/seven_guis.md:19, :38).
- architecture+security-plan+test-plan-1 · pass · findings 1 · applied 1 (services/escher-driver.md:61).
- architecture+security-plan+test-plan-2 · pass · findings 1 · applied 1 (CLAUDE.md:122).
- architecture+test-plan · pass · findings 2 · applied 2 (commands.md:28, :82).
- security-plan-1 · pass · findings 1 · applied 1 (security-summary.md:20 — «unix only» narrowed to the socket).
- security-plan-2 · pass · findings 0.
- test-plan-1 · pass · findings 2 · applied 2 (tests-summary.md:21, :26).
- test-plan-2 · pass · findings 1 · applied 1 (rules/testing.md:36).
Step-2 stale-claim fixes in the seven masters: none owed — the sweep's standing rows hold no other statement of either retired claim.
