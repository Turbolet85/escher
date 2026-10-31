proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "In the changed set's fourth kind of site, the clause on `BaseDocument::apply_select_choice` reads: `BaseDocument::apply_select_choice`, which marks a select where a key, typeahead or a `multiple` select's row click changed its chosen set and nothing for a choice of the chosen set; the rest of the line, its citation included, stands."
    sidecar: "Cross-cutting Patterns, Invalidation and state integrity: `apply_select_choice`'s changed-set site now takes a chosen set and is also reached by a `multiple` select's row click (2026-10-31-engine-debts, F6)."
    rationale: "Report Changes, F6: `apply_select_choice` takes a set and stays the one write path, and `click_select_row`, behind a new `option` arm of `handle_click`, chooses, flips or spans a row, so a pointer now reaches this site where the line names only a key or typeahead and a single chosen option; the report's `multiple-select` row disposes this line as no change, a sweep reading this proposal contests."
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 41
    section: "## Cross-cutting Patterns"
    change: "New item in the Invalidation and state integrity group: - Interaction state on a node made inert is cleared too: where `focus_node_id` resolves to a node reading `is_inert()`, `DocumentMutator::flush` calls `BaseDocument::clear_focus`, firing no DOM `blur` or `focusout`, with no `inert` arm among the attribute writes, so a subtree moved under an inert box is covered as well; `disabled` and `hidden` read as before; ratified by the founder (2026-10-31) (packages/blitz-dom/src/mutator.rs:1051-1063)"
    sidecar: "Cross-cutting Patterns, Invalidation and state integrity: focus on a node made inert is cleared at `DocumentMutator::flush`, with no `blur` or `focusout` (2026-10-31-engine-debts, F3)."
    rationale: "Report Changes, F3: the focus fixup is a new state-integrity rule beside this group's clearing of interaction state on a removed node, line 39 covering removal alone; the report's `focus-fixup` row retires the old limit at `established-decisions.md:113` and elsewhere, and this file states the rule nowhere."
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 33
    section: "## Cross-cutting Patterns"
    change: "New item in the Event processing group: - A task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed`; any app meets it and no engine line moved, and Cells instead calls its handles synchronously in the handler, the native handle acting at the call; measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines, read and not run, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md"
    sidecar: "Cross-cutting Patterns, Event processing: a `MountedData` call in a task spawned from a key handler panics on the document's `RefCell`, a measured hazard (2026-10-31-engine-debts)."
    rationale: "Report Reverted / negative API facts and Decisions & corrections, A hazard outside this chunk's fixes: the panic is measured and applies to any app; the `mounted-hazard` row reads the hazard as a new fact no master states, and this group's `mounted` clause at line 33 is where event processing is described."
note: "The `MountedData` hazard's owner remains P5's to name; this file holds no other drift, no D-retired item and no D-arch-resources or D-arch-decisions occurrence."
