# New groups — security-plan · error-handling

## Verdicts
- fanout-security-plan--cascade-new-error-handling.md 1 · distinct · no standing group states the `MountedData` panic in a task spawned from a key handler; the only other borrow panic stands at line 30, the style traversal's, a different fact
- cascade-compare-security-plan.md 1 · twin · fanout-security-plan--cascade-new-error-handling.md 1 · both state the same `RefCell already borrowed` panic of a `MountedData` call in a spawned key-handler task at `events.rs:162`, with Cells calling its handles synchronously
