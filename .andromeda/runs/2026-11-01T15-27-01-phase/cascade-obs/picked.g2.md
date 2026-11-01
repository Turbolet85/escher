### metric-coverage#1.1 · values · metrics · ## 5. Metric Coverage · lines 8–10
| Operation | What is recorded | Output | Source |
|-----------|------------------|--------|--------|
| Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:265-268); (packages/dioxus-native/Cargo.toml:60-68) |
### metric-coverage#1.2 · values · metrics · ## 5. Metric Coverage · lines 11–11
| blitz-dom `resolve` | Phase times style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs | printed, prefixed `Resolve({id}): ` | (packages/blitz-dom/src/resolve.rs:77-172) |
### metric-coverage#1.3 · values · metrics · ## 5. Metric Coverage · lines 12–12
| debug_timer | Labelled instants; total and per-step durations in ns/us/ms/s | printed | (packages/debug_timer/src/lib.rs:14-24; packages/debug_timer/src/lib.rs:33-66) |
### metric-coverage#1.5 · values · metrics · ## 5. Metric Coverage · lines 14–14
| blitz-paint LayerManager | In-process counters layers_used, layer_depth, layers_wanted and a debugging-only layer_depth_used | in-process | (packages/blitz-paint/src/layers.rs:8-16; packages/blitz-paint/src/layers.rs:58; packages/blitz-paint/src/layers.rs:81-84) |
### metric-coverage#2.1 · absence · metrics · ## 5. Metric Coverage · lines 29–29
- observed absent — metrics exporters · searched: `opentelemetry|metrics|prometheus|sentry` over the 86 slice files (apps slice)
### metric-coverage#2.2 · absence · metrics · ## 5. Metric Coverage · lines 30–30
- observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files
### metric-coverage#2.3 · absence · metrics · ## 5. Metric Coverage · lines 31–31
- observed absent — metric emission · searched: `counter!|histogram!|gauge!` over the 17 listed s06 files
### metric-coverage#2.4 · absence · metrics · ## 5. Metric Coverage · lines 32–32
- observed absent — metrics · searched: `metric|counter!|histogram` over the 8 slice files (matches are only text-layout `metrics()` calls and comments)
### metric-coverage#2.5 · absence · metrics · ## 5. Metric Coverage · lines 33–33
- observed absent — metrics · searched: `metric|counter|histogram|gauge` over the 16 listed files (no match)
### metric-coverage#2.6 · absence · metrics · ## 5. Metric Coverage · lines 34–34
- observed absent — metrics counters, gauges or histograms · searched: `metrics|counter!|histogram|gauge` over the 32 slice files (blitz-vibey-script / blitz slice); the slice's blitz-vibey-script code stands split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, upstream's side moved whole), and the take moves no dependency, feature or manifest and adds no log site, span or field — 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)
### metric-coverage#2.7 · absence · metrics · ## 5. Metric Coverage · lines 35–35
- observed absent — metrics or telemetry backends · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files
