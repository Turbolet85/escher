# Founder forks — 2026-11-20-codebase-health (phase P4)

The founder is away (the operator's directive, `inputs#I1`): an upstream-authored file is split only on his word.
Each fork below is his; the least-change option stands in the plan meanwhile, and nothing written presumes another
answer. Every reading is the 0.3.0 Epoch 6 code audit's own recipe (tokei code lines over 800; rust-code-analysis
cognitive complexity over 15, the ceiling of `decision/cognitive-complexity-ceiling`), re-run at HEAD `29b3a5fb7` over
every tracked `.rs` file (551) by `chunks/2026-11-20-codebase-health/evidence/measure2.py`, its output
`measure-head.json` beside it. A file is upstream-authored when it stands in `upstream/main` (`547d7514b`, the
2026-11-19 sync's merge base); the class is the file's, so a function escher wrote inside an upstream file reads as
upstream's here.

## F1 — the fifteen upstream-authored files over 800 code lines

**What was measured.** 15 of the 25 files over 800 are upstream's (the other 10 are escher's: three of them this
chunk brings under, the seven integration-test targets are F4). The audit read `document.rs` at 3,367
before the sync; the sync brought it to 3,470.

| file | code lines |
|---|---|
| `packages/blitz-dom/src/document.rs` | 3470 |
| `packages/blitz-dom/src/mutator.rs` | 2767 |
| `packages/blitz-vibey-cdp-server/tests/elements_panel.rs` | 1969 |
| `packages/blitz-dom/src/node/node.rs` | 1801 |
| `packages/blitz-paint/src/render.rs` | 1333 |
| `packages/blitz-dom/src/layout/inline/layout.rs` | 1110 |
| `packages/blitz-dom/src/node/element.rs` | 955 |
| `packages/blitz-vibey-script/tests/dom.rs` | 920 |
| `packages/stylo_taffy/src/convert.rs` | 916 |
| `packages/blitz-dom/src/events/pointer.rs` | 906 |
| `packages/blitz-dom/src/layout/text_transform.rs` | 848 |
| `packages/blitz-paint/src/render/background.rs` | 845 |
| `packages/blitz-dom/src/layout/construct.rs` | 841 |
| `packages/blitz-dom/src/cssom.rs` | 829 |
| `packages/stylo_taffy/src/wrapper.rs` | 816 |

**The fork.** Either:

- **A — keep them as upstream's (least change; stands meanwhile).** The fork splits no upstream-authored file. The
  next boundary audit reads them by name as kept, and v040-08's "kept by the founder's word" covers them.
- **B — split them in the fork.** A later chunk splits `document.rs` first along the audit's M2 line (focus, the
  changed set, select-list state, the geometry readers into child modules of `document`), then the others. This is a
  standing divergence in the files every upstream sync touches most, re-conflicting at every take.
- **C — offer the split upstream.** A later chunk proposes the `document.rs` split to upstream as a pull request, and
  the fork takes it with the next sync if it lands. Nothing diverges, but it waits on upstream's review.

## F2 — the ninety-four upstream-authored functions over the ceiling

**What was measured.** 101 functions read over cognitive 15. Of them 94 are in upstream-authored files (the audit
read 88 workspace-wide before the sync) and 7 are escher's, which this chunk brings under. The masters already rule
for A: `decision/complexity-ceiling-host-only` reads «engine functions over it are left as they are», its one
exception the engine's click handler, held below the ceiling at the price of a hand re-apply on each sync.

