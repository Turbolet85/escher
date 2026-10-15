# Scope — Quality gates (`2026-10-14-quality-gates`)

**Working entry (escher-0.1.0/working-route.md:98):** "Quality gates — coverage floor on driver crates and flakiness
budget for stand checks, enforced on fork CI (per test-plan §10)", with twelve `CARRY:` blocks folded below.

## What the chunk builds

- **A coverage floor on the driver crates, enforced on fork CI.** Today the `coverage` leg runs `cargo llvm-cov
  --workspace --locked --lcov …` and then `cargo llvm-cov report`, with no threshold (test-plan §9 Coverage); the one
  recorded reading is 53.23 % workspace-wide (2026-10-05, test-plan §10), long before the driver existed. The chunk
  measures the driver crates' line coverage now and sets a floor the coverage leg fails under.
- **"Driver crates" is a set to be named, not assumed** (verified at P3: research.md, Graph impact and Readings).
  `escher-driver` is one by name; its usage edges reach `dioxus-native-dom` and `blitz-test-harness` (escher's own) and
  `blitz-dom` · `blitz-traits` (engine); `escher-telemetry` is not a dependency of the driver crate — the binaries install
  it. Line coverage at HEAD: escher-driver 98.20 %, escher-telemetry 96.48 %, blitz-test-harness 91.69 %,
  dioxus-native-dom 84.25 %, identical in CI. The set is a P4 fork.
- **The floor is measured, then set at or below the reading** — never a number above what HEAD reads.
  [premise-corrected: `cargo llvm-cov report --fail-under-lines` gates the report's TOTAL only, so a per-crate floor is a
  script of ours reading `target/coverage/lcov.info` per crate directory — research.md, New files]
- **A flakiness budget for the stand checks, enforced on fork CI.** test-plan §10 reads NOT YET MEASURED for a
  flakiness budget. The chunk defines what a flake is for a stand check, measures the stand checks' repeat behaviour, and
  sets a budget CI enforces.
- **"Stand checks" is `tests/blitz-tests/tests/stand_*.rs`** (verified at P3: the last `run stand` read 168 · 0 · 7 in
  55 s warm, `target/agent-run/events.jsonl`) and, by their role, seven_guis' process-level checks of the stand binary
  (`cli_*`, `host_*`, `mcp_*`); which of them the budget covers and how CI enforces it is a P4 fork.
- **test-plan §10 moves from NOT YET MEASURED to the gate values set**, with their measured readings (a wrap amendment;
  this chunk never edits a master).

## Boundaries

- **No mutation runs (inputs#I1).** The founder's word, relayed by the operator: no `cargo mutants` run in this chunk,
  and no mutation-score gate — a gate's evidence is coverage, repeat runs and named checks, never a mutation reading.
- **Every fork that is the founder's is raised as a dialog (inputs#I1):** a new CI gate (the licence gate by name), an
  engine change for every document, a boundary widening. None of the three is decided by a lean.
- **Every deferral becomes a `.andromeda/residuals.md` line naming the founder's label (inputs#I1)** — proposed on the P5
  card's `## Beyond the plan`; the wrap writes it.
- **No `cargo clean` without the founder's word (inputs#I1).** `target/` reads 552 GB with 360 GB free on the project
  drive (verified at P3); the coverage leg reused `target/llvm-cov-target` (30 GB) at HEAD, so the plan reads its own disk
  cost and never cleans.
- **Pipeline tools run bare or redirected to a file read whole (inputs#I1).**
- No new listener, port, env var or workspace crate (verified at P3: research's lists mint none); a CI job added or a
  leg changed is a test-plan §9 and `.github/scripts/test_ci_workflows.py` matter, and a new gate is one of the forks
  above.

## Folded freight (the twelve CARRYs, each re-verified at P3)

1. **Audit reach.** cargo-deny 0.20.2's resolved graph prunes `http-cache` (blitz-net's `cache` feature) and `ravif`, so
   RUSTSEC-2024-0436 (paste 1.0.15, unmaintained) and RUSTSEC-2026-0186 (memmap2 0.5.10, unsound) never reach the `audit`
   leg though both are in the build graph; close it — a lockfile-wide scan beside it, or a graph that reaches them
   (measured at 2026-10-05-ci-gate-legs evidence/audit.md; pinned on the overseer's word, delegate overseer).
   - The mechanism as the CARRY states it, kept verbatim: "cargo-deny 0.20.2's resolved graph prunes `http-cache`
     (blitz-net's `cache` feature) and `ravif`" — measured 2026-10-05, verified at HEAD at P3: `advisories ok` while
     `Cargo.lock` holds `paste 1.0.15` and `memmap2 0.5.10`, both reached by `cargo tree --workspace --all-features`.
     [premise-corrected: a graph that reaches paste exists — `cargo deny --locked --workspace --all-features check
     advisories` reads `advisories FAILED` on RUSTSEC-2024-0436 alone; memmap2's RUSTSEC-2026-0186 is an `unsound`
     informational advisory that no invocation tried reports at `deny.toml`'s defaults (research.md, Readings)]
   - Whether a scan that can turn the audit leg red on a pruned-graph advisory is a widening of an existing gate (the
     overseer's pinned word) or a new CI gate (the founder's) is a P4 fork (verified as open at P3).
2. **Release metadata.** The root `Cargo.toml` `homepage` and `repository` still read `https://github.com/dioxuslabs/blitz`
   and become escher's (`https://github.com/Turbolet85/escher`); the README is already escher's page (directed by the
   operator, 2026-10-06).
3. **Two blitz-dom unit tests assert nothing.** `hover_state_tests` in `packages/blitz-dom/src/document.rs`
   (`hovering_text_in_anonymous_block_reports_text_cursor`, `hovering_anonymous_block_whitespace_reports_default_cursor`)
   print "skipping: no usable font" and pass with system fonts on, while test-plan §1 lists "hover cursor" among
   blitz-dom's covered behaviours.
   - The CARRY's mechanism, kept verbatim with its marker: "hypothesis: their fixture builds its elements with the
     one-argument `qual_name!`, a no-namespace element the default stylesheet's type selectors do not match, so `body`
     and `div` lay out inline and the text never gets the height the skip tests for (that mechanism measured on that
     chunk's own `changed_set_` fixture, not on these two tests)". Verified at P3 as far as HEAD can show it without an
     edit: both tests print the skip and pass under `--features system-fonts`, the fixture is one-argument throughout
     (`packages/blitz-dom/src/document.rs:2957-2961`), and the same file's `changed_set_` fixture builds the same
     container-and-block shape two-argument (`document.rs:3646-3648`). The hypothesis is decided by the fix's own
     control: with the two-argument fixture `text_has_size` must read true, else the step stops.
4. **The command span's `busy` field has no reading on a real line.** No stand step reads not settled, so no sink capture
   carries `busy`; the busy class's logged word is pinned by a unit test in `execute.rs` only; `stand_settle` already
   holds fixtures that end at the settle bound without going quiet.
5. **What the bounded package install leaves unhandled and unwitnessed.** A bound firing inside `apt-get install` can
   leave dpkg interrupted, which `.github/scripts/apt-install.sh` does not handle; the script ran on real runners in eight
   jobs, not nine (the `ci-scripts` job's guarded `python3-yaml` call unreached where PyYAML is on the image). Decide here
   whether either needs more than its record.
   - "can leave dpkg interrupted, and the next attempt's install may then fail for that reason" is stated as a named
     limit, not measured (the CARRY's own words) — verified at P3: `attempt_once` runs update then install with no dpkg
     repair between attempts (`.github/scripts/apt-install.sh:50-55`).
6. **The licence-gate question, for the founder.** `deny.toml` holds upstream's `[licenses]` table, byte-identical; no
   leg runs it; the one reading (`cargo deny --locked check licenses`, six-target graph) is exit 0, `licenses ok`, with
   two unmatched allowances, `NCSA` and `Unicode-DFS-2016`; upstream reads it with no target filter. Undecided and the
   founder's: whether the fork gates on licences, and over which graph — **a founder dialog (inputs#I1)**.
7. **Upstream lints blitz-dom with a named feature set the fork does not run** (`cargo clippy -p blitz-dom
   --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets -- -D
   warnings`); decide here whether the fork runs that line.
   - Running it on fork CI is a new CI gate — a founder fork (inputs#I1). [premise-corrected: the fork's `clippy` leg
     already lints blitz-dom with every one of those features (the workspace build turns all blitz-dom defaults on);
     what upstream's line adds is `--all-targets` (tests and benches) and the build without `text-transform-icu` — and
     it reads green at HEAD, exit 0, 0 warnings (research.md, Readings)]
8. **Per-OS test tallies not attributed.** CI#38013740580 on `9462a7e4`: linux 158 result lines (719 · 0 · 10), while
   `Coverage report`, `Test (macos)` and `Test (windows)` print 139 (macos and coverage 716 · 10, windows 712 · 8);
   known: `stand_session_lifecycle` is `cfg(unix)`; not read: why 139 and not 158, which tests make 716 − 712 and 10 − 8,
   whether the differences stood before the merge, and what the ios and android `test` steps ran.
   - Attribution is read on the latest green run (CI#38649787488) — verified at P3, every difference attributed:
     linux − macos = linux − coverage = the 19 `Doc-tests` lines (3 doctests pass); macos − windows = 22 passed · 3
     ignored, every one a `cfg(unix)` session or command-line check; ios and android build only (`cargo build --all`), no
     test step. The 2026-10-10 gap (158 − 139 = 19) is the same doctest cause (research.md, Readings).
9. **A check of ours on the hint-less font source** (the founder's ruling, 2026-10-10): upstream #1109 fetches an
   `@font-face` source with no format hint whose URL has no extension (a `data:` URL) and sniffs its format from its
   bytes; a check of ours on that path is owed before 0.1.0 ships (security-plan §Input Validation, the `@font-face` row).
10. **Two command-line paths with no named check, and three readings not taken** (from 2026-10-10-driver-cli): a write to
    a closed pipe is dropped, not a panic (`write` in `packages/escher-driver/src/cli.rs`), and a call's answer is read
    under a 30 s bound per read (`ANSWER_BOUND`); not measured: a client's own stderr by level, the session host's on any
    flow but three commands on CRUD, and the windowed binary's stderr by level under a workspace build.
11. **The range interaction model's merge surface** — the fifth direct engine change for every document, flagged
    upstreamable against upstream `main` at `434e0661`, no upstream pull request open; touches seven upstream files and
    adds `node/range.rs`; this entry owns the merge surface ahead of ship.
12. **The keyboard activation's merge surface** — the sixth to eighth direct engine changes, flagged upstreamable at
    `434e0661`, no upstream pull request open; touches four upstream files; this entry owns the merge surface ahead of
    ship.
    - For 11 and 12, "owns the merge surface" is met by a recorded statement of each change's upstream files and
      conflict risk against upstream's current `main` (verified at P3: `upstream/main` at `e72650a6`, three commits past
      `434e0661`, none touching either change's files); opening an upstream pull request is outward-facing and is not
      this chunk's without the operator's word.

## CI read at the take-up (Setup 5a)

- `eb5ba90c` (the last wrap's flip, HEAD): CI#38649787488, green, 16/16 checks, wall 910 s. Nothing to disposition.
