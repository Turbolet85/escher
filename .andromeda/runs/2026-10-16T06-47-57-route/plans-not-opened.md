# Plans — what Phase 1 opened and what it did not

Pass 1: this route run's first window (Phase 0 and Phase 1 in one window).

| Plan | Opened (lines or sections) | Not opened | A read clipped? |
|---|---|---|---|
| `architecture.md` | §Design Philosophy and §Stack and Technologies (lines 1–70); §Cross-cutting Patterns, §Project Intent and §Inherited Defaults (lines 189–227); the `## ` heading list | §Established Decisions, §Conventions, §Standard Contracts, §Occupied Resources, §Infrastructure Patterns, §Existing Scopes (only their item counts were read); the sidecar `architecture-amendments.md` and its archive | yes: the item-count script printed each §Established Decisions / §Standard Contracts / §Cross-cutting / §Existing Scopes / §Occupied Resources item cut at 70 characters, so those parts end at the cut and are not opened |
| `input.md` | whole (lines 1–48) | — | no |
| `security-plan.md` | §Threat Model Summary (lines 1–31), §API Security (lines 204–234), §Bootstrap phases (lines 274–282) | §Authentication & Authorization, §Input Validation, §Data Protection, §Dependency Security, §Secret Management, §Error Handling, §Logging & Monitoring and the stub tails; the sidecar `security-plan-amendments.md` | no |
| `design-system.md` | §Surface: cli (lines 383–406) | every other section; the sidecar `design-system-amendments.md` | no |
| `layout-templates.md` | §Surface: cli (lines 64–86) | every other section; the sidecar `layout-templates-amendments.md` | no |
| `test-plan.md` | §2 Test Strategy (lines 40–71), §6 E2E Test Strategy (lines 192–218), §9 CI Integration (lines 309–338), §10 Quality Gates (lines 339–349); key file `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md` (whole) | §1, the §3 Test Harness Contract body, §4, §5, §7, §8, §11, §12; key file `registries/contracts/test-plan/session-lifecycle.md`; the sidecar `test-plan-amendments.md` and its archive | no |
| `obs-plan.md` | §4 Span / Trace Coverage (lines 93–110); key file `registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md` (whole) | §1–§3 body, §5–§12; the sidecar `obs-plan-amendments.md` and its archive | no |
| `a11y-plan.md` | §1 A11y Scope Summary (lines 1–32), §6 Visual Design Verification (lines 223–273); key file `registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md` (whole) | §2–§5, §7–§12; the sidecar `a11y-plan-amendments.md` and its archive | no |

Beside the masters, these were opened whole in this window: `escher-0.2.0/intent.md`, `vision.md` and `requirements.md`; `.andromeda/residuals.md`; `.andromeda/master-route.md`; `escher-0.1.0/working-route.md`, `requirements.md` and `vision.md`; the version-end audit `.andromeda/runs/2026-10-15T10-44-59-code-audit/proposals.md`; and the incubator's `deferred-from-0.1.0.md`, `scope-choices-2026-10-16.md`, `capability-list.md` and `README.md`. The evolve-diagnose run's `proposals.md` (2026-10-16T04-34-02) was not opened.
