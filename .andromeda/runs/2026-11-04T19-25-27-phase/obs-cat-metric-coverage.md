@carried n=1 label=obs-plan fact=fct-3goqnphf
why: The probe's test or example code must keep metric emission absent: it adds no metrics exporter, counter, gauge or histogram, so this absence line stands unchanged after the chunk.
quote: Metrics exporters, metric emission and counters, gauges or histograms observed absent over the reading's apps, s05, s06, layout, blitz-vibey-script / blitz and s11 slices and the 16 listed files.
@carried n=2 label=obs-plan fact=fct-asz2jsep
why: The probe must not add a new printed, overlaid or in-process timing or counter to this list; its measured verdicts go into the founder's reading and into assertions, never into a printed figure.
quote: In-process timings and counters are printed, shown in the browser's in-app FPS overlay or held in process — the frame and phase timings behind the `log-times` features, blitz-dom's resolve phases, `debug_timer`, blitz-paint's layer counters, the bench, screenshot and `multi_doc` examples and the WPT runner's per-test, per-run and report figures
@carried n=3 label=obs-plan/metric-coverage fact=fct-5sph2bsu
why: The probe reads `packages/dioxus-native-dom/` (its `poll` and `events.rs`) without editing anything, so the dioxus-native frame and phase timing row must stay exactly as it is: no new timing gate or feature, and no change to the `log-times` wiring.
quote: | Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:265-268); (packages/dioxus-native/Cargo.toml:60-68) |
@carried n=4 label=obs-plan/metric-coverage fact=fct-wy742at6
why: The one printed figure in `tests/blitz-tests/`, where a probe check would live, is the bench's markdown table; a probe check must not print one like it, because nothing it reads is printed and its readings go into assertions only.
quote: | `paint_tree_bench` test | Medians of `hit()`, render on a null backend, an incremental hover-only frame and a full non-incremental frame | printed as a markdown table |
@binding n=1
**Metric Coverage absence ↔ the chunk scope's «Nothing crosses» boundary**: the scope says the probe adds no log field or span and prints or logs nothing it reads. On the obs side, §5 must gain no metric row and no absence line may flip, so the absence stays at (obs-plan fct-3goqnphf) and the printed-figure rows stay as they are (obs-plan/metric-coverage fct-wy742at6).
@binding n=2
**Metric Coverage ↔ obs-plan §6 Log Coverage silence census**: a probe check in `tests/blitz-tests/` or `examples/seven_guis/` must pass the same silence census its siblings pass (no print, subscriber or env read). The metric side matches it by adding no printed timing to the in-process list (obs-plan fct-asz2jsep) and no timing gate on the dioxus-native row (obs-plan/metric-coverage fct-5sph2bsu).
