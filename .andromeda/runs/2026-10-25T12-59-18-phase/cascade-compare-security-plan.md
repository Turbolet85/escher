# Cascade compare — security-plan · phase · 2026-10-25-cold-agent-bar
E items 97 · held 28 · lost at classify 8 · lost at strong 1 · beside a placed fact - · cascade only 12 · body 23 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### authentication-authorization#2.5 · lost at classify · .andromeda/security-plan-detail/authentication-authorization.md:30
- (.andromeda/security-plan-detail/authentication-authorization.md:30) If the chunk moves CI (the cold-agent contract tests, the `stand-repeat` targets), every ci.yml job's `GITHUB_TOKEN` must stay read-only under the one workflow-level grant, and `test_ci_workflows.py`'s assertion of it must pass in the fast leg's CI scripts.
  > | RBAC / permissions (CI tokens) | ci.yml declares a workflow-level `permissions: contents: read` and no job-level grant, so every ci.yml job's `GITHUB_TOKEN` is read-only; asserted by `test_ci_workflows.py` (.github/workflows/ci.yml:16-17; .github/scripts/test_ci_workflows.py:201-205) | GitHub Actions `permissions` |

### dependency-security#1.0 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:8
- (.andromeda/security-plan-detail/dependency-security.md:8) The chunk's tree, whatever the stand and the pipe take, must read green under the audit command over the whole workspace's graph, advisories and licences both.
  > **Audit tool:** cargo-deny 0.20.2 — `cargo deny --locked --workspace --all-features check advisories licenses` against the RustSec advisory database and the licence table, over the whole workspace's graph

### dependency-security#1.0 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:8
- (.andromeda/security-plan-detail/dependency-security.md:8) Any advisory the chunk meets must be met by a per-ID ignore with a written reason or a fix, never a blanket allow or a lowered `unmaintained`/`unsound`; the eight standing ignores are the count the chunk must keep unless it records a change.
  > `[advisories]` per-ID ignores only, each with a written reason, no blanket allow, `unmaintained` and `unsound` at their defaults (deny.toml:1-29; .github/scripts/ci-leg.sh:38). The file also holds a `[licenses]` table, which the audit leg reads since 2026-10-14-quality-gates (deny.toml:31-50; Supply chain integrity below). Eight ignores stand.

