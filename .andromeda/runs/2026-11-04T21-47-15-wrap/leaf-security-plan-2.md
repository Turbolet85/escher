@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=26 class=omission
leaf-text: The dependency audit is cargo-deny (`bash .github/scripts/ci-leg.sh audit`, config `deny.toml`):
source: fct-6a266lkz «and the local `fast` leg runs no audit — `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push»
replace-with: The dependency audit is cargo-deny (`bash .github/scripts/ci-leg.sh audit`, config `deny.toml`; the local `fast` leg runs no audit, so run it before a push — CI can go red on advisories published after the base's green audit with `Cargo.lock` and `deny.toml` unchanged):
@leaf_edit n=2 leaf=.claude/rules/security.md line=26 class=stale
leaf-text: ignored by ID, its drop owned by a `.andromeda/residuals.md` line;
source: fct-a4qh6ioi «dropped when that chain leaves paste, owned by a `residual` row in the store (target: next)»
replace-with: ignored by ID, its drop owned by a `residual` row in the store;
@leaf_edit n=3 leaf=.claude/rules/security.md line=26 class=omission
leaf-text: zip 2.4.2's RUSTSEC-2026-0367 · RUSTSEC-2026-0368 · RUSTSEC-2026-0369 (fixed only in zip 9.0, reached only through `anyrender_serialize 0.8.0` ← `browser`, no fixed release reachable) are ignored by ID on the founder's «Игнор по ID (Рекомендую)» (2026-10-17), their drop owned by a `.andromeda/residuals.md` line;
source: fct-a4qh6ioi «RUSTSEC-2026-0429, `ZipArchive::extract` aborting the process on a symlink entry with a large declared size (zip 2.4.2, each fixed only in zip 9.0)»
replace-with: zip 2.4.2's RUSTSEC-2026-0367 · RUSTSEC-2026-0368 · RUSTSEC-2026-0369 (the three on the founder's «Игнор по ID (Рекомендую)», 2026-10-17) and RUSTSEC-2026-0429 (`ZipArchive::extract` aborting the process on a symlink entry with a large declared size; external decay) — each fixed only in zip 9.0, reached only through `anyrender_serialize 0.8.0` ← `browser`, which reads entries `by_name` and never calls `extract`, no fixed release reachable — are ignored by ID, their drop owned by the store's `residual` row `res-h7acz56c`;
@leaf_edit n=4 leaf=.claude/rules/security.md line=26 class=stale
leaf-text: dropped when accesskit_unix moves to zbus 6, owned by a `.andromeda/residuals.md` line;
source: fct-a4qh6ioi «dropped when accesskit_unix moves to zbus 6, owned by a `residual` row in the store (target: next)»
replace-with: dropped when accesskit_unix moves to zbus 6, owned by a `residual` row in the store;
@leaf_edit n=5 leaf=.claude/rules/security.md line=26 class=stale
leaf-text: a measured limit owned by a `.andromeda/residuals.md` line — still review new crates by hand.
source: fct-a4qh6ioi «a measured limit owned by a `residual` row in the store (target: next)»
replace-with: a measured limit owned by a `residual` row in the store — still review new crates by hand.
