@answer n=1 entity=existing-scopes#1.26 v=no
why: the session changed no escher-driver file and no packages/escher-driver/Cargo.toml line, so the driver's session and schema claims hold
@answer n=2 entity=existing-scopes#1.27 v=no
why: apps/browser is outside everything the session changed
@answer n=3 entity=existing-scopes#1.28 v=no
why: apps/browser/persistence is outside everything the session changed
@answer n=4 entity=existing-scopes#1.29 v=no
why: Cargo.lock moved pulldown-cmark 0.13.5 to 0.13.6, but the entity's claim of comrak and pulldown_cmark backends in apps/readme still holds on the changed lock and names no version
@answer n=5 entity=existing-scopes#1.30 v=no
why: wpt/runner is outside everything the session changed
@answer n=6 entity=existing-scopes#1.31 v=yes mode=add-to
why: the new dioxus_runtime_probe target (10 tests, F8) joins the Dioxus integration group beside stale_node_mapping and harness_smoke
@answer n=7 entity=existing-scopes#1.32 v=no
why: examples/seven_guis, its eight targets and the stand boot are unchanged by the session
@answer n=8 entity=existing-scopes#1.33 v=no
why: the separate example crates under examples/* are unchanged by the session
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root example files under examples/*.rs are unchanged by the session
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixtures under examples/assets are unchanged by the session
@answer n=11 entity=existing-scopes#2.1 v=no
why: stand_cells_served and stand_load_page are unchanged, and the new probe is no stand_ file
@answer n=12 entity=existing-scopes#3.1 v=no
why: the escher-driver cli and recording #[path] test children are unchanged by the session
@answer n=13 entity=existing-scopes#3.2 v=yes mode=stale
why: the new target may read tests/fixtures or tests/session_common, which the task does not report, so the 31 and 32 reader counts may have moved
@answer n=14 entity=existing-scopes#3.3 v=no
why: inside_stand_repeat reads 0.2.0 witnesses, and the new probe is no witness of that record
@answer n=15 entity=existing-scopes#3.4 v=no
why: blitz-vibey-script's runtime and dom modules are unchanged by the session
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo_element_flow_classification and its all.rs line are unchanged by the session
@answer n=17 entity=existing-scopes#4.1 v=no
why: the probe is not an engine-fact target, and the listed engine-fact targets are unchanged
