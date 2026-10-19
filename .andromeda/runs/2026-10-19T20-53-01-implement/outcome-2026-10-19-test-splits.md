# Implement outcome — 2026-10-19-test-splits

**Outcome: green** (smoke skipped — no boot-path / UI-surface change). Next → the OPERATOR PASS, entries 23
(hygiene), 24 (fast + clean-tree guard + push, after the pre-CI commit
`chore(2026-10-19-test-splits): operator pre-CI commit, for the run this chunk's verdict reads`), 25 (CI
conclusion) and 26 (per-OS tallies into `evidence/ci.md`), then `/andromeda-wrap-session`.

## Gates
One firing of the block, `.andromeda/runs/2026-10-19T20-53-01-implement/gate-2026-10-19-test-splits.json`:
`entries 26 · green 22 · red 0 · recorded 0 · timeout 0 · not-run 4` (the four `leg = 'operator'` entries).
0 fix iterations. No deferral (rust delta 8). Measured readings in
`escher-0.2.0/chunks/2026-10-19-test-splits/evidence/counts.md`: escher-driver 92 passed, `--list` unchanged;
`cli.rs` 478 and `recording.rs` 459 lines; jscpd 0 clones; workspace tally `lines 199 passed 1045 failed 0
ignored 12`; the windows-gnu `cargo check --tests` green. Matrix: the chunk claims no cap (`matrix.py show`:
claimed 0), so no ref was written.

## Files changed
- New: `packages/escher-driver/src/cli_tests.rs`, `packages/escher-driver/src/recording_tests.rs`,
  `packages/escher-driver/src/recording_off_unix_tests.rs`.
- Modified: `packages/escher-driver/src/cli.rs`, `packages/escher-driver/src/recording.rs`,
  `examples/seven_guis/tests/common/mod.rs`, `examples/seven_guis/tests/host_log.rs`,
  `examples/seven_guis/tests/mcp_log.rs`.
- Evidence: `evidence/driver-tests-before.txt`, `evidence/log-tests-before.txt` (step 1, on the untouched tree),
  `evidence/counts.md`.
- Scope read: `scope: clean — changed 8 · listed 8 · recorded 0 …`; no scope-record line was needed.

## Deviations from plan, with justifications
1. **A fifth shared helper, `common::occurrences`.** After the four helpers step 4 names, jscpd still read 1 clone
   (`host_log.rs` 207-213 ↔ `mcp_log.rs` 232-238, the per-image occurrence sum). The plan's implementation note
   directs a residual near-copy to be shared too, never a file excluded; so it is shared, and jscpd then read 0.
2. **`host_log`'s two needle-sanity asserts are now one.** Step 4 states the shared assert as "no needle is empty
   or a part of the sentinel", which is `mcp_log`'s combined form; `host_log` had held it as two asserts
   ("no needle is empty", "the sentinel is no part of a needle"). The condition asserted is the same; only the
   message is merged. `host_log`'s own `crud-surname` assert now follows the shared asserts instead of preceding
   them.
3. **The shared needle reader's type.** `stand_needles(sentinel) -> Needles { ids, names }` (a two-field struct);
   the plan named what it returns, not its shape.
4. **The driver split was made by a script** (`.andromeda/runs/2026-10-19T20-53-01-implement/split.py`, asserting
   each base span's opening and closing lines, dedenting by exactly four spaces and refusing any line that is not),
   followed by `rustfmt` on the five files. Entries 6-9 confirm byte identity of the kept heads and token
   equality of the moved bodies. The moved bodies hold only `\`-continued string literals, whose leading
   whitespace Rust strips, so the dedent changes no string's value.
5. **`mcp_log` calls `common::recorded_needles` by path** inside its `#[cfg(unix)]` block instead of importing it,
   so the import line needs no gate of its own off unix.

## Surfaced
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started | — | `census · ps · comm escher-session, seven_guis_nati, jscpd, cargo · args — · own tree excluded: 8` → `census: none running` |

The gate entries started their own cargo, jscpd and `escher-session` children and each ended with its entry.

## Decisions & corrections
none