| function | at | cognitive |
|---|---|---|
| `compute_inline_layout_inner` | `packages/blitz-dom/src/layout/inline/layout.rs:338` | 179 |
| `compute_presentational_hints` | `packages/blitz-dom/src/stylo/presentational_hints.rs:127` | 173 |
| `resolved_style_value` | `packages/blitz-dom/src/resolved_style.rs:344` | 117 |
| `hit_inner` | `packages/blitz-dom/src/node/node.rs:1486` | 88 |
| `handle_pointermove` | `packages/blitz-dom/src/events/pointer.rs:208` | 77 |
| `write_outer_html_in_style` | `packages/blitz-dom/src/node/serialize.rs:171` | 74 |
| `dispatch_child_layout` | `packages/blitz-dom/src/layout/mod.rs:225` | 68 |
| `apply_keypress_event` | `packages/blitz-dom/src/node/text.rs:235` | 63 |
| `compute_replaced_layout` | `packages/blitz-dom/src/layout/replaced.rs:52` | 61 |
| `build_inline_layout_into` | `packages/blitz-dom/src/layout/inline/build.rs:24` | 58 |
| `handle_key_or_input_event` | `packages/blitz-dom/src/events/keyboard.rs:23` | 56 |
| `resolve_color_stops` | `packages/blitz-paint/src/gradient.rs:325` | 56 |
| `dispatch_event_inner` | `packages/blitz-vibey-script/src/runtime/events.rs:181` | 56 |
| `collect_table_cells` | `packages/blitz-dom/src/layout/table.rs:515` | 55 |
| `Toolbar` | `apps/browser/src/toolbar.rs:36` | 53 |
| `handle_winit_event` | `packages/blitz-shell/src/window.rs:624` | 52 |
| `build_inline_layout_recursive` | `packages/blitz-dom/src/layout/inline/build.rs:121` | 51 |
| `collect_inline_roots_in_range` | `packages/blitz-dom/src/traversal.rs:387` | 51 |
| `construct_entry_list` | `packages/blitz-dom/src/form.rs:187` | 49 |
| `build_paint_tree_impl` | `packages/blitz-dom/src/layout/paint_tree.rs:222` | 48 |
| `build_accessibility_node` | `packages/blitz-dom/src/accessibility.rs:136` | 47 |
| `build_table_context` | `packages/blitz-dom/src/layout/table.rs:203` | 47 |
| `emit_mutations` | `packages/blitz-vibey-cdp-server/src/session/mutations.rs:20` | 47 |
| `dispatch` | `packages/blitz-vibey-cdp-server/src/session/dom.rs:14` | 46 |
| `set_attribute` | `packages/blitz-dom/src/mutator.rs:330` | 44 |
| `collect_layout_children_with_wrap` | `packages/blitz-dom/src/layout/construct.rs:404` | 43 |
| `compute_layout_damage` | `packages/blitz-dom/src/layout/damage.rs:363` | 40 |
| `resolve_stylist` | `packages/blitz-dom/src/stylo/traversal.rs:40` | 39 |
| `stroke_text_with` | `packages/blitz-paint/src/text.rs:160` | 37 |
| `build_accessibility_tree` | `packages/blitz-dom/src/accessibility.rs:12` | 36 |
| `handle_pointerdown` | `packages/blitz-dom/src/events/pointer.rs:407` | 36 |
| `<anonymous>` | `packages/blitz-dom/src/layout/inline/layout.rs:512` | 35 |
| `draw_border` | `packages/blitz-paint/src/render/border.rs:150` | 34 |
| `main` | `wpt/runner/src/main.rs:333` | 34 |
| `<anonymous>` | `packages/blitz-dom/src/layout/inline/layout.rs:614` | 33 |
| `render_element` | `packages/blitz-paint/src/render.rs:445` | 33 |
| `get_text_selection_ranges` | `packages/blitz-dom/src/document.rs:3551` | 32 |
| `clear_attribute` | `packages/blitz-dom/src/mutator.rs:492` | 31 |
| `flush_line_decorations` | `packages/blitz-paint/src/text_decorations.rs:433` | 31 |
| `dispatch` | `packages/blitz-vibey-cdp-server/src/session/css.rs:14` | 31 |
| `add_children_to_parent` | `packages/blitz-dom/src/mutator.rs:945` | 30 |
| `visit` | `packages/blitz-vibey-script/src/dom/selection.rs:274` | 30 |
| `resolve_layout_children` | `packages/blitz-dom/src/resolve.rs:259` | 27 |
| `resolve_layout_children_recursive` | `packages/blitz-dom/src/resolve.rs:262` | 27 |
| `radial_gradient` | `packages/blitz-paint/src/gradient.rs:161` | 27 |
| `load_tests` | `wpt/runner/src/manifest.rs:89` | 27 |
| `classify_flow_children` | `packages/blitz-dom/src/layout/construct.rs:313` | 25 |
| `compute_layer_size` | `packages/blitz-paint/src/render/background.rs:744` | 25 |
| `find_inline_layout_embedded_boxes` | `packages/blitz-dom/src/layout/construct.rs:969` | 24 |
| `sub_document_session` | `packages/blitz-vibey-cdp-server/tests/sub_documents.rs:160` | 24 |
| `<anonymous>` | `packages/blitz-vibey-cdp-server/tests/sub_documents.rs:174` | 24 |
| `advance` | `packages/blitz-vibey-script/src/dom/selection.rs:358` | 24 |
| `snapshot_node_impl` | `packages/blitz-dom/src/document.rs:2117` | 23 |
| `flush_pseudo_elements` | `packages/blitz-dom/src/layout/construct.rs:685` | 23 |
| `process_added_subtree` | `packages/blitz-dom/src/mutator.rs:1301` | 23 |
| `<anonymous>` | `packages/blitz-dom/src/mutator.rs:1302` | 23 |
| `draw_line_text_shadows` | `packages/blitz-paint/src/text_decorations.rs:639` | 23 |
| `TabStrip` | `apps/browser/src/tab_strip.rs:15` | 22 |
| `tokenize` | `examples/seven_guis/src/tasks/cells.rs:74` | 22 |
| `inline_fragment_boxes` | `packages/blitz-dom/src/node/node.rs:1927` | 22 |
| `visit_inline_layout` | `packages/blitz-vibey-script/src/inner_text.rs:187` | 22 |
| `propagate_damage_flags` | `packages/blitz-dom/src/layout/damage.rs:58` | 21 |
| `restyle_for_child_list_change` | `packages/blitz-dom/src/mutator.rs:838` | 21 |
| `walk_tree` | `packages/blitz-dom/src/util.rs:91` | 21 |
| `<anonymous>` | `packages/blitz-vibey-cdp-server/src/session/dom.rs:236` | 21 |
| `print_to` | `wpt/runner/src/main.rs:244` | 21 |
| `<anonymous>` | `wpt/runner/src/main.rs:406` | 21 |
| `handle_dom_event` | `packages/blitz-dom/src/events/mod.rs:107` | 20 |
| `<anonymous>` | `packages/blitz-dom/src/layout/damage.rs:364` | 20 |
| `collect_columns` | `packages/blitz-dom/src/layout/table.rs:441` | 20 |
| `cssom_scroll_size` | `packages/blitz-dom/src/node/node.rs:2101` | 20 |
| `resolve_transforms` | `packages/blitz-dom/src/resolve.rs:175` | 20 |
| `collect_layout_children_inline_roots` | `packages/blitz-dom/src/traversal.rs:557` | 20 |
| `draw_dotted_border_edge` | `packages/blitz-paint/src/render/border.rs:475` | 20 |
| `draw_decoration_line` | `packages/blitz-paint/src/text_decorations.rs:287` | 20 |
| `handle_blitz_shell_event` | `packages/blitz-shell/src/application.rs:73` | 20 |
| `render_reference_and_compare` | `wpt/runner/src/test_runners/ref_test.rs:102` | 20 |
| `scroll` | `packages/blitz-dom/src/scrolling.rs:242` | 19 |
| `draw_scrollbars` | `packages/blitz-paint/src/render.rs:896` | 19 |
| `handle_event` | `packages/blitz-vibey-cdp-server/src/lib.rs:193` | 19 |
| `set_attribute` | `packages/dioxus-native-dom/src/mutation_writer.rs:229` | 19 |
| `clear_interaction_state_for_removed_node` | `packages/blitz-dom/src/document.rs:1492` | 18 |
| `find_inline_layout_embedded_boxes_recursive` | `packages/blitz-dom/src/layout/construct.rs:1002` | 18 |
| `inline_span_cb_flags` | `packages/blitz-dom/src/layout/inline/layout.rs:307` | 18 |
| `touch_action_pan_axes` | `packages/blitz-dom/src/events/pointer.rs:169` | 17 |
| `refresh_anonymous_block_styles` | `packages/blitz-dom/src/layout/damage.rs:200` | 17 |
| `<anonymous>` | `packages/blitz-dom/src/layout/replaced.rs:95` | 17 |
| `style` | `packages/blitz-dom/src/stylo_to_parley.rs:441` | 17 |
| `index_rules` | `packages/blitz-vibey-cdp-server/src/style_sheets.rs:542` | 17 |
| `handle_dioxus_native_event` | `packages/dioxus-native/src/dioxus_application.rs:116` | 17 |
| `dispatch_hover_change` | `packages/blitz-dom/src/events/driver.rs:70` | 16 |
| `handle_pointerup` | `packages/blitz-dom/src/events/pointer.rs:742` | 16 |
| `draw_table_borders` | `packages/blitz-paint/src/render/border.rs:548` | 16 |
| `parse_attributes_text` | `packages/blitz-vibey-cdp-server/src/session/dom.rs:326` | 16 |

