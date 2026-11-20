# Fan-out results — 2026-11-20-browser-s-surviving-mutants (wrap 2026-11-20T14-24-05-wrap)

architecture: no proposer
security-plan: none · ops-security-plan.md — reason: test code alone under cfg(test) in the browser crate; no dependency, input surface, auth or secret handling moved
design-system: no proposer
layout-templates: no proposer
test-plan: add 1 · set 1 · retire 0 · none 0 · ops-test-plan.md
obs-plan: no proposer
a11y-plan: no proposer

ops-test-plan.md
decision/unit-tests-in-source-file · reject — check 2: the fact's home is test-plan decision/inline-unit-test-modules (its except: unit tests out of line in a sibling file with the _tests suffix, declared as a cfg(test) module by path), standing again in decision/inline-cfg-test-modules; capture_tests.rs follows it, so a third copy in this row would hold one fact twice
decision/mutation-filter-anchored-on-verb · apply — check 1: routine, Accurate this-chunk addition — the scoped-filter convention is the operator's mend of plan entry 10 (report ## After implement, Decisions & corrections after implement: the sweep hazard and the mend `--re 'replace render_scene '`, `--list` read first expecting exactly 4); a mutation-run convention future runs must respect, in test-plan's quality-gates area beside the mutation-run rules
