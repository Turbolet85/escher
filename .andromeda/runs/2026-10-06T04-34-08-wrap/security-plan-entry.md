
## 2026-10-06-cold-agent-run-pipe — the cold-agent login, input rows and log; PROVISIONAL
**Section:** §Input Validation · §Secret Management (Storage · What counts as secret · NOT YET MEASURED) · §Logging & Monitoring
**Change:**
- §Input Validation: a `CLI arguments (cold-agent.sh)` row — a verb outside `run status cleanup logs`, a wrong argument count, or a task outside the allowlist `counter` prints usage and exits 2 before any precondition or `claude` call; the task reaches the session only as a fixed prompt naming no element id. A `Cold-agent MCP stub` row — −32700 / −32601; refusals `not-found` · `disabled` · `not-pressable` · `malformed`, `isError` with no state change; no argument value logged; argv only.
- §Secret Management: a Development storage bullet — the cold-agent live run reaches the model provider through the operator's own Claude Code claude.ai login held by the `claude` CLI; `apiKeySource` none, no env read, no CI secret, evidence census 0. "What counts as secret" names that login. The development-secret-storage NOT YET MEASURED narrows: the login's holder recorded, its at-rest location unmeasured.
- §Logging & Monitoring: the cold-agent pipe logs counts and identities only; its transcript is raw by design, never printed, gitignored; the one committed copy host-path-masked.
**Why:** a credential path and a new IPC input surface are a security-plan amendment first. Ratified at this wrap by the overseer under the founder's standing delegation as a Boundary widening, PROVISIONAL in the body until the founder's own word.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