**The fork.** Either:

- **A — keep them as upstream's (least change; the standing master rule; stands meanwhile).** No upstream-authored
  function is split. The audit reads them by name as kept under `decision/complexity-ceiling-host-only`.
- **B — bring named ones under in the fork.** A later chunk splits the ones the founder names, as the click handler
  was, each at the price of a hand re-apply when a sync edits it.
- **C — offer the splits upstream**, as F1's C.

## F4 — escher's seven integration-test targets over 800 code lines (who owns them)

**What was measured.** Seven of escher's ten files over 800 are integration-test targets, each one file and one test
target, by the same recipe at `29b3a5fb7`. This chunk leaves them as they are: a split moves a pinned target layout,
and the route that would own the work is the founder's. On the operator's review (`inputs#I3` of this chunk), the chunk
mints no route entry for them; the wrap carries this fork.

| target | code lines | what CI pins (`.github/scripts/ci-leg.sh`, its pins in `.github/scripts/`) |
|---|---|---|
| `tests/blitz-tests/tests/stand_requirement_sweep.rs` | 1582 | `stand-repeat` by the `stand_*` glob; its own leg check reads `ci-leg.sh` whole by `contains`, holding the leg's named list jointly with `test_stand_repeat_leg_runs_three_rounds`; one target a file (`BlitzTestsTargetsTest`) |
| `tests/blitz-tests/tests/select_popup.rs` | 1079 | the `a11y` leg's named `--test`; `stand-repeat`'s named `--test` list; one target a file |
| `tests/blitz-tests/tests/stand_keyboard.rs` | 1063 | the `a11y` leg's named `--test`; `stand-repeat` by the glob; one target a file |
| `tests/blitz-tests/tests/stand_act_error_text.rs` | 937 | `stand-repeat` by the glob; one target a file |
| `examples/seven_guis/tests/cli_commands.rs` | 868 | `stand-repeat`'s seven_guis process checks (`cargo test -p seven_guis --test cli_commands …`) |
| `tests/blitz-tests/tests/stand_mcp_tools.rs` | 846 | `stand-repeat` by the glob; one target a file |
| `tests/blitz-tests/tests/select_interaction.rs` | 828 | the `a11y` leg's named `--test`; `stand-repeat`'s named `--test` list; one target a file |

