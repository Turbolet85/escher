# Plans — what Phase 1 opened and what it did not

Pass 1 — the escher-0.3.0 route run `.andromeda/runs/2026-10-26T19-21-21-route/`, one window (adopted project:
bootstrap merging reads only each plan's `Bootstrap phases`; the other sections were opened for the chunks they cite).

| Plan | Opened (lines or sections) | Not opened | A read clipped? |
|---|---|---|---|
| `architecture.md` | body lines 1-329 (every section, read in two pages: 1-248, 249-329) | `.andromeda/architecture-detail/*` (every key file); `.andromeda/architecture-amendments.md` | no — the first page stopped at 248 by the read cap and 249-329 were read next |
| `input.md` | whole (lines 1-49) | — | no |
| `security-plan.md` | lines 1-48 (§Threat Model Summary, §Authentication & Authorization), 205-284 (§API Security, §Dependency Security, §Supply chain integrity, §Bootstrap phases), 377-426 (§Logging & Monitoring, §Compliance Controls, §Security Anti-Patterns, the Decisions Log heading) | lines 49-204 (§Input Validation, §Data Protection), 285-376 (§Secret Management, §Error Handling); `.andromeda/security-plan-detail/*`; `.andromeda/security-plan-amendments.md` | no |
| `design-system.md` | lines 1-40 (§Brand Identity, §Color Palette with §Core Colors' accent row) | lines 41-end (§Typography onward, every `## Surface:` section, the Decisions Log); `.andromeda/design-system-detail/*`; `.andromeda/design-system-amendments.md` | no |
| `layout-templates.md` | lines 1-57 (§Surface: desktop-native — Primary screens, Header, Primary navigation, Footer, IA notes) | lines 58-end (mobile-native, cli, web-spa, Decisions Log); `.andromeda/layout-templates-amendments.md` | no |
| `test-plan.md` | lines 1-144 (§1, §2, §3 to the crate-local helpers), 369-420 (§9 CI Integration, §10 Quality Gates) | lines 145-368 (§3's remainder, §4-§8), 421-end (§11, §12); `.andromeda/test-plan-detail/*`; `.andromeda/registries/test-plan-contracts.toml` and its key files; `.andromeda/test-plan-amendments.md` | a whole-section read of 115-193 was refused by the read cap; 115-144 was read in its place |
| `obs-plan.md` | lines 1-70 (§1, §2, §3 to the logging stack), 199-292 (§7, §8, §9) | lines 71-198 (§3's remainder, §4-§6), 293-end (§10-§12); `.andromeda/obs-plan-detail/*`; `.andromeda/registries/obs-plan-contracts.toml`; `.andromeda/obs-plan-amendments.md` | no |
| `a11y-plan.md` | lines 1-80 (§1, §2, §3), 101-149 (§5 Keyboard Navigation), 233-254 (§9, §10, §11) | lines 81-100 (§4), 150-232 (§6-§8); `.andromeda/a11y-plan-detail/*`; `.andromeda/registries/a11y-plan-contracts.toml`; `.andromeda/a11y-plan-amendments.md` | no |
