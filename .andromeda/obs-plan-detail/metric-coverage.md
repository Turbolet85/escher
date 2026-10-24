# obs-plan · Metric Coverage
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its Metric Coverage detail.

## 5. Metric Coverage

<!-- detail: values · metrics -->

| Operation | What is recorded | Output | Source |
|-----------|------------------|--------|--------|
| Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:265-268); (packages/dioxus-native/Cargo.toml:60-68) |
| blitz-dom `resolve` | Phase times style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs | printed, prefixed `Resolve({id}): ` | (packages/blitz-dom/src/resolve.rs:77-169) |
| debug_timer | Labelled instants; total and per-step durations in ns/us/ms/s | printed | (packages/debug_timer/src/lib.rs:14-24; packages/debug_timer/src/lib.rs:33-66) |
| Browser FPS overlay | Frame deltas in a 60-entry ring, polled every 250 ms; average FPS and ms | in-app overlay, toggled from the menu item "Toggle FPS" | (apps/browser/src/fps_overlay.rs:7; apps/browser/src/fps_overlay.rs:27-49; apps/browser/src/fps_overlay.rs:105-123); (apps/browser/src/toolbar.rs:436-439) |
| blitz-paint LayerManager | In-process counters layers_used, layer_depth, layers_wanted and a debugging-only layer_depth_used | in-process | (packages/blitz-paint/src/layers.rs:8-16; packages/blitz-paint/src/layers.rs:58; packages/blitz-paint/src/layers.rs:81-84) |
| `paint_bench` example | Per-iteration encode and raster microseconds | printed min/median/mean/max | (examples/paint_bench.rs:28-54) |
| `screenshot` example | Millisecond timings for fetch, setup, parse, asset fetch, style/layout, render and PNG write | printed | (examples/screenshot.rs:56; examples/screenshot.rs:68; examples/screenshot.rs:86; examples/screenshot.rs:95; examples/screenshot.rs:100; examples/screenshot.rs:134; examples/screenshot.rs:143) |
| `multi_doc` example | The run's time and each resolve's, read with `Instant::now()` | printed, one `println!` | (examples/multi_doc.rs:61-67; examples/multi_doc.rs:111-115) |
| `paint_tree_bench` test | Medians of `hit()`, render on a null backend, an incremental hover-only frame and a full non-incremental frame | printed as a markdown table | (tests/blitz-tests/tests/paint_tree_bench.rs:1-3; tests/blitz-tests/tests/paint_tree_bench.rs:100-134; tests/blitz-tests/tests/paint_tree_bench.rs:341-370) |
| `paint_tree_bench` external page | Node count and hoisted stacking-context entries | printed | (tests/blitz-tests/tests/paint_tree_bench.rs:271-278) |
| WPT runner, per test | Duration in ms | printed | (wpt/runner/src/main.rs:624; wpt/runner/src/main.rs:694; wpt/runner/src/main.rs:375) |
| WPT runner, per run | Pass, fail, timeout, skip, crash, subtests, fractional pass, failure buckets and total duration | printed | (wpt/runner/src/main.rs:498-518; wpt/runner/src/main.rs:783-829) |
| WPT runner, report | Report generation and write times in ms | printed | (wpt/runner/src/main.rs:837-851) |
| WPT scores (CI) | Scores computed into `wptscores.json` | published to Pages, on upstream `DioxusLabs/blitz` only (the `wpt` job is repository-guarded) | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:64-67) |

<!-- /detail -->

<!-- detail: absence · metrics -->

- observed absent — metrics exporters · searched: `opentelemetry|metrics|prometheus|sentry` over the 86 slice files (apps slice)
- observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files
- observed absent — metric emission · searched: `counter!|histogram!|gauge!` over the 17 listed s06 files
- observed absent — metrics · searched: `metric|counter!|histogram` over the 8 slice files (matches are only text-layout `metrics()` calls and comments)
- observed absent — metrics · searched: `metric|counter|histogram|gauge` over the 16 listed files (no match)
- observed absent — metrics counters, gauges or histograms · searched: `metrics|counter!|histogram|gauge` over the 32 slice files (blitz-vibey-script / blitz slice)
- observed absent — metrics or telemetry backends · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files

<!-- /detail -->
