# P5 review — 2026-10-17-upstream-sync-engine-debts

**Plan:** `escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/plan.md` (547 lines, about 250 of prose; 10 steps;
47 gate entries; 14 acceptance criteria) · **Research:** `research.md` (36 scope items) · **Scope:** `scope.md` (closed at P3)

## Goal
Merge DioxusLabs/blitz `upstream/main` at the pin `23b9b26a9`: 10 of 14 commits, workspace 0.3.0-beta.4, the bump
crate removed. Taffy stays registry 0.15, and upstream's four Taffy-main git bumps go to the next sync. The merge is
one two-parent commit with our changes mostly additive. Every escher proof then still passes, fork CI is green, the
per-OS tallies are re-read, and the sink allowlist is re-checked.

## What the merge makes us do
- **`accessibility.rs` conflict — both sides kept.** Ours stays: the `<label>` association and the closed-set
  `aria-selected`/`aria-pressed`/`aria-invalid` reads. Upstream's six new reads come in as written: `aria-labelledby`,
  `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled` and `aria-level`. Upstream's duplicate
  `aria-selected` and `aria-label` reads are dropped, which changes no outcome against any upstream test. Our label
  loop runs before upstream's labelledby loop, so `aria-labelledby` outranks `<label>` (accname 2B before 2D).
- **`Cargo.lock`:** regenerated at the pin, our two `escher-*` entries moved to beta.4. `test-that` 0.6.0 is reviewed
  by hand, and four bump-only crates leave.
- **keyboard-types 0.8:** the textual merge is clean but does not compile, so 15 of our files take
  `Key::Named(NamedKey::…)`. The harness re-exports `NamedKey`. The macOS-only `ACTION_MODIFIER` goes `SUPER` → `META`,
  which only the macOS CI job can witness. The driver's key words, schema and wire do not move.
- **Coverage:** the leg's two `llvm-cov` calls skip upstream's `keyboard_compat.rs` by filename, as a named exclusion
  that goes when upstream retires the shim. Floors unchanged.

## The founder's word
«Утвердить оба (Рекомендую)» — the founder, 2026-10-17, by question dialog in the overseer session, relayed by the
overseer (inputs#I2):
- `aria-labelledby` names are inside the ratified name crossing.
- `aria-description` to the platform adapter is ratified as upstream's behaviour, pinned by upstream's
  `accessibility_description` test.

Recorded as his word, not PROVISIONAL. The plan carries it to the wrap as an expected amendment naming the playbook
pattern «Boundary widening by upstream merge», so the wrap halts on it once, as the answer card.

## Top acceptance criteria
- Merge witness green. The pins resolve as one set: Taffy registry 0.15, beta.4, no git dependency.
- The lock holds exactly upstream's set at the pin, and the audit reads `advisories ok` and `licenses ok`.
- One read each of `aria-selected` and `aria-label`, both ours; upstream's six reads present; label loop before the
  labelledby loop. `accessibility_states`, `accessibility_names`, `accessibility_label` and `accessibility_description`
  pass.
- 127 files / 126 targets, every target run by name, no `all` target.
- `fast`, `doc`, `audit`, `a11y` (6 · 43 · 0), `coverage` (3 of 3 floors), `run stand`, `run all` and the seven_guis
  tests green.
- Fork CI green on the pushed merge. Per-OS tallies re-read against CI#39017887462, every move explained.
- Upstream adds 0 log, env or listener sites, and the trace captures read 0 needles under both builds.

## Test commands
47 entries. Baselines on the untouched tree (2026-10-17T14:22–14:29Z):
- Every red is the pre-merge state its entry exists to move. Each literal-asserting red has a must-pass reading over
  a scratch-clone merge of the pin.
- Every green guard has a must-fail control.
- All controls are in `.andromeda/runs/2026-10-17T12-18-00-phase/p5-controls.md`.

Baselines: `run all` 533 · 0 · 12; `run stand` 168 · 0 · 7; coverage dioxus-native-dom 2472/2934 = 84.25 %. The four
operator entries cover hygiene, the push behind the pre-push gate and a clean-tree guard, the CI read (`--wait 3600`)
and the per-OS tally read.

## Mechanical checks and validation-1
- All mechanical checks pass. planlint's one hit, 4 (13), is the per-OS log read: its note names a read ci.py never
  makes and carries a control fired at P4.
- Size: about 250 prose lines, inside the band.
- Validation-1: aligned with `scope.md` and the working entry.
- The coverage band B warning does not fire: 0 extracts had no domain coverage.

## Matrix
No capability is claimed; a sync makes none verifiable. Nothing to concretize.

## Beyond the plan
none
