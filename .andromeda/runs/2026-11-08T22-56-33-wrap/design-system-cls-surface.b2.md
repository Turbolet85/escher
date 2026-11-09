@answer n=1 entity=surface#6.1 v=no
why: the session touches no macOS browser titlebar and no native window code
@answer n=2 entity=surface#6.2 v=no
why: the transparent example window is outside every file the session reaches
@answer n=3 entity=surface#6.3 v=no
why: the window alpha mode in dioxus-native config.rs is not reached by the session
@answer n=4 entity=surface#7.1 v=no
why: the wasm32 canvas sizing from host CSS in dioxus-native config.rs is not reached by the session
@answer n=5 entity=surface#7.2 v=yes mode=stale
why: dioxus_document.rs is a reached file whose cited lines 168-170 and 316-318 may have moved, and no report shows the claim holds on the changed file
@answer n=6 entity=surface#7.3 v=no
why: the scrollbars feature forward in dioxus-native Cargo.toml is not reached and no dependency moved
@answer n=7 entity=surface#7.4 v=no
why: the select list scrollbar thumb in blitz-paint and its painted test are not reached and the task leaves select lists untouched
@answer n=8 entity=surface#8.1 v=no
why: the mobile urlbar sizing in browser.css is not reached by the session
@answer n=9 entity=surface#9.1 v=no
why: the mobile frame class in apps/browser is not reached by the session
@answer n=10 entity=surface#10.1 v=no
why: the Android back button in apps/browser is not reached by the session
@answer n=11 entity=surface#11.1 v=no
why: the mobile screenshot save path in apps/browser capture.rs is not reached by the session
@answer n=12 entity=surface#12.1 v=no
why: the CI workflow file ci.yml is not reached and the session changes no build target
@answer n=13 entity=surface#13.1 v=no
why: the WPT runner result colours are untouched and the session adds no colour
@answer n=14 entity=surface#13.2 v=no
why: the WPT runner bright black tokens are untouched and the session adds no colour
@answer n=15 entity=surface#13.3 v=no
why: the WPT runner OSC 8 hyperlinks are not reached and the session writes no ESC byte on any output
@answer n=16 entity=surface#14.1 v=no
why: paint_bench output is unchanged and examples/paint_bench.rs is not reached
@answer n=17 entity=surface#14.2 v=no
why: screenshot example output is unchanged and examples/screenshot.rs is not reached
@answer n=18 entity=surface#15.1 v=yes mode=add-to
why: the coverage role joins the agent-read command line rule with its one uncoloured JSON line (F1) and its refusals holding nothing of the check path or bytes (F2)
@answer n=19 entity=surface#15.2 v=yes mode=stale
why: the session adds a coverage clause before the apps clause of the usage line this entity quotes, and session_host.rs is a reached file
@answer n=20 entity=surface#16.1 v=yes mode=add-to
why: the coverage reader reads the UI graph at boot and after each replayed step, which this cli-graph entry must now carry
@answer n=21 entity=surface#17.1 v=yes mode=add-to
why: kept checks actions steps now replay under test and the coverage role replays kept checks of any step kind, which this cli-actions entry must now carry
@answer n=22 entity=surface#17.2 v=yes mode=stale
why: docs/capabilities.md is a reached file whose cited lines 282-284 the session's edit may have moved, and no report shows the claim holds on the changed file
@answer n=23 entity=surface#18.1 v=no
why: the task holds the graph read unpaged and its stated limit unchanged, the paging residual is not this session's change
@answer n=24 entity=surface#19.1 v=no
why: the task holds the actions read unpaged and its stated limit unchanged, the paging residual is not this session's change
@answer n=25 entity=surface#20.1 v=no
why: the WASM bundled font registration in wasm_hello and dioxus-native config.rs is not reached and no font moved
