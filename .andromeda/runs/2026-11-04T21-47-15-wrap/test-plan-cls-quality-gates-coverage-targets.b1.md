@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the session changes no coverage floor, leg or measured source, and the first 2026-10-05 line-coverage reading is a dated measurement it does not re-read
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=no
why: the session changes no per-crate floor, coverage_floor.py or ci-leg.sh coverage leg and re-reads no coverage, so the dated per-crate readings stand as measured
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: the stand-repeat leg, its 51 stand_ files and its seventeen named targets are unchanged, and dioxus_runtime_probe has no stand_ prefix and is named in neither
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=no
why: the mutants profile and the scoped mutation targets are unchanged, and the Cargo.lock move is only pulldown-cmark 0.13.5 to 0.13.6, so no lockfile still resolves cargo-mutants or cargo-nextest
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the session changes no function the dated cognitive-complexity reading names and re-reads no complexity, and that 85-function count claims no later tree
