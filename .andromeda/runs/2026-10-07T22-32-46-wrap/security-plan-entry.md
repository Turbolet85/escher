
## 2026-10-07-driver-command-spans — what the driver logs: one span, eight fixed-word or count fields
**Section:** §Input Validation (the Driver command schema row; the `id` row, three citations) · §Error Handling (the `Refusal` bullet) · §Secret Management (one citation)
**Change:**
- Driver command schema row: was "nothing in the crate prints, logs or fields any of them"; now the crate prints nothing, fields none of `Command`, `Call`, `ArgValue`, `Outcome`, and fields one `tracing` span per call handed to `Session::run` — target `escher_driver`, name `command`, INFO — with eight fields: `verb` (the table's word, never the caller's text), `cause`, `settled`, `busy`, `passes`, `added` · `removed` · `changed`. Never fielded: any argument, `Refusal` or `Fault`, the screen text or its length, the label, the record of ids, `in_view`, `advanced_ms`. No subscriber installed by the crate; no env var and no clock read.
- `Refusal` bullet: was "nothing prints, logs or sends one yet"; now exactly one thing of a refusal reaches a log — its cause's fixed name, as the span's `cause` field, recorded at one site for every refusal `run` returns. No `Fault`, meaning, remedy or `Display` text is fielded.
- Citations into `execute.rs` and the telemetry `lib.rs` re-pointed.
**Why:** the chunk built the span the route entry owed. A cause name holds nothing a call supplied, so the logged value cannot.
**Kept:** "reads no clock" — the timings on a printed line are the sink's. "Not yet an external-input surface" — no socket, CLI or MCP tool reaches `validate`.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/
