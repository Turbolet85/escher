# Obs validation — route draft

## Rewrite
- `File splits`: "`mcp.rs` and `schema.rs` along their seams" → "`mcp.rs` and `schema.rs` along their seams; `tool_call` and `command` spans keep their fields and nesting"
  Reason: Per obs-plan §4, the `tool_call` span is opened in `packages/escher-driver/src/mcp.rs:537-540` and parents each `command` span, so a split that moves its open/enter site has to keep the span's two closed-word fields and the parent link.

- `Upstream sync ahead of engine debts` (and the three later upstream syncs): "tests and CI prove our logic survived" → "tests and CI prove our logic survived; escher's sink allowlist still drops or redacts every new engine target and field"
  Reason: Per obs-plan §2 (Feature wiring) and §8 (Scrubbing), a workspace build sends engine events into escher's sink, so any target or field upstream adds has to be re-checked against the allowlist, as was done at 2026-10-13-upstream-sync-polish-and-ship.

- `Session recording`: "kept as a replayable check" → "kept as a replayable check, a file outside escher's sink; no recorded argument, id or typed text logged or fielded"
  Reason: Per obs-plan §4 (never fielded: any argument, `Command`, `Call`, `Outcome`) and §8 (scrub set holds `text`, `value`), a recording holds the very content the sink must never print, so it has to stay out of the telemetry path.

- `Visual lint`: "each by stable id with its measured reason" → "each by stable id with its measured reason; findings answered, never fielded on its command span"
  Reason: Per obs-plan §4, the `command` span records only the verb-table word and closed class words and never ids or screen content, so a new lint verb's findings must reach the answer only and not the span.

- `Load wait`: "waits it out within a stated bound, on a fixture" → "waits it out within a stated bound, on a fixture; a wait past its bound named by a closed busy or cause word"
  Reason: Per obs-plan §4, the `command` span already records `busy` and `cause` from closed sets (nine causes today), so a new bounded wait that times out needs a closed word there or it leaves no telemetry trace.

- `Stand requirement sweep`: "every 0.2.0 capability proven headless by an agent on the stand or a fixture beside it" → "every 0.2.0 capability proven headless on the stand or a fixture; new verbs' stderr by level holds 0 id/name needles under both builds"
  Reason: Per obs-plan §2 (Feature wiring, measured by level at 2026-10-14-stand-requirement-sweep) and the pii-scrubbing-wire contract (§3 Bootstrap phases), the sink's no-id/no-name guarantee is measured per surface, so 0.2.0's new verbs (lint, select, paging, recording) need that same census in Polish.
