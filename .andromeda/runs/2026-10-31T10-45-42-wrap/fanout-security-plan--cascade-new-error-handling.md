proposals:
  - detector: D-new-detail
    severity: warning
    fact: F12
    file: .andromeda/security-plan-detail/error-handling.md
    near: 30
    section: "## Error Handling"
    change: "- Mounted handles from a spawned task: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed` (packages/dioxus-native-dom/src/events.rs:162); measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines, read and not run; any app can meet it and no engine line moved; the 7GUIs Cells task tried that form and did not ship it, calling its handles synchronously, `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "Error Handling gains the hazard no standing text stated: a `MountedData` call in a task spawned from a key handler panics with `RefCell already borrowed`; Cells calls its handles synchronously (2026-10-31-engine-debts)."
    rationale: "The report's Reverted / negative API facts, Deviations 2 and Decisions & corrections (A hazard outside this chunk's fixes) measure the panic at `events.rs:162` and state that the async form did not ship; its Spec claims disproved records that no master states the form (pattern `mounted-hazard`)."
note: "The report writes the panic site as bare `events.rs:162`; the path packages/dioxus-native-dom/src/events.rs is taken from the body's own citations of `try_doc` and `NodeHandle` at that file's :155-177, not from a source read."
