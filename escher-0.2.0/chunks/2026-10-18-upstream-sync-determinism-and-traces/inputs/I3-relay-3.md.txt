# Orchestrator measurement — Taffy registry 0.15.0 vs upstream's git rev, /andromeda-phase P3, 2026-10-18T06:00Z

Read by the orchestrator on the dev host through `gh api repos/DioxusLabs/taffy/compare/v0.15.0...ea07c4c304216c9b48bdd88c46153c1cbe3cae10` (read-only).

- status `ahead` · ahead 14 · behind 0: Taffy main at `ea07c4c3` is the 0.15.0 release plus 14 commits, none released to crates.io (crates.io's newest `taffy` is 0.15.0, inputs#I2).
- The 14, oldest first:
  - 6c2106e3d 2026-10-13 Store a caller-defined id with each placed float, and read placements back from BlockContext (#1303) — the API change behind `place_floated_box`'s new first argument.
  - 7b1377447 2026-10-14 Update zip requirement from 8.6.0 to 9.0.1 (#1308)
  - 63779e56f 2026-10-16 grid: count calc() track sizes as fixed for auto-repeat (#1320)
  - 43083ebb2 2026-10-16 Block: apply relative inset to floated boxes (#1305)
  - 7a9f4212b 2026-10-16 Float: don't clamp the first float to the top of the block formatting context (#1304)
  - e77a7a8d9 2026-10-16 Grid: take the container baseline from the final layout of its first item (#1322)
  - 8bb96a26f 2026-10-16 Make container baselines consistent across layout modes (#1323)
  - 544617c40 2026-10-16 Add the PanGUI benchmarks (#1324)
  - c02379779 2026-10-16 Benchmarks: pass percentages to Yoga in the range 0-100 (#1325)
  - 9f9b73657 2026-10-16 Allocate the flex items Vec once, sized for the child count (#1327)
  - 21bd1cde5 2026-10-16 Flexbox: don't allocate a Vec of unfrozen items when resolving flexible lengths (#1328)
  - b8151f5ae 2026-10-16 Flexbox: compute used space and flex factor sums in one pass when resolving flexible lengths (#1333)
  - 1cb4b02a4 2026-10-17 Flexbox: don't measure the cross size of items that are stretched to a line of known size (#1332)
  - ea07c4c30 2026-10-17 Flexbox: fix intrinsic main size with negative margins on items with no content size (#1335)
- Layout behaviour commits among them: floats (#1303, #1304, #1305), grid (#1320, #1322), baselines (#1323), flexbox (#1327, #1328, #1333, #1332, #1335). The stand's CSS holds 0 `float:` declarations (`grep -rn 'float:' examples/seven_guis/src`, 0 lines); flexbox and baselines are what the stand lays out with.
- Upstream's lock delta at `fb8d6c15e` against `23b9b26a9` moves `taffy`'s source alone (registry → `git+https://github.com/DioxusLabs/taffy?rev=ea07c4c3…`, the checksum line gone) and one dependency line of `cssparser-macros` 0.7.1 (`syn 3.0.6` → `syn 2.0.119`, from upstream's 4a5ab9cc9); no package enters or leaves the lock (`git diff 23b9b26a9 upstream/main -- Cargo.lock`).
