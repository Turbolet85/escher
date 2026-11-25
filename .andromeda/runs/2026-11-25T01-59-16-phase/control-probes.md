# P5 controls — probes 1 to 3 (2026-11-25-virtual-nodes-declared)

Fired as the entries themselves through `gate.py run --chunk 2026-11-25-virtual-nodes-declared --docs-root . --entry {n}
--project-root $TMPDIR/{scratchpad}/p5-clone`, no `--run-dir`. The clone is `git clone --shared` of the project at HEAD
`b7f61dd0afe743249920a38dd596f0c4a620da66`, in the session scratchpad. It is left in place; no step removes it.

| input in the clone | entry 1 (manifests, lock) | entry 2 (blitz-dom lines removed) | entry 3 (15 implementor files) |
|---|---|---|---|
| unmodified (the clone reads as the tree) | green · exit 0 | green · exit 0 · last 0 | green · exit 0 |
| must-pass: one line appended to `packages/blitz-dom/src/node/custom_widget.rs` and to `packages/blitz-dom/src/node/mod.rs` (added lines only, step 1's shape) | green · exit 0 | green · exit 0 · last 0 | green · exit 0 |
| must-fail: one line appended to `Cargo.toml`; the line `pub use crate::node::Widget;` in `packages/blitz-dom/src/lib.rs` changed in place; one line appended to `examples/custom_widget.rs` | red · exit 1 | red · last line '0' ✗ (last: '1') | red · exit 1 |

Each entry reads both sides: green on the input step 1 makes, red on the input its bound forbids.
