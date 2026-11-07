# Controls — 2026-11-07-mounteddata-buffered-render, P5 check 4 (9)

Each guard entry fired AS THE ENTRY through `gate.py run --chunk 2026-11-07-mounteddata-buffered-render --docs-root .
--project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/432d3584-bb64-4606-96c9-c2627454c102/scratchpad/ctl
--entry {n}` over a `git clone --shared` of the project at HEAD `de5c176b69b0484b01536c41940e3444faf071ed` (the script
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/432d3584-bb64-4606-96c9-c2627454c102/scratchpad/controls.sh`).
Every entry first fired unmodified in the clone and read as it read on the real tree; each input was then made in the
clone and reverted after its firing; the clone ended with 0 changed paths.

| entry | clone role | input | reading | wanted |
|---|---|---|---|---|
| 4 | unmodified | — | red · exit 1 ✗ (exit 0), 2 B | red, as on the tree (2 `should_panic`) |
| 4 | control | both `#[should_panic(expected = "RefCell already borrowed")]` lines removed from the probe | green · exit 1 | green |
| 5 | unmodified | — | green · exit 0, 3 B | green, as on the tree (10) |
| 5 | control | one `#[test]` line removed from the probe | red · last line '10' ✗ (last: '9') | red |
| 6 | unmodified | — | green · exit 0 | green, as on the tree |
| 6 | control | `// control` appended to `packages/dioxus-native-dom/src/mutation_writer.rs` | red · exit 0 ✗ (exit 1) | red |
| 7 | unmodified | — | green · exit 0 | green, as on the tree |
| 7 | control | `# control` appended to `Cargo.toml` | red · exit 0 ✗ (exit 1) | red |
| 8 | unmodified | — | red · exit 1 ✗ (exit 2), the file absent | red, as on the tree |
| 8 | control | a silent `mutation_buffer.rs` (`//! control` and a unit struct) | green · exit 1 | green |
| 8 | control | the same file with a `trace!("control");` line | red · exit 1 ✗ (exit 0) | red |
| 8 | control | a file holding `None::<u8>.unwrap()` | red · exit 1 ✗ (exit 0) | red |
| 9 | unmodified | — | green · exit 1 (no diff yet) | green, as on the tree |
| 9 | control | `// control` appended to `dioxus_document.rs` | green · exit 1 | green |
| 9 | control | `    trace!("control");` appended to `lib.rs` | red · exit 1 ✗ (exit 0) | red |
