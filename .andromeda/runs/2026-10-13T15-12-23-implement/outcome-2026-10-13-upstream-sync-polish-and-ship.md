# /andromeda-implement outcome — 2026-10-13-upstream-sync-polish-and-ship

**Outcome: green + smoke.** The merge is staged and uncommitted (`MERGE_HEAD` = `434e06615d5f47a3df92c0f94e916df088fdcf4d`, HEAD `a703136`).

Gate block (`{tools_dir}/gate.py run`, one firing, trail in this run dir):
- `entries 50 · green 43 · red 0 · recorded 3 · timeout 0 · not-run 4`.
- The four not run are the `leg = 'operator'` entries 47–50.
- One iteration, with no fix and no soft-exit.

**Next → the OPERATOR PASS, then `/andromeda-wrap-session`.** The pass fires these in order:
1. entry 47 — `gate.py hygiene`, before the pre-CI commit;
2. the pre-CI commit that records the merge with both parents;
3. entry 48 — `ci-leg.sh fast`, the clean-tree guard, then the push;
4. entry 49 — `ci.py conclusion --sha HEAD --wait 3600`;
5. entry 50 — the job-log read, with `<id>` set to the run id entry 49 prints.

The pass writes `evidence/ci.md`; this run did not write it.

## Files changed
- **By the merge (45 files, upstream's delta):** each is listed in research.md §Files to modify. Every hunk of ours in them is kept as merged (see `evidence/merge.md`, the survival table).
- **By hand (5 files):**
  - `packages/dioxus-native-dom/src/lib.rs` (step 4): the conflict, with both sides' re-exports kept.
  - `packages/dioxus-native-dom/src/mutation_writer.rs` (step 5): the conflict, resolved to upstream's `is_bool_attr`. Our `BOOLEAN_ATTRIBUTES` constant is deleted, and the file is now byte-identical to upstream's at the pin.
  - `packages/dioxus-native-dom/src/element_id.rs` (step 6): `a_removed_element_reads_no_id` now asserts the dropped-node lifetime.
  - `Cargo.lock` (step 7): `cargo update --workspace` moved `escher-driver` and `escher-telemetry` to `0.3.0-beta.3`, those two lines only.
  - `README.md` (step 8): line 48 now names the eleven driver commands and the built screenshot.
- **Evidence:** `escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/evidence/{merge,counts,lock,features,moved-readings}.md`.
- **Scope read:** `scope: clean — changed 47 · listed 47 · recorded 0 … · excluded 107`, so no `scope-record.md` was needed.

## Deviations from plan, with justifications
- **The two resolved conflict files were staged** (`git add`, no commit). This clears the index's unmerged state so the operator's commit can record the merge. Nothing else was staged.
- **The step 6 test is an edit to a test of ours whose premise the merged engine changes.** Upstream #1185 drops a removed node at removal. The detached-parent assertion and the hand `remove_node_if_unparented` call were removed. The test name and its id assertions stand. It is recorded as non-additive in `evidence/merge.md`.
- **No other deviation.**
  - Steps 1–9 and 11 ran as written.
  - Step 10 never fired, because no gate went red. `evidence/moved-readings.md` holds only rows of the `upstream tests added` class.
  - No upstream test was edited.
  - No pin or third-party lock entry moved: the lock-set diff against upstream's lock at the pin is empty.

## Measured against the plan's predictions (all held)
- **Workspace leg:** 174 → 179 result lines · 849 → 893 passed · 0 failed · 12 ignored.
  - Per target, exactly five new targets (+38) and dioxus-native-dom's unit tests 61 → 67 (+6, upstream's). Every other target is unchanged.
- **CI scripts:** 78 → 79.
- **a11y leg:** 3 lines · 15 passed.
- **`run stand`:** 139 / 0 / 7, unchanged. No stand number moved under Taffy 0.15, #1175 or #1186.
- **`run all`:** 449 → 487 (+38).
- **Executables:** 123 → 128 · `all-target 0` in the build set, the workspace log and the a11y log.
- **By-name walk:** 0 absent over 117 of 118 files. Each of the five new files ran once.
- **Audit:** `advisories ok`. **Doc:** green.
- **blitz-dom features:** 16 in the workspace build (tracing and writing-mode on) and 10 per-package, both unchanged. The adapter-absence read is 0.
- **Windowed smoke:** `seven_guis_native` stayed up to the 10 s bound and logged `service.name=seven_guis`.

## Surfaced
None as a stop. For the wrap's reconcile, already among the plan's expected amendments:
- architecture §Established Decisions → [DOM semantics] no longer reads true. Its line "node_id_mapping entries are intentionally not cleared on removal" is contradicted: a Dioxus-removed node is now dropped and its mappings are cleared (upstream #1185).

## Recorded for later entries (step 12; nothing done in code)
- **Upstream APIs escher does not use:** `synthetic_form_event`, `UserAgentStylesheet`, `UiEvent::PointerLeave`.
- **#1186 changes which attribute selectors match Dioxus elements.** Attributes are now in the empty namespace, so the UA sheet's and the stand's own unprefixed attribute selectors match them for the first time.
  - The four lean stand tasks read no moved number.
  - Circle Drawer's `.cd-dialog input[type="range"]` (`examples/seven_guis/src/tasks/circle_drawer.rs:215`) is predicted to apply for the first time. It is unmeasured here and is for "Stand requirement sweep".
- **A falsy `inert` is now cleared**, as the 28th name of upstream's `is_bool_attr`.

## Not run locally (as the plan states)
- `msrv` (no 1.91 toolchain here), the wasm leg, and the windows, macos, ios and android legs. These are the CI run's reading, through operator entry 49.
- The mutation tier is not run (the founder's word, inputs#I2).

## Process hygiene
Measured by `ps -eo pid,comm,args` after the block and the smoke: no `seven_guis`, `escher-session`, `cargo` or `agent-run` process present.

| process | started by | final state |
|---|---|---|
| `cargo test --workspace` (step 1, pre-merge leg) | this run | terminated (exit 0) |
| `cargo update --workspace` | this run | terminated (exit 0) |
| cargo builds and test binaries of gate entries 6, 24, 25, 30, 32–34, 37–45 | this run | terminated |
| the session hosts and MCP children started by the session and seven_guis checks | a gate entry's test binary | terminated (none in the census) |
| `target/debug/seven_guis_native` (entry 46, under `timeout 10`) | this run | terminated by `timeout` (124, read as the entry's 0) |

## Decisions & corrections
- **Guard refusal:** bash-guard refused a `cd` into the skill's references directory at Setup ("moves the session cwd for every later call"). It was re-issued with absolute paths and ran.
- No operator correction or ruling was given this run. No standing learning recurred: the grep-function hazard was avoided by `command grep` in every hand probe.
