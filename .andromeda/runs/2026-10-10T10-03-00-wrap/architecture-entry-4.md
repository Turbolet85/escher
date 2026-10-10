
## 2026-10-10-driver-cli — `type` replaces; the record refuses a long id; two engine limits measured and said by the tool; the snapshot text and the diff leave in an answer
**Section:** §Standard Contracts → Driver session (the executor, the record of ids, the cause texts, the span) · §Standard Contracts → Dioxus DOM bridge (the snapshot text, the diff)
**Change:**
- `type`: was "clicks … and types"; now it replaces — where the snapshot reads a `value` for the target (an empty text input reads one) the driver presses the engine's select-all, `a` with Control, Super on macOS, then types, and with an empty text deletes the selection, so an empty text clears. Until this chunk the text was appended after the old value (measured), not inserted inside it.
- The executor's `cfg` gates: was one, the macOS backward-delete arm; now two, the second choosing the select-all's modifier.
- The record of ids: was "nothing bounds a recorded id's length … not measured"; now an id longer than `MAX_ID_BYTES` is not recorded, held by a unit test.
- A click naming a scrolled box: was a hypothesis "read from the code, not measured on a click"; now measured in both layout modes — it lands off the box's centre by the scroll offset (shift 76 on the stand at 14 Creates) or is refused `off-screen` while the box is in view.
- `covered`: a control before a scrolling box in the document, lying where a scrolled-out row extends, is refused `covered`; one after the box is not; no control of the stand's CRUD reads it.
- Four fixed texts amended so the tool says both limits and what `type` does: `covered`'s meaning, the help of `changed`, of `snapshot`'s `text` and of `type`. The cause set stays eight.
- The span: a session-level verb handed to `run` records its table word beside `unknown-verb`.
- Dioxus DOM bridge: was "no CLI or MCP command exists and nothing of it crosses the socket", the text and a diff leaving "through the returned value only"; now a hosted answer carries the text and a diff — which the driver, not dioxus-native-dom, writes as JSON — over the socket to stdout; the hosted snapshot line measures 832 to 2,187 bytes on the four tasks. No log, event or file carries either.
**Why:** `type` replacing is the founder's answer (2026-10-10, relayed by the overseer); the record's rule is the operator's at the plan's forks. Both engine limits were measured and left unfixed on the founder's answer — measure and state — and the operator's review added that the tool itself must say them, since an agent learns the tool from the tool. The operator's 2026-10-07 answer that a call runs in process is superseded by the ratified crossing.
**Kept:** both engine defects stand in the engine, pinned by `stand_act_scroll` and `stand_act_obstructed`; each fix keeps a route owner.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/
