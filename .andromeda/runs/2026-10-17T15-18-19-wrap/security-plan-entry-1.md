
## 2026-10-17-upstream-sync-engine-debts — the merged ARIA reads: `aria-labelledby` names, descriptions to the adapter, three closed values
**Section:** §Input Validation (the `aria-label` · `<label for>` row · a new `aria-describedby` · `aria-description` row · a new `aria-expanded` · `aria-disabled` · `aria-level` row · the `id` row)
**Change:**
- Names row: gains upstream's `aria-labelledby` read — referenced ids resolved to built nodes only, an unknown id ignored, the text gathered cycle-guarded, the computed `label` outranking a `<label>` and an `aria-label` (the fallback when the targets hold no text); upstream's unfiltered `aria-label` read dropped, one trimmed non-empty read left; the text stays inside the ratified name crossing.
- New descriptions row: `aria-describedby`'s targets as `described_by`, `aria-description`'s own free, unbounded text into AccessKit's description field; no log site, print or panic path; the one exit is the platform adapter — the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries it; escher routes no agent- or user-supplied input through it.
- New closed-values row: `aria-expanded` (a boolean), `aria-disabled` (`true` → disabled), `aria-level` (a number, tag and role defaults), each carrying none of the attribute's text, inside the states class; `aria-disabled` moves no snapshot reading.
- `id` row: the adapter crossing gains a fourth content class, descriptions, and its names include `aria-labelledby`'s text.
**Why:** a boundary widening by upstream merge, ratified as upstream's behaviour by the founder's own word at P4 fork 2, by question dialog in the overseer session, relayed by the overseer, and both description sources confirmed by his word on this wrap's answer card, relayed by the overseer; the closed values are a field inside a ratified carrier.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
