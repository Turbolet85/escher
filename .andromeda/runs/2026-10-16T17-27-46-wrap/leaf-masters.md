group: masters
read: CLAUDE.md · 147 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:80   class: stale
  leaf-text: «`packages/escher-driver/src/{schema,command,refusal,describe}.rs` (unit tests inline)»
  source: .andromeda/architecture.md:257 «its six shape types `ArgKind`, `ArgSpec`, `FieldKind`, `FieldSpec`, `Level` and `VerbSpec` in its private child spec, re-exported, and its tests in `schema_tests.rs`)»
  replace-with: «`packages/escher-driver/src/{schema,command,refusal,describe}.rs` (unit tests inline, bar schema's in `schema_tests.rs`; its six shape types in its private child `schema/spec.rs`, re-exported)»

- leaf: CLAUDE.md:85   class: stale
  leaf-text: «`packages/escher-driver/src/mcp.rs` (unit tests inline)»
  source: .andromeda/architecture.md:257 «with three private children, each widened to `pub(super)` and no further — framing (the request reader, the id, the protocol errors and the reply writers), listing (the handshake and the tool listing)»
  replace-with: «`packages/escher-driver/src/mcp.rs` and its three private children `mcp/{framing,listing,result}.rs` (unit tests in `mcp_tests.rs`)»
