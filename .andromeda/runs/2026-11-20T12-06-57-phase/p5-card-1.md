# P5 review — 2026-11-20-browser-s-surviving-mutants: "Browser's surviving mutants"

**Plan:** `doc:2026-11-20-browser-s-surviving-mutants/plan` rev 3. Open it in an editor at `.andromeda/cache/docs/2026-11-20-browser-s-surviving-mutants/plan.md`. No plan stands in the chunk's folder: writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-browser-s-surviving-mutants` diffs the chunk's records.

## Goal
Catch the 0.3.0 Epoch 6 audit's four browser survivors (`capture.rs:86:5`, `render_scene`'s returned size replaced by `(0,0)` · `(0,1)` · `(1,0)` · `(1,1)`) with a direct unit test. The test is a `#[cfg(test)] mod tests` appended at the end of `apps/browser/src/capture.rs`, with upstream's code above it untouched.

## What the take-up found
- **The coordinate stands.** At HEAD `80c31730e`, `render_scene` is declared at 81, and 86:5 is its body's first statement. The one upstream commit since the audit (`4f61cffac`, anyrender 0.15) moved nothing at or above 86.
- **Both callers discard the returned size** (`capture.rs:48`, `:70`, code-graph). So only a test that calls `render_scene` itself can tell the four apart.
- **Your hypothesis holds, as an equality at reads.** The returned size is `Viewport::window_size`, whatever the scale. One input larger than 1 × 1 already fails all four. The two inputs (800 × 600 at scale 1, 1280 × 720 at scale 2) each fail all four, and the second also pins that the size is physical. Implement measures it.
- **Removing the unused return value does not end the survivors (corrected premise).** cargo-mutants replaces a unit-returning function's body with `()`, so one survivor, `replace render_scene with ()`, would take the place of the four. This is cargo-mutants' documented behaviour and was not measured, since a `--list` run is a cargo-mutants run and waits for the founder's word.
- **Lean:** catch (A) over remove (B) and keep (C). The entry needs the founder's word only to keep. C stays his choice if he prefers it.

## Top acceptance criteria
1. `cargo test -p browser --locked -- capture::tests` reads `test result: ok. 2 passed`, the sizes pinned as literals.
2. Four hand controls, each writing one constant into `render_scene`'s tuple in `gate.py control`'s clone, read `control: 4 holds`. This is the four mutants caught by hand.
3. `ci-leg.sh fast` returns 0.
4. Upstream `capture.rs` keeps every line; every hunk sits after line 122; only `capture.rs` changed (`guard: clean`).
5. The operator pass's push reads `verdict: green`.
6. On the founder's word only: entries 9 and 10 read the unit green and run cargo-mutants on `render_scene` alone, all four `caught`, exit 0. Without his word both are recorded not run.

## Touchpoints
- `apps/browser/src/capture.rs` — a test module appended. No `Cargo.toml` edit, no new dependency, no other crate.

## Test commands (10 entries)
- 1 · unit · `cargo test -p browser --locked -- capture::tests` — baseline red: 0 passed, the tests this chunk writes.
- 2 · unit · `bash .github/scripts/ci-leg.sh fast` — baseline green, 487.8 s.
- 3 · probe · no line of `capture.rs` removed — green, vacuous; controls pass and fail as expected (`p5-controls.md`).
- 4 · probe · every hunk after line 122 — green, vacuous; controls pass and fail as expected.
- 5 · probe · `gate.py guard` — only `capture.rs` changed — green, vacuous; controls pass and fail as expected.
- 6–8 · operator · hygiene, push, `ci.py conclusion --wait 2400`.
- 9–10 · operator, **fired only on the founder's word** (inputs#I1) · the browser unit read green under `NEXTEST_PROFILE=mutants`, then `cargo mutants -p browser --file apps/browser/src/capture.rs --re 'render_scene' --test-tool nextest --baseline=skip --cargo-arg=--locked --timeout 600 -j 6`, writing to a per-run dir under the chunk's evidence.
- The hand controls are implement's one-shot `gate.py control` on entry 1 (the plan's §Test Commands prose).

## Checks
- Mechanical checks all pass: the fence parses; planlint reports 0 hits; the snapshot citation (I1) resolves; research's lists parse; every new entry carries a baseline.
- The three «unchanged» claims hold after the chunk.
- The plan is 139 lines including the fence. That is small, because the chunk is one test module.
- Validation-1 is aligned after one intent-incomplete amendment: scope rev 4 says the catch needs no ruling and only a keep does.
- No escalate pattern fits: nothing widens a boundary.
- CI on `80c31730e` was `in progress` at Setup and was not read as green.

## Capabilities
- **No claim.** v040-08 (Codebase health) is not provable by this chunk. Its acceptance turns on «the next boundary audit reads …», and this chunk's lifecycle does not reach that audit. After your word, a dated `notes` line records the failed concretization. No concretization preview.

## Beyond the plan
- **v040-08's claim has no owner.** After this chunk, no markerless entry carries v040-08: lines 11 and 12 are complete and this is line 14. Its acceptance waits on the Epoch 2 boundary audit. Proposed owner: a `CARRY:` on «Dependency and CI debts», the epoch's last entry, so that chunk claims v040-08 against the boundary audit's reading, which may be gated on the operator's word at the epoch's last chunk. The wrap places it on your word.
- **No later audit will re-read the four if the founder declines.** If he declines entries 9–10, the four stand caught by the hand controls alone, and no later audit lists them (`--in-diff`, no changed line of `render_scene`). Proposed owner: none, recorded in the report as the founder's decline. Or, on his word, the operator pass fires 9–10.

Apply? (yes / review / cancel)
