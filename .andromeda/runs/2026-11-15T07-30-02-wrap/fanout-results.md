# Fan-out results — 2026-11-15-upstream-sync-errors-and-explanation (resumed wrap, run dir .andromeda/runs/2026-11-15T07-30-02-wrap)

architecture: add 0 · set 4 · retire 0 · none 0 · ops-architecture.md
security-plan: add 0 · set 3 · retire 0 · none 0 · ops-security-plan.md
design-system: add 1 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: none · ops-layout-templates.md
test-plan: add 1 · set 0 · retire 0 · none 0 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md

## Dispositions

ops-architecture.md
decision/font-face-css-descriptors · apply — check 1: rule «An engine behaviour change for every document» (routine) — the take's `font-stretch` read, ratified on the founder's own word (Q2, inputs#I6 §2); expected amendment 5
decision/no-git-dependencies · reject — check 2: one fact, one form — the Taffy exception stands in the `except` cell as its two security-plan twins write it; main's ops-main-architecture.md decision/no-git-dependencies carries it, and the rule cell stays true as written
decision/deferred-upstream-dependency-bumps · apply — check 5: expected amendment 2, the Taffy-main content by registry 0.15.1 save #1303, the remainder waiting for the git-Taffy entry (addendum 1)
decision/mutation-tier-host-only · reject — check 2: the fact's home is architecture decision/mutation-tier-excludes-demo-shells (main's add) — an exclusion of two units is its own decision, not an exception to the host-only rule

ops-security-plan.md
decision/no-git-dependency · escalate → Q1 — rule «Boundary widening» (escalate): a reversal of a locked decision that admits a git source into the dependency graph, on the founder's own word «Снять для Taffy» (inputs#I6 §3) · resolved: apply (Q1) — operator, 2026-11-15: Apply all three
decision/no-git-dependencies · escalate → Q1 — the same exception at the rule's second security-plan site (inputs#I6 §3) · resolved: apply (Q1) — operator, 2026-11-15: Apply all three
decision/upstream-sync-dependency-set · apply — check 1: rule «Accurate this-chunk addition» (routine) — the take's lock-only, semver-compatible registry move named in the rule; expected amendment 4

ops-design-system.md
decision/font-face-registered-with-width · reject — check 2: the fact's home is architecture decision/font-face-css-descriptors — an engine font-matching rule, not a design token

ops-test-plan.md
decision/mutation-tier-excludes-demo-shells · reject — check 2: the fact's home is architecture decision/mutation-tier-excludes-demo-shells — the code audit reads its extra exclusions only from an architecture decision row

ops-main-architecture.md
decision/no-git-dependencies · escalate → Q1 — rule «Boundary widening» (escalate): the Taffy exception at the rule's architecture site, on the founder's own word «Снять для Taffy» (inputs#I6 §3) · resolved: apply (Q1) — operator, 2026-11-15: Apply all three
decision/upstream-take-engine-changes · reject — check 7: not_master type 8 — which take was ratified with which changes is history the sidecar holds; the two changes stand ratified in decision/font-face-css-descriptors and decision/deferred-upstream-dependency-bumps, and the standing row stays true of its own take
decision/mutation-tier-excludes-demo-shells · apply — check 1: no rule matches; the founder's recorded word «Вывести из охвата (Recommended)» (inputs#I6 §4), acted on by the operator's direction (inputs#I7), settles the add — expected amendment 8 (addendum 1)
