# Implement outcome — 2026-10-17-upstream-sync-engine-debts

**Outcome: green + smoke.** The gate block ran green on its first firing: `entries 47 · green 42 · red 0 · recorded 1 ·
timeout 0 · not-run 4` (the four `leg = 'operator'` entries 44–47). The trail is
`.andromeda/runs/2026-10-17T14-34-19-implement/gate-2026-10-17-upstream-sync-engine-debts.json`. Smoke: the plan's
`role = 'smoke'` entry 43 (the windowed `seven_guis_native`, a 10 s bound) ran green as a P2 gate and was not re-run.
Scope: `scope: clean — changed 35 · listed 35 · recorded 0 · absorbed 0 · excluded 100`
(`.andromeda/runs/2026-10-17T14-34-19-implement/scope-2026-10-17-upstream-sync-engine-debts.json`). Matrix: the chunk
claims 0 capabilities, so nothing was written.

The merge is staged, not committed: `MERGE_HEAD` = `23b9b26a9b6af5947b8f003d2c2cc768bff10b8a` on `build/escher-0.2.0`
at HEAD `143b6013`. Both conflicts are resolved and marked resolved in the index (`git add` of `Cargo.lock` and
`packages/blitz-dom/src/accessibility.rs`). The two-parent merge commit is the operator pass's pre-CI commit.

## Files changed
- **By the merge (upstream's):** `Cargo.toml`, `justfile`, `apps/bump/{Cargo.toml,src/main.rs}` (deleted),
  `packages/blitz-dom/src/{layout/inline.rs,node/text.rs,util.rs}`, `packages/blitz-shell/src/convert_events.rs`,
  `packages/blitz-vibey-script/{src/dom/element.rs,src/runtime.rs,tests/preact.rs}`,
  `packages/dioxus-native-dom/src/{events.rs,keyboard_compat.rs (new),lib.rs}`,
  `tests/blitz-tests/tests/{accessibility_description.rs (new),accessibility_label.rs (new),all.rs}`.
- **Conflicts resolved (steps 3–4):**
  - `packages/blitz-dom/src/accessibility.rs`: both sides kept, upstream's duplicate `aria-selected` and `aria-label`
    reads dropped, our label loop before upstream's `aria-labelledby` loop (`evidence/merge.md`).
  - `Cargo.lock`: one region, in blitz-tests' dependency list — our two `escher-*` dependents kept and upstream's
    `keyboard-types 0.8.3` taken. Then `cargo update -p escher-driver -p escher-telemetry` moved those two entries to
    `0.3.0-beta.4` and nothing else (`evidence/lock.md`).
- **Ours (steps 5–7):**
  - `packages/blitz-test-harness/src/lib.rs`: `NamedKey` re-exported.
  - `packages/blitz-dom/src/events/keyboard.rs`: the Enter arm and the range arms.
  - `packages/escher-driver/src/execute.rs`: `keyboard_key`, its test table, and the macOS `ACTION_MODIFIER` →
    `Modifiers::META`.
  - `packages/dioxus-native-dom/src/snapshot.rs`: one unit test.
  - The ten blitz-tests checks.
  - `.github/scripts/ci-leg.sh`: both `cargo llvm-cov` lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`,
    with a comment.
  - `.github/scripts/test_ci_workflows.py`: the pin asserts the flag.
- **Evidence (new):** `evidence/{before,merge,lock,counts,readings,sink}.md`. `evidence/ci.md` is the operator pass's
  (step 10) and is not written yet.

## Deviations from the plan, with justifications
1. **Six key sites spell the crate `::keyboard_types`, not `keyboard_types`:** `snapshot.rs`'s test module and
   `stand_keyboard`, `stand_snapshot_state`, `stand_diff`, `stand_settle`, `stand_snapshot_text`. Each of these
   modules glob-imports `dioxus::prelude::*`, which re-exports Dioxus's own `keyboard_types` (0.7). Before the merge
   both names resolved to one crate. Now they are two versions, so the bare path reads E0659 (ambiguous) in the
   checks, and E0599/E0308 in the test module, where the glob wins. The leading `::` names the crate's own 0.8
   dependency. This is still the smallest spelling change step 6 asks for, in listed files only. No table, message or
   assertion moved. Research's convention line did not foresee it.
2. **The workspace tally reads 187 result lines, not the predicted 188.** Upstream's deletion of `apps/bump` takes its
   empty `unittests src/main.rs` result line with it: +2 −1 lines, +13 passed (`evidence/counts.md`, attributed per
   target against the pre-merge `fast` log).
3. **`accessibility_label.rs` holds 10 tests**, where research's file read counted 9. It ran 10 passed.
4. **No scope-record line:** every edit falls inside research's two lists (the scope read is clean).

## Surfaced
None. No spec↔reality gap and no premise disproof. The expected amendments the plan lists under Implementation
notes are the wrap's.

## Next
The **operator pass**, then `/andromeda-wrap-session`:
- entry 44: the hygiene check, before the pre-CI commit;
- the pre-CI commit, which records the merge with two parents — `git commit` with `MERGE_HEAD` staged;
- entry 45: `fast`, then the clean-tree guard, then the push;
- entry 46: `ci.py conclusion --sha HEAD --wait 3600`;
- entry 47: the per-OS tallies, with the run id.

Each result goes into `evidence/ci.md`. The macOS `ACTION_MODIFIER` arm compiles only on macOS: entry 18's read
(`Modifiers::SUPER` count 0) and the macOS CI job are its witnesses.

## Gate deferrals
None. Every entry ran.

## Process hygiene
| Process | Started by | Final state |
|---|---|---|
| `cargo` builds and tests (entries 8, 21, 22, 27, 29–31, 33, 34, 36–38, 40, 42) | a leg (the gate run) | terminated — no `cargo` in `ps -eo pid,comm,args` at 2026-10-17T14:54:51Z |
| `seven_guis_native` (entry 43, `timeout 10`) | a leg (the gate run) | terminated — the bound's 124, none in the census at 14:54:51Z |
| `escher-session` hosts and `mcp` children (the session, cli and mcp checks) | a leg (the gate run) | terminated — none in the census at 14:54:51Z |
| `cargo check --workspace --all-targets` ×4 and `cargo update -p …` (P1, by hand) | this run | terminated — foreground calls that returned |

## Decisions & corrections
- Guard refusals: bash-guard refused a `cd` into the cargo registry (the test-that review) and a `cd` into
  `$TMPDIR/andromeda-gate/…` (the tally search). Each was re-issued once with absolute paths and ran.
- git printed `Recorded preimage` for both conflicts: `rerere` is on on this host. No recorded resolution was replayed.
