# Cascade dispositions — 2026-11-21-dependency-and-ci-debts (wrap 2026-11-22T02-02-17-wrap)

## Step 2 — the sweep
The search: `cascade.py sweep --patterns-file cascade-patterns-security-plan-1.toml --patterns-file cascade-patterns.toml`
(`cascade-2026-11-21-dependency-and-ci-debts.json`, part `sweep-pass-1.1.txt`, read whole).
- `security1-1` — the tool's term for the set of `decision/skia-default-build-fetch` (`GitHub`, the old cell's
  «from GitHub releases»), control the row's r1.
- `main-skia-nochk` — main's, the retired mechanism's phrasings over the whole amended master and every master: «no
  content check» · «no checksum» · «without a checksum» · «unverified prebuilt/archive/binaries» · «fetching/fetches
  prebuilt binaries», case-insensitive.
- Not looked for: the archive's own new wording (the sets added text; their new terms are the leaves' read, step 3).

Counts (copied): `total (2 patterns) · 22 rows over 10 files` · `per class · new 0/0 · standing 11/4 · leaf 10/5 ·
curation 1/1 · base 0/0`.

Rows:
- standing 10 (`security1-1`): architecture 2 (`dec-qhvyrurd` GitHub Actions as CI, `ovw-ygwga2zf`), security-plan 5
  (`dec-gkacphd2`, `dec-jcciehod`, `dec-jwf3ad3c`, `dec-guzjzacp`, `dec-ks63pzxp`), test-plan 1 (`dec-cgtng4i3`), obs-plan
  2 (`dec-5nul46au`, `dec-5lx7vbr7`) — no change: each names GitHub Actions, GitHub Pages or the GitHub API, a true
  claim sharing the token; none says skia is fetched unchecked.
- standing 1 (`main-skia-nochk`): security-plan `dec-jm3wczan` — no change: taffy's git lock entry has no checksum, a
  true claim sharing the token.
- curation 1: `.claude/docs/session-learnings.md:112` — no change: a GitHub-hosted job the runner pool never picks up;
  true, and preserve-verbatim.
- leaf 10, to step 3's set by the tool: `.claude/rules/security.md:28` (both patterns, ×3 for `main-skia-nochk`, the
  taffy clause and the skia clause «prebuilt binaries from GitHub releases at build time with no content check» —
  **stale**, the skia clause); `CLAUDE.md:53`, `CLAUDE.md:59`, `.claude/rules/security.md:7`,
  `.claude/docs/security-summary.md:30`, `.claude/docs/stack.md:41`, `.claude/docs/stack.md:50`,
  `.claude/docs/workflow.md:8` ×2, `.claude/docs/workflow.md:10` — GitHub Actions or taffy's checksum, no change on this
  token; each leaf is re-computed whole in step 3.
- base 0.

No cross-master citation of the retired mechanism stands in any master: no fold into this pass.

## Leaves
The set: `cascade.py leaves --all` — the WHOLE mode, on the operator's word (inputs#I8); its `epoch ·` row read
`«Epoch 2 — Debts of 0.1.0–0.3.0» · markerless 0 · gated 0 · closes`. 16 groups · 25 leaves; `unmapped
.claude/rules/host-linux.md (host)` — the host leaf, a statement no source states, no comparer's. Comparers 16/16
pass (`fanout.py check`).

`leaves --apply` (all 16 files): `total · blocks 26 · applied 23 · held 3`.
- whole · a11y-plan-1, architecture+layout-templates, architecture+obs-plan, obs-plan-1: 0 findings.
- whole · a11y-plan-2: 1 · applied 1 (a11y.md:19, a single select's `size` now read).
- whole · architecture-1: 3 · applied 2 (blitz-dom.md:21 the inline-box fallback family; :33 the events tree —
  `src/events/` holds `mod, driver, keyboard, pointer, wheel, ime`, no `focus.rs`, read at the tree) · declined 1:
  conventions.md:7 — the `//!` exception is a detail that stays in its master; no reader acts against it.
- whole · architecture-2: 2 · applied 2 (stack.md:42 — the apt cache and setup-ndk actions, `ci.yml` read; the skia legs).
- whole · architecture+security-plan+test-plan-1: 3 · applied 3 (escher-driver.md:11, :16, :18).
- whole · architecture+test-plan: 4 · applied 4 (commands.md:13, :14, :15, :62).
- whole · design-system+layout-templates: 1 · applied 1 (design-summary.md:36).
- whole · masters: 2 · applied 1 (CLAUDE.md:110, the skia legs) · declined 1: CLAUDE.md:107 — the index's
  `cargo build --workspace` line; the verified-URL rule binds the legs, its home the master row.
- whole · obs-plan-2: 3 · applied 3 (observability.md:22, :23, :32).
- whole · security-plan-1: 2 · applied 2 (security-summary.md:13, :25).
- whole · security-plan-2: 2 · applied 2 (security.md:28 the stale skia fetch; :17).
- whole · test-plan-1: 2 · applied 1 (tests-summary.md:29 the matrix skia steps) · declined 1: tests-summary.md:29#2 —
  not stale; it would only drop the founder's quoted word for the a11y leg's membership.
- whole · test-plan-2: 1 · applied 1 (verification-harness.md:25).

By hand (each drafted whole, `rec.py harness land`, dry run first), the handoff's named stale lines (inputs#I8) and the
plan's harness entry:
- `.claude/rules/a11y.md:35` and `.claude/docs/tests-summary.md:21` — the per-file test counts of `select_interaction`,
  `stand_act_select`, `select_popup`, `stand_act_popup` removed, the tests named alone (the 2026-11-18 learning: a
  docs leaf states no per-file test count) (tx 3046, 3047).
- `.claude/rules/testing.md:15` — «36 readers» and «32 checks» removed, the readers named alone (`git grep -l
  '^mod session_common;'` reads 46 and `'^mod fixtures;'` 41 at this tree — counts no leaf keeps) (tx 3048).
- `.claude/docs/stack.md:3` and `.claude/docs/conventions.md:3` — «(citations live there)» removed: a master stands as
  rows and a row holds no citation (tx 3049, 3050).
- `.claude/docs/workflow.md:8` — the fork's default branch `build/escher-0.4.0` (CARRY `cry-g3etodij`, its measurement
  the operator pass of 2026-11-20-readme-s-0-3-0-closed) (tx 3051).
- `.claude/rules/security.md:26` and `.claude/docs/security-summary.md:42` — the zip clause: the four advisories closed,
  zip 9.6.0 alone in the lock, the ignores gone, `res-h7acz56c` closed (the plan's harness entry; no comparer flagged it)
  (tx 3052, 3053).
