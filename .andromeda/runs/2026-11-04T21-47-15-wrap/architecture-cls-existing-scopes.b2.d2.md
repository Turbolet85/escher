@answer n=1 entity=existing-scopes#1.26 v=no
why: the task names no change under packages/escher-driver and the lock moves only pulldown-cmark, which this crate does not use
@answer n=2 entity=existing-scopes#1.27 v=no
why: apps/browser is outside every changed file and its module list names no dependency version
@answer n=3 entity=existing-scopes#1.28 v=no
why: apps/browser/persistence is outside every changed file and its HistoryStore claim names no dependency version
@answer n=4 entity=existing-scopes#1.29 v=no
why: the backend claim names no version and rdme still uses pulldown_cmark, so the 0.13.5 to 0.13.6 bump leaves it true, with apps/readme/src/main.rs outside the changed files
@answer n=5 entity=existing-scopes#1.30 v=no
why: wpt/runner is outside every changed file and names no dependency version
@answer n=6 entity=existing-scopes#1.31 v=yes mode=add-to
why: the new blitz-tests target dioxus_runtime_probe, F8, joins the Dioxus integration bullet beside stale_node_mapping and harness_smoke as 10 public-API tests over the stand
@answer n=7 entity=existing-scopes#1.32 v=no
why: examples/seven_guis is outside every changed file and no Cargo.toml moved
@answer n=8 entity=existing-scopes#1.33 v=no
why: the session changes nothing under examples/ and no example Cargo.toml moved
@answer n=9 entity=existing-scopes#1.34 v=no
why: the examples/*.rs files are outside the session's changes, which touch nothing under examples/
@answer n=10 entity=existing-scopes#1.35 v=no
why: the examples/assets pages are outside the session's changes, which touch nothing under examples/
@answer n=11 entity=existing-scopes#2.1 v=no
why: the probe changes no served page and no stand_cells_served file, so its 5 tests and the 47,089-byte Cells screen hold
@answer n=12 entity=existing-scopes#3.1 v=no
why: packages/escher-driver is outside every changed file, so its #[path] unit-test children and their line citations hold
@answer n=13 entity=existing-scopes#3.2 v=no
why: the probe declares none of tests/common, tests/session_common or tests/fixtures, so the reader counts 21, 32 and 31 and the 23 stand_ files hold
@answer n=14 entity=existing-scopes#3.3 v=no
why: stand_requirement_sweep.rs and its inside_stand_repeat predicate are untouched, and the probe adds no stand_ target and no ci-leg.sh line
@answer n=15 entity=existing-scopes#3.4 v=no
why: packages/blitz-vibey-script is outside every changed file and names no dependency version
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo_element_flow_classification and its all.rs mod line are untouched, the session adding a separate target only
@answer n=17 entity=existing-scopes#4.1 v=yes mode=add-to
why: the new target dioxus_runtime_probe, F8, is one file with one target in both layout modes and no stand_ prefix, so it joins this list of engine-fact targets
