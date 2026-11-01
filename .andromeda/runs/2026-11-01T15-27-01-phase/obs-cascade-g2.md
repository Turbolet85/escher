# obs · cascade read · group 2 of 4

## Carried
- (.andromeda/obs-plan-detail/metric-coverage.md:10) The take reaches the root manifest whole and conflicts the lock, so under any shape that moves them the frame and phase timings must stay gated by these features and remain log features only.
  > | Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:265-268); (packages/dioxus-native/Cargo.toml:60-68) |
- (.andromeda/obs-plan-detail/metric-coverage.md:11) The delta read names every new print, so a phase a carried engine commit adds to or drops from `resolve` must be read against this list and named in that read.
  > | blitz-dom `resolve` | Phase times style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs | printed, prefixed `Resolve({id}): ` | (packages/blitz-dom/src/resolve.rs:77-172) |
  (.andromeda/obs-plan.md:144)
  > In-process timings and counters are printed, shown in the browser's in-app FPS overlay or held in process — the frame and phase timings behind the `log-times` features, blitz-dom's resolve phases, `debug_timer`, blitz-paint's layer counters
- (.andromeda/obs-plan-detail/metric-coverage.md:12) A timer's steps are printed output, so the delta read must count a new labelled instant in the taken code as a new print even where its call site holds no print token.
  > | debug_timer | Labelled instants; total and per-step durations in ns/us/ms/s | printed | (packages/debug_timer/src/lib.rs:14-24; packages/debug_timer/src/lib.rs:33-66) |
- (.andromeda/obs-plan-detail/metric-coverage.md:14) The scope names `6fca6308d`'s `mix-blend-mode` and `isolation` as paint properties a take may carry, so where one does these counters must stay in process, and any print or log of them must be named by the delta read.
  > | blitz-paint LayerManager | In-process counters layers_used, layer_depth, layers_wanted and a debugging-only layer_depth_used | in-process | (packages/blitz-paint/src/layers.rs:8-16; packages/blitz-paint/src/layers.rs:58; packages/blitz-paint/src/layers.rs:81-84) |
- (.andromeda/obs-plan-detail/metric-coverage.md:32) The layout slice is the home of predicted conflict files, so every resolution there must add no metric, and the delta read must tell a new metric from the text-layout `metrics()` calls this search already matches.
  > - observed absent — metrics · searched: `metric|counter!|histogram` over the 8 slice files (matches are only text-layout `metrics()` calls and comments)
- (.andromeda/obs-plan-detail/metric-coverage.md:34) The conflicted `runtime/bootstrap.rs` and `runtime/events.rs` and `91ef65e21`'s script API sit in this crate, so the take must add no metric there and its own record must state its own reading of the module counts and of any dependency, feature, manifest, log site, span or field it moves, beside this record of the last take.
  > - observed absent — metrics counters, gauges or histograms · searched: `metrics|counter!|histogram|gauge` over the 32 slice files (blitz-vibey-script / blitz slice); the slice's blitz-vibey-script code stands split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, upstream's side moved whole), and the take moves no dependency, feature or manifest and adds no log site, span or field — 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)
- (.andromeda/obs-plan-detail/metric-coverage.md:30) A take brings engine code and may bring crates, so the delta read and the hand review of each entering crate must find no metrics macro, exporter or backend in these slices, or name the one found and re-state the absence it moves.
  > - observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files
  (.andromeda/obs-plan-detail/metric-coverage.md:31)
  > - observed absent — metric emission · searched: `counter!|histogram!|gauge!` over the 17 listed s06 files
  (.andromeda/obs-plan-detail/metric-coverage.md:33)
  > - observed absent — metrics · searched: `metric|counter|histogram|gauge` over the 16 listed files (no match)
  (.andromeda/obs-plan-detail/metric-coverage.md:35)
  > - observed absent — metrics or telemetry backends · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files
  (.andromeda/obs-plan.md:148)
  > Metrics exporters, metric emission and counters, gauges or histograms observed absent over the reading's apps, s05, s06, layout, blitz-vibey-script / blitz and s11 slices and the 16 listed files.

## Bindings
- **Frame and phase timing, `debug_timer` ↔ Telemetry Strategy's feature wiring**: the timing rows hold only while the manifests a take moves still forward the features and `log-phase-times` still enables the timer, so the two are read together on the moved manifests (.andromeda/obs-plan.md:50) (.andromeda/obs-plan.md:52).
- **The slice absences ↔ Telemetry Strategy's and the harness contract's absent backends**: a crate entering under any shape is read against the metrics absences here and the backend absences there as one question (.andromeda/obs-plan-detail/metric-coverage.md:35) (.andromeda/obs-plan.md:56) (.andromeda/obs-plan.md:66).
- **The blitz-vibey-script / blitz absence ↔ the last take's silence record**: this take's delta read is measured against both lines from `ebacd68bd`'s take, and its own record joins them (.andromeda/obs-plan-detail/metric-coverage.md:34) (.andromeda/obs-plan.md:99).
