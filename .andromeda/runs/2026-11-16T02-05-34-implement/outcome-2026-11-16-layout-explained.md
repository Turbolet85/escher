# Implement outcome — 2026-11-16-layout-explained

**Outcome:** green. v030-11 is built and not claimed (inputs#I5). No ledger row is owned by this chunk
(`rec.py query "select * from coverage where chunk = '2026-11-16-layout-explained'"`: 0 rows), so no `ref` was written.

**Next:** the OPERATOR PASS, entries 9 (`gate.py hygiene`), 10 (the push of `build/escher-0.3.0`) and 11
(`ci.py conclusion --sha HEAD --wait 2400`). After that, `/andromeda-wrap-session`.

## Files changed (10; `gate.py scope`: clean — changed 10 · listed 10 · recorded 0)
- New:
  - `packages/blitz-dom/src/layout_reason.rs`
  - `packages/escher-driver/src/layout.rs`
  - `packages/escher-driver/src/layout_tests.rs`
  - `tests/blitz-tests/tests/layout_reason.rs`
  - `tests/blitz-tests/tests/stand_layout.rs`
- Modified:
  - `packages/blitz-dom/src/lib.rs`
  - `packages/dioxus-native-dom/src/lib.rs`
  - `packages/escher-driver/src/execute.rs`
  - `packages/escher-driver/src/lib.rs`
  - `docs/capabilities.md`

## Gates (the final firings; trail `.andromeda/runs/2026-11-16T02-05-34-implement/gate-2026-11-16-layout-explained.json`)
- `cargo test -p escher-driver --locked --lib layout::`: green. 4 passed, 116 filtered out.
- `cargo test -p blitz-tests --locked --test layout_reason`: green. 8 passed.
- `cargo test -p blitz-tests --locked --test stand_layout`: green. 7 passed after the re-run; 6 at the first block.
- `cargo test -p blitz-tests --locked --test text_extent`: green.
- `cargo test -p blitz-tests --locked --test stand_actions`: green.
- `bash .github/scripts/ci-leg.sh fast`: green (317.93 s, re-run after the last test edit).
- `bash .github/scripts/ci-leg.sh stand-repeat`: green, `stand-repeat: 3 of 3 rounds green` (777.55 s, re-run after
  the last test edit).
- `bash .github/scripts/ci-leg.sh coverage`: green. escher-driver 5793/5912 = 97.99 % (floor 97 %), dioxus-native-dom
  4164/4633 = 89.88 % (floor 83 %), blitz-test-harness 1006/1047 = 96.08 % (floor 90 %). The last line reads
  `coverage floor: 3 of 3 crates at or above their floors`.
- The hygiene, push and CI-read entries: `not run — leg operator`. They belong to the operator pass.
- No `defer` and no deferral.
- Smoke: skipped. There is no boot-path or UI-surface change and no smoke entry.

## Controls (evidence `.andromeda/cache/chunks/2026-11-16-layout-explained/evidence/control-{1,2}.txt`)
- `rule4` (entry 2, case `every_size_cause_reads_on_its_fixture_node`): holds.
- `final-layout` (entry 3, case `every_snapshot_node_answers_and_sits_at_its_container_plus_its_offset`): holds.
- `pump`, first firing (entry 3, case `the_read_changes_nothing`): DOES NOT HOLD. A pump on a settled screen changes
  nothing that case reads.
- The stand check then gained `the_read_runs_no_pass_that_would_apply_pending_ticks`.
- `pump`, re-fired on the new case (control-2): holds.

## Deviations, with justifications
1. **An option of a closed select reads `Select` even though it has no box of its own.** The default sheet makes
   every unchosen option `display: none`. A closed select's `optgroup` is `display: contents`, so it reads `None`,
   as the implementation note says. Step 1's `Select` arm is applied to `option` elements alone.
2. **The plan names no placing word for an element whose reader answers `None`.** `Session::layout` maps it to
   `Placer::Engine(PlacedBy::Block)`, with the snapshot's bounds, no container and both causes `Layout`. Block was
   chosen as step 1's default word. This is surfaced for review.
3. **The stand's offset equality is inclusive at 1/64 px (`<= 1/64`).** The first run measured `counter-value` off by
   exactly 1/64: x 314.25 against 281.25 + 33.015625. The offset and both rects are each snapped to 1/64 px.
   The plan states the equality "to 1/64 px".
4. **The no-change case could not see a pump.** It gained a Timer case: ticks are delivered and stay unapplied
   through the read, and a later pump applies them. This makes the plan's third control hold.
5. **Two functions were split under the dev-host cognitive-complexity ceiling of 15**
   (`rust-code-analysis-cli 0.0.25`):
   - The engine's `shifts` read 17. The `scroll_shift` helper was extracted, and it now reads 13.
   - The stand check's offset test read 17. The `check_node` helper was extracted, and the test now reads 8.
   - The first gate block was stopped mid-run for this and re-fired whole.
6. **dioxus-native-dom re-exports `BoundingRect` beside the plan's list.** `LayoutReason`'s rect fields hold it, and
   the driver names it. The file is listed, so this is no scope-record line.
7. **Value forms the plan left open:**
   - `Length::Percent` holds 0–100.
   - A `flex-basis: content` reads `Length::Auto`.
   - An anchor size reads `Auto` (`None` for a max).
   - A height percentage is definite only when its container's computed height is not `auto`.
8. **The pinned CRUD row is `crud-person-0`.** The fixed box's container measured as `html`.

scope record: none. `gate.py scope` is clean, with 0 recorded.

## Surfaced
Nothing as a spec↔reality gap. Deviation 2 is a plan underdetermination, resolved as stated.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries of the gate entries and controls | this run (gate tool, control tool) | terminated. `gate.py census --comm cargo --comm escher-session --comm rustc --args stand_layout --args layout_reason --args andromeda-control`: `census: none running` |
| clone `~/.cache/andromeda-control/275cfae1` (named "its process 244999 lives, left" at both control heads) | NOT DERIVED: not this run's clone | left. Its owning process is the one to remove it |

## Decisions & corrections
- A guard refused one call. bash-guard refused `cd /home/…/andromeda-implement/references && grep …` ("moves the
  session cwd"). It was re-issued once with an absolute path and ran.
- The first background gate block was stopped by TaskStop to re-fire it after the complexity split. A killed call
  leaves no trail record; its logs stand as `{n}.log`.
- The control tool's `holds` requires every named case to read failed. Naming the old no-change case beside the new
  one would read INDETERMINATE, so control-2 names the new case alone.
