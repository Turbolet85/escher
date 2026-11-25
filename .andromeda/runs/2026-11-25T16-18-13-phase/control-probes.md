# P5 controls — 2026-11-25-circle-drawer-virtual-nodes

## Entry 1 (the manifest and lock probe)
Scratch clone: `git clone --shared` of the project at HEAD `a5d18f604a88533232d9164b5db1afd3e6159070`, at
`$TMPDIR/{scratchpad}/probe-clone` (left in place; no step deletes it).
Fired as the entry: `gate.py run --chunk 2026-11-25-circle-drawer-virtual-nodes --docs-root . --entry 1 --project-root {the clone}`.

- unmodified: `1 probe green · exit 0 · 0.0s · 0 B → 1.log` — the reading on the real tree (green, exit 0).
- input: a line `# control line` appended to `examples/seven_guis/Cargo.toml`, uncommitted:
  `1 probe red · exit 0 ✗ (exit 1) · exit 1 · 0.0s · 0 B → 1.log` — red on a manifest change.

## The local windows check (the plan's first entry 9, removed)
Baseline on the untouched tree: `9 build red · exit 0 ✗ (exit 101) · exit 101 · 7.02s · 37745 B → 9.log`. Cause read from
its log: `error: failed to run custom build command for aws-lc-sys v0.46.0`, the build script finding no
`x86_64-w64-mingw32-gcc` on this host; `unused import` 0 times — blitz-tests never compiled. A red on an existing property (a
missing host tool), never this chunk's subject: the entry is removed and CARRY 5 is read from the CI windows job's log.

## The windows job-log read (the operator entry replacing it)
Typed as the entry types it (`bash -o pipefail -c`), differing from the entry by the run id operand alone:
- CI#45619756069 (`808f9611f`, the run whose windows log recorded the warning): prints `1`, exit 0 — red against the
  entry's `expect = ['exit 1', 'last line 0']`. The log's three unused-import warnings read: `std::time::Duration` at
  `tests\blitz-tests\tests\stand_act_error_text.rs:27`, `Command` and `Stdio` at
  `examples\seven_guis\tests\host_binary.rs:10`, `snapshot` at `tests\blitz-tests\tests\virtual_nodes_in_every_read.rs:24`.
- CI#45580739068 (`c34877ccd`, before the import existed): prints `0`, exit 1 — green.

## The windows job-log read, restated at the review (the log proved read)
The operator's review: a count alone reads green on an empty log. The entry now counts, in one `awk` stage, the line
every `Test (windows)` log holds and the warning: `log unread` at exit 2 unless the log holds `Compiling blitz-tests v`
at least once, else `snapshot {n}` at exit 0 for none and 1 for any. Its expect: `['exit 0', 'last line snapshot 0']`.

The proving line, measured in both recorded windows logs, colour codes dropped: `Compiling blitz-tests v` 1 time in
CI#45619756069's, 1 time in CI#45580739068's (`Compiling blitz-tests v0.3.0-beta.4 (…\tests\blitz-tests)`).

Typed as the entry types it (`/usr/bin/bash -o pipefail -c`), the run id the only operand changed:
- CI#45619756069 (the warning present): `snapshot 1`, exit 1 — red.
- CI#45580739068 (before the import): `snapshot 0`, exit 0 — green.
- run id `1` (a refused read: the jobs call answers no job, so no log is fetched and `awk` reads an empty input):
  `log unread`, exit 2 — red. The empty-input reading.
