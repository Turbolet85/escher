# security-plan · Secret Management
Read whole. The master's broad view is .andromeda/security-plan.md; this file holds its Secret Management detail.

## Secret Management

<!-- detail: values · secrets -->

- **Production (CI):** secrets live in GitHub Actions secrets and vars:
  - A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:106-108; .github/workflows/publish-browser.yml:166-168)
  - An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:110-120; .github/workflows/publish-browser.yml:170-172)
  - Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:154-159)
  - A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:108-111)
  - The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)
- **Development (operator host, the cold-agent live run and the 2026-10-10-mcp-surface live trial only):** since 2026-10-10-mcp-surface that chunk's one-shot live trial (`evidence/live-trial.py`, stdlib Python, one `claude -p` with the cold-agent pipe's flags and `--allowedTools mcp__escher`, `claude` 2.1.296 on the dev host) reaches the model provider through the same login — no new secret source and no auth library, on the founder's word for the trial (2026-10-10, «Малое живое испытание сейчас»; recorded at that chunk's wrap on his «Записать по форку 3», 2026-10-11, relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's); the cold-agent pipe's `claude -p` session reaches the model provider through the operator's own Claude Code claude.ai login, held by the `claude` CLI outside the repository — no API key (the session reports `apiKeySource: "none"`), no env read by the pipe (census `os\.environ|getenv|ANTHROPIC_` among its patterns over `scripts/cold-agent.sh`: 0 — the stub is deleted; the pipe sets `RUST_LOG=info` on its server child and reads none), and no CI secret (CI runs only the shim contract tests) (ratified by the founder, 2026-10-06; as measured at escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/report.md); since 2026-10-15-cold-agent-test the pipe drives the driver's own MCP server (`--allowedTools mcp__escher`, `--model claude-opus-5-5`, `--max-budget-usd 3`, `claude` 2.1.296), its one live run fired exactly once through the same login — no new secret source and no auth library — on the founder's «Один запуск, до $3 (Рекомендую)» (the founder, 2026-10-15, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's; confirmed at that chunk's wrap card, «Подтвердить (Рекомендую)»);
 the committed live evidence of both runs holds no key, auth header, credential-file name, token field or e-mail address — the 2026-10-15 credential scan's one hit is the `"apiKeySource":"none"` field, naming no key (as measured at escher-0.1.0/chunks/2026-10-15-cold-agent-test/report.md)

<!-- /detail -->

<!-- detail: absence · secrets -->

- searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
- searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 s03 files
- searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 s04 files; only the formula `Token` enum in cells.rs matched
- searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files; the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:519)
- searched: `env::var|std::env|api_key` over the 17 listed s06 files
- searched: `env::var|secret|token` over the 8 s07 files
- searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (no credential match)
- searched: `std::env|env!\(|getenv` and `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
- searched: `std::env|env::var|env!` over the 32 s10 files (one match, the test below)
- searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files
- searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
- searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files
- searched: `/home/|api[_-]?key|authorization:|bearer ` and two e-mail domains (case-insensitive) over `docs/capabilities.md` and `.github/scripts/test_capability_statement.py` (0 hits); the pin reads no environment variable

<!-- /detail -->
