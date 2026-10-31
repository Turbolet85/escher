# Readings — 2026-10-31-engine-debts

Implement's one-shot runs. The listed gate entries guard each property on re-run.

## F1 and F2, red before green

`cargo test -p blitz-tests --locked --no-fail-fast --test visible_region_transformed --test hit_non_invertible_transform`,
both checks written, the engine unedited (exit 101):

| Target | Test | Reading before the fix | First failing row |
|---|---|---|---|
| `hit_non_invertible_transform` | `a_box_under_a_non_invertible_transform_takes_no_hit` | FAILED | `incremental=false: row 0: point 0: the hit answers the clip and not the box` |
| `hit_non_invertible_transform` | `an_invertible_twin_is_hit_where_it_shows` | ok | |
| `visible_region_transformed` | `a_target_in_a_node_body_under_a_transformed_world_reads_where_it_shows` | FAILED | `incremental=false: row 0: the node's centre lies inside its region` |
| `visible_region_transformed` | `a_translate_moves_the_region_by_the_translation` | FAILED | `incremental=false: the region moves as the box does, its size the same` |
| `visible_region_transformed` | `a_box_mapped_to_a_point_leaves_nothing_visible_inside_it` | FAILED | `incremental=false: row 0: nothing inside it can be seen` |
| `visible_region_transformed` | `under_a_rotation_the_region_reads_the_rotated_boxs_bounding_box` | FAILED | `incremental=false: the region is the rotated box's bounding box` |
| `visible_region_transformed` | `a_world_under_an_untransformed_viewport_shows_a_node_and_pans_one_out` | ok | |
| `visible_region_transformed` | `a_page_with_no_transform_reads_its_padding_box_in_the_client_rects_frame` | ok | |

The two `ok` rows of `visible_region_transformed` are research.md's: a plain pan/zoom world under an untransformed
viewport already reads right, and a page with no transform must not move.

## The redraw route's first hop, red before green

Entry 10's baseline reads `0 passed` on the untouched tree, blitz-shell holding no unit test. It now reads
`test result: ok. 1 passed; 0 failed`. The test states what it cannot read: `BlitzShellProvider::request_redraw` and the
application's handling of `RequestRedraw` need a real window and stay with the windowed boot smoke, entry 33.

## The smoke

- **Windowed:** entry 33, `RUST_LOG=info timeout 10 target/debug/seven_guis_native`, green on the final tree: it booted,
  ran until the timeout and logged `service.name=seven_guis`. It reads the boot alone; no input was driven.
- **`scripts/agent-run.sh`:** `boot`, `run stand` and `run all` green, entries 26, 27 and 29; `status` reads `"state":
  "passed"`, 681 passed.
- **The driver's command line, as a process per command**, Cells by keyboard, the host built alone. Every status 0:
  `start cells` · `press tab` · `press tab` · `press arrow-right` · `press arrow-down` · `press enter`, the editor added ·
  `type cell-input`, settled · `press enter`, the editor removed, `cells-grid` and `cell-B1` changed · `snapshot`, part 1 of
  5, the value read on `cell-B1` and the grid reading focused · `lint` · `stop`, the state directory gone.

## A hazard measured beside step 15

A task spawned from a key handler that also writes a signal is first polled inside `render_immediate`, which holds the
document's `RefCell` (`packages/dioxus-native-dom/src/dioxus_document.rs`, `poll`). A `MountedData` call in that task
panics with `RefCell already borrowed` at `events.rs:162`. Cells calls the handles synchronously, the native handle acting
at the call. No engine line moved for this. Measured headlessly, through the harness's `poll(None)`. The windowed shell
runs the same `poll` lines, so the same panic is expected there, but that was read from the code and not run.
