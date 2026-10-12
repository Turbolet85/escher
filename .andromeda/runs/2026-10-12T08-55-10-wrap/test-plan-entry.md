
## 2026-10-12-self-description — the self-description's checks and counts
**Section:** §1 (escher-driver · apps/seven_guis · dioxus-native-dom · tests/blitz-tests) · §2 Directory pattern · §3 (Crate-local test helpers; agent-run Proof; Stand log format) · §3 → Session lifecycle (`session-binary`, `session-proof`) · §4 escher-driver · §6 (Command line; MCP) · §9 Local baseline
**Change:**
- escher-driver: was 59 unit tests in eleven files, now 66 in twelve — error.rs 5, schema.rs 6, mcp.rs 10, the new describe.rs 2; json.rs still 7, now pinning the remedy and `differs`. §4: ten verbs in three levels, nine field kinds, thirteen node keys with help, the exit statuses, the remedies and the bound, `help` via `validate_session`, and the chunk's eleven mutation controls.
- dioxus-native-dom: 60 → 61 unit tests; the diff's tests pin `differs`.
- seven_guis: `cli_commands` was 8, now 10 (`help`; the 94/95-byte bound; each cause's `remedy`). `mcp_server` was 2, now 4 (the newest revision; `help` before any `start`; `help` the same bytes on both surfaces). Was "the one CLI comparison", now two, both unix-gated.
- blitz-tests: `stand_mcp_tools` 2 → 4 (remedies; `help` parenting no `command`); `stand_mcp_masked` 1; `stand_diff` gains the Book's `differs` = `[bounds]`. Was 108 `.rs` files / 107 targets — stale by `stand_mcp_tools` since mcp-surface — now 110 / 109, measured at this wrap.
- `session_common` readers: was 17, now 18 (`stand_mcp_tools`); `stand_mcp_masked` declares none.
- Proof: `run stand` 127 → 131 over 34 files. §9: 805 → 821 over 171 result lines.
- Stand log format: a `help` call writes one `tool_call` line, `tool="help"`, with no `command` beneath.
- Session lifecycle: `session-binary` ten verbs, the `help` ending, the error object's `remedy`, statuses from `EXIT_STATUSES`; `session-proof` the new counts and CI#38225228282 on `22fc4553` (16/16) as witness.
**Why:** the chunk's new paths carry unit and integration checks at the tiers §2 requires; the readers' count was disproved by measurement, and the target count was re-measured because it was stale.
**Kept:** the live trial's reading (asked `2025-11-25`, answered `2025-06-18`) stands as that trial measured it.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/
