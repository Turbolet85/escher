# security · Authentication & Authorization — category read

## Carried
- (.andromeda/security-plan-detail/authentication-authorization.md:30) Widening the `a11y` leg and its pin must leave ci.yml's workflow-level read-only token as it is: the "Accessibility (a11y) tests" job takes no job-level `permissions` grant, `test_ci_workflows.py` keeps asserting it, and a chunk edit to `test_ci_workflows.py` that shifts the assertion's lines re-points this row's `(.github/scripts/test_ci_workflows.py:201-205)` citation (and `ci.yml:16-17` if ci.yml's header moves).
  > | RBAC / permissions (CI tokens) | ci.yml declares a workflow-level `permissions: contents: read` and no job-level grant, so every ci.yml job's `GITHUB_TOKEN` is read-only; asserted by `test_ci_workflows.py` (.github/workflows/ci.yml:16-17; .github/scripts/test_ci_workflows.py:201-205) | GitHub Actions `permissions` |
- (.andromeda/security-plan.md:39) The body's broad line for the same control: every ci.yml job, the a11y job included after the leg widens, keeps a read-only `GITHUB_TOKEN`.
  > CI tokens are scoped by GitHub Actions `permissions` per workflow — every ci.yml job's `GITHUB_TOKEN` read-only
- (.andromeda/security-plan.md:41) The re-read adds no auth surface: the driver socket and the MCP stdio server stay without handshake, token or peer check, access the filesystem's and the spawning process's, so any session-holding check the leg takes in (`stand_act_select`) runs on these unchanged and the chunk adds no listener or credential path.
  > The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check: the socket's access is the filesystem's (state directory `0700`, a group- or other-open one refused, socket `0600`), and `start` does not check the answering pid; the MCP server answers only the process holding its stdin and stdout and binds nothing.

## Bindings
- **CI token scope ↔ tests plan §CI Integration (the a11y leg and job)**: the leg's widened target list in `.github/scripts/ci-leg.sh` and the `LegScriptTest` a11y tuple move, while the job keeps its tier, no required status check and no job-level `permissions` grant; the permissions assertion in `test_ci_workflows.py` stays green and its cited line range is re-read after the edit (.andromeda/security-plan-detail/authentication-authorization.md:30)
- **CI token scope ↔ the body's Authentication & Authorization line**: the detail row and the body's broad line state one control; a citation re-point in the row leaves the body line unchanged (.andromeda/security-plan.md:39)
- **No new auth surface ↔ arch §Occupied Resources / security plan §API Security**: the scope's "no listener, env read or crate added" holds the socket and stdio access model as recorded (.andromeda/security-plan.md:41)
