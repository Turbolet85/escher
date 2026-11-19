# Fan-out results — 2026-11-19-build-directory-held

architecture: add 16 · set 5 · retire 0 · none 1 · ops-architecture.md
security-plan: add 11 · set 1 · retire 0 · none 1 · ops-security-plan.md
design-system: none · ops-design-system.md — reason: no UI, tokens n/a, no design row made false
layout-templates: add 9 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 8 · set 1 · retire 0 · none 4 · ops-test-plan.md
obs-plan: add 13 · set 0 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: no proposer
main: ops-main-test-plan.md (add 2 · set 1) · ops-main-obs-plan.md (add 1)

Homes (check 2 — one fact, one home, the master whose question it answers; the plan's reviewed Expected amendments map where it names one):
- the sweep's argv validated before acting → security-plan decision/target-sweep-argv-validation
- the sweep's exit codes 0–3 → architecture contract/target-sweep-exit-codes/* (beside agent-run, cold-agent, coverage-floor and apt-install exit codes)
- the sweep's stdout line and its closed keys → obs-plan contract/target-sweep-line/*
- no env read of its own → security-plan decision/target-sweep-reads-no-env · no log → obs-plan decision/target-sweep-writes-no-log (main)
- CI as an env value → security-plan contract/environment-variables/ci · CI as an occupied name → architecture name/other/ci
- removal scope, keep-newer and the lock refusal → test-plan decision/target-sweep-safe-around-builds
- the keep set's composition → architecture decision/target-sweep-keep-set
- fast owns the sweep, never reddened by it → test-plan decision/local-pre-push-gate (architecture's twin row and contract/ci-leg-legs/fast kept true beside it)
- the Windows pass-through → architecture decision/windows-entries-pass-through

ops-architecture.md
decision/run-state-areas-local-only · apply — check 5: plan's reviewed expected amendment (architecture §Occupied Resources: the sweep as a remover of target/debug units alone beside the state areas); report fct-yez7fafg substantiates
contract/ci-leg-legs/fast · apply — check 5: plan's reviewed expected amendment (contract/ci-leg-legs/fast gains the sweep step); fct-yez7fafg
decision/local-pre-push-gate · apply — check 1: Accurate this-chunk addition; the cascade keeps architecture's twin of test-plan's row true (site sweep: fast-step stale by omission)
contract/ci-leg-legs/embolden · apply — check 1: Accurate this-chunk addition; embolden stays the last leg, the sweep a step after it
decision/windows-entries-pass-through · apply — check 1: Accurate this-chunk addition; target-sweep.ps1 is a pass-through
contract/environment-variables/ci · reject — check 2: the fact's home is security-plan contract/environment-variables/ci (the plan names the environment values gaining CI in security-plan); architecture keeps the occupied name name/other/ci
name/other/ci · apply — check 5: plan's reviewed expected amendment (§Occupied Resources gains CI as an env name ci-leg.sh reads)
name/other/target-sweep-py · apply — check 1: Accurate this-chunk addition; precedent name/other/cold-agent-sh
name/other/target-sweep-ps1 · apply — check 1: Accurate this-chunk addition; precedent name/other/cold-agent-ps1
contract/target-sweep-invocation/verb · reject — check 2: the fact's home is security-plan decision/target-sweep-argv-validation
contract/target-sweep-exit-codes/0 · apply — check 1: Accurate this-chunk addition; the exit codes' one home (precedent agent-run-exit-codes, coverage-floor-exit-codes)
contract/target-sweep-exit-codes/1 · apply — check 1: Accurate this-chunk addition; the exit codes' one home
contract/target-sweep-exit-codes/2 · apply — check 1: Accurate this-chunk addition; the exit codes' one home
contract/target-sweep-exit-codes/3 · apply — check 1: Accurate this-chunk addition; the exit codes' one home
contract/target-sweep-stdout/line · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/line
contract/target-sweep-stdout/verb · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/verb
contract/target-sweep-stdout/outcome · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/outcome
contract/target-sweep-stdout/cause · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/cause
contract/target-sweep-stdout/counts · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/counts
decision/target-sweep-refuses-during-build · reject — check 2: the fact's home is test-plan decision/target-sweep-safe-around-builds
decision/target-sweep-keep-set · reject — check 7: not_master type 4 — the keep pass's construction is one mechanism the code holds (test-plan decision/target-sweep-safe-around-builds keeps the closed keep list's rule)

ops-layout-templates.md
contract/target-sweep-cli/usage · reject — check 2: the fact's home is security-plan decision/target-sweep-argv-validation
contract/target-sweep-cli/exit-2 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/2
contract/target-sweep-cli/stdout · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/line
contract/target-sweep-cli/exit-0 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/0
contract/target-sweep-cli/exit-1 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/1
contract/target-sweep-cli/exit-3 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/3
contract/target-sweep-cli/cargo-lock · reject — check 2: the fact's home is test-plan decision/target-sweep-safe-around-builds
contract/target-sweep-cli/windows-pass-through · reject — check 2: the fact's home is architecture decision/windows-entries-pass-through
contract/target-sweep-cli/removal-scope · reject — check 2: the fact's home is test-plan decision/target-sweep-safe-around-builds

ops-obs-plan.md
contract/target-sweep-line/line · apply — check 5: plan's reviewed expected amendment (obs-plan §9: the sweep's stdout of closed words and counts)
contract/target-sweep-line/verb · apply — check 5: the same entry, one row an item
contract/target-sweep-line/outcome · apply — check 5: the same entry, one row an item
contract/target-sweep-line/cause · apply — check 5: the same entry, one row an item
contract/target-sweep-line/counts · apply — check 5: the same entry, one row an item
contract/target-sweep-exit-codes/0 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/0
contract/target-sweep-exit-codes/1 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/1
contract/target-sweep-exit-codes/2 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/2
contract/target-sweep-exit-codes/3 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/3
decision/target-sweep-touches-only-stale-units · reject — check 2: the fact's home is test-plan decision/target-sweep-safe-around-builds (the state areas' side architecture decision/run-state-areas-local-only)
decision/target-sweep-no-log-no-env · reject — check 2: its env clause's home is security-plan decision/target-sweep-reads-no-env; its no-log clause stands alone as main's decision/target-sweep-writes-no-log
decision/fast-leg-owns-target-sweep · reject — check 2: the fact's home is test-plan decision/local-pre-push-gate
decision/target-sweep-refuses-under-build-lock · reject — check 2: the fact's home is test-plan decision/target-sweep-safe-around-builds

