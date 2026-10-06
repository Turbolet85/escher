# security extract

## Relevance
partial — a docs-only rewrite of the root `README.md`; no new surface, input path, dependency or secret. The security share is what the public front page may disclose and what security posture it may claim.

## Constraints
- The README carries no secret: none of the material security-plan §Secret Management "What counts as secret" names may appear, whether CI signing material, the `WPT_GITHUB_TOKEN` / `GITHUB_TOKEN` tokens or the operator's Claude Code login. Any workflow, secret or var named in a "try it" or CI line is named only, never given a value (per security-plan §Secret Management §Storage).
- If the README mentions the cold-agent run, it describes the run's auth as security-plan §Secret Management §Storage (Development) states it: the operator's own `claude` CLI login, no API key, no env read. It gives no instruction to set an API key or credential env var, and it names no credential file. That bullet is PROVISIONAL pending the founder's word, so the README must not present it as settled policy.
- The README makes no security-hardening claim the plan does not support. It must not say the engine is safe for untrusted URLs, sandboxed, origin-isolated or size- or time-bounded. security-plan §API Security records that the request size limit, request timeout, CORS/same-origin and CSP are absent. security-plan §Input Validation (the `file:` URLs row) records that `file:` reads have no path restriction. This is the security side of the chunk's "never invent capability" rule.
- The README's "Why it exists" and "plans" lines may say that no served API, listener or port exists, because security-plan §API Security records it. Any planned driver (CLI + MCP) must be described as planned, not built, and the README must not imply that a network-reachable endpoint exists or is coming. A new socket or port is a registration first, per that section's "No served API surface" finding.
- The README does not advertise signed or published escher binaries. security-plan §Dependency Security §Supply chain integrity (Signed artifacts) records that the fork produces no signed artifact, because the publish job is repository-guarded to `DioxusLabs/blitz`. Upstream download or release links that the README keeps must be attributed to upstream Blitz.
- The operator gave the contact address on the entry for a public page. It is not a secret under security-plan §Secret Management "What counts as secret". It belongs in the README's Author / Contact section only.
- No host path, local socket path or `target/` artifact path from the operator host enters the README. This follows the host-path masking that security-plan §Logging & Monitoring (cold-agent pipe bullet) applies to committed copies.

## Patterns to follow
- State absences as recorded facts, not as guarantees. The plan reads "observed absent" and "NOT YET MEASURED". The README's honesty split between built and planned should mirror that, and should not upgrade "not measured" into "secure" (security-plan §Threat Model Summary attack-surface vectors, §API Security).
- Name commands, never values. Where the README shows `bash scripts/agent-run.sh boot` / `run stand` or `just seven_guis`, it gives the verb grammar security-plan §Input Validation (the agent-run.sh and cold-agent.sh rows) records. It gives no credential, token or env value.
- Keep upstream's credit and its dual Apache-2.0 / MIT licence statement, and `stylo_taffy`'s MPL-2.0 licence, as the scope's Boundaries require. The security plan's licence-compliance item reads NOT YET MEASURED, so this extract adds no licence rule.

## Anti-patterns to avoid
- Pasting a token, key, credential-file name, auth header or env value into the README, including in a "try it" snippet or a CI badge URL (security-plan §Secret Management §Storage, "Never in code").
- Advertising capabilities the plan records as absent, such as "secure by default", "sandboxed", "safe on untrusted content", "rate-limited" or "authenticated API", or treating the planned driver as a live network service (security-plan §API Security, §Authentication & Authorization "Auth approach: none observed").
- Linking upstream's signed or published browser builds as escher's own (security-plan §Dependency Security §Supply chain integrity).

## Contract bindings
- security ↔ tests: if P4 adds a hygiene check on the README (for a credential pattern, a host path or a security-hardening claim), it binds to the existing evidence-hygiene census the plan records. That census, in security-plan §Secret Management §Storage (Development), counts keys, auth headers, credential-file names, token fields and e-mail addresses. The README's contact address is the operator-given exception, scoped to `README.md`. The chunk's committed evidence should not copy the address into evidence files, so that census stays clean.
- security ↔ arch: a README line that describes the driver's future transport must agree with arch §Occupied Resources. That table registers no port or listener. security-plan §API Security records "No served API surface".

## Acceptance criteria contributions
- A credential grep over `README.md` finds no secret value. The grep is case-insensitive over `token|secret|password|api[_-]?key|authorization|BEGIN .*PRIVATE KEY` plus the named CI secrets. Its hits are allowed only as prose names with no value, and the e-mail address appears only in the Author / Contact section (per security-plan §Secret Management "What counts as secret").
- `README.md` contains no host path from the operator host: 0 hits for the home-directory prefix and for the Claude session temp-dir prefix (`claude-` under the OS temp dir) (per security-plan §Logging & Monitoring, cold-agent pipe bullet's host-path masking).
- `README.md` makes no claim of a served API, sandboxing, origin isolation or request limits. Any mention of the driver (CLI + MCP) sits under "planned" wording (per security-plan §API Security).
- The chunk's diff touches only `README.md`: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every workflow are unchanged, so the dependency pinning and the audit leg's inputs stand (per security-plan §Dependency Security, Pinning and CI integration).