Beside the legs, `docs/capabilities.md`'s a11y-leg file count and `stand-repeat` named-target count are held to the
leg script by `test_capability_statement.py`, so a target renamed or split there moves those counts by hand.

**The fork.** Either:

- **A — keep them (least change; stands meanwhile).** The seven stay one file each. The next boundary audit reads
  them by name as kept, and v040-08's "kept by the founder's word" covers them.
- **B — a route entry splits them.** The founder adds an entry to the route (its place his) that splits each into
  helpers in its check's own file or the shared test modules, keeping one target a file and every pin. Splitting a
  file into helpers moves no target; only a split into several targets moves a pin.

## F3 — the upstream-taken tests' duplication (restated; still unanswered)

The previous chunk's F1 (`.andromeda/runs/2026-11-20T01-58-20-phase/founder-forks.md`, sealed) stands unanswered:
whether upstream's `packages/blitz-vibey-cdp-server/tests/` and `packages/blitz-vibey-script/tests/` clone pairs, 58
pairs / 611 lines of the whole 3.74 % duplication reading at `4ff03a306`, are kept as upstream's (A, least change,
standing) or folded in the fork (B). On the operator's directive (`inputs#I1`) this WATCH «rides the same way» as F1
and F2: this chunk edits no upstream-taken test and presumes no answer. The `WATCH:` on «Codebase health» (tx 2537)
goes on observing the reading until he answers.
