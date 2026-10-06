# Security Summary — escher

_Distilled from `.andromeda/security-plan.md` (adopted reading — current truth, not intent). wrap-session's cascade re-derives it when the plan changes._

## Posture

escher is a native-API engine library with no served API, no listener and no authentication. Its risk is in what it ingests: arbitrary remote HTML/CSS/images/fonts/sub-documents, JavaScript run by `blitz-vibey-script`, and CLI arguments of the examples. The application-level threat model is NOT YET MEASURED.

**Tier:** 0
**Auth approach:** none (observed absent in every slice)

## Threat model highlights (attack surface as measured)
- **Remote web content** → iframe depth cap 10, `@import` depth cap 16, stale iframe responses discarded by request id; snapshotting a never-styled node is skipped (Stylo would panic).
- **Script execution** → default `ScriptFetcher` accepts only `file:` / `data:`; JS `fetch()` is GET/HEAD only through the embedder's fetcher; same-origin / CORS observed absent; every runtime event is `isTrusted`; the browser's JS is behind the non-default `javascript` feature.
- **`file:` reads** → blitz-net reads any `file:` path with no restriction; the WPT net provider joins request paths onto the WPT base.
- **Outbound HTTP** → 6 concurrent requests per host; no response-size cap, no timeout (observed absent); TLS via reqwest `native-tls`.
- **CLI input** → examples parse URLs with `Url::parse` (+ `https://` retry), numeric args with defaults; `bump` validates target + semver.
- **rdme markdown** → comrak with `unsafe: true` passes raw HTML through.
- **CI** → post-results workflow checks out trusted scripts from the default branch; `ci.yml` declares a workflow-level `permissions: contents: read` with no job grant, pins every action to a commit SHA and references no secret. The publish (signing), WPT and post-results jobs carry `github.repository == 'DioxusLabs/blitz'`, so no fork ref reaches "Signed Builds", "WPT" or their secrets.

## Data classifications (as measured)
| Class | Examples | Handling |
|---|---|---|
| CI secrets | signing key, Android keystore + passwords, Apple certificate, `WPT_GITHUB_TOKEN`, `GITHUB_TOKEN` | GitHub secrets/vars; written files removed in `always()` steps |
| Local user data (browser app) | history `history.sqlite3`, HTTP cache, cookies, clipboard, chosen file paths | unencrypted on disk; history in-memory on mobile; "Clear history" / "Clear Cache" |
| Logged values | request URLs, cache dir path, attribute values | escher's stderr sink (`seven_guis_native`) redacts them by allowlist; the upstream apps' `fmt::init()` and the WPT runner's `env_logger` log them as-is |

## Universal anti-patterns
> NO RECORDED INTENT — the plan carries none yet; Tier 1 warnings and `.claude/rules/security.md` hold the measured invariants.

## Not yet measured (owners on the working route)
- The dependency audit's reach — paste / memmap2 advisories in optional chains unseen by cargo-deny's resolved graph → "Quality gates".
- Logging redaction (`logging-redaction-wire`) — discharged for escher's sink; the upstream sinks stay unscrubbed (no owner).
- Opt-in OTel export — egress plus the `OTEL_EXPORTER_OTLP_HEADERS` credential path, a founder decision → "Driver command spans".
- Driver/MCP surface — local to the invoking user, no listener or auth surface → "MCP surface".
- TLS policy, key management, retention, SBOM, secret scanning, security-event logging → no owner yet.

## Path-scoped enforcement
See `.claude/rules/security.md` (always loaded).

## Critical decisions
> NO RECORDED INTENT — the Decisions Log is empty; the measured stance is in the body sections.

---

**Full plan:** `.andromeda/security-plan.md`.
