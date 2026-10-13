# Moved readings — 2026-10-13-upstream-sync-polish-and-ship

Written under step 10 whether or not a stop occurs. Read 2026-10-13 on the merged, uncommitted tree.

**No check of ours read a new value, so no row of the class `our check reads differently` exists and there was no stop.** Every stand check passed in both builds, in both layout modes where it loops over them:
- `bash scripts/agent-run.sh run stand`: passed 139 · failed 0 · ignored 7. This is unchanged from the chunk start.
- the workspace leg: 0 failed.

The shared tables of `tests/blitz-tests/tests/common/mod.rs` were not touched. The guard over `examples/seven_guis` exits 0. No stand markup, id, CSS, viewport or font changed.

No upstream test went red under our build: the five new files and dioxus-native-dom's six new unit tests all passed. So no row of the class `upstream test meets our engine change` exists either.

| reading | old → new | read by | stated where | cause |
|---|---|---|---|---|
| workspace result lines | 174 → 179 | gate entry 26 | test-plan §9 | upstream tests added — five new `tests/blitz-tests/tests/*.rs` files (#1174, #1175, #1176, #1187, #1189) |
| workspace passed | 849 → 893 | gate entry 26 | test-plan §9 | upstream tests added — 38 in the five new files, 6 in dioxus-native-dom (`mutation_writer.rs`, `dioxus_document.rs`) |
| dioxus-native-dom unit tests | 61 → 67 | gate entries 24, 26 | test-plan §1 inventory | upstream tests added — `mutation_writer.rs` +2, `dioxus_document.rs` +4 |
| `run all` passed | 449 → 487 | gate entry 40 | test-plan §3 / §9 | upstream tests added — the five new files |
| CI-scripts tests | 78 → 79 | gate entry 31 | test-plan §9 | upstream tests added — `.github/scripts/test_wpt_area_changes.py` (#1183) |
| workspace test executables | 123 → 128 | gate entry 30 | test-plan §4 | upstream tests added — the five new files as targets of their own |
| blitz-tests top-level files | 113 → 118 | gate entry 27 (the by-name walk) | test-plan §2 | upstream tests added — the five new files |
