# P5 review — 2026-11-21-dependency-and-ci-debts · «Dependency and CI debts» (card 2, after the review fold)

**The plan:** `doc:2026-11-21-dependency-and-ci-debts/plan`, rev 5, 440 lines with its fence. Its render, to open in an
editor, is `.andromeda/cache/docs/2026-11-21-dependency-and-ci-debts/plan.md`. No plan stands in a chunk's folder, and
writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-dependency-and-ci-debts` shows a diff of the
chunk's records.

## What the review changed (the operator's review of p5-card-1.md, inputs#I5)
**New step 4, the five archives measured on this host.** It runs after gates 1–2 and before any later step.
- The script itself fetches each of the five archives from its release URL into `target/skia-archives/` and verifies
  it against the record (new gate 8, last line 5).
- `evidence/skia-archives-measured.md` records, per target, the sha256 the script computed beside the release's
  published digest, `equal` (new gate 9, last line 5).
- A key that does not resolve, or a sum that differs, stops implement at that step. The record is never edited to
  match a download.
- The recorded sums are then ones this chunk measured.
- The old steps 4–9 are now 5–10, every reference renumbered. The operator entries are now 10–12 and the gated entry
  13.

**What step 4 proves, and what it does not:**
- it proves each recorded key resolves to a published archive, with its sum measured here;
- that each non-linux build *requests* exactly that key stays the derivation from anyrender_skia's per-target
  features, witnessed by CI under `FORCE_SKIA_BINARIES_DOWNLOAD`;
- the linux key alone is also read from a real build (gate 5).

The plan says so in its Implementation notes and its CI criterion.

**Unchanged:** everything else stands as on card 1.

## The words this plan rests on
- **The founder's ruling:** the verified skia archive, «Проверенный архив (Recommended)» (inputs#I2 §1a).
- **The operator:** the directive (inputs#I1), the technical answers (inputs#I3), the review (inputs#I5).
- **The founder's three forks:** F1 A · F2 A · F3 A (inputs#I4) — delegate the operator, 2026-11-21. These are
  PROVISIONAL for the founder's review and are not his words.

## Top acceptance criteria
- **The audit leg** reads `advisories ok, licenses ok` with no `warning[yanked]`. Each `deny.toml` reason names its
  wait; RUSTSEC-2026-0459's names `res-j4x7iiqc`.
- **`evidence/advisory-standing.md`** enumerates the eleven advisory IDs once each.
- **The verified archive:**
  - script and leg tests read OK;
  - after `fast`, the dev host's skia build reads a `FROM: file://` URL;
  - **all five archives are fetched and verified on this host before any CI run, each sum measured beside its
    published digest**.
- **CI** reads `verdict: green` on the final HEAD, every row building from its verified file.
- **The cancelled-runner watch** is retired by its rule.
- **The adapter probe** reads 0.
- **`v040-08`** is gated on the Epoch 2 boundary audit.

## Test commands — 13 entries, baselines read at P5
| # | entry | baseline |
|---|---|---|
| 1 | `test_skia_archive.py` | red, the file is absent (predicted) |
| 2 | `test_ci_workflows.py` | green |
| 3 | `ci-leg.sh audit` | red on the yanked warning alone (predicted) |
| 4 | `ci-leg.sh fast` | seen green |
| 5 | skia `FROM: file://` probe | red, `https://` only (predicted) |
| 6 | adapter probe | green, 0 |
| 7 | advisory evidence count | red, the file is absent (predicted) |
| 8 | **five archives fetched and verified by the script** | red, the script is absent (predicted); its literal read 5 over five plan-shaped lines |
| 9 | **measured-sums evidence count** | red, the file is absent (predicted); its literal read 5 over five plan-shaped lines |
| 10–12 | operator: hygiene · push · `ci.py --wait 2700` | — |
| 13 | gated `v040-08` metric row | red, 0 rows; control reads 1 row |

**Mechanical checks re-run after the fold:** the fence parses; planlint reports 0 hits; all five snapshots are cited
(I1–I4 unchanged, I5 new); the lists parse.

## Concretization previews (before approval)
- **`v040-07` — claimed.** As on card 1, the verified-archive clause added:
  - «`bash .github/scripts/ci-leg.sh audit` reads `advisories ok, licenses ok` with no `warning[yanked]`.
  - `deny.toml`'s six ignores each name the release they wait on, RUSTSEC-2026-0459's its owner `res-j4x7iiqc`.
  - `evidence/advisory-standing.md` enumerates the eleven advisory IDs once each — zip ×4 closed; sqlite, paste,
    zbus ×3, ttf-parser and memmap2 each a wait on a named release; none without an outcome — and states the
    cancelled-runner watch retired by its rule, with its 60-run census and the hand re-run remedy.
  - CI reads green on the final HEAD.»

  The original outcome is not weakened.
- **`v040-08` — claimed GATED**, its acceptance as written, its block «the Epoch 2 boundary audit of escher-0.4.0
  recorded, its reading cleared by the operator word (CARRY cry-hpakkesz; inputs#I3)». This chunk's frozen line takes
  that `BLOCKED-ON` after the word.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | A cross-version residual row owning the RUSTSEC-2026-0192 ignore (a wait on a winit 0.31 beta chain release past ttf-parser 0.25.1), minted at the wrap. Its id is re-pointed into `deny.toml`'s reason by a CARRY on «Upstream sync ahead of the canvas». | the wrap; then «Upstream sync ahead of the canvas» |
| 2 | The six residual rows of this cap, dispositioned at the wrap: `res-h7acz56c` closed, fixed; `res-q2tllspz` retired by its rule (F2 A); `res-j4x7iiqc`, `res-kdhnfj5p`, `res-p73cueyv` and `res-uqt2yd2y` re-carried as stated waits (F1 A, F3 A). | the wrap |
| 3 | F1 A · F2 A · F3 A held PROVISIONAL until the founder's own word. | the wrap; the founder |

Apply? (yes / review / cancel)
