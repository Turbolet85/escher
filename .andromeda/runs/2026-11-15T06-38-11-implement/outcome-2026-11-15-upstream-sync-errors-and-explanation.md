# Implement outcome — 2026-11-15-upstream-sync-errors-and-explanation

**Outcome: green + smoke.** Run `.andromeda/runs/2026-11-15T06-38-11-implement`. The gate block went green on its first firing: entries 1–13 green, 0 red; the operator entries 14–16 were not run. Gate trail: `.andromeda/runs/2026-11-15T06-38-11-implement/gate-2026-11-15-upstream-sync-errors-and-explanation.json`.

Setup's line on the phase run's matrix trail, as `rec.py runs --kind '*-phase' --file '*-{marker}.json'` printed it:
`2026-11-15T05-31-22-phase/matrix-2026-11-15-upstream-sync-errors-and-explanation.json · open · 517 B`

## Files changed
- `packages/blitz-dom/src/document.rs` (+1 +1), from `f5ea1caf1`: `width: overrides.width` at the `@font-face` override site, and `width: None` in the `font_face_override_tests` fixture.
- `packages/blitz-dom/src/net.rs` (+8), from `f5ea1caf1`: the `FontFaceOverrides::width` field and its read from Stylo's `font_width` range.
- `wpt/WPT_COMMIT`, from `c30096724`: `07e827e4…` → `bbb30d0f…`.
- `wpt/runner/timeout-quarantine.txt` (+4), from `d2ee20b98`: four `dynamic-load-events` entries.
- `Cargo.lock`, by `cargo update -p taffy --precise 0.15.1`: `taffy` 0.15.0 → 0.15.1, its version and checksum only, source still the crates.io registry.
- The three picks, `git -c rerere.enabled=false cherry-pick --no-commit f5ea1caf1 c30096724 d2ee20b98`, are staged and each applied clean. The tree holds no sequencer state. The three `(cherry picked from commit …)` trailers are in `evidence/lock.md`.
- Chunk evidence under `.andromeda/cache/chunks/2026-11-15-upstream-sync-errors-and-explanation/evidence/`:
  - `lock.md`: 3 taken; the Taffy move and its 13-of-14 table; 13 new left out; the 99 re-read; 112 total; trailers.
  - `taffy.md`: the discharge record of `cry-r2cidrqy`.
  - `readings.md`: the merge surfaces, and all ten carries.
  - `sink.md`: the census at 0, and the two behaviour changes, each PROVISIONAL (Q2).
  - `counts.md`.

## Gate results (P2)
| # | Entry | Reading |
|---|---|---|
| 1 | no manifest moves | green, exit 0 |
| 2 | no `git+` source in the lock | green, exit 1, last line 0 |
| 3 | taffy 0.15.1 in the lock | green, exit 0, last line 1 |
| 4 | sink census over the taken `packages/` delta | green, exit 1, last line 0 |
| 5 | `ci-leg.sh fast` | green: 239 binaries · 1461 passed · 0 failed · 13 ignored; embolden 6 passed |
| 6 | `agent-run.sh boot` | green |
| 7 | `run stand` | green: run.end 294 passed · 0 failed · 8 ignored, outcome passed |
| 8 | `run all` | green: run.end 825 passed · 0 failed · 13 ignored, outcome passed |
| 9 | windowed boot smoke | green: up 10 s, `service.name=seven_guis` logged |
| 10–12 | `doc` · `a11y` · `msrv` | green, exit 0 each |
| 13 | `audit` | green: `advisories ok, licenses ok` |

Every count equals the trial's reading and the plan's baseline. No count moved (`evidence/counts.md`).

- **Smoke (P3):** the `role = 'smoke'` entry 9 ran as a P2 gate, so it was not re-run.
- **Matrix:** no capability is owned by the chunk. The coverage query returned 0 rows, and the phase run's audit read "claimed … 0". No `ref` was written, as the plan's last criterion states.
- **Deferrals:** none. The chunk has Rust delta, so every gate ran.
- **Guard entries:** the plan holds no `gate.py guard` entry.

