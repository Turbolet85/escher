# P5 review — 2026-11-21-dependency-and-ci-debts · «Dependency and CI debts»

**The plan:** `doc:2026-11-21-dependency-and-ci-debts/plan`, rev 2, 397 lines. Its render, to open in an editor, is
`.andromeda/cache/docs/2026-11-21-dependency-and-ci-debts/plan.md`. No plan stands in a chunk's folder, and writing in
that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-21-dependency-and-ci-debts` shows a diff of the chunk's records.

## Goal
Close Epoch 2's dependency and CI debts (`v040-07`), with no line of Rust changed:
- every advisory gets one outcome: fixed, a wait on a named release, or not closing;
- skia's ruled verified archive is built;
- the cancelled-runner watch is retired;
- the `CI=true` rebuild is stated as a measured limit;
- the yanked `async-executor` is updated;
- `v040-08` is claimed gated on the Epoch 2 boundary audit.

## The words this plan rests on
- **The founder's ruling:** the verified skia archive, «Проверенный архив (Recommended)» (inputs#I2 §1a).
- **The operator's directive:** inputs#I1.
- **The operator's technical answers:** inputs#I3 (the operator, 2026-11-21).
- **The founder's three forks:** F1 A · F2 A · F3 A (inputs#I4), each the least-change option. These are the
  operator's answers in his stead — delegate the operator, 2026-11-21 — and stand PROVISIONAL for his review in the
  morning. They are not his words. His own word supersedes them by rule. None widens a boundary or changes behaviour.

## Top acceptance criteria
- **The audit leg** reads `advisories ok, licenses ok` with no `warning[yanked]`.
  - `deny.toml` keeps its six IDs; each reason names the release it waits on.
  - RUSTSEC-2026-0459's reason names its owner, `res-j4x7iiqc`.
- **`evidence/advisory-standing.md`** enumerates the eleven advisory IDs once each:
  - zip ×4 closed;
  - sqlite, paste, zbus ×3, ttf-parser and memmap2 each a wait on a named release.
- **The verified archive:**
  - the script refuses a mismatched archive and leaves no file;
  - the record's tag is held to the lock's `skia-bindings`;
  - every skia-building leg runs cargo under `FORCE_SKIA_BINARIES_DOWNLOAD=1`;
  - the matrix step stands on every row;
  - after `fast`, the dev host's skia-bindings output names a `FROM: file://` URL.
- **CI** reads `verdict: green` on the final HEAD, every linux leg and the windows, macos, ios and android rows
  building from the verified file. This confirms the four derived keys.
- **The cancelled-runner watch** is retired by its rule, with the 60-run census and the hand re-run remedy stated.
- **The stand's adapter probe** reads 0 after the lock move.
- **`v040-08`** is gated on the Epoch 2 boundary audit's metric row.

## Touchpoints
- **New:**
  - `.github/skia-archives.toml`;
  - `.github/scripts/skia_archive.py`;
  - `.github/scripts/test_skia_archive.py`.
- **Modified:**
  - `.github/scripts/ci-leg.sh`;
  - `.github/scripts/test_ci_workflows.py`;
  - `.github/workflows/ci.yml`;
  - `deny.toml` (two reasons);
  - `Cargo.lock` (async-executor 1.14.0 → 1.14.1).

## Test commands — 11 entries, baselines read at P5
| # | entry | baseline |
|---|---|---|
| 1 | `test_skia_archive.py` | red, the file is absent (predicted) |
| 2 | `test_ci_workflows.py` | green |
| 3 | `ci-leg.sh audit` | red on the yanked warning alone (predicted) |
| 4 | `ci-leg.sh fast` | seen green, previous chunk |
| 5 | skia `FROM: file://` probe | red, every output reads `https://` (predicted); its literal re-spelt from the build output |
| 6 | adapter probe | green, 0 |
| 7 | evidence count | red, the file is absent (predicted) |
| 8–10 | operator: hygiene · push · `ci.py --wait 2700` | — |
| 11 | gated `v040-08` metric row | red, 0 rows; the same query on 0.3.0 Epoch 6 reads 1 row |

There are two one-shot controls: the sum check removed, and the `FORCE_SKIA_BINARIES_DOWNLOAD` export removed.

**Mechanical checks:** the fence parses; planlint reports 0 hits; all four snapshots are cited and unchanged; the
modify paths exist; the archive directory is gitignored.

## Concretization previews (before approval)
- **`v040-07` — claimed by this chunk.**
  - Now: «The audit leg reads green; each advisory is closed by a reachable update or stated as a wait on a named
    release, and any that does not close is named; the cancelled-runner watch is retired by its rule or kept by the
    founder's word.»
  - Proposed: «`bash .github/scripts/ci-leg.sh audit` reads `advisories ok, licenses ok` with no `warning[yanked]`.
    `deny.toml`'s six ignores each name the release they wait on, RUSTSEC-2026-0459's its owner `res-j4x7iiqc`.
    `evidence/advisory-standing.md` enumerates the eleven advisory IDs once each — zip ×4 closed by upstream's AnyRender
    0.15 take; sqlite, paste, zbus ×3, ttf-parser and memmap2 each a wait on a named release; none without an outcome
    — and states the cancelled-runner watch retired by its rule, with its 60-run census and the hand re-run remedy. CI
    reads green on the final HEAD.»
  - Nothing of the outcome is weakened. The enumeration is implement's own (by construction).
- **`v040-08` — claimed GATED.**
  - The acceptance stands as written.
  - Its block: «the Epoch 2 boundary audit of escher-0.4.0 recorded, its reading cleared by the operator word (CARRY
    cry-hpakkesz; inputs#I3)».
  - After the word, this chunk's frozen line takes that `BLOCKED-ON`. Its ref waits on your recorded clearance. No
    stand-in verifies it.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | A cross-version residual row owning the RUSTSEC-2026-0192 ignore (a wait on a winit 0.31 beta chain release past ttf-parser 0.25.1), minted at the wrap. Its id is re-pointed into `deny.toml`'s reason by a CARRY on «Upstream sync ahead of the canvas». | the wrap; then «Upstream sync ahead of the canvas» |
| 2 | The six residual rows of this cap, dispositioned at the wrap: `res-h7acz56c` closed, fixed; `res-q2tllspz` retired by its rule (F2 A); `res-j4x7iiqc`, `res-kdhnfj5p`, `res-p73cueyv` and `res-uqt2yd2y` re-carried as stated waits (F1 A, F3 A). | the wrap |
| 3 | F1 A · F2 A · F3 A held PROVISIONAL until the founder's own word. | the wrap; the founder |

Apply? (yes / review / cancel)
