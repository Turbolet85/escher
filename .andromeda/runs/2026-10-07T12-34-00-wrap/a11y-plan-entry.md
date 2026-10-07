
## 2026-10-07-command-and-refusal-schema — the driver's `disabled` cause is stated on the presence reading
**Section:** §7 Screen Reader Support → Accessibility tree output (the snapshot's `enabled` clause)
**Change:** beside "`enabled` follows the presence of `disabled`", the clause now records that the driver's `disabled` refusal cause is stated on the same presence reading — its meaning reads "the element carries the `disabled` attribute, which the snapshot reads as not enabled" — named and worded only, with nothing yet detecting it; and that the driver's schema defines no role or name set of its own: `role` and `name` are field names of its result nodes.
**Why:** the chunk worded the cause, and a cause worded on the attribute's value instead of its presence would disagree with what the snapshot reads for `disabled="false"`. Rule for later chunks: the entry that detects `disabled` reads the snapshot's `enabled`, never the attribute's value.
**Kept:** no detector of this plan covers the addition; it was raised from the plan's reviewed list.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/
