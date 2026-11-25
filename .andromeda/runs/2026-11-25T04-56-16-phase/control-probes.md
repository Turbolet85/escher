# P5 known-positive control — 2026-11-25-virtual-nodes-as-elements, gate entry 1

Entry 1 (`git diff --quiet 739132d5fce08e0f08cd5287587c70b65bfd94c9 -- {the seven manifests and the lock}`), fired as
the entry through `gate.py run --chunk 2026-11-25-virtual-nodes-as-elements --docs-root . --entry 1 --project-root
$TMPDIR/{scratchpad}/p5-clone-probe`, over a `git clone --shared` of the project at HEAD `739132d5f`. The clone is left
in place; no plan step or P5 step deletes it.

| firing | the clone's change | reading |
|---|---|---|
| 1 | none | green · exit 0 (as on the real tree) |
| 2 | a line appended to `packages/dioxus-native-dom/src/snapshot.rs` | green · exit 0 (a source edit passes) |
| 3 | and a line appended to `packages/escher-driver/Cargo.toml` | red · exit 1 (a manifest edit is caught) |
