
## 2026-10-19-visual-lint — the gated part: the `lint` verb inside the command span, no field
**Section:** §3 Observability Harness Contract · §4 Span / Trace Coverage · §6 Log Coverage → Logged events · §8 PII Scrubbing & Compliance
**Change:**
- §3: `Session::run` holds an eighth instance verb, `lint`. `Outcome::Linted { findings }` is written by `to_json` alone as `{"findings":[…]}`, never logged or fielded.
- §4:
  - `Session::lint` is reached by the `lint` verb inside its call's `command` span and opens none of its own (was «no verb, never handed to `Session::run`»).
  - An accepted `lint` records `verb` alone; a lint's findings and their count are never fielded.
  - `tool_call`'s `tool` is one of twelve words, `lint` added. A `lint` tool call parents one `command` child, measured in process.
- §6:
  - The `command` line's words number twelve, eight instance-level (was eleven and seven). The «ten until … nine until» history is dropped.
  - A lint's line is `span="command" verb="lint"` and holds no finding; `host_log` and `mcp_log` each drive one `lint` with 0 needles.
  - `tool_call`'s words number twelve.
- §8: the verb table holds eight instance verbs with `lint`, and `stand_act_spans` reads its lines (34 calls per mode).
**Why:** F2 is the founder's own word, «Без поля (Рекомендую)» (no new span field), with F1 «Новая команда lint (Рекомендую)». Both were given by question dialog in the overseer session and relayed verbatim by the overseer (2026-10-20, chunk inputs#I7). His card confirm is inputs#I11.
**Ref:** .andromeda/runs/2026-10-20T07-57-08-wrap/