### dependency-security#1.0 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:9
- (.andromeda/security-plan-detail/dependency-security.md:9) If the Dependency audit turns red at this chunk's CI over a `Cargo.lock` and `deny.toml` unchanged from a green base, it must be handled as the recorded external-decay pattern — per-ID ignores on a recorded word, the red and green runs cited — never folded silently.
  > on the operator's word under the playbook's External decay pattern (2026-10-17) — external decay: the three advisories turned the Dependency audit red (CI#39055555592 on `3eda07bc`) over the tree `45fcd733` that had read green (CI#39043080454), and the ignores read green (CI#39056637407 on `ae8c8daf`)

### dependency-security#1.0 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:10
- (.andromeda/security-plan-detail/dependency-security.md:10) A crate the extended stand (or a load-in-flight stand-in) pulls into seven_guis' or blitz-tests' build graph enters the audit's reach, so any such addition must be audited as part of the resolved graph.
  > The audit's reach is cargo-deny's resolved graph, not the whole lockfile.

### dependency-security#1.0 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:11
- (.andromeda/security-plan-detail/dependency-security.md:11) The chunk's audit reading must match the latest recorded baseline — no ignore added, every standing ignore still encountered, the same two warnings and no new one.
  > with no ignore added; every standing ignore is still encountered, and the leg prints two warnings, `yanked` clipboard-win 5.4.1 and `license-not-encountered` NCSA (deny.toml:46)

### dependency-security#2.1 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:17
- (.andromeda/security-plan-detail/dependency-security.md:17) Any package that enters `Cargo.lock` with this chunk (e.g. for Cells, the select or a held load on the stand) must come from crates.io with a checksum and be reviewed by hand on these five points.
  > each from crates.io with a checksum and reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent

### dependency-security#6.1 · lost at classify · .andromeda/security-plan-detail/dependency-security.md:48
- (.andromeda/security-plan-detail/dependency-security.md:48) Licences gate every push: any dependency the chunk adds must keep the leg reading `licenses ok` under upstream's unchanged `[licenses]` table.
  > the audit leg runs it on every push since 2026-10-14-quality-gates — `licenses` over the whole workspace's graph, reading `licenses ok` on the dev host and in CI#38692629940 — so the fork gates on licences

### error-handling#2.1 · lost at classify · .andromeda/security-plan-detail/error-handling.md:18
- (.andromeda/security-plan-detail/error-handling.md:18) The load in flight on the stand must not reach a dioxus event-data conversion that panics — an `image` or `media` event handler on the stand would hit `unimplemented!()`, so the stand's load must be carried without one.
  > twelve dioxus event-data conversions (cancel, animation, clipboard, composition, drag, image, media, selection, toggle, transition, resize, visible) call `unimplemented!()`

### input-validation#1.35 · lost at classify · .andromeda/security-plan-detail/input-validation.md:125
- (.andromeda/security-plan-detail/input-validation.md:125) Carrying a load in flight on the stand must not route an agent- or user-supplied URL through the fetch path, and it must keep the stand offline on its bundled font. Breaking either is the P4 fork's to rule.
  > escher routes no agent- or user-supplied URL through this path today — no driver verb takes a URL, and the stand boots offline on its bundled font

### secret-management#2.13 · lost at strong · .andromeda/security-plan-detail/secret-management.md:33
- (.andromeda/security-plan-detail/secret-management.md:33) Every file the chunk commits (the moved `scripts/cold-agent.sh` prompt, `.github/scripts/test_cold_agent.py`, the run's evidence) must read 0 on the same host-path and credential scan the last committed statement passed, and a moved test or pin must read no environment variable.
  > searched: `/home/|api[_-]?key|authorization:|bearer ` and two e-mail domains (case-insensitive) over `docs/capabilities.md` and `.github/scripts/test_capability_statement.py` (0 hits); the pin reads no environment variable

### threat-model-summary#5.1 · lost at classify · .andromeda/security-plan-detail/threat-model-summary.md:43
- (.andromeda/security-plan-detail/threat-model-summary.md:43) The run and the extended stand must keep to the carried set the committed statement names (no description, values masked) and add no carrier; a moved claim there is the wrap's to re-read, never a phase edit.
  > the carried set is ids, names, masked values, the snapshot text and diffs, a shot as an `image` block painted masked and a lint's findings, no description; what the client does onward is the client's — a statement of the ratified crossing that adds no carrier

### threat-model-summary#1.1 · lost at classify · .andromeda/security-plan-detail/threat-model-summary.md:9
- (.andromeda/security-plan-detail/threat-model-summary.md:9) A load in flight on the stand must not route an agent- or user-supplied URL through the net provider, which fetches `http(s)` and reads `file:` with no path restriction; any provider it adds is a P4 fork against this vector.
  > The net provider fetches arbitrary `http(s)` URLs through reqwest, reads `file:` URLs from local disk, and decodes `data:` URLs (packages/blitz-net/src/lib.rs:153-173).

## Held

- api-security#1.0
- api-security#2.20
- api-security#2.4
- api-security#2.6
- authentication-authorization#2.6
- authentication-authorization#2.7
- data-protection#1.1
- data-protection#2.1
- dependency-security#3.1
- error-handling#1.1
- input-validation#1.10
- input-validation#1.109
- input-validation#1.12
- input-validation#1.13
- input-validation#1.15
- input-validation#1.16
- input-validation#1.17
- input-validation#1.57
- input-validation#1.6
- input-validation#1.61
- input-validation#1.65
- input-validation#1.9
- logging-monitoring#1.1
- logging-monitoring#1.2
- logging-monitoring#2.1
- secret-management#1.2
- threat-model-summary#3.1
- threat-model-summary#4.1

## Cascade only

- api-security#2.5
- input-validation#1.11
- input-validation#1.23
- input-validation#1.24
- input-validation#1.53
- input-validation#1.58
- input-validation#1.64
- input-validation#1.66
- input-validation#1.79
- input-validation#1.83
- secret-management#2.11
- secret-management#2.3
