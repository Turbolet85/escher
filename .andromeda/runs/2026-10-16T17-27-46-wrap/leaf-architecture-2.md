group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/escher-driver.md:34   class: omission
  leaf-text: «`execute` carries two macOS ones, the backward-delete arm of `press` and the select-all's modifier.»
  source: .andromeda/architecture.md:257 «since 2026-10-16-file-splits its six shape types `ArgKind`, `ArgSpec`, `FieldKind`, `FieldSpec`, `Level` and `VerbSpec` in its private child spec, re-exported, and its tests in `schema_tests.rs`»
  replace-with: «`execute` carries two macOS ones, the backward-delete arm of `press` and the select-all's modifier. Since 2026-10-16-file-splits `schema` holds its six shape types `ArgKind`, `ArgSpec`, `FieldKind`, `FieldSpec`, `Level` and `VerbSpec` in its private child `spec`, re-exported, and its tests in `schema_tests.rs`.»

- leaf: .claude/docs/services/escher-driver.md:27   class: omission
  leaf-text: «The idle expiry is checked when a request arrives — no timer, no thread.»
  source: .andromeda/architecture.md:257 «since 2026-10-16-file-splits with three private children, each widened to `pub(super)` and no further — framing (the request reader, the id, the protocol errors and the reply writers)»
  replace-with: «The idle expiry is checked when a request arrives — no timer, no thread. Since 2026-10-16-file-splits the `mcp` module has three private children, each widened to `pub(super)` and no further — `framing` (the request reader, the id, the protocol errors and the reply writers), `listing` (the handshake and the tool listing) and `result` (a tool call's arguments read into a call, its result and its written form) — and its tests in `mcp_tests.rs`.»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «`schema.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli.rs` 5 · `client.rs` 2 · `mcp.rs` 11»
  source: .andromeda/architecture.md:115 «escher-driver's `mcp::tests` and `schema::tests` in `mcp_tests.rs` and `schema_tests.rs` (packages/escher-driver/src/mcp.rs:285-287; packages/escher-driver/src/schema.rs:477-479)»
  replace-with: «`schema_tests.rs` 6 · `command.rs` 7 · `execute.rs` 3 · `json.rs` 7 · `cli.rs` 5 · `client.rs` 2 · `mcp_tests.rs` 11»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «`mcp.rs`'s sessions are booted from an empty app»
  source: .andromeda/architecture.md:115 «five unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent»
  replace-with: «`mcp_tests.rs`' sessions are booted from an empty app (`schema::tests` and `mcp::tests` each a `#[cfg(test)]` child in a file of its own, declared through `#[path]` at its parent, since 2026-10-16-file-splits, moved out whole with test paths unchanged)»
