# Fan-out results — 2026-11-15-taffy-from-upstream-git

architecture: add 0 · set 4 · retire 0 · none 0 · ops-architecture.md
security-plan: add 0 · set 4 · retire 0 · none 0 · ops-security-plan.md
design-system: none · ops-design-system.md — reason: no UI element, no spec master edit, no public symbol, no tally moved; every picked row stands true
layout-templates: no proposer
test-plan: none · ops-test-plan.md — reason: no new test, runner, gate, CI job or floor; every tally equal to HEAD's
obs-plan: none · ops-obs-plan.md — reason: no hot-path op, log site, telemetry dependency or PII surface
a11y-plan: none · ops-a11y-plan.md — reason: no UI element, no a11y or log schema change, a11y tallies unmoved

ops-architecture.md
decision/taffy-layout · apply — check 5: the plan's Expected amendment «architecture · decision/taffy-layout — Taffy 0.15 from the crates.io registry → git Taffy pinned at e90751fd», P5-approved (inputs#I4), the report's Dependencies bullet substantiating it
decision/no-git-dependencies · apply — check 5: the plan's Expected amendment «the rule half "taffy is a registry version" stale; restate with the except as it stands», the except unchanged
decision/deferred-upstream-dependency-bumps · apply — check 5: the plan's Expected amendment «Taffy now git at e90751fd with #1303; the five later bumps wait with 0980cce5f for the option-A entry», the founder's F1 «C сейчас, A отдельно (Recommended)» (relayed verbatim by the overseer, 2026-11-15, inputs#I2)
decision/upstream-take-engine-changes · reject — check 7: not_master type 8 — the set lengthens a row still true with one take's commit count, history the sidecar and the route hold; the eight commits' ratification (rul-47a6zcoz routine, F2 «Ратифицировать взятое (Recommended)», relayed verbatim by the overseer, 2026-11-15, inputs#I2) stands under decision/upstream-behaviour-changes-ratified-per-take as it reads, recorded in the report and this landing's sidecar entry, not PROVISIONAL

ops-security-plan.md
decision/engine-dependency-pins · apply — check 5: the plan's Expected amendment «security-plan · decision/engine-dependency-pins — "Taffy the registry 0.15" → git Taffy at e90751fd»
decision/no-git-dependencies · apply — check 5: the plan's Expected amendment «"taffy and parley are crates.io registry versions with checksums" stale for taffy», parley's half kept
decision/no-git-dependency · apply — check 1: no rule; the report's Dependencies bullet states the git source, its rev, no `[patch]` and no checksum line; the except restates the founder-ratified Taffy-only exception as taken, widening nothing past it
decision/upstream-sync-dependency-set · apply — check 5: the plan's Expected amendment «security-plan · decision/upstream-sync-dependency-set — the Taffy except (the lean above)», P5-approved (inputs#I4); the Taffy-only exception itself ratified earlier by the founder («Снять для Taffy»)
