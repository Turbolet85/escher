# security · API Security — category read

## Carried
- (.andromeda/security-plan-detail/api-security.md:8) The re-read and the widened `a11y` leg must add no listener, port or served API: the driver session's socket stays the one local listener and the `mcp` role binds nothing, so the chunk's new or re-run rows open no surface.
  > No served network API exists — no TCP or UDP port; the one listener is local: the driver session's Unix-domain socket `{state_dir}/session.sock`
- (.andromeda/security-plan-detail/api-security.md:31) A re-asserted keyboard row that depends on time, such as the flight booker select's typeahead buffer that starts afresh after 1 s of the document's clock, must move time only through the harness's `VirtualClock` (`tick`, `advance_time` or the driver's `advance`), never through wall time or a wait.
  > — on a harness-built document the harness's `VirtualClock`, which only `tick`, `advance_time` and the driver's `advance` move
- (.andromeda/security-plan-detail/api-security.md:31) A session-held witness taken into the leg (`stand_act_select`) must move time on the held instance only by an explicit `advance` within its bound, and must add no wait argument or result field to any verb.
  > time on a held instance moves only when a caller asks for it with `advance`, 1 to 60000 ms a call

## Bindings
- **No new listener ↔ the chunk's scope boundary and arch §Occupied Resources**: the scope's "No carrier, verb, key name, field, log field, span, listener, env read or crate added" holds the API Security line that the session socket stays the one listener; a listener the leg's widening brought in would first need an Occupied Resources registration and a security-plan amendment (.andromeda/security-plan-detail/api-security.md:8)
- **The virtual clock ↔ a11y-plan §9 and test-plan (the stand keyboard and select witnesses)**: a Tab, Enter/Space or select-typeahead row re-asserted or added under the `a11y` leg must stay deterministic by moving only the harness's `VirtualClock`, as the settle control requires, so the leg can stay in its tier with no wall-time wait (.andromeda/security-plan-detail/api-security.md:31)
- **No wait on a verb ↔ the body's settle line**: the body states that "no verb has an argument or a result field of a wait", and the chunk adds no verb or field (.andromeda/security-plan.md:213)
