@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — signing and PR-token jobs got a `github.repository == 'DioxusLabs/blitz'` guard so no fork ref reaches secrets. `--locked` was extended to every ci.yml cargo leg (`examples/wasm_hello` excepted), and a unit test pins that ci.yml references no `secrets.`. This is the origin of decision/locked-builds and the CI-secrets rows. A skia checksum step or fetch script added to ci.yml must keep its cargo legs locked and must not add a `secrets.` reference. A ref-only `if:` edit to a guarded workflow would re-arm signing on the fork.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — the origin of the cargo-deny 0.20.2 audit job and the root `deny.toml`: per-ID ignores with a written reason, no blanket allow, `unmaintained`/`unsound` at their defaults. It also added SHA-pinned ci.yml `uses:` and the workflow-level `permissions: contents: read`. The first ignore was RUSTSEC-2026-0192 (ttf-parser 0.25.1, through the exact winit beta pin). paste (RUSTSEC-2024-0436) and memmap2 (RUSTSEC-2026-0186) were then pruned from the gate. This is where decision/ci-token-read-only, decision/ci-actions-pinned-to-sha, decision/cargo-deny-audit-tool and decision/dependency-audit-ci-gate began, and it is the base for scope items 2, 3 and 6.

@history_item n=3 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — precedent for a CI script's argv row: `apt-install.sh`, called from ci.yml `run:` steps, validates package names and options and exits 2 before `sudo` runs. It reads no env var and names no secret or GitHub context variable. A new per-platform skia fetch/check script under `.github/scripts/` is the same kind of surface and owes the same Input Validation row.

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (dependency security after the sync) — the sync took upstream's dependency set whole, with each entering crate reviewed by hand. `deny.toml` gained upstream's `[licenses]` table (13 allowed expressions), not gated at first. The upstream-only workflows dropped the apt-cache action. This is the background to decision/licence-gate's byte-for-byte table and decision/upstream-sync-dependency-set.

@history_item n=5 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the 28-name falsy predicate; no git dependency; the sync's dependency delta) — at this point no git dependency was left in the root manifest: taffy moved to crates.io 0.15.0 with a checksum. The audit read `advisories ok` with `deny.toml` unchanged. This is the origin of decision/no-git-dependencies and decision/no-git-dependency, which the 2026-11-15 entries later gave the Taffy-only exception.

@history_item n=6 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — standing practice: every dependency an escher crate takes is recorded as a reviewed Pinning bullet, with the lock delta and the audit read. A version move this chunk makes (rusqlite, rav1e/ravif/image, accesskit_unix, anyrender_serialize) owes the same review record.

