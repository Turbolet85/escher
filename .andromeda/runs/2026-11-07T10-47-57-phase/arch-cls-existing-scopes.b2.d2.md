@answer n=1 entity=existing-scopes#1.26 v=no
why: escher-driver is a consumer of the bridge through the held stand; its session, verbs, refusals and command span are its own code, which the chunk neither calls nor changes.
@answer n=2 entity=existing-scopes#1.27 v=no
why: the entity lists browser modules by name only and states no rule over the render path that the chunk must honour.
@answer n=3 entity=existing-scopes#1.28 v=no
why: HistoryStore persistence is not touched by the render path (not_needed).
@answer n=4 entity=existing-scopes#1.29 v=no
why: the readme Markdown viewer states no Dioxus document rule and the render path does not reach its comrak or pulldown_cmark backends.
@answer n=5 entity=existing-scopes#1.30 v=no
why: the WPT runner is not a Dioxus document path (not_needed).
@answer n=6 entity=existing-scopes#1.31 v=yes
why: it names the stand_* checks the chunk re-runs as its witnesses (stand_boot, stand_element_ids, stand_act_*, stand_keyboard, stand_snapshot_state) and the Dioxus integration checks stale_node_mapping and harness_smoke that witness the bridge's node mapping.
@answer n=7 entity=existing-scopes#1.32 v=no
why: the seven_guis stand composition, the escher-session binary and its targets are the stand's own code the chunk does not change, and the document-built witness it re-runs is carried by blitz-tests' stand_boot entity.
@answer n=8 entity=existing-scopes#1.33 v=no
why: the separate example crates are named by path only and state no rule over the Dioxus render path.
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root examples are blitz-dom engine examples, none a Dioxus document, and state no render rule the chunk must honour.
@answer n=10 entity=existing-scopes#1.35 v=no
why: the examples/assets HTML fixtures and fixture forms are not_needed for this chunk.
@answer n=11 entity=existing-scopes#2.1 v=yes
why: stand_cells_served and stand_load_page are stand_* checks the chunk re-runs and they state measured values (the Cells screen at 47,089 bytes in 740 lines, and the load-fetch click returning load-done reading "Loaded") that the buffered render must keep.
@answer n=12 entity=existing-scopes#3.1 v=no standing=S1
why: it states the unit-test home convention, unit tests in a #[path] child file of their own, for escher-driver's cli and recording modules.
@answer n=13 entity=existing-scopes#3.2 v=yes
why: the shared stand tables in tests/common/mod.rs (the CRUD tab_order of six stops with crud-list third, ENABLED_TAB_ORDER after return is chosen, and the focused helper) are values that the re-run stand_keyboard and stand_requirement_sweep read, and the focus change must keep them.
@answer n=14 entity=existing-scopes#3.3 v=no
why: it is a private predicate in stand_requirement_sweep reading 0.2.0 witness membership in stand-repeat, and the chunk adds no witness row and no stand-repeat membership.
@answer n=15 entity=existing-scopes#3.4 v=no
why: the blitz-vibey-script JavaScript runtime is not reached, since a Dioxus document runs no page script (not_needed).
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo_element_flow_classification is an engine layout test that is neither on the render path nor a re-run witness.
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact targets (visible_region_transformed, hit_non_invertible_transform, open_state, checkable_change_event, text_range_no_change_event, key_press_after_container_lost_its_box) run on parsed blitz-dom pages rather than DioxusDocument, and none is a re-run witness.
@answer n=18 entity=existing-scopes#4.2 v=yes
why: dioxus_runtime_probe is the file this chunk edits, and it states the file's shape (no print, env read, subscriber or listener, no stand_ prefix) and the two MountedData hazard pins the chunk inverts.
