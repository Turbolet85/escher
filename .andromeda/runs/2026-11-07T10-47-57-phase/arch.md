@extract_section name=relevance
relevant
@extract_section name=constraints
- Dioxus stays "0.7.3" with no git dependency, no patch or vendored crate and no pin bumped; upstream `286874cc1` (Dioxus 0.8.0-alpha.3) waits for the next version and only its buffered render is ported onto the 0.7 bridge (per architecture §Project Intent and §Inherited Defaults).
- The MountedData hazard is recorded as a measured hazard with no engine change, its fix owned by this route entry: `DioxusDocument::poll` holding the document's `RefCell` across `render_immediate`. The chunk must turn that decision into an engine change for every Dioxus document, and the wrap must amend the decision (per architecture §Established Decisions → dec-dioxus-integration).
- Mutations reach the DOM only through the existing writer path, and `mutation_writer.rs` is held byte-identical to upstream at `434e0661`. A buffer replaying 0.7's `WriteMutations` calls has to feed that writer, not a second DOM write path. Whether the replay builds the same document as the direct write is research's question (per architecture §Standard Contracts → contract-dioxus-dom-bridge).
- The `mounted` listeners have to keep firing after `initial_build` and after each `poll`, in their present order, so the buffered apply must come before `flush_queued_mounted_events` (per architecture §Cross-cutting Patterns → crosscut-event-processing).
- One incremental pipeline over versioned node ids: both layout modes must produce identical layouts, and the stable element ids and snapshot must not change (per architecture §Design Philosophy).
- The change adds no new workspace crate, env var, listener or public surface. The new buffer module stays inside `packages/dioxus-native-dom` as a crate-private (`pub(crate)`) module (per architecture §Occupied Resources and §Conventions).
@extract_section name=patterns
- Shared state is `Rc<RefCell<…>>`, and borrow conflicts are avoided by taking data out and restoring it. Buffering the render and then taking the borrow once to apply it follows this (per architecture §Cross-cutting Patterns and §Conventions).
- DOM mutations go through `DocumentMutator`, which flushes deferred work on `Drop`. The buffered apply should keep using that one mutator path inside `MutationWriter` (per architecture §Conventions).
- Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file, which is where the ported `task_polled_during_render_can_access_document` belongs in `dioxus_document.rs`. An integration test file opens with a `//!` doc naming its behaviour and runs in both layout modes, so the module text of `tests/blitz-tests/tests/dioxus_runtime_probe.rs` has to be re-said (per architecture §Conventions).
- New modules open with `//!` docs, and an ordering-sensitive block is marked "WARNING: DO NOT REORDER" (per architecture §Conventions).
@extract_section name=anti_patterns
- No 0.8-only parts of `286874cc1`: no stack-machine `WriteMutations`, no `StackWriter`/`BlitzBackend`, no keyboard-types 0.8 move, and no change to `Cargo.toml`'s Dioxus requirement (per architecture §Project Intent).
- The buffer's contents (ids, tag and attribute names, text) must not be logged or printed. No new log field or span, and no `NodeId` crosses the driver boundary (per architecture §Established Decisions → dec-driver-session).
- No `clippy::unwrap_used` or `clippy::expect_used` without a justification next to it in the new module (per architecture §Conventions).
@extract_section name=bindings
- arch ↔ tests: the hazard pin in `dioxus_runtime_probe` is referenced from test-plan §6 E2E Test Strategy (per architecture §Established Decisions → dec-dioxus-integration). Inverting its two `should_panic` tests changes both plans.
- arch ↔ obs: the buffer must not reach telemetry. escher-telemetry's engine-target `SAFE_FIELDS` filter must not be widened (per architecture §Standard Contracts → contract-telemetry-bootstrap).
- arch ↔ all domains: the work stays in the locked crate `packages/dioxus-native-dom`, and no workspace crate is added (per architecture §Occupied Resources).
@extract_section name=acceptance
- `git diff 434e0661 HEAD -- packages/dioxus-native-dom/src/mutation_writer.rs` is empty, or each change is named with its reason (per architecture §Standard Contracts → contract-dioxus-dom-bridge).
- `Cargo.toml` still requires Dioxus "0.7.3", `Cargo.lock` still resolves dioxus-core 0.7.10, and no git dependency or vendored Dioxus crate is added (per architecture §Inherited Defaults).
- At wrap, the dec-dioxus-integration hazard fact is amended from "no engine change" to the buffered render, as an engine change for every Dioxus document (per architecture §Established Decisions).
- `cargo clippy --workspace --locked -- -D warnings` and rustdoc `-D warnings` stay green with the new buffer module (per architecture §Inherited Defaults).
