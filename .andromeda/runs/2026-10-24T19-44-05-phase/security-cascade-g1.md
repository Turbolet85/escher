# security · cascade read · group 1 of 4

## Carried
- (.andromeda/security-plan-detail/authentication-authorization.md:30) The chunk edits `test_ci_workflows.py` (a merge surface reached whole) and widens the a11y leg run by a ci.yml job, so ci.yml must keep its workflow-level `permissions: contents: read` with no job-level grant, and the assertion of it in `test_ci_workflows.py` must still stand and pass after the a11y-tuple change.
  > ci.yml declares a workflow-level `permissions: contents: read` and no job-level grant, so every ci.yml job's `GITHUB_TOKEN` is read-only; asserted by `test_ci_workflows.py` (.github/workflows/ci.yml:16-17; .github/scripts/test_ci_workflows.py:201-205)
- (.andromeda/security-plan.md:39) The broad rule the entity details: the "Accessibility (a11y) tests" job, its tier unchanged, must run with a read-only `GITHUB_TOKEN` and take no new grant, environment or secret, however its test list grows.
  > CI tokens are scoped by GitHub Actions `permissions` per workflow — every ci.yml job's `GITHUB_TOKEN` read-only, the publish job repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.

## Bindings
- **authentication-authorization (CI token scope) ↔ the body's supply-chain pins**: the one file `test_ci_workflows.py` holds both the read-only-token assertion (.andromeda/security-plan-detail/authentication-authorization.md:30) and the 40-hex `uses:` pin assertion (.andromeda/security-plan.md:247), so the a11y-tuple edit must leave every one of its CI invariants intact, not only the tuple it moves.
- **authentication-authorization (CI token scope) ↔ the body's lockfile verification**: the widened a11y leg in `ci-leg.sh` runs under the read-only token (.andromeda/security-plan-detail/authentication-authorization.md:30) and must keep passing `--locked` on every cargo command it runs (.andromeda/security-plan.md:264).
