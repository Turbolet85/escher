# Report — 2026-10-05-fork-ci-reached

**Chunk:** Fork CI on the build branch — cached, fast/slow split, failure artifacts, signing jobs excluded
**Date:** 2026-10-05T22:02Z
**Commits:** `4268555d chore(2026-10-05-fork-ci-reached): operator pre-CI commit, for the run this chunk's verdict reads` (`git log --format='%h %s' 50c13b59..HEAD`)

## Changes (structured — detectors read this)
- **Files:** new `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`; modified `Cargo.toml`,
  `.github/workflows/ci.yml`, `.github/workflows/publish-browser.yml`, `.github/workflows/wpt.yml`,
  `.github/workflows/wpt-post-results.yml` (`gate.py scope`: changed 7 · listed 7 · recorded 0). Chunk folder:
  `evidence/{red-before-green-test_ci_workflows.txt, baseline-rerun.md, operator-pass.md}`.
- **Symbols / APIs:** no Rust symbol. Two CI-script surfaces:
  - `.github/scripts/ci-leg.sh {leg}` — bash, `set -euo pipefail`; legs `fmt` (`cargo fmt --all --check`) ·
    `clippy` (`cargo clippy --workspace --locked -- -D warnings`) · `test` (`cargo test --workspace --locked`) ·
    `ci-scripts` (`python3 -m unittest discover -s .github/scripts`) · `build` (`cargo build --workspace --locked`) ·
    `msrv` (`cargo +1.91 build --workspace --locked`) · `counter` (`cargo build -p counter --locked`) · `wasm`
    (`examples/wasm_hello` `cargo build --target wasm32-unknown-unknown` without `--locked` — no `Cargo.lock` there —
    then `seven_guis` and `todomvc` `--lib --target wasm32-unknown-unknown --no-default-features --features hybrid
    --locked`) · `doc` (`cargo doc --locked`, bare) · `fast` (fmt → clippy → test → ci-scripts, stops at the first red).
    Exports `RUSTDOCFLAGS="-D warnings"` (the existing workflow-wide value, ci.yml `env:` — not a new handle); writes
    the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg start, via `tee`; exits with the leg
    command's status; unknown/missing leg → leg list on stderr, exit 2. Relative to cwd (no `cd`). The local pre-push
    gate is `bash .github/scripts/ci-leg.sh fast`.
  - `.github/scripts/test_ci_workflows.py` — `unittest`, 3 TestCases / 12 tests: `CiWorkflowTest` (a trigger · b
    cache+save-if · c fast/slow needs · d linux job→leg mapping and leg exists · e failure upload per job · g no
    `secrets.` · h no perl · i matrix platforms), `UpstreamGuardTest` (f), `LegScriptTest` (cargo shim on PATH: failing
    leg exits non-zero and writes its log with the marker · log truncated · unknown leg exits 2). The CI-scripts leg
    now runs 16 tests (4 prior + 12).
  - No port, socket, listener or new env var; no new secret reference.
- **Crates / modules:** none.
- **Dependencies:** no Cargo dependency added or bumped. One CI-script dependency added: PyYAML (Ubuntu
  `python3-yaml`), imported by `test_ci_workflows.py` and ensured by the `ci-scripts` job (`python3 -c 'import yaml'
  || apt-get install -y python3-yaml`); dev host PyYAML 6.0.3 (research.md). (Corrected at P2 Validate — this bullet
  first read "none added".) New GitHub Action references (tag-pinned, none `@latest`):
  `Swatinem/rust-cache@v2` on 7 more jobs (it was already used by `matrix_test`), `actions/upload-artifact@v7` on 10
  jobs (already used by wpt.yml / publish-browser.yml). zizmor `--offline` over `.github/workflows`: the only class
  that moved is `unpinned-uses` 41 → 58 (+17 = 7 rust-cache + 10 upload-artifact); no new finding class (HEAD vs tree,
  per-class `uniq -c`).
