# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the fast and doc legs and the stand-repeat leg's three zero-failure rounds over the stand_*.rs glob, which holds the touched stand_requirement_sweep, must stay green
ci-integration#2.1 · no · the cache wiring and its timings; the chunk changes no workflow, job or cache, and the task's Not needed names caches
ci-integration#3.1 · no · the apt-install bound and its retries; the chunk changes no install step and no leg of ci-leg.sh calls the script
ci-integration#4.1 · yes · the two builds differ in engine features (tracing on under cargo test --workspace alone) and the scope names both, so both touched tests must pass under each
ci-integration#5.1 · no · per-OS CI test tallies of past runs, re-read at each wrap; the chunk adds or removes no test, so no count it records moves
ci-integration#6.1 · yes · the baseline's exit-0 reads of ci-leg.sh fmt, clippy, doc and coverage are the gates the fast and doc legs and escher-driver's 97 % floor must keep green
ci-integration#7.1 · yes · the coverage leg's coverage_floor.py run at escher-driver=97, which the new helpers in findings must keep the crate above
