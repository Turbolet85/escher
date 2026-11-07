@answer n=1 entity=metric-coverage#1.1 v=no
why: the session moves no Cargo feature, manifest or log-times gate, so the frame and phase timing feature wiring stands
@answer n=2 entity=metric-coverage#1.2 v=no
why: the session edits only dioxus-native-dom and the probe file, not blitz-dom's resolve.rs, so its printed phase lines stand
@answer n=3 entity=metric-coverage#1.3 v=no
why: the debug_timer crate is not among the session's reached files and the session adds no timer or print
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay and toolbar under apps/browser are not reached by the dioxus-native-dom bridge change
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint's layers.rs is not reached, and the buffered render adds no layer counter
@answer n=6 entity=metric-coverage#1.6 v=no
why: examples/paint_bench.rs is not a reached file and the bridge change does not alter its encode or raster timing
@answer n=7 entity=metric-coverage#1.7 v=no
why: examples/screenshot.rs is not a reached file and its millisecond timings are untouched
@answer n=8 entity=metric-coverage#1.8 v=no
why: examples/multi_doc.rs is not a reached file and its Instant timings are untouched
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench.rs is not among the session's reached files, which are the probe file dioxus_runtime_probe.rs and the three dioxus-native-dom sources
@answer n=10 entity=metric-coverage#1.10 v=no
why: paint_tree_bench.rs is not among the session's reached files, so its external page node count stands
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner under wpt/runner is not reached by the session
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner under wpt/runner is not reached by the session, so its per-run tallies stand
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT runner's report generation under wpt/runner is not reached by the session
@answer n=14 entity=metric-coverage#1.14 v=no
why: the wpt workflow and its Pages publish step are unchanged, as no CI step moved
@answer n=15 entity=metric-coverage#2.1 v=no
why: the session adds no metrics exporter or OTel surface and reaches no apps-slice file, so the absence stands
@answer n=16 entity=metric-coverage#2.2 v=no
why: the session adds no counter, histogram or gauge macro and reaches none of the listed s15 files, so the absence stands
@answer n=17 entity=metric-coverage#2.3 v=no
why: the session adds no metric emission and reaches none of the listed s06 files, so the absence stands
@answer n=18 entity=metric-coverage#2.4 v=no
why: the session reaches none of the 8 slice files and adds no metric token, so the absence stands
@answer n=19 entity=metric-coverage#2.5 v=no
why: the session reaches none of the 16 listed files and adds no metric token, so the absence stands
@answer n=20 entity=metric-coverage#2.6 v=no
why: blitz-vibey-script is not reached by the session, and the task records its script error surface and metric absence as untouched
@answer n=21 entity=metric-coverage#2.7 v=no
why: the session reaches none of the 21 listed s11 files and adds no metrics or telemetry backend