- **Schema / config:**
  - `Cargo.toml`: `[profile.dev] debug = "line-tables-only"` added before `[profile.profile]` (the `test` profile
    inherits it). Deletion probe `git diff -U0 50c13b59 -- Cargo.toml | grep -cE '^-([^-]|$)'` → 0; addition probe
    lists exactly `+[profile.dev]` and `+debug = "line-tables-only"`. `production`, every other profile and every pin
    unchanged.
  - `ci.yml` `on.push.branches`: `main`, `v0.*`, **`build/**`** (pull_request, per-ref concurrency with
    cancel-in-progress, workflow `env` unchanged).
  - `ci.yml` jobs: fast set `fmt`, `clippy`, `test-features-default`, `ci-scripts` with no `needs`; every other job
    (`build-msrv`, `build-features-default`, `build-counter`, `build-wasm-examples`, `doc`, `matrix_test`) `needs:
    [fmt, clippy, test-features-default, ci-scripts]`. Each of the 9 linux jobs runs its leg through `bash
    .github/scripts/ci-leg.sh {leg}`. `Swatinem/rust-cache@v2` on every compiling job (7 linux + matrix) with
    `save-if: ${{ github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/build/') }}`; linux jobs use
    the action's default key (per job), matrix keeps `key: target`, `cache-all-crates`, `cache-bin` rule. `ci-scripts`
    ensures PyYAML (`python3 -c 'import yaml' || apt-get install python3-yaml`). The `perl -pi.bak -e 's/opt-level =
    2/opt-level = 0/g' Cargo.toml` step removed from all 5 jobs that had it. `matrix_test`: `linux` platform entry
    removed (windows, macos, ios, android remain); command gains `--locked`, runs under `shell: bash`, output `tee`d to
    `target/ci-logs/matrix-${{ matrix.platform.name }}.log`. The dead `Free Disk Space (Ubuntu)` condition
    (`ubuntu-24.04` vs the matrix's `ubuntu-latest`) is unchanged.
  - Failure artifacts: every leg job ends `actions/upload-artifact@v7`, `if: failure()`, `name: ci-log-{job id}`
    (matrix `ci-log-matrix-${{ matrix.platform.name }}`), `path: target/ci-logs/{leg}.log` (one file),
    `retention-days: 7`, `if-no-files-found: ignore`. No path outside `target/ci-logs/`. Logs are merged
    stdout+stderr of cargo/python, unscrubbed.
  - Upstream-only guards: `publish-browser.yml` job `release-cli` `if: github.repository == 'DioxusLabs/blitz'`;
    `wpt.yml` job `wpt` the same `if`, job `trigger-archive` `if: github.repository == 'DioxusLabs/blitz' &&
    github.ref == 'refs/heads/main'`; `wpt-post-results.yml` job `post-results` `if: github.repository ==
    'DioxusLabs/blitz' && github.event.workflow_run.event == 'pull_request' && … conclusion == 'success'`. No step,
    trigger, secret reference or write/`always()`-remove pair changed inside them. The ref-keyed `environment:` /
    step `if:` expressions in publish-browser.yml stay as they were (now unreachable on a fork).
- **Spec-master edits:** none (implement wrote none).
- **Counts / qualifiers moved:**
  - ci.yml job count unchanged at 10; matrix platforms 5 → 4 (linux dropped) (`test_ci_workflows.py` (i)).
  - ci.yml jobs carrying a rust-cache: 1 → 8 (7 linux + matrix_test) (`grep -c 'Swatinem/rust-cache@v2'
    .github/workflows/ci.yml` → 8).
  - ci.yml commands with `--locked`: 0 → every cargo leg (the script) + the matrix command; `examples/wasm_hello` exempt.
  - Local baseline (test-plan §9) re-measured cold/warm from `cargo clean` (evidence/baseline-rerun.md): `cargo
    build --workspace --locked` 63.90 / 2.07 s (was 120.26 / 5.54) · `cargo test -p blitz-tests --locked` 45.36 /
    6.51 s (was 486.45 / 7.41), 61 result lines, 255 · 0 · 3 · `cargo test --workspace --locked` 52.10 / 11.44 s (was
    1632.88 / 13.08), 108 lines, 407 · 0 · 3; cold total 161.36 s vs 2239.59 s; `target/debug` 32G (was 85G, a
    different `target/` history), `target/debug/deps` 25G (was 65G).
  - Debuginfo share of `accessibility_roles` (gate entry 9): 82 232 966 of 149 737 088 bytes (55 %), was
    331 175 198 of 402 155 872 (82 %).
  - CI-scripts leg tests 4 → 16.
- **Dev-tool versions:** none — rustc/cargo 1.99.0 re-read unchanged on the dev host (baseline-rerun.md §Conditions);
  zizmor used read-only, version not recorded.
- **Harness / gate surface:** new CI steps and leg script as above; the local pre-push gate `bash
  .github/scripts/ci-leg.sh fast`; CI jobs' merged-output log per leg at `target/ci-logs/{leg}.log`. No agent-run
  script, status shape or xtask verb changed.
- **Cross-project / external claims:**
  - GitHub Actions on `Turbolet85/escher`: CI run **37375560233**, event `push`, sha `4268555dd2ea` (the pre-CI
    commit), attempt 1 (cold) `verdict: green · checks 13/13 · wall 1255 s`; attempt 2 (`gh run rerun`, warm)
    `verdict: green · checks 13/13 · wall 475 s` (ci.py v1.0; evidence/operator-pass.md). Per-job cold→warm seconds:
    test 315→169 · clippy 189→42 · build 275→67 · msrv 258→63 · counter 175→45 · wasm 268→67 · doc 32→17 · windows
    760→302 · macos 563→156 · ios 420→80 · android 485→176 · fmt 13→16 · ci-scripts 4→4. Only the CI workflow fired
    on the push (`gh run list --commit`): Publish Browser, WPT, Post WPT results did not.
  - Actions cache after the warm run: `active_caches_count` 12 · `active_caches_size_in_bytes` 9 728 732 229 (≈ 97 %
    of the 10 GB per-repository budget) — 11 `v0-rust-*` entries (one per compiling job) + 1 `cache-apt-pkgs_*`, all
    on `refs/heads/build/escher-0.1.0` (`gh cache list`). The usage API read right after the cold run lagged (count 1).
  - `inputs.py verify`: `I1 · message: overseer (the operator pair), pasted by the operator 2026-10-05 ~20:51Z ·
    copy · n/a` (a message has no live source); cited 10×; 0 drifted · 0 vanished · 0 broken · 0 uncited · 0 UNPARSED.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - plan.md §Implementation notes: "the cold run creates one entry per compiling job (6 linux + 4 matrix = 10,
    counted from steps 4-5)" — measured 11 rust caches (7 compiling linux jobs + 4 matrix; research.md §Measured facts
    itself lists 7 uncached compiling jobs) + 1 apt cache = 12 (`gh cache list`; cache usage API count 12). Plan-only
    claim (no master states it).
  - plan.md §Acceptance "predicted: the warm test leg far below the cold one" — measured 315 → 169 s (−46 %); recorded
    as measured, the prediction was a forecast, not a gate.
  - plan.md entry 15's literal `gh run view <id> --json jobs …` (no `-R`) exited 1, `HTTP 404` against
    `repos/DioxusLabs/blitz/…`: this checkout carries an `upstream` remote (`DioxusLabs/blitz`) and no `gh repo
    set-default`, so a bare `gh run …` resolves upstream. Re-fired with `-R Turbolet85/escher`, exit 0. (`ci.py`
    resolved the push remote `origin` correctly.) Plan-only (no master names that command).
