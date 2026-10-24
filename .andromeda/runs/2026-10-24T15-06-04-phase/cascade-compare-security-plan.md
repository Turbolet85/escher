# Cascade compare — security-plan · phase · 2026-10-24-stand-requirement-sweep
E items 84 · held 26 · lost at classify 2 · lost at strong 0 · cascade only 9 · body 27 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### dependency-security#5.1 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:40
- (.andromeda/security-plan-detail/dependency-security.md:40) If the sweep takes a dependency, it must be reviewed in the established way: no package enters `Cargo.lock` unreviewed, and the audit reads `advisories ok` with `deny.toml` unchanged.
  > reviewed at that chunk: no package enters `Cargo.lock` (its one changed line `"color",` inside blitz-test-harness's dependency list), the audit leg reads `advisories ok` with `deny.toml` unchanged

### dependency-security#2.6 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:22
- (.andromeda/security-plan-detail/dependency-security.md:22) The sweep must bump none of the coupled pins and take no git dependency.
  > the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", no git dependency;

## Held

- api-security#1.0
- api-security#2.20
- api-security#2.6
- authentication-authorization#2.5
- authentication-authorization#2.6
- authentication-authorization#2.7
- data-protection#1.1
- data-protection#2.1
- dependency-security#1.0
- error-handling#1.1
- input-validation#1.12
- input-validation#1.13
- input-validation#1.15
- input-validation#1.16
- input-validation#1.57
- input-validation#1.58
- input-validation#1.61
- input-validation#1.62
- input-validation#1.65
- input-validation#1.66
- input-validation#1.69
- input-validation#1.8
- logging-monitoring#1.1
- logging-monitoring#1.2
- threat-model-summary#3.1
- threat-model-summary#4.1

## Cascade only

- dependency-security#2.2
- dependency-security#2.3
- dependency-security#4.1
- dependency-security#6.1
- input-validation#1.10
- input-validation#1.11
- input-validation#1.17
- input-validation#1.5
- input-validation#1.53