@history_item n=7 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the audit became `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace, and the licence gate came in on the founder's «Проверка лицензий». On the new graph paste (RUSTSEC-2024-0436) reaches the gate and is ignored by ID. memmap2 RUSTSEC-2026-0186 is kept out by the `unsound` default, a measured limit rather than pruning. Both are owned by residual lines. This is the direct origin of decision/dependency-audit-cargo-deny's `except` and decision/licence-gate, and of scope items 2 and 3 (res-kdhnfj5p, res-uqt2yd2y).

@history_item n=8 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — the zip 2.4.2 ignores RUSTSEC-2026-0367/0368/0369 came in as external decay: the audit went red over a `Cargo.lock` and `deny.toml` identical to a green run. The stated reach is `anyrender_serialize 0.8.0` ← `browser` (capture), entries read `by_name` and never extracted. No fixed release was reachable (anyrender_serialize requires zip ^2.1). The ignores drop when that chain reaches zip 9.0. The by-ID ignore with a named owner is the founder's decision. This is scope item 5.

@history_item n=9 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (eight advisory ignores: the zbus family's three) — RUSTSEC-2026-0370/0371/0372 are unmaintained and folded into zbus 6. They are reached only through zbus 5.19 + atspi 0.29 under accesskit_unix 0.24.0 ← accesskit_xplat ← blitz-shell, and drop when accesskit_unix moves to zbus 6. Trap: that chunk's (security) criterion «with `deny.toml` unchanged» was left UNMET as written. This is scope item 4 (res-p73cueyv).

@history_item n=10 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the ten-commit take's lock record, tokio past RUSTSEC-2026-0386) — precedent for the audit rule's remedy: a semver-compatible, lock-only update (tokio 1.53.2 → 1.54.1, manifest unchanged, no ignore added). It also recorded two standing warnings, `yanked` clipboard-win 5.4.1 and `license-not-encountered` NCSA. Each advisory this chunk closes by an update should follow this form.

@history_item n=11 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (dioxus-cli in the upstream-only publish workflow) — the upstream-only publish workflow installs dioxus-cli 0.8.0-alpha.3, or from git at upstream's rev, behind its repository guard. This is CI tooling outside the fork's pin rule. A skia fetch step must not be placed in a guarded upstream-only workflow.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (RUSTSEC-2026-0414, the pulldown-cmark lock update; the free-disk action at its two CI sites) — the pulldown-cmark advisory was fixed by a `Cargo.lock`-only semver-compatible update with `deny.toml` unmoved. Separately, `jlumbroso/free-disk-space` was pinned by SHA on `Stand checks repeated` as a CI action site. It was not shown to cure the runner running out of disk, and its `matrix_test` step was skipped on every row. These are precedents for the advisory remedy, for ci.yml action pins, and for CI runner flakiness (the cancelled-runner watch).

@history_item n=13 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port, two test-only TCP binds; the `matrix_test` free-disk step retired; the `embolden` leg, locked) — the API row says no *served* TCP/UDP port and the session socket stays the one listener, which sits behind decision/session-socket-sole-listener. The `matrix_test` free-disk step was removed, with `test_only_stand_repeat_frees_disk` pinning the action. A new locked cargo leg went into `ci-leg.sh` `LEGS`/`FAST_LEGS`. Precedent: a new leg or step in `ci-leg.sh`/ci.yml is pinned by a `test_ci_workflows.py` check and stays locked.

@history_item n=14 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (four sub-entries) — (a) RUSTSEC-2026-0429 (zip `extract` symlink abort) was added as the fourth zip ignore, which put the count at nine. The ruling is scoped to «the three» and the reach is `deserialize` reading `by_name`, never `extract`. This was a second external decay (CI#41911351081). (b) The audit residual owners became store rows: zip `res-h7acz56c`, plus the paste, zbus and memmap2 rows. (c) pulldown-cmark 0.13.6 fixed RUSTSEC-2026-0433 by a lock-only update. (d) A CI audit can go red on advisories published after a green one, and the local fast leg runs no audit. That rule is the origin of decision/local-audit-before-push.

@history_item n=15 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Dependency Security: 1 row) — `~ decision/ci-actions-pinned-to-sha` guard: now `WorkflowPinTest` in `test_ci_workflows.py` reads every workflow file, not ci.yml alone. Any action a skia step adds, in any workflow, must be pinned to a 40-hex SHA or that test fails.

@history_item n=16 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `+ decision/seeded-source-in-process-seed-only`. The seeded source takes no seed from an env var, file or carrier. It is a picked row: CI or skia work that adds env reads must not feed it.

@history_item n=17 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Dependency Security: 3 rows) — `~ decision/no-git-dependency`, `~ decision/no-git-dependencies`: Taffy-only git exception added. `~ decision/upstream-sync-dependency-set`: git-pinned content only by a lock-only semver-compatible registry update. Any update that would bring another git source breaks these rows.

@history_item n=18 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Dependency Security: 4 rows) — `~ decision/engine-dependency-pins`: Taffy is a git pin. `~ decision/no-git-dependencies`: parley and every other crate are registry versions. `~ decision/no-git-dependency` except: Taffy's integrity rests on its full rev, with no lock checksum. `~ decision/upstream-sync-dependency-set`: Taffy's own git pin is allowed. A dependency move here has to keep the lock, audit and MSRV legs green under these rows.

@history_item n=19 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — `~ decision/engine-dependency-pins`: Taffy rev e90751fd → a3997d55, the pin the picked row holds now. Advisory updates must not move it.

@history_item n=20 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Dependency Security rows) — `~ decision/upstream-sync-dependency-set`: a coupled registry set upstream moved may enter with its packages (AnyRender 0.15, seven packages). `− decision/free-disk-only-on-stand-repeat`: the free-disk step now runs on two jobs and the rule's home is the test plan's CI integration. Both matter for the CI-debt and coupled-pin reasoning.

@history_item n=21 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/advisory-ignores-by-id`: each `deny.toml` ignore reason names its owner as a residual row by id, and no product file names the residuals file. Every ignore this chunk keeps, edits or adds (zip, zbus, paste, ttf-parser) must follow this convention. Dropping an ignore retires its residual row.

@history_item n=22 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Input Validation 1, Secret Management 2, Dependency Security 1) — `+ decision/target-sweep-argv-validation`, `+ decision/target-sweep-reads-no-env` and `+ contract/environment-variables/ci`: `CI` is a contentless flag read only by `ci-leg.sh`'s fast arm, tested for empty or unset, and when set skips the sweep. `~ decision/locked-builds` reaches the sweep's cargo runs. This is the chunk that left CARRY `cry-sqod24yl` (scope item 9). Any change to how `ci-leg.sh` reads `CI` must restate this contract row.

@history_item n=23 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (API Security, Dependency Security, Data Protection, Secret Management rows) — `~ decision/engine-dependency-pins`: Stylo 0.24, winit `=0.31.0-beta.4`, which is the coupled pin ttf-parser rides (scope item 6). `+ decision/skia-default-build-fetch`: Skia is built by default with an unchecked build-time binary fetch from GitHub releases, a supply-chain widening that scope item 7's verified archive narrows; the row must be restated when the archive is verified. `~ decision/session-socket-sole-listener`, plus `~ absence/served-network-api` and `~ overview/api-security`: the one listener of a default build, with CDP opt-in. `+ decision/clipboard-via-winit`. `+ contract/environment-variables/blitz-cdp-port` and `blitz-devtools-frontend-dir`: the precedent for registering each new env read such as `SKIA_BINARIES_URL`.

@history_item n=24 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Input Validation: 3 rows) — `~ decision/seeded-source-in-process-seed-only`: a script document's building code may seed it. This is the picked row's current wording, still reading no env var, file or carrier.
