@carried n=1 label=a11y-plan/a11y-scope-summary fact=fct-r6zswm2d
why: The probe's code in `packages/dioxus-native-dom/`, `examples/seven_guis/` and `tests/blitz-tests/` must leave the stand's graph with no platform adapter, as the last re-read measured it, and must not take up the adapter the founder deferred.
quote: re-read on the upstream take read at `e42ee1f25` (merge base `23b9b26a9`): `Cargo.toml`, `Cargo.lock`, `deny.toml` and every `*/Cargo.toml` unchanged by the take, and the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
@carried n=1 run=2 label=a11y-plan/a11y-scope-summary fact=fct-r6zswm2d
quote: kept so on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14, by question dialog in the overseer session, relayed by the overseer as his chosen option label): the adapter on the stand binary and its windowed AT witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`
@carried n=2 label=a11y-plan/a11y-scope-summary fact=fct-lxyhqmtz
why: The probe's served task beside the lean one (Cells or the load page) must keep its stated Tab sequence, keyboard and contrast rows and its keyed, lint-0 checks passing.
quote: Cells (`seven_guis::stand::boot_cells`, mounting `Task::Cells` in `TaskShell`) and the load page (`seven_guis::stand::boot_load`, its `load-fetch` Button and `load-done` Paragraph), both served by `escher-session`; neither is a `LeanTask`; each holds a stated Tab sequence (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) that `stand_keyboard` walks, with keyboard rows of its own (the load page's `load-fetch` taking Enter and Space, Cells' arrows and Enter on `cells-grid`) and contrast rows in `stand_contrast`, and their own checks read them keyed, `unkeyed_actionable` empty, the load page's lint 0
@carried n=3 label=a11y-plan/a11y-scope-summary fact=fct-ip333e77
why: A probe check placed in `tests/blitz-tests/` must build on this crate's existing accessibility feature set and dev-dependency, adding no feature or dependency of its own.
quote: **blitz-tests** — `blitz-dom` and `dioxus-native-dom` are built with the `accessibility` feature and `accesskit` is a dev-dependency (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/Cargo.toml:21; tests/blitz-tests/Cargo.toml:33)
@carried n=4 label=a11y-plan fact=fct-r5pehooz
why: The chunk's a11y target is tier 0; the probe must hold the three criteria where it touches the stand and must add no other.
quote: **A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`
@carried n=5 label=a11y-plan fact=fct-77ewqepn
why: The probe's "reacts to a click" reading (link 2 for `click`) must stay a verdict in the founder's reading and must add no snapshot field, because a residuals line owns that reading.
quote: no snapshot field says an element reacts to a click — that reading owned by a `.andromeda/residuals.md` line (target: next).
@carried n=6 label=a11y-plan fact=fct-iq4fjjyz
why: A probe reading node → component through the bridge's element ids must leave each element's stable id as its `author_id`, with no second role or name mapping.
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one; on the stand every interactive-role node of the four lean tasks reads a non-empty name and the tree the shell hands the adapter equals the harness's tree node for node; the roles and names the checks expect are stated once, in the shared module's tables — never a second role or name mapping.
@carried n=7 label=a11y-plan fact=fct-zyrpeqlm
why: Any escher-side change to `dioxus_document.rs`'s `poll`, and any MountedData fix option the reading brings, must keep the tree refresh going through the `Document::accessibility_tree` override in both layout modes.
quote: The accessibility tree is built from the document on InitialTreeRequested and refreshed on change: a poll that takes a non-empty changed set rebuilds the platform tree, both paths through the `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches it; proven headlessly on the stand in both layout modes
@carried n=8 label=a11y-plan fact=fct-xhpyuh5z
why: A probe measuring link 4 (state → the nodes that read it) through the engine's changed set must drain it first and must count only what marks it.
quote: `changed_nodes` is the engine's changed set, the nodes written since the last `take_changed_nodes`: an in-document mutation, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing mark it
@carried n=8 run=2 label=a11y-plan fact=fct-xhpyuh5z
quote: a harness settle neither reads nor drains it and a fresh harness document's set is non-empty, so a check reading `has_changes()` as "this step changed something" drains first.
@carried n=9 label=a11y-plan fact=fct-6kcxmj7u
why: The `a11y` leg's fourteen files and the eight `stand-repeat` files must keep passing after the probe's changes; the probe's own test joins neither list unless the plan chooses so.
quote: ci.yml's `a11y` job, "Accessibility (a11y) tests", runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked` over fourteen files, `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` — in the slow tier behind the four fast jobs, with no required status check
@binding n=1
**Dioxus crates' adapter reach ↔ the chunk's no-fork boundary and the body's broad line**: the probe adds no dependency or feature that would bring `accesskit_xplat` or `accesskit_winit` into the stand's graph, and Dioxus stays 0.7.3; the deferred adapter stays owned by its `re-carried:0.2.0` residuals line (a11y-plan/a11y-scope-summary fct-r6zswm2d) ↔ (a11y-plan fct-kbjv4usk)
@binding n=2
**Served pages ↔ the probe's served task and the body's keyboard harness**: the served task the probe runs beside the lean one keeps its `stand_keyboard`, `stand_contrast`, `stand_load_page` and `stand_cells_served` rows green (a11y-plan/a11y-scope-summary fct-lxyhqmtz) ↔ (a11y-plan fct-j36rkqej)
@binding n=3
**blitz-tests feature set ↔ the body's snapshot and author-id rules**: a probe check in `tests/blitz-tests/` reads ids and listeners through the crate's existing `accessibility` build, adding no snapshot field and no second role or name mapping (a11y-plan/a11y-scope-summary fct-ip333e77) ↔ (a11y-plan fct-iq4fjjyz) ↔ (a11y-plan fct-77ewqepn)
@binding n=4
**The tree refresh in `poll` ↔ the MountedData fork the reading brings**: each fix option the reading brings to the founder names what it does to the tree refresh that `poll` drives (a11y-plan fct-zyrpeqlm); the chunk decides none of them (none)
@binding n=5
**The a11y CI leg ↔ test-plan §10**: the a11y leg and the `stand-repeat` leg keep passing under test-plan §10's zero-failure budget (a11y-plan fct-6kcxmj7u)
