
## 2026-10-06-compact-snapshot-serialization — the snapshot's text form reads the model; a file input's value is masked
**Section:** §2 Feature exposure · §7 Accessibility tree output
**Change:**
- Feature exposure: the `accessibility` feature was said to gate the `accessibility_tree` override, the snapshot model and the actionable-key check; now also that model's text form `Snapshot::to_text`, all three named by the crate doc.
- Accessibility tree output: the value mask was a password input's only; now a file input reads the same marker where its `value` attribute is non-empty, its path in no field of the snapshot.
- Accessibility tree output adds that `Snapshot::to_text` is a further reader of the snapshot model, never of the tree or of a control: each line's role and name are the snapshot node's, there is no second role or name mapping, and there is one line per snapshot node; on the stand no line reads `focused` at boot and exactly `back-btn`'s does after a Tab press, in both layout modes.
- 1 line citation re-pointed (`snapshot.rs`); the feature citation now spans the new module's declaration.
**Why:** the chunk built the text form on the snapshot model. The plan listed these as expected amendments and neither a11y detector's invariant covers them, so the wrap raised them itself.
**Ref:** .andromeda/runs/2026-10-06T23-58-14-wrap/
