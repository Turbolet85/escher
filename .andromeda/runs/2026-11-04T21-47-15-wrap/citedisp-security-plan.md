@disposition n=1
row: held fct-73ojmpsr apps/browser/Cargo.toml:14 — refused at 2026-10-24T12-21-29-wrap, not touched since «default = ["hyb…»
disposition: holds — stands (line 14 is the default feature list, holding "cookies" and "cache")
@disposition n=2
row: held fct-4kkbivro packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…»
disposition: holds — stands (535-539 is the `let Some(display) = node.display_style() else` arm that logs at info and returns)
@disposition n=3
row: stretched fct-a4qh6ioi deny.toml:1-29 → 1-30 — changed inside
disposition: claim false — raised at Validate (the fact's «Eight ignores stand» is false: deny.toml now holds nine per-ID ignores, RUSTSEC-2026-0429 for zip 2.4.2 added at line 26 by 9b3b29714, and the fact never names it; the [graph] six targets, all-features and the per-ID-only [advisories] with written reasons at 1-30 hold)
@disposition n=4
row: held fct-a4qh6ioi .andromeda/residuals.md:29 — refused at 2026-10-27T02-06-43-wrap, not touched since «2026-10-16-fast…»
disposition: claim false — raised at Validate (.andromeda/residuals.md is gone from the work tree, moved into the records store's bases family by 031e983d9, so line 29 names no line of the tree; the zip-ignores residual itself read re-carried:0.3.0, target next, at the base)
@disposition n=5
row: held fct-a4qh6ioi .andromeda/residuals.md:30 — refused at 2026-10-27T02-06-43-wrap, not touched since «2026-10-17-upst…»
disposition: claim false — raised at Validate (.andromeda/residuals.md is gone from the work tree, moved into the records store's bases family by 031e983d9, so line 30 names no line of the tree; the zbus-family residual itself read re-carried:0.3.0, target next, at the base)
@disposition n=6
row: held fct-d2yejoup tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…»
disposition: holds — stands (1-5 is the module doc stating the snapshot stand's ids, roles, states and determinism)
