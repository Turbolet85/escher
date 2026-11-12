@row_add n=1 row=decision/coverage-floors area=inherited-defaults rec=1,2
name: Per-crate coverage floors
rule: A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness.
guard: CoverageFloorTest
@row_add n=2 row=decision/engine-tracing-gated-per-call-site area=inherited-defaults rec=3,4,5
name: Engine tracing gated per call site
rule: Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off.
except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.
@row_add n=3 row=decision/js-microtasks-drained area=cross-cutting-patterns rec=6
name: JS microtasks drained after each entry
rule: JS microtasks are drained after eval, timers and event dispatch.
@row_add n=4 row=decision/resolve-before-js-reads area=cross-cutting-patterns rec=6
name: Style and layout resolved before JS reads
rule: Style and layout are resolved before JS geometry, hit-test, selection and computed-style reads.
@row_add n=5 row=contract/net-handler/error area=design-philosophy rec=7,9,10
boundary: NetHandler
item: error
promise: Receives the URL and error of a failed load; the trait's default does nothing. The stylesheet handler responds with the error, so a failed stylesheet stops blocking render; a wrapping handler must forward it or the failure stops at the wrapper.
@row_add n=6 row=decision/wpt-no-js-fast-path area=design-philosophy rec=8
name: Fast no-JS path for layout-only WPT tests
rule: WPT attr tests that only check layout run on a fast no-JS path.
