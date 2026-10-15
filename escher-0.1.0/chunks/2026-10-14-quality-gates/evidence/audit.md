# The audit leg: the whole workspace's graph, and licences — steps 10 and 11

cargo-deny 0.20.2. The leg now runs `cargo deny --locked --workspace --all-features check advisories licenses`
(`.github/scripts/ci-leg.sh`), over `deny.toml`'s six targets with `[graph] all-features = true`.

| reading | config | exit | verdict line | errors |
|---|---|---|---|---|
| the leg — gate entry 9, implement's first block pass, 2026-10-15 | committed `deny.toml` (sha256 `dd1e3ef8…7d323`) | 0 | `advisories ok, licenses ok` | none |
| the control — the RUSTSEC-2024-0436 entry removed from a copy, passed through `--config` | `$TMPDIR/{scratchpad}/deny-no-paste.toml` (one line fewer) | 1 | `advisories FAILED, licenses ok` | one: `error[unmaintained]: paste - no longer maintained`, ID RUSTSEC-2024-0436 |
| memmap2, measured, not taken — `unsound = "all"` added to `[advisories]` in a copy | `$TMPDIR/{scratchpad}/deny-unsound.toml` | 1 | `advisories FAILED` | one: `error[unsound]: Unchecked pointer offset in crate memmap2`, ID RUSTSEC-2026-0186 |

The committed `deny.toml` was not touched by either copy: its sha256 is unchanged across both runs and `git diff`
shows only this chunk's edit (the header and the one ignore).

- The widened graph reaches paste: the control's one error is the advisory the old leg's pruned graph never
  reported (CARRY 1). With the ignore, `[advisories] ignore` holds two per-ID entries, each with a written
  reason — RUSTSEC-2026-0192 (ttf-parser, unchanged) and RUSTSEC-2024-0436 (paste 1.0.15, a proc-macro of
  `rav1e 0.8.1` ← `ravif` ← `image`, reached through `anyrender_svg` and the apps' `avif`; no patched
  release; its owner the `.andromeda/residuals.md` line the card proposes). `unmaintained` and `unsound`
  stay at their defaults.
- memmap2 0.5.10's RUSTSEC-2026-0186 (unsound, `Mmap::advise_range`) is reported only with `unsound = "all"`;
  at the defaults the leg does not list it for a transitive crate. Not taken (the founder's «Аудит всего
  workspace (Рекомендую)», whose option records it as a measured limit) — card row 2.
- Unmatched licence allowances (`warning[license-not-encountered]`), by licence name, on the leg's graph:
  `NCSA` alone. `Unicode-DFS-2016`, unmatched on the old six-target graph without `--workspace` (CARRY 6's
  reading), is matched on the whole workspace's graph.
- The advisory database is the one cargo-deny fetches on each run (no `--disable-fetch`).