ops-security-plan.md
decision/target-sweep-argv-validation · apply — check 5: plan's reviewed expected amendment (security-plan §Input Validation: the sweep's argv refused before acting)
contract/target-sweep-cli/0 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/0
contract/target-sweep-cli/1 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/1
contract/target-sweep-cli/2 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/2
contract/target-sweep-cli/3 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/3
decision/target-sweep-removes-only-stale-units · reject — check 2: the fact's home is test-plan decision/target-sweep-safe-around-builds
decision/target-sweep-reads-no-env · apply — check 5: plan's reviewed expected amendment (security-plan: no env read of its own)
contract/environment-variables/ci · apply — check 5: plan's reviewed expected amendment (the environment values gain CI, read by ci-leg.sh fast alone); a flag of no content, no crossing
decision/target-sweep-line-closed-words · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/line, its no-log clause obs-plan decision/target-sweep-writes-no-log
decision/target-sweep-closed-refusal-causes · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/cause
decision/sweep-never-reddens-fast-leg · reject — check 2: the fact's home is test-plan decision/local-pre-push-gate
decision/locked-builds · apply — check 1: Accurate this-chunk addition; every keep run passes --locked (fct-yez7fafg)

ops-test-plan.md
decision/local-pre-push-gate · apply — check 5: plan's reviewed expected amendment (decision/local-pre-push-gate gains the sweep after a green verdict, skipped under CI)
decision/target-sweep-safe-around-builds · apply — check 5: plan's reviewed expected amendment (test-plan §9: the sweep, its keep list and lock)
contract/target-sweep-invocation/verbs · reject — check 2: the fact's home is security-plan decision/target-sweep-argv-validation
contract/target-sweep-invocation/environment · reject — check 2: the fact's home is security-plan decision/target-sweep-reads-no-env and obs-plan decision/target-sweep-writes-no-log
contract/target-sweep-stdout/line · reject — check 2: the fact's home is obs-plan contract/target-sweep-line/line
contract/target-sweep-exit-codes/0 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/0
contract/target-sweep-exit-codes/1 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/1
contract/target-sweep-exit-codes/2 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/2
contract/target-sweep-exit-codes/3 · reject — check 2: the fact's home is architecture contract/target-sweep-exit-codes/3

ops-main-test-plan.md
decision/build-directory-held-by-sweep · apply — check 5: plan's reviewed expected amendment (test-plan §9: the cause measured, an edit minting none), the proposer's @row_none F1 left it rowless
decision/local-baseline · reject — check 7: not_master type 5 — one run's measured sizes lengthen a row still true; the footprint stands in the report, the evidence and the sidecar (the expected entry so dispositioned)
decision/ci-set-fast-builds-apart · reject — check 7: not_master type 9 — an open measured limit is the route's CARRY (P5), stated as measured in the report's close (inputs#I4)

ops-main-obs-plan.md
decision/target-sweep-writes-no-log · reject — check 7: not_master type 4 — the code holds it; the one-line record is obs-plan contract/target-sweep-line/line's promise

Check 6 — spec claims disproved: the three (one hash in all four directories; no Compiling line; the deps count unchanged) are the plan's wording, no master site (the pre-pass sweep's controls for incremental-dir, compiling-line and deps-count never fired over the masters); disposed in the report's close (Outcome, final, the two controls in substance) and test-plan's @row_none F7.
Check 5 — every Expected amendments entry dispositioned: test-plan §9 (cause, sweep, day numbers, pre-push gate, local-baseline) · architecture contract/ci-leg-legs/fast · architecture §Occupied Resources (remover, CI name) · security-plan (argv, no env, CI value) · obs-plan §9 (stdout line; touching only units → test-plan's safe-around-builds row and architecture's state-areas row).
Check 3 — scope record: none; no line outside the intent.
Check 1 — no boundary widening: the sweep is a dev-host script reached by no carrier, and CI is a content-free flag read by ci-leg.sh (the operator's P5 word, inputs#I3).
