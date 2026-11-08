@answer n=1 entity=metric-coverage#1.1 v=no
why: the session changes no manifest, feature wiring or Cargo.toml, so the log-times and log-frame-times gating stands
@answer n=2 entity=metric-coverage#1.2 v=no
why: the session moves no blitz-dom source, so the resolve phase prints in resolve.rs stand
@answer n=3 entity=metric-coverage#1.3 v=no
why: the session does not touch packages/debug_timer, so its labelled instants and durations stand
@answer n=4 entity=metric-coverage#1.4 v=no
why: the session does not touch apps/browser, so the FPS overlay ring and polling stand
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint is untouched, so the LayerManager in-process counters stand
@answer n=6 entity=metric-coverage#1.6 v=no
why: the session does not touch examples/paint_bench.rs or blitz-paint, so its timings stand
@answer n=7 entity=metric-coverage#1.7 v=no
why: the session does not touch examples/screenshot.rs, so its millisecond timings stand
@answer n=8 entity=metric-coverage#1.8 v=no
why: the session does not touch examples/multi_doc.rs, so its run and resolve timings stand
@answer n=9 entity=metric-coverage#1.9 v=no
why: the session edits only the shared common module's helpers, not the bench's own medians in paint_tree_bench.rs, which it does not reach
@answer n=10 entity=metric-coverage#1.10 v=no
why: the external-page node count and hoisted entries are printed by paint_tree_bench.rs, which the session does not change
@answer n=11 entity=metric-coverage#1.11 v=no
why: wpt/runner is untouched and the session adds no WPT timing, so the per-test durations stand
@answer n=12 entity=metric-coverage#1.12 v=no
why: wpt/runner is untouched and the session adds no WPT counter, so the per-run tallies stand
@answer n=13 entity=metric-coverage#1.13 v=no
why: wpt/runner is untouched, so the report generation and write timings stand
@answer n=14 entity=metric-coverage#1.14 v=no
why: wpt.yml and the WPT publishing are untouched (untouched item 8), so the wptscores.json publication stands
@answer n=15 entity=metric-coverage#2.1 v=no
why: the session adds no metric, counter or timing (untouched item 4) and reaches no apps file, so the absence of metrics exporters still holds
@answer n=16 entity=metric-coverage#2.2 v=no
why: the session adds no counter!, histogram! or gauge! emission (untouched item 4), so the absence over the s05 files still holds
@answer n=17 entity=metric-coverage#2.3 v=no
why: the session adds no counter!, histogram! or gauge! emission (untouched item 4), so the absence over the s06 files still holds
@answer n=18 entity=metric-coverage#2.4 v=no
why: the session adds no metric or counter text (untouched item 4), so the absence over the slice still holds with only text-layout metrics() matches
@answer n=19 entity=metric-coverage#2.5 v=no
why: the session adds no metric, counter, histogram or gauge (untouched item 4), so the no-match reading over the listed files still holds
@answer n=20 entity=metric-coverage#2.6 v=no
why: blitz-vibey-script is untouched (untouched item 2) and the session adds no metric, so the absence of metrics still holds
@answer n=21 entity=metric-coverage#2.7 v=no
why: the session adds no metric or exporter (untouched item 4), so the absence over the s11 files still holds
