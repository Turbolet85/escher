# arch · standing conventions

- S1 · each integration test file covers one behavior and opens with a //! doc naming it; tests first assert the fixture produces the condition; scenarios run in both layout modes; unit tests in a cfg(test) mod tests at the file's end or a #[path] child file · Conventions (conventions file, conv-tests)
  (architecture/established-decisions fct-3oxpzjue) dec-layout
  (architecture/existing-scopes fct-vfh4fl66) scopes
- S2 · stand checks share helpers through modules under tests/ that are no test target; blitz-tests keeps one test target per file · Conventions (conventions file, conv-tests)
  (architecture/conventions fct-cnla3gly) conv-tests
  (architecture/design-philosophy fct-dpohxdsn) philosophy
- S3 · modules and files open with //! docs, public items carry ///, doc comments state the rationale for constants · Conventions (conventions file, conv-documentation)
  (architecture/conventions fct-aqtiieoj) conv-documentation
- S4 · cargo fmt, clippy -D warnings and rustdoc -D warnings over every library crate pass, each run through ci-leg.sh; expect_used and unwrap_used allowed only beside a justification · Conventions (conventions file, conv-formatting-and-lints)
  (architecture/conventions fct-kjxsappk) conv-formatting-and-lints
  (architecture/inherited-defaults fct-77ljaxug) defaults
  (architecture/stack-and-technologies fct-ovms5x7l) stack
- S5 · crate internals pub(crate) or pub(super); consuming builders with_ and without_ · Conventions (conventions file, conv-visibility-and-naming)
  (architecture/conventions fct-h6rtvaqy) conv-visibility-and-naming
- S6 · the per-crate coverage floor and the stand-repeat leg's zero-failure budget hold for new code and stand tests · Infrastructure Patterns, CI/CD; occupied-resources file, res-ci-infrastructure
