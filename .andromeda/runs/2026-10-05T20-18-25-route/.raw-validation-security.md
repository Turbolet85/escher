# Security validation — route draft

## Rewrite
- `Fork CI reached`: "cached builds, fast checks apart from slow" → "cached builds, fast checks apart from slow, signing and cross-repo-token jobs excluded from the fork"
  Reason: Per security-plan §Secret Management (Storage), the only secrets are the CI signing material and `WPT_GITHUB_TOKEN`, so the fork pipeline must not depend on or expose them before any secret-management work exists.
- `Supply-chain and coverage legs`: "dependency-audit gate and coverage report on fork CI" → "dependency-audit gate, pinned third-party actions, least-privilege workflow tokens and coverage report on fork CI"
  Reason: Per security-plan §Dependency Security (Pinning), `cache-apt-pkgs-action` is referenced at `@latest`, and per §Authentication & Authorization (RBAC table), ci.yml has no workflow-level `permissions` block; both are CI hygiene gaps that belong in the Foundation gate.
- `Snapshot state fidelity`: "disabled reads disabled, typed value reads back" → "disabled reads disabled, typed value reads back, password and file-path values masked"
  Reason: Per security-plan §Data Protection (Local user data handled) and the `Role::PasswordInput` mapping in §Authentication & Authorization, the snapshot sends control values to agent transcripts, CLI JSON and MCP results, so masking must be in the model before diff and act-by-id build on it.
- `Command and refusal schema`: "refusal causes each with a remedy" → "refusal causes each with a remedy, malformed arguments refused at the boundary"
  Reason: Per security-plan §Input Validation, the new CLI and MCP command boundary is not in the validation table, and no validation library is set up, so this boundary needs its own validation.
- `MCP surface`: "local with no network listener" → "local to the invoking user, no network listener, no auth surface"
  Reason: Per security-plan §Bootstrap phases (auth-scaffolding-baseline) and §API Security, the project has no auth and serves no API, so a local-only, no-listener boundary replaces auth scaffolding and should be stated in the chunk's scope rather than added as a separate auth chunk.
