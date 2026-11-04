@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=42 class=omission
leaf-text: zip 2.4.2's RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 (each fixed in zip 9.0, no fixed release reachable)
source: fct-a4qh6ioi «and RUSTSEC-2026-0429, `ZipArchive::extract` aborting the process on a symlink entry with a large declared size (zip 2.4.2, each fixed only in zip 9.0)»
replace-with: zip 2.4.2's RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 and RUSTSEC-2026-0429 (`ZipArchive::extract` aborting the process on a symlink entry with a large declared size; each fixed only in zip 9.0, no fixed release reachable)
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=42 class=omission
leaf-text: which never extracts to disk
source: fct-a4qh6ioi «whose `deserialize` opens a `ZipArchive` and reads entries `by_name` into memory and never calls `extract`»
replace-with: whose `deserialize` reads entries `by_name` into memory and never calls `extract`
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=42 class=stale
leaf-text: are ignored by ID on the founder's «Игнор по ID (Рекомендую)» (2026-10-17), their drop owned by a `.andromeda/residuals.md` line (target: next)
source: fct-a4qh6ioi «dropped when that chain reaches zip 9.0 or leaves zip, owned by the store's `residual` row `res-h7acz56c` (target: next)»
replace-with: are ignored by ID, the three on the founder's «Игнор по ID (Рекомендую)» (2026-10-17), their drop owned by the store's `residual` row `res-h7acz56c` (target: next)
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=42 class=stale
leaf-text: its drop owned by a `.andromeda/residuals.md` line (target: next)
source: fct-a4qh6ioi «dropped when that chain leaves paste, owned by a `residual` row in the store (target: next)»
replace-with: its drop owned by a `residual` row in the store (target: next)
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=42 class=stale
leaf-text: dropped when accesskit_unix moves to zbus 6, owned by a `.andromeda/residuals.md` line (target: next)
source: fct-a4qh6ioi «dropped when accesskit_unix moves to zbus 6, owned by a `residual` row in the store (target: next)»
replace-with: dropped when accesskit_unix moves to zbus 6, owned by a `residual` row in the store (target: next)
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=42 class=stale
leaf-text: a measured limit owned by a `.andromeda/residuals.md` line (target: next).
source: fct-a4qh6ioi «a measured limit owned by a `residual` row in the store (target: next)»
replace-with: a measured limit owned by a `residual` row in the store (target: next).
@leaf_edit n=7 leaf=.claude/docs/security-summary.md line=42 class=omission
leaf-text: - The dependency audit's reach — since 2026-10-14-quality-gates the leg checks the whole workspace's graph (`--workspace --all-features`):
source: fct-6a266lkz «and the local `fast` leg runs no audit — `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push»
replace-with: - The dependency audit's reach — a CI audit can go red on advisories published after the base's green audit with `Cargo.lock` and `deny.toml` unchanged, and the local `fast` leg runs no audit, so `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push; since 2026-10-14-quality-gates the leg checks the whole workspace's graph (`--workspace --all-features`):
