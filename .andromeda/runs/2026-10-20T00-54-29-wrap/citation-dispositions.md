sweep: base: e97a7f41 — the previous sweep (2026-10-19T23-02-04-wrap) · re-pointed 16 · changed 0 · stretched 6

Read: each stretched range at its new numbers in the work tree (its first and last line and the lines this chunk added
inside it — `mod text_extent;` · `pub use text_extent::TextExtent;` in blitz-dom's lib.rs, `mod lint;` · `pub use
lint::{Bounds, Defect, Finding, Reason};` in escher-driver's lib.rs, by the report's new-text listing), and each citing
line by a window at its offset (`cascade.py window`). Every stretched range is a crate root's module or re-export list,
and the chunk's added lines fall inside it, so each range still names the list the claim cites.

stretched .andromeda/architecture.md:48 packages/blitz-dom/src/lib.rs:94-121 → 96-124 — changed inside → holds (96-124: the crate root's `pub use` block, `NodeId` through `NodeTree`, `TextExtent` added inside; the claim: taffy re-exported in blitz-dom's public API)
held .andromeda/architecture.md:128 Cargo.toml:47-58 — refused at 2026-10-13T15-50-37-wrap, not touched since «blitz-dom =…» → holds — stands (Cargo.toml outside this chunk's changed set, `gate.py scope` `changed 6 · listed 6`; read true at 2026-10-19T23-02-04-wrap)
held .andromeda/architecture.md:135 examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (file outside the chunk's changed set)
stretched .andromeda/architecture.md:151@c4535 packages/blitz-dom/src/lib.rs:75-129 → 77-132 — changed inside → holds (77-132: the crate root's public re-exports, `pub use cssom` through `decode_font_bytes`; the claim: «public re-exports from the crate root»)
stretched .andromeda/architecture.md:191@c754 packages/escher-driver/src/lib.rs:113-150 → 119-158 — changed inside → holds (119-158: the private `mod` list and the `pub use` block, `mod lint;` and `pub use lint::{…}` inside; the claim names the re-exports from private modules — the new `lint` re-export is a fact for Validate, not a citation fault)
stretched .andromeda/architecture.md:191@c25040 packages/escher-driver/src/lib.rs:116-150 → 122-158 — changed inside → holds (122-158: the module list and re-exports; the claim: the schema modules carry no `cfg` gate and no serialization crate — unchanged)
stretched .andromeda/architecture.md:324 packages/blitz-dom/src/lib.rs:37-85 → 37-87 — changed inside → holds (37-87: blitz-dom's module declarations, `mod document;` through the last `mod`; `mod text_extent;` added inside — the module list the row enumerates gains `text_extent`, a fact for Validate)
stretched .andromeda/architecture.md:346@c4096 packages/escher-driver/src/lib.rs:115-150 → 121-158 — changed inside → holds (121-158: the re-exports and the module list the row's crate description cites; its listing of what is re-exported gains `lint`'s four types, a fact for Validate)
held .andromeda/security-plan.md:234 .github/workflows/wpt-post-results.yml:29-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Fin…» → holds — stands (file outside the chunk's changed set)
held .andromeda/security-plan.md:351 packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…» → holds — stands (file outside the chunk's changed set)
held .andromeda/security-plan.md:385 examples/screenshot.rs:41-42 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (file outside the chunk's changed set)
held .andromeda/design-system.md:36 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (file outside the chunk's changed set)
held .andromeda/design-system.md:46 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (file outside the chunk's changed set)
held .andromeda/design-system.md:134 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (file outside the chunk's changed set)
held .andromeda/test-plan.md:200 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…» → holds — stands (file outside the chunk's changed set)
held .andromeda/obs-plan.md:41 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (file outside the chunk's changed set)
held .andromeda/a11y-plan.md:12 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (file outside the chunk's changed set)
held .andromeda/a11y-plan.md:16 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (file outside the chunk's changed set)
out-of-range .claude/docs/session-learnings.md:173 host.rs:350 — the file held 231 lines at the base → curation home — P3 (a preserve-verbatim home; `host.rs` names escher-driver's file by basename in a dated learning; carried as at 2026-10-19T23-02-04-wrap)
