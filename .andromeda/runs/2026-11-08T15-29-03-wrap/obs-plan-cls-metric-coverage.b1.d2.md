@answer n=1 entity=metric-coverage#1.1 v=no
why: the session adds no timing log feature or Cargo feature, and the Cargo.toml files it cites are not among the changed files
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom is diffed unchanged, so the resolve phase timing it describes is not reached
@answer n=3 entity=metric-coverage#1.3 v=no
why: the debug_timer crate is not among the session's changed files
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser app's FPS overlay is untouched and no browser file is among the changed files
@answer n=5 entity=metric-coverage#1.5 v=no
why: the blitz-paint LayerManager counters are untouched and blitz-paint is not among the changed files
@answer n=6 entity=metric-coverage#1.6 v=no
why: examples/paint_bench.rs is not among the changed files
@answer n=7 entity=metric-coverage#1.7 v=no
why: examples/screenshot.rs timings are untouched, the session changes the screenshot verb in escher-driver and not this example
@answer n=8 entity=metric-coverage#1.8 v=no
why: examples/multi_doc.rs is not among the changed files
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench.rs is not among the changed files and its hit and render medians are untouched
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external-page printout is untouched and the file is not reached
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner is not among the changed files and its per-test timing is unchanged
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner is not among the changed files and its per-run tallies are unchanged
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT runner is not among the changed files and its report timings are unchanged
@answer n=14 entity=metric-coverage#1.14 v=no
why: .github/workflows/wpt.yml is not among the changed files and the WPT scores are unchanged
@answer n=15 entity=metric-coverage#2.1 v=no
why: the session adds no metrics exporter, so the absence still holds on its changed files
@answer n=16 entity=metric-coverage#2.2 v=no
why: the session adds no counter!, histogram! or gauge! use, so the absence still holds on its changed files
@answer n=17 entity=metric-coverage#2.3 v=no
why: the session adds no counter!, histogram! or gauge! use, so the absence still holds on its changed files
@answer n=18 entity=metric-coverage#2.4 v=no
why: the session adds no metric or counter use, so the absence still holds on its changed escher-driver and dioxus-native-dom files
@answer n=19 entity=metric-coverage#2.5 v=no
why: the session adds no metric, counter, histogram or gauge word to its changed files, so the absence still holds
@answer n=20 entity=metric-coverage#2.6 v=no
why: the session adds no counter, histogram or gauge and no dependency, feature or manifest, so the absence and its blitz-vibey-script counts still hold
@answer n=21 entity=metric-coverage#2.7 v=no
why: the session adds no metrics or telemetry backend, so the absence still holds on the listed s11 files
