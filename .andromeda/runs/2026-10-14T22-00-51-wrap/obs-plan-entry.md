
## 2026-10-14-stand-requirement-sweep — the census over three new files; the workspace host's engine line printed twice
**Section:** §3 Logging stack (the headless-stand census bullet) · §8 Values logged as-is · §6 Logged events (the `tool_call` bullet) · §2 Feature wiring
**Change:**
- §3: `common/mod.rs` readers 19 → 21 and `session_common/mod.rs` 20 → 21; `stand_requirement_sweep`, `stand_non_lean_tasks` and `password_caret_click` install no subscriber, read no env var and print nothing, the silence census reading 0 over them and the four extended files.
- §8, §6, §2: re-read on the same flows — the workspace-built host writes 6 lines / 978 B, the `blitz_dom::document` `focussed node` INFO line printing twice (was 5 / 860 B, once); the `mcp` role 11 lines workspace-built (was 10); built alone unchanged (4 / 742 B; 9); needles 0 at `info` and `trace` under both builds. The dated readings stand as dated, the re-reads appended.
**Why:** measured at this chunk. No log site, field or span added — the second line is the engine's existing event firing once more; why it fires twice is recorded, not established. The C16 CARRY asked for the re-read.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/
