# Curation — 2026-11-21-dependency-and-ci-debts (wrap 2026-11-22T02-02-17-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "Any cargo run on the shared `target/` without `SKIA_BINARIES_URL` … re-runs skia-bindings' build script …; name the run that unset it from Cargo's fingerprint, never from a process census"
  Tier 3 (.claude/docs/session-learnings.md): + "CI's toolchain moves the build directory's layout"
  Corrected: testing.md 2026-11-14 entry, its «Extended 2026-11-18» clause, in place (exempt from the cap)
  Filters: 0 dup · 3 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

## Applied
- Tier 2 · `.claude/rules/testing.md` `## Session Additions`, 2026-11-22 (tx 3055).
  Proof: the report's *Decisions & corrections* «Sweep hazard» and «A narrow-basis attribution was withdrawn» —
  Cargo's `run-build-script-build-script-build.json` read `{'var': 'SKIA_BINARIES_URL', 'val': None}` inside `fast`'s
  own sweep (`evidence/red-gate5-firing1.md`); the first attribution, to rust-analyzer from a census, was withdrawn.
- Tier 3 · `.claude/docs/session-learnings.md`, «CI's toolchain moves the build directory's layout» (tx 3056).
  Proof: `evidence/operator-pass.md` — CI#44965103559 and CI#44967824760 red on the witness step with no `output` file;
  CI#44971052397's four lines `…/debug/build/skia-bindings/{hash}/run/stdout:  FROM: file://…`; rustc 1.100.0 on the
  runners (ios log line 419), cargo 1.99.0 on the dev host.
- Correction · `.claude/rules/testing.md:69`, the 2026-11-14 entry's «Extended 2026-11-18» clause, in place with
  `[corrected 2026-11-22: …]` (tx 3055).
  Proof: `evidence/control-1.txt` — `control sum-unchecked · entry 1 · holds` · `control url-unexported · entry 2 ·
  holds`, both unittest cases of the `ci-scripts` leg, at `gate v1.38`.

## The Decisions & corrections entries, each disposed
- Implement: «The operator's ruling at P1 (inputs#I7): FORCE dropped, `{key}` kept, the fallback a limit, the witness
  added» — not a learning: task-specific (Filter 2); its homes are security-plan `decision/no-force-skia-binaries-download`
  and `decision/skia-archive-fallback-source-build`, landed at P2.
- Implement: «Sweep hazard» — applied, Tier 2 testing.md.
- Implement: «A narrow-basis attribution was withdrawn» — applied, folded into the same Tier 2 entry (its last clause).
- Implement: «A standing learning did not recur» (testing.md's 2026-11-14 note) — corrected in place at its own entry.
- After implement: «The operator's word after stop 1» — not a learning: a ruling on this chunk's witness (Filter 2).
- After implement: «The operator's word after stop 2» — not a learning: as above (Filter 2).
- After implement: «A narrow-basis guess withdrawn» (fix 1 kept the file name `output`) — applied, inside the Tier 3
  entry (a witness pinned to a layout).
- After implement: «A measured hazard: a CI toolchain ahead of the dev host's moves the build directory's layout» —
  applied, Tier 3.
- After implement: «Store journal folded into an unpushed commit by `--amend`» — not a learning: a one-off of the
  operator pass's push guard (Filter 2).

## The citation sweep's curation row
- `.claude/docs/session-learnings.md:412` `host.rs:350` (out-of-range, `citation-dispositions.md`) — no change: an
  illustrative example of a prefix collision inside a preserve-verbatim entry, no citation of the tree.
