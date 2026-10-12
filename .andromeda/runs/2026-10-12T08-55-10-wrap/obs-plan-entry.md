
## 2026-10-12-self-description — `help` in the two spans' closed sets; the census; readers eighteen
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand and escher-driver bullets) · §4 Span / Trace Coverage (escher-driver, both spans) · §6 Log Coverage (examples: `escher-session`'s own lines · escher-driver's two line classes)
**Change:**
- §4: `help` joins `tool_call`'s `tool` and `command`'s `verb` closed sets; no field is added (eight and two). A `help` call answered by the server leaves a `tool_call`, ending `accepted`, with no `command` child; an argument to it is refused `malformed`. A `help` handed to `Session::run` is refused `unknown-verb` with its table word recorded. The command line's `help` ending and `describe()` write no record.
- §6: the `command` line was "the verb table's nine words", now ten. A `help` call's `tool_call` line has no `command` beside it. `escher-session`'s endings gain `help` (0 · 1 · 2), its session-error object gains a fixed `remedy`, and its usage line lists `help`.
- §3: was "seventeen of the eighteen checks that hold a session", now eighteen readers of `session_common`, `stand_mcp_tools` among them. `stand_mcp_masked` installs no subscriber, reads no env var and declares no shared module. The module `describe` installs no subscriber, reads no env var and prints nothing; its clock is not stated by the report.
**Why:** no log field was added and no new text reaches a log — the self-description and remedies are an answer on stdout, never a log line; the readers' count was disproved by measurement.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/
