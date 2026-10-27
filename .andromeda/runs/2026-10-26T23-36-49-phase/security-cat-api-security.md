# security · API Security — category read

## Carried
- (.andromeda/security-plan-detail/api-security.md:8) The MCP tool listing that `the_listing_is_the_twelve_verbs_of_the_table` asserts must stay the twelve driver verbs served over stdio, with `help` answered by the server itself and running nothing on a session; the refactor of that test must keep asserting exactly this surface, adding and dropping no tool.
  > Since 2026-10-10-mcp-surface the `escher-session mcp` role also serves the driver's verbs as MCP tools — twelve: the tenth, `help` (since 2026-10-12-self-description), answered by the server itself from the schema's fixed tables and running nothing on a session, the eleventh, `screenshot`, and the twelfth, `lint` — over its own stdin and stdout
- (.andromeda/security-plan.md:209) The broad line of the same surface: the MCP role must still bind nothing and keep its bounded requests and results after the test refactor, with no new listener or carrier.
  > the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results
- (.andromeda/security-plan-detail/api-security.md:30) Factoring `lint.rs::findings` into helpers or a table walk must keep the `lint` verb's shape: no argument, one result field `findings`, no step run, nothing settled and no time moved.
  > the eighth, `lint`, taking no argument and answering `findings`, running no step and settling nothing, so it moves none either
- (.andromeda/security-plan-detail/api-security.md:31) The refactored lint walk must wait on no load in flight; only the acting verbs wait, bounded by `LOAD_WAIT`.
  > an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot`, `screenshot` and `lint` wait on none
- (.andromeda/security-plan.md:213) No helper the factoring introduces may add a wait argument or a wait result field to any verb.
  > Settle and loads in flight: a harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`, so the absent request timeout cannot hang a settle or a command; no verb has an argument or a result field of a wait.

## Bindings
- **lint walk runs no step and waits on nothing ↔ the body's Input Validation (driver command schema)**: the `lint` verb's no-argument, `findings`-only, no-step shape (.andromeda/security-plan-detail/api-security.md:30) is the same closed twelve-verb table `validate` checks (.andromeda/security-plan.md:77). The refactor of `findings` changes neither side, and the command span keeps fixed words and counts only, never a finding (.andromeda/security-plan.md:79).
- **twelve MCP tools ↔ the body's Input Validation (MCP server)**: the listing the test asserts (.andromeda/security-plan-detail/api-security.md:8) is the MCP server's tool surface, which is typed and then passed through `validate` or `validate_session` (.andromeda/security-plan.md:83). A table-walk rewrite of the test must assert the same rows against `VERBS`.
- **no wait field ↔ the body's API Security broad line**: the detail's settle control (.andromeda/security-plan-detail/api-security.md:31) is summarised by the body (.andromeda/security-plan.md:213). The scope's boundary "No behaviour, verb, field, carrier, log site or span changes" holds both.
