# Cascade dispositions — 2026-10-06-upstream-sync-element-identity

## Step 1 — bodies applied (order: mechanical re-point first, then the semantic amendments)
- O1 + E1 — one scratch script over the seven masters (registries: 0 citations into the merged paths, 0 root
  `Cargo.toml` citations): the merge's measured line map (`git diff -U0 ab936a9e HEAD` hunks per merged path) for
  every `{path}:{N}[-{M}]` citation into the 16 merged non-manifest paths; 4 hand overrides read against HEAD
  (architecture `convert.rs:432-435`→`436-439`, `510-512`→`546-548`; security-plan `436-437`→`440-441`,
  `513-514`→`549-550`); 1 left for A1 (`convert.rs:522-534`); the root `Cargo.toml:N≥61` citations whose token stood in
  the masters at `1503df2e` re-pointed +1 per number ≥ 61 (operator: fix now). Asserted: digit-stripped text
  unchanged, line counts unchanged, 0 unmapped. Citations re-pointed (tokens) — merge map / root manifest +1:
  architecture 113 / 43 · security-plan 40 / 2 · design-system 17 / 0 · layout-templates 4 / 0 · test-plan 19 / 1 ·
  obs-plan 20 / 6 · a11y-plan 16 / 1. Spot-checks: `document.rs` 2454→2336 and 2824→2720 cite identical text at
  base and HEAD; the 53 root-manifest numbers verified by content before the write (46 `Cargo.toml@1503df2e^:N ==
  HEAD:N+1`, 7 pin lines against `ab936a9e:N+1`).
- A1 + A2 — architecture §Established Decisions [CSS approximations] (line 85).
- O2 — architecture §Project Intent: new `Upstream sync:` line after the `Origin:` line (kept byte-identical — setup
  reads `Origin: adopted at {sha}`).
- O3 — security-plan §Error Handling (line 321).
- T1 — test-plan §9 Local baseline (line 314); O4 — test-plan §1 (line 27) and §4 (line 148).

## Step 2 — sweep (`cascade.py sweep`, `cascade-patterns.toml`, 9 patterns, every control fired)
Patterns: the retired claims' words and mechanisms (`left unset`; `mapped to none|maps? to none`); the amended
subject (`align-items|justify-items|item_alignment|content_alignment`; ``defaults? to `?safe``); the moved count
(`430 (passed|·)`); the inventory (`1 in mod\.rs|8 in fuzzy\.rs`); the merge base (`0f60502e`); the stale root-manifest
numbers (`Cargo.toml:(103|113)`, `Cargo.toml:198-199`). Dropped by the tool (control never fired on the pre-pass
masters): `per-item (default|alignment)`, `normal[^.;)]{0,60}stretch` — swept by hand, with `\bstretch`, over the seven
masters, every registry file, CLAUDE.md, `.claude/rules/*`, `.claude/docs/**`, the playbook and the drift-base (39
files): 0 hits.

Rows (the tool's listing), each dispositioned:
- `.andromeda/architecture.md:85` align-items ×2 — amended (A1/A2: the new text). No retired wording remains on the line.
- `.andromeda/architecture.md:85` safe-default — amended (A2), true as written.
- `.andromeda/test-plan.md:314` count-430 — no change: the headless-stand re-count stays as history; T1 appended the 431 re-count.
- `.claude/docs/tests-summary.md:28` count-430 (leaf) — re-derived: 431 · 0 · 4 at this chunk, 430 kept as history.
- `.andromeda/test-plan.md:27` wpt-inventory ×2 — amended (O4: `1 in attr_test.rs` added).
- `.andromeda/architecture.md:201` adopted-at — no change: the adoption base stays true; O2 added the sync line beside it.
- `.claude/docs/workflow.md:15` adopted-at (leaf) — re-derived: the upstream sync base added beside the adoption.
- `.andromeda/architecture.md:48` cargo-103 (new) — E1's own re-point (`Cargo.toml:102-112` → `103-113`, the taffy
  stanza at HEAD 103-113); correct.
- left-unset · mapped-none · cargo-198 — 0 rows (control fired): no standing restatement anywhere swept.
Curation homes and judgment bases: 0 rows on every pattern.

## Step 3 — leaves re-derived
- Changed sources: all seven masters (the re-point touched each); semantic changes in architecture, security-plan, test-plan.
- Re-derived with a change: `.claude/docs/tests-summary.md` (count), `.claude/docs/workflow.md` (sync base).
- Re-computed, no change: CLAUDE.md `GENERATED:setup:*` (no block states the CSS approximations, the sync base or a
  root-manifest line; pointer table unchanged); `.claude/docs/{stack,conventions,commands,gotchas}.md` and
  `services/*.md` (carry no file:line citation into the merged paths or the root manifest — shift-map scan 0, root
  `Cargo.toml:N` scan 0; no alignment claim — sweep 0); `security-summary.md` (no `mapped to none` restatement);
  `design-summary.md`, `obs-summary.md`, `a11y-summary.md` (digit-only changes in their sources, no citations in the
  leaves); `.claude/rules/*` (the `wpt/runner/**` glob in testing.md unaffected).
- Lateral binds: test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema — neither side's claims changed (digits only).
