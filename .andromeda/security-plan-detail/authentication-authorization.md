# security-plan · Authentication & Authorization
Read whole. The master's broad view is .andromeda/security-plan.md; this file holds its Authentication & Authorization detail.

## Authentication & Authorization

<!-- detail: absence · auth -->

- searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 s03 files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)
- searched: `auth|login|session` (case-insensitive, excluding "Authors") over the 86 s04 files
- searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files
- searched: `authenticat|authoriz|login|session|credential` over the 17 listed s06 files
- searched: `auth|session|login|permission` over the 8 s07 files
- searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
- searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
- searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 s10 files (matches only the hyperlink URL component and an "author stylesheets" comment)
- searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)
- searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
- searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files

<!-- /detail -->

<!-- detail: values · auth -->

| Requirement | Implementation | Stack reference |
|-------------|---------------|-----------------|
| Token / session storage | With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89) | reqwest cookie store |
| RBAC / permissions (CI tokens) | wpt-post-results.yml grants `pull-requests: write`, `actions: read`, `contents: read` (.github/workflows/wpt-post-results.yml:8-11) | GitHub Actions `permissions` |
| RBAC / permissions (CI tokens) | wpt.yml grants `contents: read`, `pages: write`, `id-token: write` (.github/workflows/wpt.yml:18-21) | GitHub Actions `permissions` |
| RBAC / permissions (CI tokens) | The publish job runs only in `DioxusLabs/blitz` (`if: github.repository == 'DioxusLabs/blitz'`); there it grants `contents: write` and uses environment "Signed Builds" only on main or `ci-test` branches, so no fork ref — the fork's `main`, `ci-test/*`, `build/**` — reaches the environment or its secrets (.github/workflows/publish-browser.yml:37-41) | GitHub Actions repository guard + environment |
| RBAC / permissions (CI tokens) | ci.yml declares a workflow-level `permissions: contents: read` and no job-level grant, so every ci.yml job's `GITHUB_TOKEN` is read-only; asserted by `test_ci_workflows.py` (.github/workflows/ci.yml:16-17; .github/scripts/test_ci_workflows.py:201-205) | GitHub Actions `permissions` |
| Local IPC access (driver session socket) | No handshake, token, secret or peer-credential check: access is the filesystem's — the state directory is created `0700`, a present one open to its group or to others is refused (`StateDirNotPrivate`), and the socket file is set `0600`, so only the owning user reaches it; `start` does not check that the answering pid is the child it spawned (packages/escher-driver/src/host.rs:91-102; packages/escher-driver/src/host.rs:124) | `std` file modes (unix) |
| Local stdio (driver MCP server) | Since 2026-10-10-mcp-surface: no handshake, token, secret or peer check — the server answers only the process holding its stdin and stdout, the MCP client that spawned `escher-session mcp` as the invoking user, and the session it boots lives in that process; it binds nothing, so the socket's `0700` / `0600` modes do not apply (packages/escher-driver/src/mcp.rs:44-112) — the crossing ratified by the founder (2026-10-10, «Ратифицировать как есть») | process stdio |

<!-- /detail -->
