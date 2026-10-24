# security · picked accounting · group 4 of 4

## Picked
api-security#1.0 · not in scope · the chunk binds no listener and adds no socket or MCP check to the leg (stand_act_select runs in process); its rule lives in host.rs and mcp.rs, which the change does not reach, so it is only the re-check every change owes
api-security#2.6 · not in scope · the chunk adds no verb, argument, result field or settle code and the stand boots offline, so no load is in flight; the settle and LOAD_WAIT rule lives in settle.rs and the executor, which are not reached — only the re-check every change owes
dependency-security#1.0 · carried · the audit leg's `--locked` whole-workspace invocation sits at ci-leg.sh:38 beside the a11y leg line the chunk widens, and test_ci_workflows.py pins its words — both are merge surfaces
dependency-security#5.1 · carried · stand_contrast's re-read rows must use the contrast reader as it is, with no package entering Cargo.lock and no dependency added to blitz-tests
dependency-security#6.1 · carried · the licence gate runs in the same audit leg line (ci-leg.sh:38) next to the edited a11y line, and the audit tuple test_ci_workflows.py pins asserts it
secret-management#1.1 · carried · test_ci_workflows.py, a merge surface, asserts the repository guard on the jobs that reach these secrets and that ci.yml holds no secret reference; the a11y tuple edit must leave both intact
error-handling#1.1 · not in scope · the chunk writes no refusal, cause, fault or written form, and stand_act_select joins the leg unchanged; refusal.rs and error.rs are not reached, so it is only the re-check every change owes
logging-monitoring#1.1 · not in scope · the chunk adds no log site, field, span or binary that installs the sink, and the leg's tests install no sink; escher-telemetry and the seven_guis binaries are not reached, so it is only the re-check every change owes
