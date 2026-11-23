# P5 review — 2026-11-23-circle-drawer-on-a-canvas · revision 1

**Cause:** the operator's word (`inputs#I4`, `.andromeda/runs/2026-11-23T22-07-35-phase/relay-1.md`): gate 9 and its
acceptance line read `4+0`, step 4 names four dependencies; nothing else changes. The stopped implement run
(`.andromeda/runs/2026-11-23T21-22-31-implement`) measured the lock's `seven_guis` entry gaining `anyrender`,
`blitz-dom`, `parley`, `peniko`, no package added; `blitz-dom` because `Widget::paint` names
`blitz_dom::node::ComputedStyles`, which nothing seven_guis already depends on re-exports.

**What changed (plan rev 3 → 5):**
- Step 4 — `[dependencies]` gains `anyrender`, `blitz-dom`, `parley`, `peniko`; the lock entry gains four lines.
- Files to modify — `examples/seven_guis/Cargo.toml` names the four crates; `Cargo.lock` «four dependency lines».
- Test Commands — the gate-9 line «four lines added»; gate 9 `expect = ['exit 0', 'last line 4+0']`, its note naming
  the four crates, `baseline = "green — exit 0, last line 4+0: read at revision 1 on the tree the stopped implement
  run left, its step 4 in place; the take-up read red on the untouched lock, last line empty"`.
- Acceptance (gates) — «the lock probe reads `4+0`».
- Implementation notes — one bullet: steps 1–10 stand done in the tree by the implement run (its outcome file, red
  logs, `evidence/control-1.txt` 5 holds); `/andromeda-implement` re-enters at its measuring half (its P2), the
  gates fired again on the tree as left, no step re-written.

**Goal, other acceptance criteria, touchpoints and every other gate:** unchanged.

**Research:** lists synced to the revision's delta (+0 −0), rev 3; not re-derived since the take-up's rev 2.

**Checks:** gate dry-run parses (16 entries) · `gate.py baseline`: gate 9 baseline-run green `4+0`, the other entries
carried over unchanged with their baselines · `planlint`: hits 0 · `inputs.py verify`: 4 entries, uncited 0 ·
`scope lists: parsed 11 items` · `scope sync: PASS · delta +0 −0`.

**Capabilities:** matrix#v040-09 (claimed at the take-up, now `implemented`) — the revised plan still proves it; its
acceptance unchanged, no re-claim.

**Plan to open:** `.andromeda/cache/docs/2026-11-23-circle-drawer-on-a-canvas/plan.md` — no plan stands in the
chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
{tools_dir}/rec.py diff --root . 2026-11-23-circle-drawer-on-a-canvas` is a diff of the chunk's records.

## Beyond the plan
none

Apply? (yes / review / cancel)
