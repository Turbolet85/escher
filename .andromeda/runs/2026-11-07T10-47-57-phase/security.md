@extract_section name=relevance
partial
@extract_section name=constraints
- security-plan §Error Handling (detail error-handling, the "Mounted handles from a spawned task" row) records the `RefCell already borrowed` panic in `DioxusDocument::poll` as an open panic path any app can meet, its fix routed to this entry; the chunk must close it at both render sites (`initial_build`, `poll`) so a `MountedData` call from a task polled during render reaches the document with no panic, in both layout modes.
- security-plan §Error Handling (Graceful degradation) requires `try_doc` to keep returning `None` while the document is mutably borrowed; the buffered apply must leave that degradation and `MountedError::OperationFailed` for missing nodes intact. Whether the apply window still yields `None` to a handle called mid-apply is research's question.
- security-plan §Error Handling (Panic paths, Dioxus nodes) lists `element_to_node_id` unwrapping and `NodeHandle::node`/`node_mut` panicking; replaying buffered mutations must add no new `unwrap`, `expect` or panic path on ids the vdom supplies, and a replay that meets an id the document lacks must not panic where the direct write did not.
- security-plan §Input Validation (Dioxus mutations: `data-dioxus-id` as `usize`, unsupported attribute value types ignored, falsy boolean attributes removed, `dangerous_inner_html` through `set_inner_html`) requires the replayed buffer to pass through the same `MutationWriter` checks, in the same order, as the direct write; a recorder that bypasses or reorders them is a validation change. Whether replay is equivalent is research's question.
- security-plan §Input Validation (the widening rule beside the driver session socket rows) requires the chunk to cross nothing new: no verb, carrier, field, exit or log site; per that rule an engine change for every Dioxus document that opens no carrier is no boundary widening, and the scope records it ratified.
- security-plan §Logging & Monitoring (escher's own sink) bars content-named fields from escher targets; the buffer (ids, tag and attribute names, text, a password's value among it) must not be logged, printed or exposed in a span, and security-plan §Input Validation (Markup attributes, `MASKED_VALUE`) requires a password's or file input's value to stay masked in the snapshot and its diff after the change.
- security-plan §Dependency Security requires that the port add no package to `Cargo.lock` and keep Dioxus at 0.7 (no patch, no vendored crate), so the `audit` leg's graph does not change.
@extract_section name=patterns
- The existing `MutationWriter` (`packages/dioxus-native-dom/src/mutation_writer.rs`), kept byte-identical to upstream at `434e0661`, is the single validated write path for Dioxus mutations (per security-plan §Input Validation, Dioxus mutations rows).
- `try_doc` / `try_element_to_node_id` return `Option` rather than panic, the degradation pattern for borrow-time and missing-node cases (per security-plan §Error Handling, Graceful degradation).
- Tests pin panic guards against stale ids and `unreachable!()` (`stale_node_mapping`, `stale_interaction_state`), the existing witnesses the scope keeps (per security-plan §Error Handling, Graceful degradation).
- Each engine change for every document is recorded with its witness test and "ratified" (per security-plan §Input Validation, the engine-change rows).
@extract_section name=anti_patterns
- Do not add an `unwrap`, `expect` or `panic!` on a vdom-supplied id or attribute in the buffer or its replay (per security-plan §Error Handling, Panic paths).
- Do not emit the buffer's contents (text, attribute values, a masked control's value) to any log, span or stdout (per security-plan §Logging & Monitoring, escher's own sink).
@extract_section name=bindings
- security ↔ tests: the two inverted `dioxus_runtime_probe` pins and the ported `task_polled_during_render_can_access_document` are the witnesses that the §Error Handling panic path is closed; `stale_node_mapping` and the `stand_*` checks keep the §Error Handling degradation and §Input Validation masking unchanged.
- security ↔ obs: the in-process buffer adds no log field or span (security-plan §Logging & Monitoring ↔ obs's sink allowlist).
@extract_section name=acceptance
- `dioxus_runtime_probe`'s two former `should_panic(expected = "RefCell already borrowed")` tests pass as no-panic tests in both layout modes, and the ported upstream test passes (per security-plan §Error Handling).
- grep of the new buffer module and the changed `dioxus_document.rs` hunks shows no new `unwrap`/`expect`/`panic!` on vdom-supplied data and no `tracing`/`log`/`println!` call naming buffer contents (per security-plan §Logging & Monitoring).
- `git diff` of `Cargo.lock` adds no package and `cargo deny --locked --workspace --all-features check advisories licenses` stays green (per security-plan §Dependency Security).
- The snapshot and diff checks that pin `MASKED_VALUE` for password and file inputs still pass unchanged (per security-plan §Input Validation).
