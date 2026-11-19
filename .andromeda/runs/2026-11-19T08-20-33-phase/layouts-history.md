@history_item n=1 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added `scripts/agent-run.sh` to §Surface: cli (verbs boot · run · status · cleanup · logs, selections stand · all · a blitz-tests file name, usage on stderr exit 2, stdout JSON lines only, raw cargo output kept in `target/agent-run/run.log`, `scripts/agent-run.ps1` pass-through). This is the source of the contract/agent-run-cli/* rows. The chunk measures agent-run.sh's cargo invocations and may change them as part of the mend. Its `run.log` lives under `target/`, so a sweep must keep it. A selection keyed by blitz-tests file name is at risk if the mend merges the per-file test binaries.

@history_item n=2 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage`: added the closed `session` selection, matched before a file name, which runs the escher-session binary's flows in seven_guis. A mend that turns per-file binaries into one per leg changes what "a blitz-tests file name" selects and how `session` reaches seven_guis's tests. If so, the usage row is amended again, as it was here.

@history_item n=3 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added `scripts/cold-agent.sh` (verbs run · status · cleanup · logs, JSON-lines stdout, transcript in `target/cold-agent/transcript.jsonl`, `.ps1` pass-through). This is the pipe whose cargo footprint the chunk measures. Its artifacts sit under `target/`, so a sweep must not remove them.

@history_item n=4 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent `run` needs an already built `target/debug/escher-session` and exits 3 because the pipe builds nothing. This is the origin of decision/cold-agent-pipe-builds-nothing and contract/cold-agent-cli/exit-3. Constraint: measuring or mending must not add a build step to the pipe. A sweep of `target/debug` that removes the binary turns the pipe's precondition into a failure.

@history_item n=5 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the pipe's live server child runs `mcp --record` into `target/cold-agent/record` (`mkdir -m 700`, owner-only, removed by `cleanup`). This is the origin of contract/cold-agent-cli/record-dir. Trap: a sweep that "touches `target/` alone" must not delete this owner-only record directory or the checks kept in it. Cleanup owns that directory.

@history_item n=6 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>`: `check-{n}.json` written 0600 in a 0700 directory and never removed by escher (later rows: removed only when its recording fails). This is the source of decision/session-recording-replayable-check and contract/escher-session-record/check-file. A `target/` sweep must respect these kept checks.

@history_item n=7 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds each call's verb, arguments, acceptance, answer digest and masked answer, and `Check::replay` replays it. This explains the "replayable" in decision/session-recording-replayable-check. Replays run without recording, so the coverage and replay tests are among the invocations the chunk measures.

@history_item n=8 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: cli: 2 rows) — `~ contract/cold-agent-cli/usage` changed six apps to seven, and `+ contract/escher-session-served-tasks/order` was added. This is the latest state of the cold-agent usage row that the chunk's pipe measurement rests on.

@history_item n=9 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native: 8 rows) — `~ decision/stand-lints-clean-at-boot` now covers six pages, including the order page, and `+ contract/stand-element-ids/order-*` was added. Every stand test that guards these rows (stand_lint and the id checks) must keep its assertions and both layout modes if the chunk merges or moves test files.

@history_item n=10 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. This is the origin of decision/headless-stand-pinned-viewport. Stand test files moved into one binary per leg must keep this pinned harness default and its per-test fresh mount.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the escher-session host is the third consumer of the TaskShell mount, and a held instance reads as a fresh boot. This is the origin of decision/held-instance-reads-as-fresh-boot (guard `stand_session_fresh`). The guard test must keep its assertions and both layout modes under any test-layout change.

@history_item n=12 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — the citations into the harness `Default` impl that holds 800x600, scale 1 and light were re-pointed after lines were added above them. Precedent: a test-harness or layout move shifts cited lines, and the master's citations must be re-pointed at wrap.

@history_item n=13 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount, and escher-session's coverage role) — the coverage role replays kept checks on fresh mounts, and the held instance still reads as a fresh boot in both layout modes. Ratified by 2026-11-15T05-21-42-wrap. The coverage replay path and its tests are among the cargo work the chunk measures. Distinct from the `coverage` CI leg (llvm-cov), but both exist.

@history_item n=14 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — author ids on the non-lean tasks (temp-*, circle-*, cell-*), and `unkeyed_actionable()` reads 0 on all seven tasks. This is the source of decision/every-stand-control-keyed and the temp/circle/cell id rows. The tests that read it must keep their assertions if their files are merged into one binary.

@history_item n=15 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — `crud-person-{person.id}` and `task-card-{slug}` ids were added, the founder's rule that every actionable element reads an author key. These are the sources of contract/stand-element-ids/crud-person-id and task-card-* rows, and this chunk's test moves must not change what is asserted about them.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (desktop-native) — `flight-type` select ids replaced the toggle buttons, and the boot-lint reading gained the load page. These are the sources of decision/flight-type-select-boot-reading and the flight-type* rows, guarded by `stand_snapshot`, whose two-layout-mode test must survive a test-file consolidation unchanged.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the citations the chunk's line moves re-pointed, the load page's Tab sequence, the Cells grid, `stand_contrast` over the load page and Cells, the flight booker's boot lint reading) — 13 citations into test files (`stand_contrast.rs`, `stand_lint.rs`, `stand_load_page.rs`) and `LOAD_TAB_ORDER` / `CELLS_TAB_ORDER` in `tests/blitz-tests/tests/common/mod.rs` were re-pointed. `stand_lint`'s `at_boot` is `[0, 0, 0, 0]` in both layout modes, and `cells-grid` is the 677th reacting element read by `stand_non_lean_tasks`. Trap: merging or moving the per-file blitz-tests into one binary per leg moves these cited paths and lines. The wrap must re-point them, and the tests keep their assertions.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the citations the chunk's line moves re-pointed) — six citations, four of them into `stand_contrast.rs`, were re-pointed. This is the same precedent: the master cites stand test files by path and line, so a test-layout mend owes citation re-points.

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the tab-order citations after the actionable-at-boot table) — the load page and Cells grid citations were re-pointed to `tests/blitz-tests/tests/common/mod.rs:104-109`. Moving or restructuring `tests/blitz-tests/tests/` (for example a shared module under a single test binary) breaks this citation, so it must be re-pointed.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint at boot in both layout modes. This is the origin of decision/stand-lints-clean-at-boot (guard `stand_lint`), a guard test that must keep its assertions and both modes through any consolidation.

@history_item n=21 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (decision · contract · absence), with guards re-pointed and each row verified against the code. This explains the row handles and `guard:` fields (stand_session_fresh, stand_lint) that this chunk's test-layout changes must keep pointing at real tests.
