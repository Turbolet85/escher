# Leaf cascade listing — first run on cascade.py v1.6

Recorded at wrap P1 of 2026-10-12-headless-screenshot, on the operator's word ("this is the first run on it, so record
what the leaf cascade lists, verbatim, in the run dir"). Both set calls ran with no master amended yet: the listing
is what P2's step 3 would start from, not a P2 result. Each block below is the tool's stdout, byte for byte.

This chunk closes Epoch 5 (`route.py epoch`: Epoch 5 — Agent surfaces · markerless 0 · pending 1, this chunk), so P2's
step 3 runs the WHOLE mode — the first block. The PASS mode, at P1, has no changed source — the second block.

## `cascade.py leaves --run-dir {run_dir} --marker 2026-10-12-headless-screenshot --all` — exit 0

```
cascade v1.6 · bcc7d0ba
architecture-1 · whole · 7 leaves · 36686 B · 1 source file(s)
  leaf .claude/docs/conventions.md · 5302 B
  leaf .claude/docs/gotchas.md · 6047 B
  leaf .claude/docs/services/blitz-dom.md · 5497 B
  leaf .claude/docs/services/blitz-paint.md · 1915 B
  leaf .claude/docs/services/blitz-shell.md · 2782 B
  leaf .claude/docs/services/blitz-traits.md · 1740 B
  leaf .claude/docs/services/dioxus-native-dom.md · 13403 B
  source .andromeda/architecture.md
architecture-2 · whole · 3 leaves · 45788 B · 1 source file(s)
  leaf .claude/docs/services/escher-driver.md · 34674 B
  leaf .claude/docs/stack.md · 7408 B
  leaf .claude/docs/workflow.md · 3706 B
  source .andromeda/architecture.md
masters · whole · 1 leaves · 33380 B · 14 source file(s)
  leaf CLAUDE.md · 33380 B
  source .andromeda/architecture.md
  source .andromeda/security-plan.md
  source .andromeda/design-system.md
  source .andromeda/layout-templates.md
  source .andromeda/test-plan.md
  source .andromeda/registries/test-plan-contracts.toml
  source .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md
  source .andromeda/registries/contracts/test-plan/session-lifecycle.md
  source .andromeda/obs-plan.md
  source .andromeda/registries/obs-plan-contracts.toml
  source .andromeda/registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md
  source .andromeda/a11y-plan.md
  source .andromeda/registries/a11y-plan-contracts.toml
  source .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md
architecture+layout-templates · whole · 1 leaves · 11627 B · 2 source file(s)
  leaf .claude/docs/services/seven_guis.md · 11627 B
  source .andromeda/architecture.md
  source .andromeda/layout-templates.md
architecture+test-plan · whole · 2 leaves · 15700 B · 5 source file(s)
  leaf .claude/docs/commands.md · 10734 B
  leaf .claude/docs/services/blitz-test-harness.md · 4966 B
  source .andromeda/architecture.md
  source .andromeda/test-plan.md
  source .andromeda/registries/test-plan-contracts.toml
  source .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md
  source .andromeda/registries/contracts/test-plan/session-lifecycle.md
architecture+obs-plan · whole · 1 leaves · 8773 B · 4 source file(s)
  leaf .claude/docs/services/escher-telemetry.md · 8773 B
  source .andromeda/architecture.md
  source .andromeda/obs-plan.md
  source .andromeda/registries/obs-plan-contracts.toml
  source .andromeda/registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md
security-plan · whole · 2 leaves · 25543 B · 1 source file(s)
  leaf .claude/docs/security-summary.md · 16192 B
  leaf .claude/rules/security.md · 9351 B
  source .andromeda/security-plan.md
design-system+layout-templates · whole · 1 leaves · 4993 B · 2 source file(s)
  leaf .claude/docs/design-summary.md · 4993 B
  source .andromeda/design-system.md
  source .andromeda/layout-templates.md
test-plan · whole · 3 leaves · 37359 B · 4 source file(s)
  leaf .claude/docs/tests-summary.md · 25199 B
  leaf .claude/rules/testing.md · 6067 B
  leaf .claude/rules/verification-harness.md · 6093 B
  source .andromeda/test-plan.md
  source .andromeda/registries/test-plan-contracts.toml
  source .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md
  source .andromeda/registries/contracts/test-plan/session-lifecycle.md
obs-plan · whole · 2 leaves · 20591 B · 3 source file(s)
  leaf .claude/docs/obs-summary.md · 12894 B
  leaf .claude/rules/observability.md · 7697 B
  source .andromeda/obs-plan.md
  source .andromeda/registries/obs-plan-contracts.toml
  source .andromeda/registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md
a11y-plan · whole · 2 leaves · 15099 B · 3 source file(s)
  leaf .claude/docs/a11y-summary.md · 8531 B
  leaf .claude/rules/a11y.md · 6568 B
  source .andromeda/a11y-plan.md
  source .andromeda/registries/a11y-plan-contracts.toml
  source .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md
unmapped .claude/rules/host-linux.md (host)
total · whole · 11 group(s) · 25 leaves · 255539 B · unmapped 1
```

## `cascade.py leaves --run-dir {run_dir} --marker 2026-10-12-headless-screenshot` — exit 0

```
cascade v1.6 · bcc7d0ba
baseline 938cd1b8 (pre-CI parent — the oldest of 1 pre-CI commit(s), 5173f7e1)
pass: no source changed — no group
```

## Against the last whole-mode run (2026-10-12-self-description wrap, cascade before v1.6)

That wrap's handoff records 10 groups and 25 leaves, with CLAUDE.md in seven groups (60 of its 130 findings
conflicted and were merged by hand). Under v1.6: 11 groups, 25 leaves, and CLAUDE.md in ONE group, `masters`, read
against all seven masters and their seven registry files (14 source files). One leaf is unmapped,
`.claude/rules/host-linux.md (host)`, a host leaf no master states. No `empty` row.
