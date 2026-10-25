# Classified — tests · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.0 · no · a whole-workspace coverage reading, dropped with the coverage leg as no crate changes and the statement claims no coverage
quality-gates-coverage-targets#2.1 · no · the per-crate coverage floors and coverage_floor.py, dropped with the coverage leg as no crate changes; the README edit reaches neither ci-leg.sh nor coverage_floor.py
quality-gates-coverage-targets#3.1 · no · the stand-repeat leg's zero-failure budget, rounds and counts, dropped as no test target is added, removed or renamed; stand-repeat is no fast leg, so the done-gate never runs it
quality-gates-coverage-targets#4.1 · no · the mutation tier, dropped as no unit of code is written or changed; it runs on the dev host alone, never in the done-gate
