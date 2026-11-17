# arch · standing conventions

- S1 · each new or edited public item and module carries its doc comment, and spec or rationale where it states a constant · conventions
  decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
- S2 · each new test has a descriptive name, asserts one behaviour and its fixture's condition first · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
  decision/select-tests-name-element · Select tests name the element — The select tests the driver reads name the element and never the role.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S3 · driver unit tests live in path children of the module they test, at the file's end · existing-scopes
  decision/driver-unit-tests-in-path-children · Driver unit tests in path children — An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
  decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
  decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
  decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
- S4 · every expect or unwrap the new code adds carries its justification; errors are enums · conventions
  decision/error-enums · Error types as enums — Error types are enums with `Display` and a `From` per wrapped error.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
- S5 · driver spans are built by hand, fields set per call site, and any new CI Python test uses unittest on the stdlib · conventions
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
  decision/driver-hand-built-spans · Driver spans built by hand — escher-driver builds its spans by hand, with no `#[instrument]`.
  decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no global subscriber.
