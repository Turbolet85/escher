
## 2026-10-07-refusal-detection — the `disabled` cause is detected
**Section:** §7 Screen Reader Support → Accessibility tree output
**Change:** the driver's `disabled` refusal is detected on the snapshot's presence reading and no other — `Session::run` refuses a `click` or a `type` when the node's `enabled` is `Some(false)`, the first of its three screen-level checks, with nothing dispatched and focus as it was; on the stand `crud-delete` and `flight-return-date` are refused and acted on once they read enabled (was "named and worded only, with nothing yet detecting it").
**Why:** the chunk built the detection. Rule kept: `disabled` is read from the snapshot's `enabled`, never from the attribute's value, from focusability or from a role list of the driver's own.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/
