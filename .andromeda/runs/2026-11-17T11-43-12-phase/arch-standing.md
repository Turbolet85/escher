# arch · standing conventions

- S1 · every taken upstream integration test stands in blitz-tests as one behaviour per file and one test target per file · conventions
  decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
- S2 · every new or taken regression test names its behaviour descriptively and cites the upstream change it guards · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/regression-tests-cite-upstream · Regression tests cite upstream — Regression tests cite upstream issue or PR URLs.
- S3 · every action a CI job step names is pinned to a 40-hex commit SHA and the workflow keeps read-only permissions · stack-and-technologies · infrastructure-patterns
- S4 · every CI workflow change is held by the CI workflow contract tests, and cargo legs run locked · standard-contracts
  decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
  decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
- S5 · every dependency line is declared once in the workspace and inherited by member crates · conventions
  decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency.
  decision/workspace-shared-dependencies · Shared workspace dependencies — The Cargo workspace declares its dependencies once in a shared workspace dependency table.
