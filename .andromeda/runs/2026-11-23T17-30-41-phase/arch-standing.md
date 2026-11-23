# arch · standing conventions

- S1 · each new check runs its scenario in both layout modes · conventions
- S2 · each new integration test lives in blitz-tests, one test target per file, one behaviour per file, declared in the blitz-tests manifest's one test table · design-philosophy · conventions · standard-contracts
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate. · except: blitz-vibey-script keeps its own integration tests in its tests directory
- S3 · each new test has a descriptive name and asserts its fixture's condition first · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S4 · each new stand check takes its shared helpers and tables from the shared fixtures module, stated once, helpers in non-target modules · conventions
  decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
  decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
- S5 · each new stand check boots offline under the stand's pinned environment and bundled typography, its readings identical on every CI OS · standard-contracts · infrastructure-patterns
- S6 · new code carries module and public item docs, justified expect and unwrap, annotated unsafe and locally scoped lint allows · conventions
  decision/annotated-unsafe · Annotated unsafe blocks — `unsafe` blocks carry an `allow unsafe_code` or a `// SAFETY:` comment. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`.
  decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
  decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants.
  decision/example-docs · Example header docs — Examples open with a `//!` comment describing what they demonstrate. · except: Most upstream examples open with a block comment or none.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
  decision/unsafe-justified · Unsafe blocks annotated — `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`.
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
- S7 · an engine change cites the spec at its implementation and its regression check names the upstream behaviour it changes · design-philosophy · conventions
  decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines.
  decision/regression-tests-cite-upstream · Regression tests cite upstream — Regression tests cite upstream issue or PR URLs.
  decision/spec-url-citations · Spec URLs cited at implementations — Spec URLs are cited above or next to implementations.
