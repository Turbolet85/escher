# arch · standing conventions

- S1 · each new test sits in a cfg(test) tests module at the end of its source file or a path child file; each integration test file covers one behavior, opens with a doc comment naming it, runs its scenario in both layout modes, and first asserts the fixture produces the condition · conventions
  (architecture/conventions fct-iemqu3j3) conv-tests
  (architecture/design-philosophy fct-dpohxdsn) philosophy
  (architecture/existing-scopes fct-5fs3477c) scopes
  (architecture/existing-scopes fct-vfh4fl66) scopes
- S2 · each new stand fixture or shared page is stated once in tests/blitz-tests/tests/fixtures/mod.rs, a page with one reader staying in its check's file, and stand checks share helpers through non-target modules under tests · conventions
  (architecture/conventions fct-cnla3gly) conv-tests
- S3 · each new module or file opens with a doc comment, public items carry doc comments, and constants state their rationale · conventions
  (architecture/conventions fct-aqtiieoj) conv-documentation
- S4 · each new line passes cargo fmt, clippy with -D warnings and rustdoc -D warnings through the ci-leg.sh legs, with expect_used or unwrap_used allowed only beside a justification · conventions
  (architecture/conventions fct-kjxsappk) conv-formatting-and-lints
  (architecture/inherited-defaults fct-77ljaxug) defaults
  (architecture/stack-and-technologies fct-ovms5x7l) stack
- S5 · each new span or log line in escher-driver is built by hand with tracing ungated, no instrument attribute, carrying no id, name, value or read content · cross-cutting-patterns
  (architecture/cross-cutting-patterns fct-4onw7te5) crosscut-logging-and-timing
  (architecture/inherited-defaults fct-fgkqqy76) defaults
- S6 · each new crate-internal item is pub(crate) or pub(super), and each new error type is an enum with Display and From per wrapped error · conventions
  (architecture/conventions fct-h6rtvaqy) conv-visibility-and-naming
