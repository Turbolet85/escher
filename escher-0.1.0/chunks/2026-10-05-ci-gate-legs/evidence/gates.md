# Host gate readings — /implement run 2026-10-05T22-50-45 (plan step 9)

Final full block (`gate.py run`, bare): `entries 19 · green 14 · red 0 · recorded 1 · timeout 0 · not-run 4`
(the four `leg = 'operator'` entries — the hygiene read, the push, the CI conclusion, the per-job timings).

Numbering note: the plan's prose counts host entries 1-14 and operator entries 15-18, but the fenced block holds 19
entries — the tool numbers the host gates 1-15 (entry 15 is the report-only Cargo.lock delta) and the operator pass
16-19. The acceptance text's "entry 17" (the `ci.py conclusion` read) is the tool's entry **18**; the timings read
(`gh run view <id>`) is entry **19**.

| entry | gate | reading |
|---|---|---|
| 1 | `python3 -m unittest discover -s .github/scripts` | `Ran 23 tests` · `OK` (16 at base) |
| 5 | workspace test counts over `target/ci-logs/test.log` | `407 0 3` — the test-plan §9 Local baseline, unchanged |
| 6 | `ci-leg.sh doc` (`cargo doc --workspace --no-deps --locked`, `RUSTDOCFLAGS="-D warnings"`) | exit 0 · `Generated …/target/doc/accesskit_xplat/index.html and 26 other files` · no `output filename collision` line (the cold run of the same command at 22:52Z also exit 0, 0 `error`/`warning` lines) |
| 7 | `ci-leg.sh audit` | `advisories ok` (one per-ID ignore, RUSTSEC-2026-0192 — `evidence/audit.md`) |
| 8 | `ci-leg.sh a11y` | `running 6 tests` / `running 6 tests` / `running 3 tests`; three `test result: ok.` (6 + 6 + 3 = 15 passed) |
| 9 | `ci-leg.sh coverage` | exit 0; `TOTAL … 54417 regions, 25885 missed, 52.43% · 3593 functions, 1634 missed, 54.52% · 34900 lines, 16321 missed, 53.23%` — no threshold. The instrumented test run sums `404 0 3` (cargo-llvm-cov runs no doctests by default). `warning: 167 functions have mismatched data` printed by llvm-cov |
| 10 | `test -s target/coverage/lcov.info` | exit 0 (151 `SF:` source files) |
| 11 | unpinned `uses:` lines in ci.yml | `0` (base: 39) |
| 12 | a11y-plan §9 search over ci.yml | `5` (base: 0) |
| 13 | root `Cargo.toml` diff lines | `0` |
| 14 | cfg / tracing / println! outside doc comments in the packages+examples diff | `0` |
| 15 | Cargo.lock name/version delta (recorded) | `-version = "0.23.43"` / `+version = "0.23.45"` — rustls alone, as predicted |

The coverage line totals moved by a few lines across this session's three reads (a hand `cargo llvm-cov report
--workspace` over the first instrumented run's data, then the two green gate runs: missed lines 16296 → 16302 → 16321
of 34900) — run-to-run variance of the instrumented test set, reported as read; no threshold reads it.
