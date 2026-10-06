# security extract

## Relevance
partial — the chunk opens no served surface, auth path or secret. Its security reach is the no-live-network boot, how the bundled font is loaded, any new dependency edge, and keeping env reads and log fields within the plan's recorded set.

## Constraints
- The headless boot must not reach the network through `blitz-net`. Per security-plan §API Security (rows "Request size limit" and "Request timeout"), it has no response-size cap and no request timeout. Per §Input Validation (row "`file:` URLs (net provider)"), it reads `file:` paths with no restriction. Each check must run with no net provider or with one that refuses every fetch, so a stand check emits no outbound request and no unrestricted disk read. Whether `Harness`/`DioxusDocument` already defaults to a no-op provider is research's question.
- The `dioxus` scheme is answered from the local file system by `serve_asset` (security-plan §API Security, row "`dioxus` scheme"). Whether the headless stand's asset references (such as the bundled font) reach that path, or any other fetch path, is research's question. Load the bundled `DejaVuSans.woff2` from in-tree bytes, and do not send it through a provider fetch.
- Per security-plan §Input Validation (rows "Fetched fonts", for format sniffing and the WOFF-decompression fallback), font bytes pass through sniffing and a silent fallback. On the new font-context surface, a bundled-font load failure must be a typed `Result` or a clear test failure, never silent acceptance. Per §Error Handling "Panic paths" (DOM internals), the font-context mutex locks are unwrapped, so the new option must not add another panic path on input it does not control.
- Any new dependency edge (blitz-tests → seven_guis, the harness → a font asset, or a new crate) must keep `Cargo.lock` committed, inherit versions with `workspace = true` and build with `--locked`. Per security-plan §Dependency Security (sub-sections "Pinning" and "Supply chain integrity"), the cargo-deny audit leg must stay green. That gate's resolved graph misses some advisories (§Dependency Security, "Audit tool"), so any new registry crate needs a hand review.
- Per security-plan §Secret Management, under "Environment values read", the source's environment reads are the closed set recorded there. A pinned viewport, a font switch or a timer/time seam must be configured in code (such as `HarnessOptions` or a stand constant), never by a new env var.
- Per security-plan §Logging & Monitoring, a new user-content log field is not allowed. escher's scrub covers only the `escher_telemetry::init` sink ("Log format and backends"). Stand checks and any new harness or stand code must emit no URL, path, attribute-value or text field. Any engine log stays behind the `tracing` feature.

## Patterns to follow
- Typed net-provider errors: the WPT runner's provider maps failures to a typed `WptNetProviderError` and logs them, without panicking (security-plan §Error Handling, "Error format"). A stand-side refuse-all provider, if one is needed, should return a typed error in the same way and never `unwrap`.
- Dependency inheritance: blitz-tests' dev-dependencies all use `workspace = true` (security-plan §Dependency Security, "Pinning"). A new edge should follow the same form, and a crate that is not published keeps `publish = false`.
- The 7GUIs input-validation contract: the flight booker validates `dd.mm.yyyy`, including the month range, leap years and return-not-before-start (security-plan §Input Validation, row "Example apps · 7GUIs inputs"). The flight-booker proof check exercises this validation and does not bypass it.
- Graceful query degradation: query APIs return `Result<Option<NodeId>>`, and a missing element is `None` (security-plan §Error Handling, "Graceful degradation"). The harness query helper panics on a selector that does not parse (§Input Validation, row "Test harness queries"), so checks pass only constant, valid selectors to it.

## Anti-patterns to avoid
- Do not route a stand check's boot through `blitz_net::Provider` (or `reqwest`), and do not give it a live or `file:` URL (security-plan §API Security; §Input Validation, row "`file:` URLs").
- Do not silence a new advisory with a blanket allow or by relaxing `unmaintained`/`unsound`. The only accepted fixes are a semver-compatible update or a per-ID ignore with a written reason (security-plan §Dependency Security, "Audit tool").
- Do not add a new `std::env::var` read for stand configuration (security-plan §Secret Management, "Environment values read").

## Contract bindings
- security ↔ arch: a new workspace dependency edge or a public `HarnessOptions` net/font option is an arch §Occupied Resources / §Standard Contracts registration at wrap. The security side is to keep the dependency-pinning rules in security-plan §Dependency Security.
- security ↔ tests: the no-network rule needs a harness-level way to run with no network, and the test-plan's harness section owns that switch. The CI security gate (the `audit` leg) stays a separate job (security-plan §Dependency Security, "CI integration").
- security ↔ obs: the scrub in escher-telemetry is a process-global sink (security-plan §Logging & Monitoring). Fresh-per-check boots must not re-install it per check. obs owns where it is installed, and security owns the rule that adds no new content fields.

## Acceptance criteria contributions
- No stand check or headless-boot code path builds a `blitz_net` provider or issues a live, `http(s)` or `file:` fetch. Check this with a grep over the new or changed stand, harness and test files for `blitz_net`/`Provider::new`/`reqwest`. A refuse-all or `None` net configuration is asserted on the boot path (per security-plan §API Security).
- `bash .github/scripts/ci-leg.sh audit` passes. The `Cargo.lock` diff adds only workspace path crates, or adds registry crates that each have a written hand review. Every changed manifest entry uses `workspace = true` (per security-plan §Dependency Security).
- The chunk's diff adds no new `std::env::var`/`env::var`/`getenv` read. Check by grepping the diff (per security-plan §Secret Management).
- The chunk's diff adds no new log call site with a `url`, `path`, `href`, `src`, `text`, `value` or `html` field, and any engine log stays `#[cfg(feature = "tracing")]` (per security-plan §Logging & Monitoring).
