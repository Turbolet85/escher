proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 11
    section: "## Dependency Security"
    change: RUSTSEC-2026-0414 (pulldown-cmark 0.13.4, fixed in 0.13.5) is fixed by the lockfile update to pulldown-cmark 0.13.5, in the lock alone, semver-compatible with no coupled pin, `deny.toml` unmoved and no ignore added; it is reached through `rdme`, which never sets `ENABLE_WIKILINKS`, so no reader was exposed; the audit leg reads `advisories ok` and `licenses ok`, CI#41192433140 green; the operator's call, not put to the founder (2026-10-31) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
    sidecar: Dependency Security gains the lock-only pulldown-cmark 0.13.4 to 0.13.5 update past RUSTSEC-2026-0414, `deny.toml` unmoved, from 2026-10-31-engine-debts.
    rationale: The report's **Dependencies** bullet states the bump, its advisory, its reach through `rdme` and the operator's call on inputs#I11 and inputs#I12, and **Expected amendments** names security-plan §Dependency Security for pulldown-cmark 0.13.5, with no line of this file naming the advisory; line 11 is the precedent, the same lock-only fix form for tokio. D-security-deps holds, since the chunk adds no crate and no ban list exists (§Security Anti-Patterns).
note: The free-disk action site the report's **Harness / gate surface** adds is no crate and is pinned by sha, so it falls under the rule of the CI tooling line of §Dependency Security in the body, which stays true; no proposal here, and neither D-security-input nor D-security-auth nor the report's **Retired** bullet reaches this file.
