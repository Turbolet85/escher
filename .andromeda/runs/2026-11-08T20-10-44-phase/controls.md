# P5 controls — 2026-11-08-handler-coverage

Each control fired as the entry, `gate.py run --chunk 2026-11-08-handler-coverage --docs-root . --entry {n} --project-root {clone}`,
over a scratch clone, `git clone --shared` of the project at HEAD `8d6c57130342c9c238d8683026d34a999b0dc89b`, at
`$TMPDIR/{scratchpad}/ctl`. Only the verdicts are recorded here; the inputs lived in the clone.

## Entry 8 — the unchanged-paths guard
- Clone, unmodified: green · exit 0 (as on the tree).
- Clone, a line `# control` appended to `Cargo.toml`: red · exit 1 — the guard reads a change inside its paths.

### Re-fired after the review took `mutation_writer.rs` out of the guard's paths (the operator's review, P5)
- Tree: green · exit 0 (`8.2.log`).
- Clone, unmodified: green · exit 0.
- Clone, a line `# control` appended to `Cargo.toml`: red · exit 1.
- Clone, a line `// control` appended to `packages/dioxus-native-dom/src/mutation_writer.rs`: green · exit 0 — the file is outside the guard.

## Entry 9 — the silence census
- Clone, unmodified: green · exit 1 · last line 0, vacuous (the three files absent, as on the tree).
- Clone, the three files minted silent (`//!` doc and one empty item each): green · exit 1 · last line 0 — the must-pass reading on a non-empty input.
- Clone, `println!` added to `packages/escher-driver/src/coverage.rs`: red · exit 0 — the census reads a print site.
