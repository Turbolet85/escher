group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «73 inline unit tests in thirteen files»
  source: .andromeda/test-plan.md:21 «- **escher-driver:** 73 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 17 in two `#[path]` test files»
  replace-with: «73 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 17 in two `#[path]` test files»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`schema.rs` 6 (the eleven verbs at their levels»
  source: .andromeda/test-plan.md:21 «schema_tests.rs 6 (`schema::tests`, in schema.rs until 2026-10-16-file-splits; the verbs are named in order, each at its level»
  replace-with: «`schema_tests.rs` 6 (`schema::tests`, in `schema.rs` until 2026-10-16-file-splits; the eleven verbs at their levels»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp.rs` 11 (10 until 2026-10-12-headless-screenshot;»
  source: .andromeda/test-plan.md:21 «mcp_tests.rs 11 (`mcp::tests`, in mcp.rs until 2026-10-16-file-splits; 10 until 2026-10-12-headless-screenshot;»
  replace-with: «`mcp_tests.rs` 11 (`mcp::tests`, in `mcp.rs` until 2026-10-16-file-splits; 10 until 2026-10-12-headless-screenshot;»

- leaf: .claude/rules/testing.md:14   class: stale
  leaf-text: «(one module sits in a file of its own: dioxus-native-dom's `bridge_tests`, a `#[path]` child of `dioxus_document`, so it reaches that module's private fields)»
  source: .andromeda/test-plan.md:137 «five unit-test modules sit out of line, each declared `#[cfg(test)]` with `#[path]` at its parent»
  replace-with: «(five modules sit in files of their own, each declared `#[cfg(test)]` with `#[path]` at its parent: dioxus-native-dom's `bridge_tests`, a child of `dioxus_document`, so it reaches that module's private fields, and, since 2026-10-16-file-splits, moved out whole with their test paths unchanged, blitz-dom's `document::changed_set_tests` and `document::font_reading_tests` and escher-driver's `mcp::tests` and `schema::tests`)»
