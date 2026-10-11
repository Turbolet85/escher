
## 2026-10-10-mcp-surface — the stand graph re-read; MCP results read the snapshot model
**Section:** §1 (the Dioxus crates bullet) · §7 (the snapshot model's readers)
**Change:**
- §1: re-read with escher-driver's fourth dependency, `serde_json` — one lock line, no package, neither platform adapter in the stand graph (`accesskit_(xplat|winit)` 0 rows).
- §7: an MCP tool result is a further reader of the snapshot model and of nothing else — a `snapshot` result is `to_text`'s output, an acting call's its JSON line with the diff, no second role or name mapping; on the stand a `snapshot` block equals `Session::run`'s screen text byte for byte in both layout modes.
**Why:** the snapshot's text reaches an MCP client by the crossing the founder ratified (2026-10-10, relayed by the overseer; his confirm at this wrap's Validate). No interactive element and no assistive-technology reading entered.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/
