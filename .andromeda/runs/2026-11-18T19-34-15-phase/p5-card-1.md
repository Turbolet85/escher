# P5 review — 2026-11-18-cold-agent-live-run, revision 1 (plan rev 4 → 6)

**Cause:** /andromeda-implement stopped surfaced on entry 12 (inputs#I4). The credential probe's `api[_-]?key` arm
matched the client's init field NAME `apiKeySource` (value `none`) in `live-transcript.jsonl` line 1. **Word:**
inputs#I5, your directive for this revision, verbatim in `.andromeda/runs/2026-11-18T19-34-15-phase/relay-1.md`.

## What changed
- **Entry 12 `run`.** One arm of the regex changed.
  - Before: `api[_-]?key`
  - After: `api[_-]?key(?![a-z])[\x22\x27]?\s*[:=]\s*[\x22\x27]?[^\s\x22\x27,}]+`, under `(?i)` as before.

  The api-key arm now counts the name only when no letter follows it and a `:` or `=` and a non-empty value come
  after. So `apiKeySource` is not counted (the `S` follows `Key`), and `"api_key":"sk-…"` and `API_KEY=…` are
  counted. The `authorization:`, `bearer ` and e-mail arms, the at-least-four-files assert, the file set (live-* plus
  docs/capabilities.md) and `expect` are byte-equal. The quote marks are spelled `\x22` and `\x27`, so the shell and
  TOML pass the pattern through unescaped.
- **Entry 12 `baseline` and `note`.** Rewritten for the revision and its controls. `new = true` is kept.
- **Implementation notes.** One added bullet, «Revision 1 — where /implement re-enters», which the skill's revision
  requires. Steps 1-8 of the stopped run stand. /implement re-enters at entry 12 alone, then writes v030-15's `ref`,
  then does step 9. It touches no tree, no evidence and no stash, reset or clean, and never re-fires entry 9.
- **Scope rev 4.** One bullet records the word, closed by the controls below.
- **Research rev 3.** `## Revision 1` with an empty delta.
- **No other plan line moved.**

## Controls (`.andromeda/runs/2026-11-18T19-34-15-phase/p5-controls.md`)
Each control fired as entry 12 itself from the store's plan.

| Input | Reading |
|---|---|
| The real evidence, on the tree (read only) | **green**, last line 0 |
| The same evidence copied into a scratch clone | green, 0 |
| Planted `{"api_key":"sk-ant-ctl0000000000"}` | **red**, 1 |
| Planted `Bearer ctl0000000000` | **red**, 1 |
| Planted `ctl.person@example.com` | **red**, 1 |
| Planted `Authorization: ctl0000000000` | **red**, 1 (added beside your three) |
| `{"apiKeySource":"none"}` planted a second time | green, 0 |

Per-arm counts on the real evidence: the old api-key arm 1, the new one 0, and the other arms 0.

## Mechanical checks
- `gate.py scope --sync`: PASS, delta +0 −0.
- `gate.py run --dry-run`: 20 entries parse.
- `gate.py baseline`: entry 12 `new (key)`, baselined above. The other entries carry the take-up's baselines,
  unchanged.
- `planlint.py`: hits 0. Check 12 (d) lists the take-up's «unchanged» claim on scripts/cold-agent.sh. It is
  unchanged by this revision and consistent with step 1.
- `inputs.py verify`: 5 entries, 0 drifted, 0 uncited.
- `gate.py scope --lists-only`: parsed.
- Validation-1: aligned. The revision changes only the instrument of the security criterion. The intent and the
  scope's work are unmoved.

**Research:** lists synced to the revision's delta (+0 −0), rev 3; not re-derived since the take-up's rev 2.

## Capability
v030-15 stays claimed by this chunk (`planned, gated`). Its acceptance does not read entry 12, so the revised plan
still proves it as concretized at the take-up. No preview change, no re-claim and no un-claim.

## Your edits
The plan's render is `.andromeda/cache/docs/2026-11-18-cold-agent-live-run/plan.md`, to open in an editor. No plan
stands in the chunk's folder. Writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-cold-agent-live-run` diffs the chunk's
records.

## Beyond the plan
none (standing — `.andromeda/runs/2026-11-18T18-34-15-phase/p5-card-1.md`: none)
