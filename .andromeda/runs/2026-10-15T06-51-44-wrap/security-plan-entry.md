
## 2026-10-14-quality-gates — the whole-workspace audit, the licence gate, the owed checks discharged
**Section:** §Dependency Security → Audit tool, Supply chain integrity (Lockfile verification, Licence compliance) · §Input Validation (apt-install.sh · coverage_floor.py, new row · Driver session socket · escher-session CLI · Fetched fonts · `@font-face` source) · §Secret Management → Environment values read · §Logging & Monitoring
**Change:**
- Audit: `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph; the leg reads the `[licenses]` table (was "does not read"). Two ignores: RUSTSEC-2026-0192 and RUSTSEC-2024-0436 (paste 1.0.15, a proc-macro of `rav1e` ← `ravif` ← `image`).
- The pruning reach claim is history of the old invocation. On the new graph paste reaches the gate; memmap2 RUSTSEC-2026-0186 is kept out by the `unsound` default, not pruning — a measured limit. Each is owned by a `.andromeda/residuals.md` line.
- Licence compliance: the fork gates on licences — was "an open question, owned by the route entry Quality gates". Lockfile verification lists the `stand-repeat` leg and both clippy lines.
- Input Validation: `apt-install.sh` takes ten calls with its bounded repair, the usage gate still exiting 2 before `dpkg`. New row `coverage_floor.py`: `CRATE_DIR=PERCENT` ≤ 100, no duplicate, else exit 2; prints no source path.
- The 30 s answer bound and the closed pipe are each pinned by a check. The `@font-face` hint-less source is covered by `font_face_sniffed_source`, which also witnesses the sniff — was "still owed".
- Environment values: `TMPDIR`, read by escher-driver's client unit test through `std::env::temp_dir()`; the library's code reads none beyond `RUST_LOG`.
- Logging: three stderr readings by build and level, 28 runs, 0 needles.
**Why:** the founder's «Аудит всего workspace (Рекомендую)» and «Проверка лицензий (Рекомендую)», relayed verbatim by the overseer as his chosen option labels; the owed check is the one his «Утвердить + проверка» (2026-10-10) asked for. The test-only `TMPDIR` read is the operator's answer at this wrap: no crossing.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/
