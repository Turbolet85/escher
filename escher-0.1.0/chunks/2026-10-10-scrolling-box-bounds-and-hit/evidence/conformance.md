# The conformance reading — CSSOM View, before and after the two fixes

Report-only: it grades no criterion (`inputs#I4`; plan steps 4 and 10). Figures and test names written by hand from
the runner's own summary and its two result files; no file of the checkout, no build output and no raw runner log is
in the repository or in a run dir. The checkout is `target/wpt-checkout/`, the kept result files
`target/wpt-reading/before/` and `target/wpt-reading/after/` (`wpt_expectations.txt`, `wptreport.json`) — all three
ignored by git.

## What was run

- **The tests:** web-platform-tests at the commit `wpt/WPT_COMMIT` pins, `71b4d5f0eb7628a5d5f7cd2ee868ce1b1b5dc010`,
  fetched at depth 1; the suite `css/cssom-view`.
- **The runner:** `cargo run --release -p wpt --locked -- css/cssom-view` with `WPT_DIR=target/wpt-checkout`, the build
  and both runs under `taskset -c 12-15,28-31`. A workspace-feature build of the engine: the `writing_mode.rs` body of
  the bounds reader.
- **Before:** the engine as built at `1b195c9a` (no engine line edited; the three new check files alone were in the
  tree). Started 2026-10-10T14:35:00Z.
- **After:** the same checkout, the engine with plan steps 5 and 6 in (the bounds reader in both bodies with its three
  re-based callers, and the hit walk). Started 2026-10-10T14:42:44Z. No later engine edit was made; the mutation
  controls ran after this reading and each restore was proven by sha256 (`controls.md`).

## The cost, measured (the 30-minute bound held)

| Step | Wall clock |
|---|---|
| fetch at depth 1 | 18 s — the checkout's `.git` holds 151,654,135 bytes |
| checkout of the fetched commit | 5 s — the work tree with its `.git` reads 1.1 G (`du -sh`) |
| release build of the runner (`cargo build --release -p wpt --locked`) | 98 s — cargo's own line reads 1m 37s |
| **fetch and build together, the bounded part** | **121 s of the 1,800 s bound** |
| the "before" run | 19 s — the runner's own line reads 18.86 s |
| the "after" run, its rebuild of the runner over the changed engine included | 37 s — the rebuild's cargo line reads 17.51 s |

## The two counts

The runner's own summary, the same lines in both runs except where shown:

| Reading | Before | After |
|---|---|---|
| tests found · skipped · run | 247 · 32 · 215 | 247 · 32 · 215 |
| tests passed · failed · crashed · timed out | 69 · 146 · 0 · 0 | 69 · 146 · 0 · 0 |
| subtests run | 2292 | 2292 |
| **subtests passed** | **966 (42.15%)** | **967 (42.19%)** |
| tests passed, counting partial tests | 86.10 | 86.27 |

The report file's own tally (`wptreport.json`), which names 245 tests and 2269 subtests in both runs: tests PASS 69 ·
FAIL 146 · SKIP 30 in both; subtests PASS 955 · FAIL 1314 before, PASS 956 · FAIL 1313 after. The two tallies differ
from each other by the same amount in both runs (the summary counts 2 more tests and 23 more subtests than the report
names); which tests make that difference was not looked into.

## Every test whose result moved

Compared by test name and by subtest name over both `wptreport.json` files, and line by line over both
`wpt_expectations.txt` files (247 lines each).

| Test | Subtest | Before | After |
|---|---|---|---|
| `css/cssom-view/getBoundingClientRect-scroll.html` | getBoundingClientRect for a scrolled scroll container | FAIL | PASS |

- **Fail to pass: 1 subtest** — the one above. It is the bounds fix read on a document we do not own: a scroll
  container whose content is scrolled reads its own rect. The test as a whole still reads FAIL (5 of its 6 subtests
  pass after, 4 before; `wpt_expectations.txt` line `FAIL YYYNNY` → `FAIL YYYYNY`); its one remaining failing subtest,
  "getBoundingClientRect for a scrolled display none box", failed before and after.
- **Pass to fail: none.** No test and no subtest moved the other way; no test changed between PASS, FAIL and SKIP at
  the test level.
- **The hit fix moved nothing in this suite**, in either direction: every `elementFromPoint` and `elementsFromPoint`
  subtest reads the same before and after.

## What this reading is not

- Not a gate and not a baseline: it has no listed entry, and a later run compares against nothing recorded here but
  these figures.
- Not a reading of the per-package build: the runner names blitz-dom's `writing-mode` feature, so the `document.rs`
  body of the reader was not what these tests ran.
- One suite of one commit; `css/css-overflow` and the pointer-event suites were not run.
