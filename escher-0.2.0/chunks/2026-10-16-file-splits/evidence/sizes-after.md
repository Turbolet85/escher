# Sizes after — 2026-10-16-file-splits

Read by the audit's instrument, tokei 14.0.0, Rust code lines, one file per call
(`tokei -o json {file}` → `["Rust"]["code"]`). Read 2026-10-16T17:04Z on the working tree over base `4f393991`.
The "before" column is `research.md` §Conventions to follow (sizes at HEAD, and the spans measured on scratch copies);
the forecast column is `plan.md` §Implementation notes.

| file | before | forecast | after |
|---|---|---|---|
| `packages/blitz-dom/src/document.rs` | 2,793 | about 2,567 | 2,573 |
| `packages/blitz-dom/src/document_changed_set_tests.rs` | — (span 204) | — | 201 |
| `packages/blitz-dom/src/document_font_reading_tests.rs` | — (span 22) | — | 19 |
| `packages/escher-driver/src/mcp.rs` | 1,140 | about 210 | 217 |
| `packages/escher-driver/src/mcp/framing.rs` | — (span 172) | about 172 | 167 |
| `packages/escher-driver/src/mcp/listing.rs` | — (span 80) | about 80 | 91 |
| `packages/escher-driver/src/mcp/result.rs` | — (span 103) | about 103 | 115 |
| `packages/escher-driver/src/mcp_tests.rs` | — (span 579) | about 579 | 579 |
| `packages/escher-driver/src/schema.rs` | 840 | about 417 | 421 |
| `packages/escher-driver/src/schema/spec.rs` | — (span 96) | about 96 | 97 |
| `packages/escher-driver/src/schema_tests.rs` | — (span 327) | about 327 | 324 |

Every driver file of the split is at or under 800 code lines; the largest is `mcp_tests.rs` at 579.
`document.rs` stays the largest file and over 800. Its six code lines over the forecast are the two `#[cfg(test)]` ·
`#[path]` · `mod` declarations. Its lines 1–3461 are byte for byte as at the base. The split files differ from their
spans by their `//!` and `use` lines and by rustfmt's re-wraps after `pub(super)` and the dedent.
