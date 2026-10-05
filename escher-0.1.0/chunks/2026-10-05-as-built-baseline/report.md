# Report — 2026-10-05-as-built-baseline

**Chunk:** workspace build + blitz-tests green on this host, wall-clock recorded
**Date:** 2026-10-05T20:36Z
**Commits:** none since `last_wrap` 2026-10-05T18:45:36Z (HEAD `eaca28ba`, the prior 0-pending wrap's commit; this chunk's work rides this wrap's commit)

## Changes (structured — detectors read this)
- **Files:** `escher-0.1.0/chunks/2026-10-05-as-built-baseline/evidence/baseline.md` (new — the baseline record, six `##` sections) · `escher-0.1.0/chunks/2026-10-05-as-built-baseline/report.md` (this). No file under `packages tests examples apps wpt`, no `Cargo.toml` / `Cargo.lock` (basis: gate `git status --porcelain -- packages tests examples apps wpt Cargo.toml Cargo.lock` → no output, final implement run and this wrap's light gate).
- **Symbols / APIs:** none — no source edited.
- **Crates / modules:** none added · removed · changed.
- **Dependencies:** none added · bumped (`--locked` on every cargo command resolved the committed `Cargo.lock` unchanged).
- **Schema / config:** none.
- **Spec-master edits:** none by /implement (masters read-only there).
- **Counts / qualifiers moved:** none — verified: no master states a blitz-tests / workspace tally (`grep -nE '\b(59|61|255|407|108)\b'` over test-plan.md, architecture.md, a11y-plan.md → 0 tally hits; the `61` hits are the s12 slice-file count, a different quantity). The chunk measured NEW values (below), it moved none.
- **Dev-tool versions:** none installed or upgraded. First recorded dev-host reading (2026-10-05T19:48Z, unchanged from P3's 2026-10-05 read): toolchain `rustc 1.99.0 (b940084d7 2026-09-28)` / `cargo 1.99.0 (5f94df478 2026-08-27)`, `stable-x86_64-unknown-linux-gnu (default)`, no `rust-toolchain*` file, no `.cargo/config*`; Arch packages standing in for CI's apt `libfontconfig1-dev` + build-time python3: `fontconfig 2:2.18.3-2` · `openssl 3.6.4-1` · `pkgconf 3.0.7-1` · `python 3.14.7-1`; fonts sans-serif → Liberation Sans · serif → Liberation Serif · monospace → JetBrainsMono Nerd Font; host Omarchy 4.0.4 (Arch-based), kernel 7.2.5, 32 CPUs, 62 GiB. The flake pins `rust-bin.stable."1.90.0"` (flake.nix:31), below the workspace MSRV `rust-version = "1.91.0"` (Cargo.toml:40, :246) though its own comment says "Keep in sync with `rust-version`" — not used on this host. Basis: `evidence/baseline.md` §Host.
- **Harness / gate surface:** none (no `scripts/agent-run.*`, no CI step, no xtask; the chunk's gates are plan-local).
- **Cross-project / external claims:** none — no external input snapshotted (`inputs.py verify` → `inputs: absent`). No CI run read (fork has 0 active workflows — scope.md §CI verdict at take-up).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **CI runs a rustdoc `-D warnings` gate over the library crates** — stated as a doc gate at architecture.md:67 (Code quality row: "`RUSTDOCFLAGS: "-D warnings"` … Formatting, lint and doc gates"), :104 ("rustdoc warnings are errors"), :155 ("fmt, clippy and rustdoc gates"), :174 (CI/CD jobs list "docs"), :217 (Inherited Defaults "rustdoc `-D warnings`"); distilled at `.claude/docs/commands.md:35` ("`RUSTDOCFLAGS="-D warnings" cargo doc` — the docs gate") and CLAUDE.md Critical Warnings ("rustdoc `-D warnings` pass"). MEASURED: CI's docs job runs bare `cargo doc` (ci.yml:124); the root manifest is the lib-less package `blitz-examples` (Cargo.toml:238-239), so it documents no library crate (P5: `Finished … in 0.13s`, no `Documenting` line). The workspace form `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --keep-going` is RED (exit 101): 21 `Documenting` lines, 3 crates could not document (`blitz-dom`, `blitz-vibey-script`, example `transparent`), 9 rustdoc errors — 5 unresolved links (`blitz_traits::Document::poll` ×2, `Config`, `Node::style`, `tracing::error`), 2 redundant explicit link targets, 1 URL not a hyperlink, 1 public doc (`offset_parent`) linking private `Self::is_offset_parent`; plus 1 cargo warning: output filename collision at `target/doc/blitz/index.html` (bin `blitz` of package `browser`, apps/browser/Cargo.toml:9, vs lib package `blitz`). Read identically on P5's run, the cold run and both warm runs. Basis: `evidence/baseline.md` §Wider gates; gate trails `.andromeda/runs/2026-10-05T19-48-32-implement/`. Owner of making the gate real and green: "CI gate legs" (working-route.md:15), named at the P5 review on the founder's word relayed by the overseer.
- **Expected amendments (from plan):**
  1. arch §Infrastructure Patterns → CI/CD — the docs job's bare `cargo doc` documents only the root `blitz-examples` package → **carried**: Spec claims disproved #1 (sites located by `grep -c 'RUSTDOCFLAGS\|rustdoc'` → architecture.md 1+3 hits, lines 67/104/155/217 + :174 by `grep -n 'docs, cross-platform'`; test-plan.md 1 hit at :64 is doc-examples, a different subject; other masters 0).
  2. arch §Stack and Technologies → Build environment — this host's measured toolchain + Arch package stand-ins as a verified dev-host reading → **carried**: Dev-tool versions (site `grep -n 'Build environment' architecture.md` → 1 hit, :65; `libfontconfig` → architecture.md 1 hit, :174; other masters 0).
  3. test-plan §9 CI Integration — the local baseline reading (commands, cold/warm figures, wider-gate outcomes) as the reference "Fork CI reached" compares against → **carried**: Outcome gates + the Baseline figures bullet below (site `grep -n 'CI Integration' test-plan.md` → 1 hit, :262; obs-plan/a11y-plan each 1 hit — their own §9 headings, not this subject).
- **Baseline figures (new measured values — for test-plan §9):** dev profile, workspace default features, `--locked`, after `cargo clean` (cold) then an immediate re-run (warm), gate-tool wall-clock seconds:
  `cargo build --workspace --locked` 120.26 cold / 5.54 warm (cargo `Finished` 2m 00s) · `cargo test -p blitz-tests --locked` 486.45 / 7.41 (test-profile compile 8m 00s; 61 result lines, 255 passed · 0 failed · 3 ignored) · `cargo test --workspace --locked` 1632.88 / 13.08 (compile 27m 00s; 108 result lines, 407 · 0 · 3) · `cargo fmt --all --check` 0.46 / 0.44, exit 0 · `cargo clippy --workspace --locked -- -D warnings` 34.35 / 0.32, exit 0, 0 warnings · workspace rustdoc 4.23 / 0.96, exit 101. Cold entries ran in sequence in one `target/` (each after the previous left artifacts). The 3 ignored are `paint_tree_bench`'s `#[ignore]` tests. Basis: `evidence/baseline.md` §Wall-clock / §Tests.
- **Coverage of new surfaces:** none — no new external surface, hot-path op or UI element.

## Deviations from intent
- **Record §Commands item 10 cites the plan instead of quoting it** (plan step 4: "every gate `run` verbatim"). The credential probe (gate `cat …/evidence/*.md | grep -ciE …`, expect `exit 1` + `last line 0`) counted its own pattern in the verbatim copy: the warm run read `1`, the sole hit being that line (no credential). Item 10 now points at `plan.md` §Test Commands gate 10 with the reason; re-run green (`0`, exit 1). Step 4 and gate 10 are jointly unsatisfiable for gate 10's own text — a plan-authoring defect, fixed in the record (the chunk's own file), plan untouched.
- **One finding beyond the P5 reading:** the rustdoc output-filename-collision warning (`target/doc/blitz/index.html`) — recorded in §Wider gates; not counted by `-D warnings`.
- **Cold figures differ from P5's** (blitz-tests 486.45 s vs 744.88 s; build 120.26 s vs 182.3 s): P5 ran over a different `target/` state; the record states the two are not one basis rather than comparing them.
- **The block ran three times at implement** (cold; warm; a full re-run after the item-10 fix) — the warm column holds the first warm run's seconds.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 0 · excluded 35; the evidence folder is inside the version workspace).

## Decisions & corrections
- Operator decisions carried from P4 (plan Provenance): the four wider gates are RECORDED, never counted green ("Record them"); the cold basis is from-empty ("cargo clean first").
- Sweep hazard: a credential/secret grep run over a record that quotes the grep's own command self-matches (pattern `password=|token=` inside the quoted `run`) — a record that must list its probes verbatim cannot also pass a probe over itself; cite the probe's location instead.
- Sweep hazard: CI's `cargo doc` / `RUSTDOCFLAGS="-D warnings" cargo doc` without `--workspace` looks like a docs gate but documents only the root lib-less `blitz-examples` package — a "doc gate passes" reading from the bare form proves nothing about the library crates.
- Tool behaviour: `gate.py run` backgrounded writes its per-entry lines only at exit (stdout block-buffered off a tty) — a monitor on the task output stays silent; progress is readable from the gate log dir, and `PYTHONUNBUFFERED=1` streams it.
- Host hook: the project's PreToolUse Bash guard blocks a leading `cd` into a subdirectory — use a subshell `( cd DIR && … )` or absolute paths.

## Outcome
- Acceptance (re-asserted against the diff — one new evidence file + this report, no source):
  - `cargo build --workspace --locked` exits 0 from a `cargo clean`ed `target/` — **met** (cold run, 120.26 s, `target/` absent before entry 1).
  - `cargo test -p blitz-tests --locked` exits 0, log lacks `test result: FAILED`, every file compiled and run, `incremental_oracle` included — **met** (61 result lines: 59 files + lib unittests + doc-tests; 255 · 0 · 3; `incremental_oracle` 7 · 0 · 0).
  - `… --test text_selection_anonymous_block -- --nocapture` exits 0 and lacks `skipping: no usable font` — **met** (2 passed, no skip line).
  - `git status --porcelain -- packages tests examples apps wpt Cargo.toml Cargo.lock` prints nothing — **met** (every run).
  - `evidence/baseline.md` carries the six sections (probe reads `6`) with the required content — **met**.
  - the evidence holds no credential (probe reads `0`), no env dump, no raw log — **met** (after the item-10 fix; `gate.py hygiene` clean at implement).
  - no verification-matrix capability claimed — **met** (`matrix.py show --chunk` → claimed 0; pool unclaimed 15).
- Gates (implement's final full run; re-run by this wrap's light gate):
  - `cargo build --workspace --locked` — green (exit 0)
  - `cargo test -p blitz-tests --locked` — green (exit 0 · lacks `test result: FAILED`)
  - `cargo test -p blitz-tests --locked --test text_selection_anonymous_block -- --nocapture` — green (exit 0 · lacks `skipping: no usable font`)
  - `cargo test --workspace --locked` — recorded: exit 0, 407 · 0 · 3 (not asserted)
  - `cargo fmt --all --check` — recorded: exit 0, no output
  - `cargo clippy --workspace --locked -- -D warnings` — recorded: exit 0, 0 warnings / errors
  - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --keep-going` — recorded: **exit 101** (9 errors, 3 crates undocumented — Spec claims disproved #1) → owner "CI gate legs"
  - `git status --porcelain -- packages tests examples apps wpt Cargo.toml Cargo.lock` — green (exit 0 · no output)
  - `grep -cE '^## (Host|…|Wider gates)$' …/evidence/baseline.md` — green (exit 0 · last line 6); red on the cold run by construction
  - `cat …/evidence/*.md | grep -ciE …` — green (exit 1 · last line 0) after the item-10 fix
  - No `defer`, no `leg`, no smoke (no boot-path; P3 skipped — no boot-path / UI-surface change). No gate deferral: every gate ran on both passes.
- Watches: none folded.
- Outcome basis: implement's P4 report as given in this session's conversation; no operator directive between implement and this wrap; post-implement artifacts: `evidence/baseline.md`.
- Process hygiene: implement's census — 3 gate runs + `cargo clean` started by implement, all terminated; 2 progress monitors (one expired, one stopped). Re-measured at this report: `ps -eo args | grep -E 'gate\.py|cargo |rustc|rustdoc'` → 0 rows.
