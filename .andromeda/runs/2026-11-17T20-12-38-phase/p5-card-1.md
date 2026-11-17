# P5 review — 2026-11-17-stand-requirement-sweep «Stand requirement sweep»

**The plan, to open in an editor:** `.andromeda/cache/docs/2026-11-17-stand-requirement-sweep/plan.md`, rendered from
`doc:2026-11-17-stand-requirement-sweep/plan` rev 4. No plan stands in a chunk's folder, and writing in that file changes
no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-17-stand-requirement-sweep` shows a diff of the chunk's records.

## Goal
Every 0.3.0 requirement (v030-01…17, bar v030-15) is recorded beside the check that witnesses it on the stand or a
fixture. Each integration witness is shown to run inside `stand-repeat`, and the non-screen rows say why none runs there.
The record is a third table in `stand_requirement_sweep.rs` (your «Same file»). Its membership check reads the
`stand-repeat)` arm of `ci-leg.sh` alone, and `STAND_REPEAT_NAMED` widens 10 → 17. Two unasserted readings get checks:
the 0.3.0 answers' masks, and the five surviving browser mutants. Floors, agent-run, stderr and stand layout are
re-read and recorded.

## Top acceptance criteria
1. `stand_requirement_sweep`'s new `every_0_3_0_requirement_names_its_witness_inside_stand_repeat` holds a 16-row
   table. It names no witness exactly for v030-02, v030-03 and v030-16, and every other row names a witness the
   `stand-repeat)` arm runs.
2. Membership is read from the `stand-repeat)` arm alone. Its controls `leg-arm` (drop `inert_subtree` from the arm
   alone), `named-17` and `row-030` read `holds` (CARRYs `cry-omhvfewt`, `cry-ivqdcli5`).
3. `stand_masks_on_new_answers`, in both layout modes:
   - `graph`, `actions` and `layout` answers, a kept check and every explored path's check hold the typed sentinel 0
     times;
   - the kept `type` reads `MASKED_STAND_IN`;
   - a file input reads `MASKED_VALUE` over `serve_mcp`, the first file-input reading on MCP;
   - the `mask` control on `snapshot.rs:302` reads `holds`.
4. The browser's five `make_doc_config` field tests each read red on their field's deletion. Each is proven by a one-shot
   `gate.py control`, and **no cargo-mutants run** is made (CARRY `cry-iu6m4bqf`, your word).
5. `stand-repeat` 3 of 3, `coverage` 3 of 3 at or above floor, agent-run `boot`, `run stand`, `run session` and
   `run all` green, `fast` green, `guard: clean`, CI `verdict: green` on the pushed HEAD.

## Your P4 word, as the plan holds it
- **Browser test placement:** the killing tests stay in the **inline** `#[cfg(test)] mod tests` of
  `apps/browser/src/document_loader.rs`. `make_doc_config` is not reachable from an integration test: the crate builds
  only `[[bin]] blitz` (`apps/browser/Cargo.toml:8-9`) and has no `lib.rs`. This is the "say so on the P5 card" arm of
  your note.
- **`caused` on stderr:** stated as read in process. No served task logs or panics (`examples/seven_guis/src/tasks/`: 0
  sites), so no new served fixture is added.

## Touchpoints
- new `tests/blitz-tests/tests/stand_masks_on_new_answers.rs`
- modify `tests/blitz-tests/tests/stand_requirement_sweep.rs` (1,109 lines → predicted about 1,450, measured at
  implement)
- modify `apps/browser/src/document_loader.rs` (test module only)
- No `ci-leg.sh`, `test_ci_workflows.py` or `agent-run.sh` change: every 0.3.0 witness already runs inside
  `stand-repeat` (scope rev 2).

## Test commands (14 entries; 3 operator)
- **Fast:** `cargo test -p browser --locked --bin blitz document_loader::tests` · `cargo test -p blitz-tests --locked
  --test stand_requirement_sweep` · `… --test stand_masks_on_new_answers` · `gate.py guard --base 66fbe33be`.
- **Legs:** `ci-leg.sh fast` · `stand-repeat` · `coverage`.
- **Agent-run:** `boot`, `run stand`, `run session`, `run all`.
- **Operator:** hygiene → push (clean-tree form) → `ci.py conclusion --sha HEAD --wait 2700`, carried because the mask
  check adds `cfg(unix)` items.
- **Baselines on the untouched tree:**
  - every entry green, but entry 3, red as predicted (no target yet);
  - `run stand` 327 · 0 · 10 and `run all` 874 · 0 · 15 (the last Proof readings were 262 and 743);
  - `stand-repeat` 3 of 3;
  - `coverage` 97.73 % · 90.53 % · 95.99 % (floors 97 · 83 · 90).

## Capabilities
None claimed. The pool is `[v030-15]` alone, owned by «Cold-agent bar», so there is no concretization preview. Each
witnessed cap gets a dated ledger note at the wrap; status, ref and acceptance stay as they read.

## Validation
- Mechanical checks pass: the fence parses, the lists parse, planlint reads 0 hits, there are no inputs, step order
  holds, and no process is killed by name.
- **WARN, check 7:** no code-graph trace was recorded. The change surface is file-private test symbols and a new test
  file (research §Graph impact).
- Validation-1 reads **aligned**: P3's closure corrected four premises, and the plan builds on the corrected scope.
- The rows files were read by search, about 303 KB past the 250,000 B whole-read bound. Each cited row was found in its
  rows file or its standing list.

## Beyond the plan
none
