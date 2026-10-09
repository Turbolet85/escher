sweep: base: blame — no earlier sweep; each citing line's own last commit · re-pointed 40 · changed 6 · stretched 15 · first sweep: "Write all 40 (Recommended)" — the operator, 2026-10-09, through the question dialog of this wrap · withheld 0
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:45-52 → 46-53 «dioxus-native = { workspace = true, feature…» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:66 → 67 «nucleo = "0.5"» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:74 → 75 «tokio = { workspace = true, features = ["time", "…» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:77 → 78 «mimalloc = { version = "0.1.48", optional = true }» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:79-80 → 80-81 «[target.'cfg(not(any(target_os = "android",…» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:82-83 → 83-84 «[target.'cfg(target_os = "android")'.depend…» → holds
stretched .andromeda/architecture.md:66 .github/workflows/ci.yml:34-400 → 34-401 — changed inside «build-msrv:» → holds
moved .andromeda/architecture.md:66 .github/scripts/test_ci_workflows.py:12 → 15 «import yaml» → holds
moved .andromeda/architecture.md:66 .github/workflows/ci.yml:394 → 395 «- uses: actions/upload-artifact@043fb46d1a93c7…» → holds
moved .andromeda/architecture.md:66 .github/scripts/test_ci_workflows.py:163-176 → 184-197 «def test_j_every_action_is…» → holds
moved .andromeda/architecture.md:80 apps/browser/Cargo.toml:13 → 14 «default = ["hybrid", "cookies", "cache", "screens…» → holds
moved .andromeda/architecture.md:80 apps/browser/Cargo.toml:18-26 → 19-27 «vello = ["dioxus-native/vello"]» → holds
moved .andromeda/architecture.md:141@c2346 .github/workflows/ci.yml:394-400 → 395-401 «- uses: actions/upload-artifact…» → holds
stretched .andromeda/architecture.md:141@c2635 .github/scripts/test_ci_workflows.py:80-264 → 101-392 — changed inside «class CiWor…» → holds
moved .andromeda/architecture.md:149@c1234 .github/workflows/ci.yml:391-392 → 392-393 «mkdir -p target/ci-logs» → holds
stretched .andromeda/architecture.md:180@c384 .github/workflows/ci.yml:34-400 → 34-401 — changed inside «build-msrv:» → holds
moved .andromeda/architecture.md:180@c1748 .github/workflows/ci.yml:388-392 → 389-393 «- name: test» → holds
moved .andromeda/architecture.md:180@c2257 .github/scripts/test_ci_workflows.py:163-176 → 184-197 «def test_j_every_ac…» → holds
moved .andromeda/security-plan.md:52 .github/scripts/test_ci_workflows.py:178-182 → 199-203 «def test_l_token_is_read_…» → holds
moved .andromeda/security-plan.md:118@c2523 tests/blitz-tests/tests/stand_snapshot_state.rs:445-494 → 458-507 «#[test]» → holds
moved .andromeda/security-plan.md:175 apps/browser/Cargo.toml:13 → 14 «default = ["hybrid", "cookies", "cache", "scree…» → holds
moved .andromeda/security-plan.md:175 apps/browser/Cargo.toml:30-31 → 31-32 «cache = ["blitz-net/cache"]» → holds
moved .andromeda/security-plan.md:215 apps/browser/Cargo.toml:55 → 56 «blitz-net = { workspace = true, features = ["ht…» → holds
moved .andromeda/security-plan.md:229 .github/workflows/ci.yml:392 → 393 «${{ env.RUST_CARGO_COMMAND }} ${{ matrix.pla…» → holds
moved .andromeda/security-plan.md:233 .github/scripts/test_ci_workflows.py:163-169 → 184-190 «def test_j_every_action_…» → holds
moved .andromeda/security-plan.md:233 .github/workflows/ci.yml:379 → 380 «uses: awalsh128/cache-apt-pkgs-action@553a35…» → holds
moved .andromeda/security-plan.md:248 .github/workflows/ci.yml:392 → 393 «${{ env.RUST_CARGO_COMMAND }} ${{ matrix.pla…» → holds
moved .andromeda/test-plan.md:33 .github/workflows/ci.yml:388-392 → 389-393 «- name: test» → holds
moved .andromeda/test-plan.md:98@c1390 tests/blitz-tests/tests/stand_snapshot_state.rs:75-82 → 79-86 «fn parsed_markup…» → holds
stretched .andromeda/test-plan.md:144 .github/scripts/test_ci_workflows.py:80-264 → 101-392 — changed inside «class CiWor…» → holds
moved .andromeda/obs-plan.md:9 apps/browser/Cargo.toml:27-29 → 28-30 «log-times = ["log-frame-times", "log-phase-times…» → holds
moved .andromeda/obs-plan.md:9 apps/browser/Cargo.toml:36 → 37 «tracing = ["dioxus-native/tracing", "blitz-html/tracin…» → holds
moved .andromeda/obs-plan.md:39 apps/browser/Cargo.toml:36 → 37 «tracing = ["dioxus-native/tracing", "blitz-html/traci…» → holds
moved .andromeda/obs-plan.md:45 apps/browser/Cargo.toml:27-29 → 28-30 «log-times = ["log-frame-times", "log-phase-time…» → holds
moved .andromeda/obs-plan.md:245 examples/seven_guis/src/lib.rs:13 → 16 «console_error_panic_hook::set_once();» → holds
moved .andromeda/obs-plan.md:326 .github/workflows/ci.yml:391-400 → 392-401 «mkdir -p target/ci-logs» → holds
moved .andromeda/a11y-plan.md:15 apps/browser/Cargo.toml:13 → 14 «default = ["hybrid", "cookies", "cache", "screenshot…» → holds
moved .andromeda/a11y-plan.md:15 apps/browser/Cargo.toml:38 → 39 «accessibility = ["dioxus-native/accessibility"]» → holds
moved .andromeda/a11y-plan.md:73 tests/blitz-tests/tests/stand_snapshot_state.rs:263-299 → 267-303 «#[test]» → holds
moved .andromeda/a11y-plan.md:204 tests/blitz-tests/tests/stand_snapshot_state.rs:263-299 → 267-303 «#[test]» → holds
stretched .andromeda/architecture.md:11 examples/transparent/src/main.rs:4-13 → 4-13 — changed inside → holds
changed .andromeda/architecture.md:66 .github/workflows/ci.yml:191 → ? — rewritten «- run: python3 -c 'import yaml' ||…» → holds — re-pointed by hand to 191 (the line read; it is the number as it stood, so no digit moved)
stretched .andromeda/architecture.md:105 .github/workflows/ci.yml:164-184 → 164-184 — changed inside → holds
changed .andromeda/architecture.md:136@c14030 tests/blitz-tests/tests/stand_snapshot_state.rs:1-6 → ? — the end rewritten → holds — re-pointed by hand to 1-7
stretched .andromeda/architecture.md:159 .github/workflows/ci.yml:164-184 → 164-184 — changed inside → holds
changed .andromeda/architecture.md:180@c1424 .github/workflows/ci.yml:46 → ? — rewritten «- run: sudo apt-get update &…» → holds — re-pointed by hand to 46 (the line read; it is the number as it stood, so no digit moved)
stretched .andromeda/architecture.md:225 .github/workflows/ci.yml:164-184 → 164-184 — changed inside → holds
stretched .andromeda/architecture.md:243 packages/blitz-dom/src/traversal.rs:46-48 → 46-48 — changed inside → holds
changed .andromeda/architecture.md:263@c3156 tests/blitz-tests/tests/stand_snapshot_state.rs:1-6 → ? — the end rewritten → holds — re-pointed by hand to 1-7
changed .andromeda/security-plan.md:113 tests/blitz-tests/tests/stand_snapshot_state.rs:1-6 → ? — the end rewritten → holds — re-pointed by hand to 1-7
stretched .andromeda/test-plan.md:7 .github/workflows/ci.yml:78-98 → 78-98 — changed inside → holds
stretched .andromeda/test-plan.md:8 .github/workflows/ci.yml:186-199 → 186-199 — changed inside → holds
stretched .andromeda/test-plan.md:97 packages/blitz-vibey-script/src/document.rs:108-133 → 108-133 — changed inside → holds
stretched .andromeda/test-plan.md:329 .github/workflows/ci.yml:271-298 → 271-298 — changed inside → holds
stretched .andromeda/obs-plan.md:80 packages/blitz-vibey-script/src/document.rs:242-261 → 242-261 — changed inside → holds
changed .andromeda/a11y-plan.md:270@c2801 tests/blitz-tests/tests/stand_snapshot_state.rs:1-6 → ? — the end rewritten → holds — re-pointed by hand to 1-7
stretched .andromeda/a11y-plan.md:328 .github/workflows/ci.yml:247-268 → 247-268 — changed inside → holds
out-of-range escher-0.1.0/working-route.md:69@c2099 tests/blitz-tests/tests/accessibility_roles.rs:213 — the file held 198 lines at the base → route — frozen, no change
out-of-range escher-0.1.0/working-route.md:69@c2107 tests/blitz-tests/tests/accessibility_roles.rs:260 — the file held 198 lines at the base → route — frozen, no change
out-of-range escher-0.1.0/working-route.md:69@c2115 tests/blitz-tests/tests/accessibility_roles.rs:284 — the file held 198 lines at the base → route — frozen, no change

How the rows above were read (2026-10-09, this wrap; the block's read is `first-sweep-read.md`, written before the ask):
- The 40 rows of the block were read true before the write and written by `apply --first` with an empty withheld list: `wrote` architecture 18 · security-plan 9 · test-plan 3 · obs-plan 6 · a11y-plan 4, each `read back digits-only`.
- `stretched`, both ends standing (11): the cited lines read in the work tree, the citing sentence true of them — the three `ci.yml:164-184` rows name the `clippy` job, whose install step at 176 is the line this chunk rewrote; `ci.yml:78-98` the `test` job, `:186-199` the `ci-scripts` job, `:247-268` the `a11y` job, `:271-298` the `coverage` job, each whole and each with one rewritten install line inside; `examples/transparent/src/main.rs:4-13`, `packages/blitz-dom/src/traversal.rs:46-48` and the two `packages/blitz-vibey-script/src/document.rs` ranges changed inside before this chunk and still show what their sentences say.
- `changed`, `ci.yml:191` and `ci.yml:46`: each line was rewritten by this chunk and read — 191 is `python3 -c 'import yaml' || bash .github/scripts/apt-install.sh python3-yaml`, still the `ci-scripts` job ensuring PyYAML as `python3-yaml`; 46 is `bash .github/scripts/apt-install.sh libfontconfig1-dev`, still an Ubuntu job installing `libfontconfig1-dev`. The claims hold at the numbers as they stand; that the install now goes through the script is a claim the sentences do not make yet, and is the reconcile's.
- `changed`, `stand_snapshot_state.rs:1-6` (4): the file's `//!` doc, which this chunk's edit made lines 1-7 (line 6 rewritten, line 7 added; read with the base's 1-6 beside it). Re-pointed by hand, the digits alone, in architecture.md (2), security-plan.md (1) and a11y-plan.md (1); the search `stand_snapshot_state\.rs`?:1-[0-9]*` over the seven masters, `.andromeda/registries/`, `CLAUDE.md`, `.claude/docs/`, `.claude/rules/` and the working route returned those four sites and no other.
- `out-of-range` (3): `working-route.md:69` is this chunk's own frozen line. The three numbers are `ci.yml` lines in a backticked list — `.github/workflows/ci.yml` (`:46`, `:68`, `:90`, `:112`, `:176`, `:213`, `:260`, `:284`) — that the tool reads against `accessibility_roles.rs`, a path cited earlier on the line; never written.
- After the hand re-points, the diff of the five masters from HEAD is 28 lines replaced by 28, identical with every run of digits masked.
- After the reconcile's apply (the same wrap): the two citing lines of the rows left at the number as it stood — architecture.md:66 (`ci.yml:191`) and architecture.md:180 (`ci.yml:46`) — were amended to say the install goes through the script, so both citing lines changed in this wrap and neither number is left as the base's own; the two `test_ci_workflows.py:101-392` citing sentences (architecture.md:141, test-plan.md:144) now name `InstallScriptTest`, which that range spans.
- No `leaf` row and no `fenced` · `naked` · `suffix` row was printed (`listed, not written 0`); `unresolved 0`.
