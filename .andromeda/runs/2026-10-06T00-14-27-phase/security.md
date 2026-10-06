# security extract

## Relevance
relevant: the scrub layer is the security-plan's open `logging-redaction-wire` bootstrap item. The OTel crates are new dependencies under the audit gate. The opt-in exporter is a new outbound egress, and its switches are new env reads.

## Constraints
- The scrub layer is meant to discharge the `logging-redaction-wire` bootstrap item, which security-plan §Bootstrap phases still records as open (redaction absent, see §Data Protection "Logs"). It must cover at least the user-content fields that security-plan §Logging & Monitoring "What is logged" names: `url` in blitz-net and in blitz-dom resource-load failures, the cache-directory `path`, and `error`. It must also cover dioxus-native asset logs, which print the full request with Debug formatting as message text rather than as a named field. A scrub keyed on field names would not see that text. Whether it reaches values embedded in messages is research's question.
- New crates (`opentelemetry*`, `tracing-opentelemetry`, the exporter's transport stack) must pass the cargo-deny advisories gate set by security-plan §Dependency Security "Audit tool". `deny.toml` resolves with `all-features = true`, so these crates are audited even behind an optional feature. A finding is fixed by a semver-compatible update, or ignored per ID with a written reason. The gate only covers cargo-deny's resolved graph (the paste/memmap2 prune), so new crates also need a hand review.
- Per security-plan §Dependency Security "Pinning" and "Supply chain integrity", new dependencies are declared once in the workspace and inherited with `workspace = true`. `Cargo.lock` stays committed, and every leg still builds `--locked`.
- Security-plan §API Security records no served API surface. §Threat Model Summary "Attack surface" lists no telemetry egress vector. The OTel exporter is a new outbound vector, so the plan needs an amendment covering the endpoint, its transport and its TLS stack. That amendment must land before the egress ships. Today the plan's only recorded TLS is reqwest `native-tls` in blitz-net (§Data Protection "In transit"). Which TLS stack `opentelemetry-otlp` brings, and whether it touches the coupled pins, is research's question.
- Security-plan §Secret Management "Never in code" requires that source reads no secret. The new env reads (opt-in switch, filter, endpoint) join the non-secret list in §Secret Management "Environment values read". The OTel SDK conventionally reads credential-bearing variables (`OTEL_EXPORTER_OTLP_HEADERS`) on its own. Whether the chosen exporter does so implicitly is research's question. If it does, that read is a secret path and needs a recorded decision.
- Panic messages can carry user content. Security-plan §Error Handling "Panic paths" records panics that format URLs and file paths (`resolve_url`, `launch_url`, `preact_script`'s `could not resolve {raw_path}`). The panic-hook event therefore needs to pass the same scrub before any sink. The hook must also chain, so the WPT runner can still catch panics per test (§Error Handling "Graceful degradation").
- JS console output is script-supplied, untrusted content. It goes to the `log` crate under target `js_console` (security-plan §Logging & Monitoring "What is logged"). If the bootstrap bridges `log` into `tracing`, that content reaches the sinks and the exporter. It must then be scrubbed or filtered.

## Patterns to follow
- Engine call sites stay behind `#[cfg(feature = "tracing")]` with macros that compile only with that feature (security-plan §Logging & Monitoring "Log format and backends"). The bootstrap does not change engine crates' gating.
- Keep stdout clean the way the JS console route does: console output goes to `log` at debug under its own target (security-plan §Logging & Monitoring "What is logged"). Logs go to stderr or a named file.
- Ignore advisories per ID with a written reason, as the one standing `deny.toml` ignore does (security-plan §Dependency Security "Audit tool").
- Map panics to typed results without changing them, as the WPT runner does with `HandlerPanic` and its CRASH result (security-plan §Error Handling "Graceful degradation"). The hook observes the panic and re-raises it through the chained previous hook.

## Anti-patterns to avoid
- Do not add a blanket advisory allow. Do not set `unmaintained` or `unsound` to none to admit OTel crates (security-plan §Dependency Security "Audit tool").
- Do not log whole requests or documents with Debug formatting, as dioxus-native's asset logs do (security-plan §Logging & Monitoring "What is logged"). Do not add any new log field that carries user content unscrubbed.
- Do not connect, or build an exporter, by default. There is no recorded egress (security-plan §API Security, §Threat Model Summary), so the off state must make no outbound connection.

## Contract bindings
- Scrub layer ↔ obs-plan §8 field list ↔ tests: the redacted field set is shared with obs-plan's scrub list. Test fixtures use sentinel values, not real PII.
- Dependency gate ↔ CI: the ci.yml `audit` job runs `ci-leg.sh audit` on every push (security-plan §Dependency Security "CI integration"). New crates are gated there, in the slow tier.
- Egress, env vars and new crate ↔ arch §Occupied Resources: the arch registration and the security-plan amendment (§API Security, §Threat Model Summary, §Secret Management "Environment values read") land together.
- Panic hook ↔ the WPT runner's per-test panic catch (security-plan §Error Handling "Graceful degradation"). Chaining keeps the CRASH result path unchanged.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh audit` passes with the OTel crates in `Cargo.lock`. Any new `deny.toml` ignore is per ID with a written reason, and every leg still builds `--locked` (per security-plan §Dependency Security).
- A test emits events whose `url`, `path` and `error` fields, message text and panic payload carry a sentinel string. The sentinel must be absent from the captured sink output, and from exporter output when export is enabled (per security-plan §Bootstrap phases `logging-redaction-wire` and §Logging & Monitoring).
- With the opt-in unset, a test confirms that no exporter is built and no outbound connection is attempted (per security-plan §API Security).
- The bootstrap's env reads are exactly the registered non-secret names, and none of them carries a credential. Any implicit SDK read of credential-bearing OTLP variables is either disabled or recorded as a decision (per security-plan §Secret Management).
