# P5 controls — 2026-11-07-ui-graph

Each fired as the entry itself through `gate.py run --chunk 2026-11-07-ui-graph --docs-root {project root} --entry {n}
--project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/dff806c9-d379-4b95-abfc-3b921eb8e930/scratchpad/clone`,
the clone a `git clone --shared` of the project at HEAD `be4180b83ec3b5838bbfa628df3e01e30c2d982a`.

| entry | clone input | reading |
|---|---|---|
| 11 | unmodified (the three files absent) | green · exit 1 (as on the tree) |
| 12 | unmodified | green · exit 0 · last line 2 (as on the tree) |
| 13 | unmodified | green · exit 0 (as on the tree) |
| 11 | `graph.rs`, `signal_registry.rs`, `stand_graph.rs` each one `//!` line (must pass) | green · exit 1, last line 0 |
| 11 | the same, `graph.rs` also holding `println!` (must fail) | red · exit 0 (one file counted) |
| 12 | a third `info_span!` appended to `packages/escher-driver/src/json.rs` (must fail) | red · last line 3 |
| 13 | a line appended to `Cargo.toml` (must fail) | red · exit 1 |
