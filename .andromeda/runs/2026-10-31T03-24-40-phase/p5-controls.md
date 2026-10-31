# P5 controls — 2026-10-31-engine-debts

Check 4 (9)'s controls, each fired as the entry through `gate.py run --plan … --entry {n} --project-root {clone}`, no `--run-dir`, over a scratch clone (`git clone --shared` of the project, checked out at `2109f1c2ecaf56e22db53d7af04d07c32f3db400`) under `$TMPDIR/{scratchpad}/ctl.*`, which each run removed. Entry numbers are the block's as read at this P5. Fired 2026-10-31T07:46Z to 07:47Z.

| Entry | Input in the clone | Reading |
|---|---|---|
| 1, the preservation guard | unmodified | green, exit 0 |
| 1 | must fail: a line appended to `packages/blitz-paint/src/lib.rs` | red, exit 1 |
| 1 | must fail: a line appended to `deny.toml` | red, exit 1 |
| 1 | must pass: a line appended to `packages/blitz-dom/src/util.rs`, outside the guard | green, exit 0 |
| 2, the silence census | unmodified: the five new files absent | red, exit 2, as on the tree |
| 2 | must pass: the five files minted holding no token | green, exit 1, last line 0 |
| 2 | must fail: a `println!` in one | red, exit 0 |
| 3, the driver's span sites | unmodified | green, last line 2 |
| 3 | must fail: a third `info_span!` in `lint.rs` | red, last line 3 |
| 4, the added-lines census | unmodified: no change yet | green, exit 1, last line 0 (an empty input) |
| 4 | must pass: a plain comment added in `util.rs` | green, exit 1, last line 0 |
| 4 | must fail: an added `println!` in `util.rs` | red, exit 0 |
| 4 | held out: a `println!` in `packages/blitz-shell/src/event.rs` | green, exit 1, last line 0 |
| 8, the old limit's word | unmodified | red, exit 0, the word in the driver's texts and the statement |
| 8 | must pass: the word replaced in `lib.rs`, `refusal.rs`, `docs/capabilities.md` | green, exit 1, last line 0 |
| 19, the file count | unmodified | red, last line 148, as on the tree |
| 19 | must pass: the five new files minted | green, last line 153 |
| 22, the audit | must pass: `cargo update -p pulldown-cmark --precise 0.13.5` in the clone | green, `advisories ok, licenses ok` |
| 23, the lock's delta | unmodified | green, exit 1, last line 0 (an empty input) |
| 23 | must pass: that one bump | green, exit 1, last line 0 |
| 23 | must fail: `cargo update -p clipboard-win` in place of it | red, exit 0 |

**The audit's red on the untouched tree**, 2026-10-31T07:47Z: RUSTSEC-2026-0414 on `pulldown-cmark` 0.13.4, reached through `rdme`, `advisories FAILED, licenses ok`. The bump moves one package, four lock lines (`git diff --stat` 2 insertions, 2 deletions), and `apps/readme/src/markdown/pulldown_cmark.rs:7-12` never sets `ENABLE_WIKILINKS`. `clipboard-win` 5.4.1 reads a yanked warning and no failure, on the base and over the bump alike.

**A slip, no harm:** a first firing of these controls used numbers from before the lock's entry landed, so it ran `ci-leg.sh a11y` and `agent-run.sh boot` on the tree and in a clone. Each read green and neither writes outside `target/`. `git status --short -- packages tests examples docs .github Cargo.lock` printed nothing afterwards.

**One control found the entry wrong, and the entry moved.** Entry 2's first spelling named `session_common/mod.rs`, and its must-pass read red: `std::env::current_exe` at `:345` matched `std::env`. That is a path for `shot_prefix`'s re-run and no env read, and this chunk does not edit the module, so it left the list by file, as `.andromeda/obs-plan-detail/log-coverage.md:124`'s own list does. The rows above are the re-spelt entry's.

**The job-log read**, the last operator entry's text with `<id>` = 40487650912, its log read to a scratch file first, 2026-10-31 at this P5: Test [default features] 209 · 1125 · 0 · 12; Test (macos) 190 · 1122 · 0 · 12; Coverage report 190 · 1122 · 0 · 12; Test (windows) 190 · 1076 · 0 · 9; Accessibility (a11y) tests 14 · 85 · 0 · 0; Stand checks repeated 204 · 864 · 0 · 21, equal to `.andromeda/test-plan-detail/ci-integration.md:50`.

**The literal `Running tests/{name}.rs`** in the new targets' entry needs no must-pass: it is cargo's own stderr form, read in `target/ci-logs/test.log` by the standing every-target entry, the representation the entry reads.
