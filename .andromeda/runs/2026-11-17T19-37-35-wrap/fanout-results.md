# Fan-out results — 2026-11-17-answer-writer-by-family

architecture: none 1 · ops-architecture.md
security-plan: none · ops-security-plan.md — reason: the split moves no written byte, symbol, dependency, log site or secret handling; every picked row stays true.
design-system: no proposer
layout-templates: no proposer
test-plan: set 1 · add 1 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: no log site, print, env read or span added (`info_span!` sites stay 2); every picked row stays true.
a11y-plan: no proposer

## Dispositions

ops-test-plan.md
decision/test-module-split-keeps-paths · reject — check 2: the widening of a split module's children to `pub(super)` and no further is the answer writer's layout, which ops-architecture.md's `@row_none` (F1, type 4) holds is the code's, not a master's; one fact keeps one home. The tests' by-name imports are a sweep hazard for curation (P3, the report's Decisions & corrections), not a test rule. The row stays true as it stands.
decision/runnerless-cancelled-job-rerun · reject — check 3: the re-run of a CI job no runner took is the operator's directive for this pass (`operator-pass.md`), not a decision of the project's intent, and the previous wrap rejected the same row on the same ground. The recurrence routes to P5's Recurrence-watch row.