- **Expected amendments (from plan):** (site search: `grep -nE` over the seven masters for
  `perl|save-if|rust-cache|v0\.\*|Signed Builds|warp|blitz-wpt-results|matrix_test|profile\.|Build profiles|upload-artifact|retention|CI Integration|Local baseline|Surfaces under test`)
  - architecture §Infrastructure Patterns → CI/CD — carried: Schema/config (trigger, caches + save rule, perl removed,
    fast/slow `needs`, `ci-leg.sh`, per-leg failure logs 7 days, linux matrix entry removed, upstream guards). Sites:
    architecture.md:174 (1 hit — the CI/CD bullet; states `main`/`v0.*` only, the opt-level rewrite, matrix with
    linux, rust-cache saves only on main); also architecture.md:67 (Code quality row: `cargo clippy --workspace -- -D
    warnings` citing ci.yml:98-109 — now through the leg script with `--locked`).
  - architecture §Established Decisions [Build profiles] — carried: Schema/config (`[profile.dev] debug =
    "line-tables-only"`). Site: architecture.md:74 (1 hit; lists `profile`…`tiny`, cites Cargo.toml:195-234 — lines
    shifted by 3).
  - architecture §Occupied Resources → CI infrastructure — carried: Schema/config (guards). Site: architecture.md:151
    (1 hit: environments "Signed Builds"/"WPT", warp runner); architecture.md:137 (§Standard Contracts → CI
    contracts: the `repository-dispatch` to `DioxusLabs/blitz-wpt-results` — now also repository-guarded).
  - security-plan §Authentication & Authorization — carried: Schema/config (`release-cli` guard). Sites:
    security-plan.md:48 (RBAC row, "Signed Builds" only on main or `ci-test`), security-plan.md:229 (Signed artifacts
    bullet, same claim) — 2 hits.
  - test-plan §9 CI Integration — carried: Schema/config + Harness/gate surface. Site: test-plan.md:264 (Platform:
    "PRs and pushes to main/v0.*"). test-plan §9 Local baseline — carried: Counts/qualifiers moved (re-measure).
    Site: test-plan.md:273-277 (dev profile without the debuginfo level, the 2026-10-05 figures). test-plan §1
    Surfaces under test — carried: Counts/qualifiers moved (matrix 5 → 4). Site: test-plan.md:31 ("windows, macos,
    linux"). 3 sites.
  - obs-plan §9 — carried: Schema/config (failure artifacts: merged-output log per leg, uploaded on failure, 7-day
    retention, unscrubbed). Site: obs-plan.md:288 (NOT YET MEASURED line naming "artifact upload … retention") and the
    §9 table (obs-plan.md:279-284) — 1 hit for `retention`, 1 for `blitz-wpt-results` (obs-plan.md:283: WPT archive
    "on main" — now upstream-only).
- **Coverage of new surfaces:**
  - `ci-leg.sh` → validation leg-name allow-list✓ · instrumentation per-leg log file✓ · PII n/a (build output; URLs
    and paths unscrubbed per obs-plan §8) · tests unit (`LegScriptTest`) · a11y n/a · tokens n/a
  - ci.yml failure-artifact uploads → validation path confined to `target/ci-logs/` (test e)✓ · instrumentation n/a ·
    PII raw✗ (unscrubbed build logs, 7-day retention; no user data is produced by the legs) · tests unit (e) · a11y
    n/a · tokens n/a
  - upstream-only guards → validation `github.repository` guard✓ · instrumentation n/a · PII n/a · tests unit (f) ·
    a11y n/a · tokens n/a

## Deviations from intent
- Step 8 (`test_ci_workflows.py`) written before steps 3-7, so the plan-required red reading could be taken against
  the untouched workflows (no stash, no copy).
- Step 9 (the `cargo clean` re-measure) ran before the gate block, so the block ran warm and did not distort the
  baseline timings.
- Tests stricter than the plan's letter: (c) `needs` must EQUAL the fast four; (d) pins the exact job → leg map; (e)
  pins each job's own log path, `if: failure()` and `retention-days: 7`; one extra test asserts log truncation.
- Three short WHY comments: `ci-leg.sh` header, a fast/slow note above `jobs:`, the matrix's no-linux note.
- Removed a 0-byte untracked file `2{t+=}` (created 21:07Z, before implement started — a mis-parsed shell redirect
  from an earlier session); it was the scope read's only row.
- Operator pass entry 15 re-fired with `-R Turbolet85/escher` after the literal form exited 1 (see Spec claims
  disproved). The cold attempt's job rows were read with the same `-R` form before entry 13 fired.
- scope record: none — gate.py scope clean, 0 recorded (changed 7 · listed 7).

## Decisions & corrections
- Operator directive (2026-10-05/06, this session): "go — run the operator pass now, entries 10-15 in order, as the
  P5 approval recorded; then stop with the readings (the wrap follows in this window)".
- Correction (self, this wrap): the operator-pass evidence first recorded "10 rust caches + 1 apt = 11"; re-counted
  from `gh cache list` as 11 + 1 = 12 and corrected before this report.
- Sweep hazard: `perl` matches inside "hy**perl**ink" — all 5 master lines `grep -c perl` counts are hyperlink text
  (security-plan.md:36, :52; design-system.md:388; architecture.md:64, :241); the CI rewrite's real site is
  architecture.md:174, which spells it "rewrite `opt-level = 2` to `0`" and never says perl. Search the rewrite by
  `opt-level = 2`, not by `perl`.
- Sweep hazard: in this checkout a bare `gh run …` / `gh api` without `-R` resolves to the `upstream` remote
  (`DioxusLabs/blitz`), not the fork — every `gh` read of the fork's runs needs `-R Turbolet85/escher`.
- Measured, not forecast: the GitHub Actions cache store holds 9.73 GB after one cold run (≈ 97 % of 10 GB); a
  lockfile or toolchain change mints new keys and will evict LRU entries.

## Outcome
- Acceptance, re-asserted against the diff:
  - Build-branch pipeline green — MET: push of `4268555d` to `build/escher-0.1.0` triggered CI run 37375560233,
    `verdict: green · checks 13/13` (cold).
  - Cached builds — MET: every compiling ci.yml job carries `Swatinem/rust-cache@v2` with a save rule naming `main`
    and `build/` (test b); warm re-run of the same sha green (475 s vs 1255 s); job timings recorded beside it.
  - Host-reproducible legs — MET: `ci-leg.sh fmt/clippy/build/test` exit 0 on the dev host; `--nocapture` run lacks
    the font-skip line; every linux leg step invokes the script and its leg (test d); the a11y tests run in the fast
    `test` leg (workspace tests, 407 · 0 · 3).
  - Fast/slow split — MET: test (c); windows, macos, ios, android legs remain (test i); CI timings show the slow legs
    starting after the fast four completed.
  - Failure artifacts — MET: `LegScriptTest` (log written, status propagated); test (e) — upload `if: failure()`,
    under `target/ci-logs/` only, 7 days; no path reaches `apps/browser/keystore.jks` or `Dioxus.toml`.
  - Signing-secret jobs excluded — MET: test (f) on every job of the three upstream workflows; test (g) no `secrets.`
    in ci.yml; the push fired none of the three workflows.
  - `Cargo.toml` gains only the stanza — MET: deletion probe 0, addition probe exactly the stanza; MSRV leg is `cargo
    +1.91 build` (CI job `MSRV Build [Rust 1.91]` green).
  - Local baseline re-measured cold and warm with the counts 255 · 0 · 3 and 407 · 0 · 3 — MET (evidence/baseline-rerun.md).
  - No verification-matrix capability claimed — MET (`matrix.py show --chunk`: claimed 0).
- Gates (implement run `.andromeda/runs/2026-10-05T21-11-23-implement/`, gate v1.10):
  - `python3 -m unittest discover -s .github/scripts` — green (exit 0; 16 tests OK)
  - `bash .github/scripts/ci-leg.sh fmt` — green
  - `bash .github/scripts/ci-leg.sh clippy` — green (33.82 s)
  - `bash .github/scripts/ci-leg.sh build` — green
  - `bash .github/scripts/ci-leg.sh test` — green
  - `cargo test -p blitz-tests --locked --test text_selection_anonymous_block -- --nocapture` — green (exit 0 · lacks
    `skipping: no usable font`)
  - `git diff -U0 50c13b59… -- Cargo.toml | grep -cE '^-([^-]|$)'` — green (exit 1 · last line 0)
  - `git diff -U0 50c13b59… -- Cargo.toml | grep -E '^\+[^+]'` — green (exit 0 · both stanza lines)
  - the `accessibility_roles` debuginfo probe — recorded (`debug 82232966 of 149737088 file bytes`)
  - `gate.py hygiene` (operator) — exit 0, `hygiene: clean` (evidence/operator-pass.md)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.1.0` (operator) — exit 0, pushed `50c13b59..4268555d`
  - `ci.py conclusion --sha HEAD --wait 6000` (operator, cold) — exit 0, `verdict: green`, run 37375560233
  - `gh run rerun <id>` (operator) — exit 0
  - `ci.py conclusion --sha HEAD --wait 6000` (operator, warm) — exit 0, `verdict: green`, run 37375560233 attempt 2
  - `gh run view <id> --json jobs …` (operator, report-only) — literal form exit 1 (HTTP 404 on upstream); `-R
    Turbolet85/escher` form exit 0, rows recorded
  - smoke: skipped — no boot-path / UI-surface change.
- Watches: none folded.
- Outcome basis: the operator pass's final state — commit list `4268555d` (one pre-CI commit, no fix commit) and
  the final HEAD's CI run 37375560233 (attempts 1 and 2, both green on `4268555dd2ea`) in evidence/operator-pass.md;
  implement's P4 report (this conversation) for the host gates, deviations and the baseline re-measure. This wrap's
  commit adds only the chunk's bookkeeping/spec edits on top of the measured sha.
- Process hygiene: implement's census — the baseline re-measure script, `gate.py run`, `ci-leg.sh fast` started by
  this session, all terminated; operator pass — `ci-leg.sh fast` and the two `ci.py` waits terminated (exited). Host
  process list re-read at wrap time: no cargo / rustc / ci-leg / gate.py / ci.py process.
