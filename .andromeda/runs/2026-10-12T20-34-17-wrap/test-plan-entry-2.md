
## 2026-10-12-headless-screenshot — the harness and session contracts gain the shot; §3 → session-lifecycle
**Section:** §3 Harness Core · Crate-local test helpers (stand checks · session checks) · Stand log format · Agent-run contract → Proof · §3 → Session lifecycle (session-start · session-wire · session-binary · session-proof) · §5 Session ↔ held instance · Session host ↔ what it writes · §6 Command line · MCP
**Change:**
- Harness Core: `screenshot` / `screenshot_rect`, `PixelRect`, `Raster`, `ScreenshotError::{FontsUnavailable, EmptyRect}` — at the harness's own time, `VelloCpuImageRenderer`, white composite, clip and `EmptyRect`, no font refused with nothing painted, no print, panic or log.
- Helpers: six of the thirteen driver-action checks read `mod common;` (17 readers); `session_common` gains `screenshot()`, `screenshot_of(id)` and `shot(session, call)`, `act` reading `None` for a shot; was 18 readers, now 20.
- Stand log format: `screenshot` joins `command.verb` and `tool_call.tool`, no field added; a shot's line holds no image byte (0 at `trace`, host and `mcp` role, both builds) — in step with obs-plan §4.
- Proof: `run stand` was 131 / 0 / 6 over 34 files; now 139 / 0 / 7 over 36.
- session-start: `run` takes `screenshot` and returns `Outcome::Shot { width, height, png }` — no diff, nothing changed; id refused as `click`'s, a crop of no pixel `off-screen`, no font `fonts-unavailable`. session-wire: a shot's PNG crosses in band as padded base64 in its answer line, the bound unchanged. session-binary: was ten verbs, six instance-level; now eleven, seven. session-proof: `cli_commands` 11, `mcp_server` 5, `stand_mcp_tools` 5, `stand_screenshot_repeat` 1, thirteen driver-action files 46, the crate 72; CI 38297557417.
- §5: a shot through `Session::run` changes nothing; nine causes; `host_log` / `mcp_log` read no image byte. §6: `cli_commands` 11 (seven instance verbs, the shot's `png` equal in process); `mcp_server` 5 (eleven tools, image then text), `stand_mcp_tools` 5.
**Why:** the in-band image on the wire is the founder's fork 1, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's answer card confirmed the same way, 2026-10-13); the rest is the chunk's counted tests.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/
