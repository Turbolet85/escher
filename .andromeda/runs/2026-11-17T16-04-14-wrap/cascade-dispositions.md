# Cascade dispositions — 2026-11-17-upstream-sync-ahead-of-polish-and-ship

## Step 2 — the sweep
The search: `cascade.py sweep` (baseline c74c7f3e, the parent of the oldest pre-CI commit) over two pattern files, one call — `cascade-patterns-security-plan-1.toml` (the tool's, from the one `rows-land` dry run that wrote a file: `security1-1` = `matrix_test`, the retired free-disk row's distinctive term) and `cascade-patterns.toml` (main's, the amended claims' old wording and mechanism over the whole masters: `m-hybrid` the hybrid renderer name, `m-nopkg` «enters no package», `m-standrep` the stand-repeat-only free-disk phrasing, `m-skrifa` skrifa / read-fonts). The other five landed files' dry runs wrote no file («patterns: none: … changed prose alone»), so step 3's leaves read those rows' deltas. Listing `sweep-pass-1.1.txt`, read whole: `total (5 patterns) · 12 rows over 6 files` · `per class · new 0/0 · standing 4/1 · leaf 8/5 · curation 0/0 · base 0/0`.

- `m-hybrid`: 0 rows, its control fired (architecture `decision/anyrender-rendering-abstraction`'s previous revision) — no master, leaf, curation home or base still names the hybrid renderer.
- architecture `dec-7xt56qlk` (m-nopkg) — no change: «no package enters the lockfile» is the why of that chunk's own seed-source decision, true of it.
- architecture `dec-qohuepcx` (m-nopkg) — no change: the same, the screenshot decision's own why.
- architecture `dec-qvdsgthp` (m-skrifa) — no change: skrifa for font metrics stays true.
- architecture `dec-oi76y4w6` (m-skrifa, `edited mixed ×5`) — amended: the new `decision/skrifa-matches-parley`, its text read whole at the read-back (the amended wording, no duplicate of the retired «matches parley and vello»).
- leaf rows (8 over 5 files): `.claude/docs/tests-summary.md:29` (security1-1), `.claude/docs/services/escher-driver.md:11`, `.claude/docs/stack.md:26` · `:27` (m-nopkg — other chunks' own «no package entering», true of them), `.claude/docs/services/escher-driver.md:61`, `.claude/docs/services/seven_guis.md:38` (m-standrep — «dropped from `stand-repeat` alone», another subject), `.claude/docs/gotchas.md:7`, `.claude/docs/stack.md:18` (m-skrifa) — re-derived in step 3, the leaves set reading them from this listing.
- curation 0 · base 0: no curation home or judgment base quotes the retired wording.

## Leaves
Pass mode (`cascade.py leaves`: `epoch · «Epoch 6 — Polish & ship» · markerless 5 · gated 0 · open`; `total · pass · 11 group(s) · 21 leaves · 598284 B · unmapped 1`). One comparer a group, batch `leaf`, 11 of 11 pass. The apply over ten files: `total · blocks 5 · applied 5`.
- architecture-1 · pass · 1 finding · applied 1 (`services/blitz-dom.md:15`, omission: the layout-time scroll clamp beside `scroll_to`/`scroll_by`'s).
- architecture-2 · architecture+layout-templates · architecture+security-plan+test-plan-1 · architecture+test-plan · layout-templates · test-plan-2 · pass · 0 findings each.
- security-plan-1 · pass · 1 · applied 1 (`security-summary.md:13`, omission: the SVG image/feImage href fetch, ratified widening).
- security-plan-2 · pass · 1 · applied 1 (`rules/security.md:13`, omission: the same, beside the stamping constructor).
- test-plan-1 · pass · 2 · applied 2 (`tests-summary.md:29` ×2, retired: the free-disk guard renamed and the test job beside stand-repeat).
- architecture+security-plan+layout-templates+test-plan · pass · 1 finding · by hand 1: `cascade.py leaves --apply` refused the file (its `@leaf_read` of CLAUDE.md quoted the line above the last one), and the re-spawn was refused because the file had failed no `check`. The one finding, `gotchas.md:7` «skrifa must match parley and vello» → the amended rule, was checked against `decision/skrifa-matches-parley` and drafted whole, then landed by `rec.py harness land` (tx 1810).
- by hand, outside the comparers: `CLAUDE.md:9` «anyrender 0.14 / Vello 0.11» → «anyrender 0.15 / Vello 0.12», named stale by P1's site sweep (`anyrender` pattern, a CLAUDE.md leaf row) against the report's Dependencies, which read the workspace's pins (`Cargo.toml` anyrender 0.15.0, vello 0.12); landed by `rec.py harness land` (tx 1811). A version line is inventory no master row holds, so no comparer of a changed row reads it; it was fixed here because it was stale.
- unmapped `.claude/rules/host-linux.md` (host) — the host's leaf, no source of this pass states it: no comparer, no edit.
- declined: none.
