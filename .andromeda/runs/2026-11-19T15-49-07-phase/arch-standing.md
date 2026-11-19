# arch · standing conventions

- S1 · integration tests live in blitz-tests, one behaviour per file and one test target per file · conventions · design-philosophy
- S2 · scenarios and stand checks run in both layout modes, which give identical layouts · conventions · design-philosophy
- S3 · dependencies are declared once in the workspace; crates inherit package metadata and take the workspace licence; in-repo crates are depended on without default features · conventions · infrastructure-patterns
  decision/app-crate-takes-own-tracing · App crates that log take tracing themselves — An app crate that logs takes tracing itself as a workspace dependency. · why: dioxus-native's prelude re-exports no tracing.
  decision/driver-uses-tracing · tracing in the driver — The driver takes tracing from the workspace with no cargo feature.
- S4 · GitHub Actions are pinned to commit SHAs and CI runs with read-only permissions · stack-and-technologies · infrastructure-patterns
- S5 · engine tracing is gated per call site and every escher binary prints only through the escher-telemetry sink · conventions · cross-cutting-patterns
  decision/engine-tracing-gated-per-call-site · Engine tracing gated per call site — Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off. · except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.
  decision/telemetry-one-install-site · One telemetry install site per binary — Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread.
- S6 · code a resolution writes keeps the code conventions — justified expect and unwrap, annotated unsafe, spec URLs cited, known gaps marked TODO or FIXME, module and public item docs · conventions
  decision/annotated-unsafe · Annotated unsafe blocks — `unsafe` blocks carry an `allow unsafe_code` or a `// SAFETY:` comment. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`.
  decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines.
  decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants.
  decision/example-docs · Example header docs — Examples open with a `//!` comment describing what they demonstrate. · except: Most upstream examples open with a block comment or none.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/known-gap-markers · Known gaps marked TODO or FIXME — Known gaps are marked with `TODO` or `FIXME`.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
  decision/spec-url-citations · Spec URLs cited at implementations — Spec URLs are cited above or next to implementations.
  decision/todo-fixme-gap-markers · Known gaps marked TODO/FIXME — Known gaps are marked with `TODO`/`FIXME`.
  decision/unsafe-justified · Unsafe blocks annotated — `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`.
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
