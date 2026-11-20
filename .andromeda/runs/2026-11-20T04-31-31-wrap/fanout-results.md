# Fan-out results — 2026-11-20-stand-test-setup-shared

architecture: add 0 · set 1 · retire 0 · none 0 · ops-architecture.md
security-plan: none · ops-security-plan.md — reason: the chunk moved repeated stand-check setup into shared test helpers; no product surface, dependency, schema or auth change; every picked row stays true
design-system: none · ops-design-system.md — reason: no product symbol, surface, UI element or token touched; no master row cites a moved line
layout-templates: no proposer
test-plan: add 0 · set 1 · retire 0 · none 0 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: no span, log site, subscriber or env read changed; the shared modules stay silent
a11y-plan: none · ops-a11y-plan.md — reason: no UI element or schema changed; the harness settle and its guard untouched

ops-architecture.md
decision/shared-test-modules · reject — check 2: the fact's home is test-plan decision/shared-stand-tables-in-one-module (the same exception — a check keeps its own copy only where its body or message differs — offered twice; test-plan's row states the helpers shared once and already holds the per-file exceptions, architecture's row states the modules' layout in subdirectories)

ops-test-plan.md
decision/shared-stand-tables-in-one-module · apply — check 1: no playbook rule matches and nothing structural; the report's Symbols / APIs (fct-xr33tt4v) names each helper kept private with a differing body or message, so the row's except gains that exception beside the per-file items it already lists
