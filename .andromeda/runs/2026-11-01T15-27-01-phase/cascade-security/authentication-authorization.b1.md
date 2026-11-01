### authentication-authorization#1.1 · absence · auth · ## Authentication & Authorization · lines 8–8
- searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 s03 files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)
### authentication-authorization#1.2 · absence · auth · ## Authentication & Authorization · lines 9–9
- searched: `auth|login|session` (case-insensitive, excluding "Authors") over the 86 s04 files
### authentication-authorization#1.3 · absence · auth · ## Authentication & Authorization · lines 10–10
- searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files
### authentication-authorization#1.4 · absence · auth · ## Authentication & Authorization · lines 11–11
- searched: `authenticat|authoriz|login|session|credential` over the 17 listed s06 files
### authentication-authorization#1.5 · absence · auth · ## Authentication & Authorization · lines 12–12
- searched: `auth|session|login|permission` over the 8 s07 files
### authentication-authorization#1.6 · absence · auth · ## Authentication & Authorization · lines 13–13
- searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
### authentication-authorization#1.7 · absence · auth · ## Authentication & Authorization · lines 14–14
- searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
### authentication-authorization#1.8 · absence · auth · ## Authentication & Authorization · lines 15–15
- searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 s10 files (matches only the hyperlink URL component and an "author stylesheets" comment)
### authentication-authorization#1.9 · absence · auth · ## Authentication & Authorization · lines 16–16
- searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)
### authentication-authorization#1.10 · absence · auth · ## Authentication & Authorization · lines 17–17
- searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
### authentication-authorization#1.11 · absence · auth · ## Authentication & Authorization · lines 18–18
- searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files
### authentication-authorization#2.1 · values · auth · ## Authentication & Authorization · lines 24–26
| Requirement | Implementation | Stack reference |
|-------------|---------------|-----------------|
| Token / session storage | With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89) | reqwest cookie store |
### authentication-authorization#2.2 · values · auth · ## Authentication & Authorization · lines 27–27
| RBAC / permissions (CI tokens) | wpt-post-results.yml grants `pull-requests: write`, `actions: read`, `contents: read` (.github/workflows/wpt-post-results.yml:8-11) | GitHub Actions `permissions` |
### authentication-authorization#2.3 · values · auth · ## Authentication & Authorization · lines 28–28
| RBAC / permissions (CI tokens) | wpt.yml grants `contents: read`, `pages: write`, `id-token: write` (.github/workflows/wpt.yml:18-21) | GitHub Actions `permissions` |
### authentication-authorization#2.4 · values · auth · ## Authentication & Authorization · lines 29–29
| RBAC / permissions (CI tokens) | The publish job runs only in `DioxusLabs/blitz` (`if: github.repository == 'DioxusLabs/blitz'`); there it grants `contents: write` and uses environment "Signed Builds" only on main or `ci-test` branches, so no fork ref — the fork's `main`, `ci-test/*`, `build/**` — reaches the environment or its secrets (.github/workflows/publish-browser.yml:37-41) | GitHub Actions repository guard + environment |
### authentication-authorization#2.5 · values · auth · ## Authentication & Authorization · lines 30–30
| RBAC / permissions (CI tokens) | ci.yml declares a workflow-level `permissions: contents: read` and no job-level grant, so every ci.yml job's `GITHUB_TOKEN` is read-only; asserted by `test_ci_workflows.py` (.github/workflows/ci.yml:16-17; .github/scripts/test_ci_workflows.py:201-205) | GitHub Actions `permissions` |
### authentication-authorization#2.6 · values · auth · ## Authentication & Authorization · lines 31–31
| Local IPC access (driver session socket) | No handshake, token, secret or peer-credential check: access is the filesystem's — the state directory is created `0700`, a present one open to its group or to others is refused (`StateDirNotPrivate`), and the socket file is set `0600`, so only the owning user reaches it; `start` does not check that the answering pid is the child it spawned (packages/escher-driver/src/host.rs:91-102; packages/escher-driver/src/host.rs:124) | `std` file modes (unix) |
### authentication-authorization#2.7 · values · auth · ## Authentication & Authorization · lines 32–32
| Local stdio (driver MCP server) | Since 2026-10-10-mcp-surface: no handshake, token, secret or peer check — the server answers only the process holding its stdin and stdout, the MCP client that spawned `escher-session mcp` as the invoking user, and the session it boots lives in that process; it binds nothing, so the socket's `0700` / `0600` modes do not apply (packages/escher-driver/src/mcp.rs:44-112) — the crossing ratified by the founder (2026-10-10, «Ратифицировать как есть») | process stdio |
