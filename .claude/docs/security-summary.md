# Security Summary — escher

_Distilled from `.andromeda/security-plan.md` (adopted reading — current truth, not intent). wrap-session's cascade re-derives it when the plan changes._

## Posture

escher is a native-API engine library with no served network API, no port and no authentication; its one listener is local — the driver session's owner-only Unix-domain socket, which carries lifecycle messages only. Its risk is in what it ingests: arbitrary remote HTML/CSS/images/fonts/sub-documents, JavaScript run by `blitz-vibey-script`, and CLI arguments of the examples. The application-level threat model is NOT YET MEASURED.

**Tier:** 0
**Auth approach:** none (observed absent in every slice)

## Threat model highlights (attack surface as measured)
- **Remote web content** → iframe depth cap 10, `@import` depth cap 16, stale iframe responses discarded by request id; snapshotting a never-styled node is skipped (Stylo would panic).
- **Script execution** → default `ScriptFetcher` accepts only `file:` / `data:`; JS `fetch()` is GET/HEAD only through the embedder's fetcher; same-origin / CORS observed absent; every runtime event is `isTrusted`; the browser's JS is behind the non-default `javascript` feature.
- **`file:` reads** → blitz-net reads any `file:` path with no restriction; the WPT net provider joins request paths onto the WPT base.
- **Outbound HTTP** → 6 concurrent requests per host; no response-size cap, no timeout (observed absent); TLS via reqwest `native-tls`.
- **CLI input** → examples parse URLs with `Url::parse` (+ `https://` retry), numeric args with defaults; `bump` validates target + semver; `scripts/agent-run.sh` allowlists its verb and admits a run selection only as `stand`, `all` or an existing `^[a-z0-9_]+$` blitz-tests file stem (else exit 2, no cargo call); `scripts/cold-agent.sh` allowlists its verb and the task `counter` (else exit 2, before any precondition or `claude` call).
- **Cold-agent pipe (ratified by the founder, 2026-10-06)** → an operator-host-only `claude -p` session on the operator's own Claude Code login (`apiKeySource` none, no API key, no env read, no CI secret) reaches the model provider; its only tools are a stdlib stdio MCP stub that refuses an unknown id, a disabled button, a press on the display and a malformed argument (`isError`, no state change) and logs no argument value.
- **Local IPC — the driver session socket (the founder's choice, 2026-10-07)** → `escher_driver::serve` binds `session.sock` in a caller-named state directory, created `0700` (a group- or other-open one refused), the socket `0600`: reachable by the owning user only, with no other authentication; the wire is a closed line grammar (`hello v1`, `stop v1`, a 64-byte request bound, a 2 s I/O bound, refusals changing nothing) carrying a pid, a label of `a-z0-9-` and a count — nothing of the screen; `escher-session <task> <state-dir>` takes a closed argv and exits 2 before booting anything on any other; unix only; no idle expiry.
- **rdme markdown** → comrak with `unsafe: true` passes raw HTML through.
- **CI** → post-results workflow checks out trusted scripts from the default branch; `ci.yml` declares a workflow-level `permissions: contents: read` with no job grant, pins every action to a commit SHA and references no secret. The publish (signing), WPT and post-results jobs carry `github.repository == 'DioxusLabs/blitz'`, so no fork ref reaches "Signed Builds", "WPT" or their secrets.

## Data classifications (as measured)
| Class | Examples | Handling |
|---|---|---|
| CI secrets | signing key, Android keystore + passwords, Apple certificate, `WPT_GITHUB_TOKEN`, `GITHUB_TOKEN` | GitHub secrets/vars; written files removed in `always()` steps |
| Local user data (browser app) | history `history.sqlite3`, HTTP cache, cookies, clipboard, chosen file paths | unencrypted on disk; history in-memory on mobile; "Clear history" / "Clear Cache" |
| Snapshot content (in-process) | stable ids, accessible names, a text control's value — in `DioxusDocument::snapshot`, its text form `Snapshot::to_text` and its diff `Snapshot::diff` | returned to the caller only (ratified by the founder, 2026-10-07): no log, event, socket, file or command of escher's carries any of the three — and since the sink drops every record from a target outside its allowlist, a sink-installing binary's stderr carries no author-key id and no accessible name at any `RUST_LOG` level (typed text not measured); a password's and a file input's value read the fixed mask, while the file input's DOM attribute and form submission still hold the host path |
| Logged values | request URLs, cache dir path, attribute values | escher's stderr sink (`seven_guis_native`, `escher-session`) redacts them by a target allowlist — engine targets print only safe fields, escher targets redact content-named fields, and a record from any other target is dropped whole at every level (no element id and no accessible name on stderr at `debug` or `trace`); the upstream apps' `fmt::init()` and the WPT runner's `env_logger` log them as-is |
| Harness test logs | the agent-run contract's JSON-line events; raw cargo/libtest output in `target/agent-run/run.log` | events carry no content-named field and no captured output; `run.log` unscrubbed like `target/ci-logs/`, gitignored, never printed |
| Operator credential (dev host) | the operator's Claude Code claude.ai login, used by the cold-agent live run | held by the `claude` CLI outside the repo (at-rest location unmeasured); no artifact, event or verdict carries it |
| Agent-session transcript | `target/cold-agent/transcript.jsonl` — model output, tool I/O, host paths | raw by design, never printed, gitignored; events and verdict carry counts and identities only; the one committed copy host-path-masked |

## Universal anti-patterns
> NO RECORDED INTENT — the plan carries none yet; Tier 1 warnings and `.claude/rules/security.md` hold the measured invariants.

## Not yet measured (owners on the working route)
- The dependency audit's reach — paste / memmap2 advisories in optional chains unseen by cargo-deny's resolved graph → "Quality gates".
- Logging redaction (`logging-redaction-wire`) — discharged for escher's sink: engine targets and content-named fields redacted, records from targets outside the allowlist dropped at every level (delivered by "Sink target allowlist"); the upstream sinks stay unscrubbed (no owner).
- Opt-in OTel export — not in escher 0.1.0 (ruled by the founder, 2026-10-06): no egress and no `OTEL_EXPORTER_OTLP_HEADERS` credential path; the transport and the credential path stay undecided → `.andromeda/residuals.md` (a later version).
- Driver/MCP surface — local to the invoking user, no listener or auth surface → "MCP surface".
- TLS policy, key management, retention, SBOM, secret scanning, security-event logging → no owner yet.

## Path-scoped enforcement
See `.claude/rules/security.md` (always loaded).

## Critical decisions
> NO RECORDED INTENT — the Decisions Log is empty; the measured stance is in the body sections.

---

**Full plan:** `.andromeda/security-plan.md`.