## Deviations from plan, with justifications
1. **The take's stat reads 4 files, +15 −1, not +14 −1.** `git diff --cached --stat ba44c3559` reads +15 −1 on this tree, and on the trial worktree `target/tmp/trial-eae` too. The plan's Goal and research's §The trial take wrote +14, a miscount: `net.rs` +8, `document.rs` +2, quarantine +4, WPT_COMMIT +1 −1. Nothing in the tree differs from the trial. Recorded in `lock.md`.
2. **The block was fired before P1's evidence steps 3–6 were written.** This overlapped the block's runtime (about 15 min) with writing the records. No gate reads those files. `counts.md` (step 7) was written from the block's own logs after it ended.
3. **Scope record:** none. `gate.py scope` read `clean — changed 5 · listed 5 · recorded 0`, with no out-of-list edit.

## Surfaced
- **No spec↔reality gap** blocks the chunk.
- **Master rows that bind this work but that the plan does not cite** (the operator asked for these by name):
  - **security-plan `decision/wpt-quarantine-entries-valid`** (`dec-h5xumtn4`): a timeout-quarantine entry holds a whitespace-free path and a known reason, with no duplicate. `d2ee20b98` adds four entries, all `dynamic-load-events`, a reason in `KNOWN_REASONS` (`wpt/runner/src/test_runners/mod.rs:411`). No path repeats, and the runner's own validation test ran in the fast leg.
  - **test-plan `decision/wpt-pinned-commit`** (`dec-6mzkhoo4`): the conformance suites run at the commit `wpt/WPT_COMMIT` pins. `c30096724` moves that pin `07e827e4…` → `bbb30d0f…`.
  - **security-plan `decision/no-git-dependencies`** (`dec-kxrtstbq`): "The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile". This row is distinct from the cited security-plan `decision/no-git-dependency` (`dec-jm3wczan`). Its rule still holds. Its `verified` evidence quotes `Cargo.lock:8040-8041` with taffy 0.15.0's checksum `3c8480887e…`, which this take replaces with `31f7bd2d97…`. So the quote goes stale at the wrap; the evidence needs a re-verify, not the rule an amendment.
  - Read and judged context only, not binding: architecture `decision/wpt-quarantine-skipped-by-default` (`dec-rzjn35ju`). The new entries are skipped by default; the take changes nothing of the rule.
- **PROVISIONAL marks carried for the wrap:**
  - `founder-forks.md` Q1: the git Taffy stays out; registry 0.15.1 is taken.
  - Q2: `f5ea1caf1`'s `font-stretch` read and taffy 0.15.1's layout fixes, two behaviour changes for every document. Both are recorded in `evidence/sink.md`.
  - No fork arose in this run, so nothing new went to `founder-forks.md`.

## Process hygiene
`gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args agent-run.sh`, run bare at 2026-11-15T06:54:16Z, printed "census: none running". `agent-run.sh status` reads booted, last run `all` passed.

| Process | Started by | Final state |
|---|---|---|
| `cargo` / `rustc` (fast, doc, a11y, msrv, audit, agent-run builds) | this run (gate entries 5–8, 10–13) | terminated |
| `seven_guis_native` (smoke) | this run (gate entry 9) | terminated (`timeout 10`, exit 124 → entry exit 0) |
| `escher-session` (stand checks' hosts) | this run (inside entries 5, 7, 8) | terminated |

The trial worktrees `target/tmp/trial-eae` and `target/tmp/trial-eae-git`, with their build dirs, stand untouched. Per the plan's implementation notes, the operator removes them after the wrap.

## Next
The operator pass:
- entry 14, `gate.py hygiene` (before its land and the pre-CI commit);
- entry 15, the push (`git push origin build/escher-0.3.0`);
- entry 16, `ci.py conclusion --sha HEAD --wait 2100`.
Then the per-OS and stand-repeat tallies by test name go into `evidence/ci.md`, and only then `/andromeda-wrap-session`.

## Decisions & corrections
- Operator directive at invocation: the founder is away, so a fork goes to `founder-forks.md` with no dialog. No fork arose.
- Operator directive at invocation: in a controls file the case is the test name alone. This run fired no control, because the plan's controls were P5's (`p5-controls.md`).
- Guard refusal: the Bash guard refused a `cd` into the skill's references directory ("moves the session cwd"). It was re-issued as two Read calls, which ran.
- A standing learning recurred and was applied: host-linux.md 2026-10-27, `rerere.enabled=false` on the picks.
