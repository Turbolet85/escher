@answer n=1 entity=existing-scopes#1.26 v=no
why: the driver crate's manifest and source files are not among the reached files, and its text names no bridge render path or MountedData call
@answer n=2 entity=existing-scopes#1.27 v=no
why: the browser app's source files and module list are not reached, and its text names no MountedData call or bridge render path
@answer n=3 entity=existing-scopes#1.28 v=no
why: browser-persistence's HistoryStore export and its files are not reached by the change
@answer n=4 entity=existing-scopes#1.29 v=no
why: the rdme viewer's source files are not reached, and its text names no bridge render path or MountedData call
@answer n=5 entity=existing-scopes#1.30 v=no
why: the wpt runner's modules and test runners are not reached by the change
@answer n=6 entity=existing-scopes#1.31 v=no
why: its integration-test files are not reached, and its falsy-attribute and stable-element-id claims still hold because the replay writes the same mutations through the byte-identical writer with stable ids unchanged
@answer n=7 entity=existing-scopes#1.32 v=no
why: the seven_guis crate, its stand boot, its escher-session binary and its test targets are not reached, and its native dependency list is unchanged
@answer n=8 entity=existing-scopes#1.33 v=no
why: the separate example crates' manifests and sources are not reached by the change
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root example files are not reached, and none of them names a Dioxus bridge render path or a MountedData call
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixture pages are not reached, and their claims concern CSS and layout rather than the bridge's render path
@answer n=11 entity=existing-scopes#2.1 v=no
why: stand_cells_served and stand_load_page are not reached, and the row names no Cells MountedData scroll_to call, its boot, scroll, click and keyboard claims holding on the unchanged mutation stream
@answer n=12 entity=existing-scopes#3.1 v=no
why: the driver's cli and recording unit-test files are not reached by the change
@answer n=13 entity=existing-scopes#3.2 v=no
why: the shared test modules are not reached, and the change adds no test or reader to them
@answer n=14 entity=existing-scopes#3.3 v=no
why: the stand_requirement_sweep target predicate and its path constants are not reached, and the change moves no stand-repeat witness
@answer n=15 entity=existing-scopes#3.4 v=no
why: blitz-vibey-script's runtime and dom element files are not reached by the change
@answer n=16 entity=existing-scopes#3.5 v=no
why: the pseudo-element flow test file and the all.rs target list are not reached by the change
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact test files are not reached, and none of them names the Dioxus bridge or a MountedData call
@answer n=18 entity=existing-scopes#4.2 v=yes mode=contradict
why: its two should_panic MountedData pins become passing spawned_acts tests that assert focus acts, the discriminating witness for F5 on Dioxus 0.7
