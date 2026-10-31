# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · standing S7 · fmt, clippy and rustdoc -D warnings through ci-leg.sh, with expect_used and unwrap_used next to a justification
conventions#2.1 · no · the chunk edits no Cargo.toml and bumps or adds no dependency, so the workspace-dependency and package-metadata rules reach nothing it changes
conventions#3.1 · no · standing S6 · new code carries //! and /// docs and cites the spec URL above or next to the implementation
conventions#4.1 · no · standing S12 · a known gap left in code is marked TODO or FIXME
conventions#5.1 · no · standing S12 · new crate internals stay pub(crate) or pub(super)
conventions#6.1 · no · standing S11 · a new engine log line gates tracing per call site with a cfg(not(feature = "tracing")) fallback
conventions#7.1 · no · the task names no unsafe block, no new ElementData or DocumentData field and no borrow workaround, so nothing the chunk writes must meet these shapes
conventions#8.1 · yes · the exception it records, pseudo_element_flow_classification asserting in neither layout mode, is the run the scope edits to for incremental in [false, true], and tests/all.rs is the held target it names
conventions#9.1 · no · standing S4 · unit tests sit in a #[cfg(test)] mod tests at the end of the source file or in a #[path] child file of their own
conventions#9.2 · no · standing S3 · a shared fixture page, boot or hold lives once in tests/blitz-tests/tests/fixtures/mod.rs, and a page with one reader stays in its check's file
conventions#9.3 · no · the chunk adds no row to the 0.2.0 requirement sweep or the MCP listing test, the two tests whose row walk it describes, and reaches neither file
conventions#10.1 · no · the chunk edits no app, example or binary, and its rows read the served stand pages without changing them
conventions#11.1 · no · standing S5 · a new HTML fixture follows the fixture layout: numbered cases under headings, a CSS comment per case, and ./-relative sibling assets
