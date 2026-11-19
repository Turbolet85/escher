# Plans — what Phase 1 opened and what it did not

Pass 1 (this window, 2026-11-19).

| Plan | Opened (sections or pages) | Not opened | A read clipped? |
|---|---|---|---|
| `doc:architecture` | `tree`; §Project Intent; §Existing Scopes; §Inherited Defaults | §Design Philosophy, §Stack and Technologies, §Established Decisions, §Conventions, §Standard Contracts, §Occupied Resources, §Infrastructure Patterns, §Cross-cutting Patterns; sidecar (`--amendments`) | no |
| `doc:input` | `tree` (heading list only) | every section; sidecar | no |
| `doc:security-plan` | `tree`; §Threat Model Summary; §Bootstrap phases (derive for route / setup-project); `contracts` (n/a, no keyed section) | §Authentication & Authorization, §Input Validation, §Data Protection, §API Security, §Dependency Security, §Secret Management, §Error Handling, §Logging & Monitoring, §Compliance Controls, §Security Anti-Patterns, §Security Decisions Log; sidecar | no |
| `doc:design-system` | `tree` (heading list only) | every section; sidecar | no |
| `doc:layout-templates` | `tree` (heading list only) | every section; sidecar | no |
| `doc:test-plan` | `tree`; §9 CI Integration (both twins); §10 Quality Gates & Coverage Targets (both twins); `contracts` listing | §1–§8, §11, §12; key `session-lifecycle`; key `bootstrap-phases-derive-for-route-setup-project` (0 rows); sidecar | no |
| `doc:obs-plan` | `tree`; key `bootstrap-phases-derive-for-route-setup-project` (whole); `contracts` listing | §1, §2, §4–§12; sidecar | no |
| `doc:a11y-plan` | `tree`; §9 CI Integration (both twins); key `bootstrap-phases-derive-for-route-setup-project` (whole); `contracts` listing | §1–§8, §10–§12; sidecar | no |

Every plan's bootstrap was delivered in 0.1.0–0.3.0 (the three opened bootstrap keys name built facts: the sink's
resource keys, the `a11y` CI job, the sink allowlist). No bootstrap chunk is owed. Each entry below cites the intent
finding behind it, or a plan part this table lists as opened.
