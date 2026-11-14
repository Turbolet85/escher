# Fan-out results — 2026-11-14-seeded-randomness (wrap .andromeda/runs/2026-11-14T07-12-40-wrap)

architecture: add 8 · set 1 · retire 0 · none 0 · ops-architecture.md (re-spawned once, EVERY_TASK_FACT; the first file `.raw-ops-architecture.md`)
security-plan: add 3 · set 0 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: add 2 · set 0 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 4 · set 1 · retire 0 · none 2 · ops-test-plan.md
obs-plan: add 3 · set 1 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md — reason: no product UI, accessibility-tree, ARIA, focus, keyboard, a11y or log-schema change; every picked row true as written.

## Dispositions

ops-architecture.md
decision/accessibility-feature-gating · apply — check 5: the plan's expected amendment (Cross-cutting Patterns, `seed` joins the ungated modules), the report's `ungated` sweep add-to `dec-wdtb3kpq`; rul-ipb3atbm
contract/dioxus-native-dom-api/seeded-random · apply — check 5: the plan's expected amendment (Standard Contracts, the seeded source), report Symbols / APIs; rul-ipb3atbm
contract/dioxus-native-dom-api/use-seeded-random · apply — check 5: the plan's expected amendment (Standard Contracts, `use_seeded_random` and `DEFAULT_SEED`), report Symbols / APIs; rul-ipb3atbm
contract/test-harness/run-seed · apply — check 5: the plan's expected amendment (the harness's default run seed beside `contract/test-harness/virtual-clock`), report Symbols / APIs; rul-ipb3atbm
decision/one-run-seed · apply — check 5: the plan's expected amendment (Cross-cutting Patterns, the run seed as a root context, ungated); the one home of the one-seed pattern (check 2); rul-ipb3atbm
decision/no-seed-on-carriers · reject — check 3: its rule reads «every run reads the default seed», false for an in-process run whose builder gives a root seed (report Symbols / APIs, `from_vdom`), and names `HarnessOptions`, no carrier; main's own add of the same handle in ops-main-architecture.md
absence/seeded-page-script-random · reject — check 7: not_master type 9 — an owed item the route's residual and the incubator's deferred list hold, not a capability gone without; its owner the residual P5 writes
decision/test-binary-reexec-spawn · reject — check 7: not_master type 4 — a test's own spawn pattern, no reason, threat or ratification; the test code holds it (the plan's expected amendment so dispositioned)
decision/no-prng-crate · apply — check 1: rul-ipb3atbm — the report's Dependencies (no crate, splitmix64 by hand, `RandomState` std) and Reverted / negative API facts (a PRNG crate rejected)

ops-main-architecture.md
decision/no-seed-on-carriers · apply — check 5: the plan's expected amendment (Standard Contracts, the absence, F1's A PROVISIONAL), named by the entry itself — settled without a halt as a recorded direction (check 1); an absence, no widening; state provisional for the founder (`inputs#I2`, `inputs#I4`, `inputs#I5`)

ops-security-plan.md
decision/no-run-seed-on-driver-carriers · reject — check 2: the fact's home is architecture decision/no-seed-on-carriers
decision/seeded-source-in-process-seed-only · apply — check 1: rul-ipb3atbm — report Coverage of new surfaces (takes a `u64` from in-process code alone, no external input, no env read) and Symbols / APIs (`from_host`, std only)
decision/seeded-source-holds-no-state-out · reject — check 2: the fact's home is obs-plan decision/seeded-random-debug-type-name-only, which fit read not_master type 4 — the code holds it

ops-layout-templates.md
decision/stand-boot-default-run-seed · reject — check 2: the fact's home is architecture decision/one-run-seed (with contract/test-harness/run-seed)
decision/no-seed-on-driver-carriers · reject — check 2: the fact's home is architecture decision/no-seed-on-carriers

ops-test-plan.md
contract/harness-construction/from-vdom · apply — check 5: the plan's expected amendment via the report's `from_vdom` sweep add-to `ctr-fr4ocsu2`; rul-ipb3atbm
decision/run-randomness-from-one-seed · reject — check 2: the fact's home is architecture decision/one-run-seed
decision/seeded-runs-checked-in-both-modes · reject — check 7: not_master type 5 — the new check's tests retold, no choice or reason; the test file `stand_seeded_randomness` holds it (the plan's expected amendment so dispositioned)
decision/no-seed-on-driver-carriers · reject — check 2: the fact's home is architecture decision/no-seed-on-carriers
decision/second-process-by-test-binary-reexec · reject — check 2: the fact's home is architecture decision/test-binary-reexec-spawn

ops-obs-plan.md
decision/checks-silent · apply — check 1: rul-ipb3atbm — report Symbols / APIs (the re-run child prints `seeded-digest {mode} {digest}` lines alone, read by its parent)
decision/seeded-random-source-silent · reject — check 7: not_master type 5 — one component's entry in the silence census, no choice or reason; the census gate and the code hold it (the plan's expected amendment so dispositioned)
decision/no-run-seed-on-carriers · reject — check 2: the fact's home is architecture decision/no-seed-on-carriers
decision/seeded-random-debug-type-name-only · reject — check 7: not_master type 4 — how one type's hand `Debug` is built, PII n/a; the code and its unit test `debug_writes_no_state` hold it
