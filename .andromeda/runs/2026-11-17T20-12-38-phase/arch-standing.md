# arch · standing conventions

- S1 · each new integration test is its own blitz-tests test target, one file, one behaviour · conventions
- S2 · each new stand or fixture scenario runs in both layout modes · conventions
- S3 · each new test asserts its fixture's precondition first and numbers its fixture cases · conventions
  decision/feature-fixture-numbered-cases · Numbered cases in feature fixtures — Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/numbered-fixture-cases · Numbered fixture cases — Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S4 · each new test is descriptively named, with shared assertion helpers marked track-caller · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
- S5 · shared stand-check helpers and fixtures are stated once, in non-target modules in subdirectories · conventions
  decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
  decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
- S6 · each new expect or unwrap in a test carries its justification, and each new file opens with module docs · conventions
  decision/example-docs · Example header docs — Examples open with a `//!` comment describing what they demonstrate. · except: Most upstream examples open with a block comment or none.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
