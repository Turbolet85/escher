# Light gate — 2026-10-16-file-splits (P7.1)

`gate.py run --plan escher-0.2.0/chunks/2026-10-16-file-splits/plan.md` (trail `gate-2026-10-16-file-splits.json`, logs
`$TMPDIR/andromeda-gate/2026-10-16-file-splits/wrap-2026-10-16T17-27-46`), no `--skip`, no `--void`:
`entries 26 · green 22 · red 1 (13) · recorded 0 · timeout 0 · not-run 3`.

- Entries 1–12 and 14–23: green, the fast leg 154.31 s, the doc and a11y legs green.
- Entries 24 · 25 · 26 (`leg = 'operator'`): not run by plain `run`, re-verified from
  `escher-0.2.0/chunks/2026-10-16-file-splits/evidence/operator-pass.md` — hygiene `clean`, the fast leg and push green
  (`history moved … 4f393991→54d4ea21`, the intended push), `ci.py conclusion` `54d4ea21ff66 verdict: green · checks
  17/17` on CI#38937698317. HEAD is still `54d4ea21` (the final HEAD of the pass); every atom holds.
- Entry 13 (`git diff --name-only 4f3939914e525e284a32beb91bdc97972d40c904 -- packages examples tests apps wpt
  Cargo.toml Cargo.lock | grep -c -v -x -E '<the three modified sources>'`, expect `exit 1` · `last line 0`): **red on
  the probe's form, not on the tree** — exit 0, last line 8.
  - The 8 are the eight new files the plan names. At implement they were untracked, which `git diff` does not list, so
    the entry read green. The operator pass's pre-CI commit `54d4ea21` tracked them, so the same diff from the base
    now lists them: `git diff --name-only 4f393991 54d4ea21 -- …` reads 11 paths, the chunk's 11.
  - The same diff with entry 14's allow-list for the new files joined reads `0`, exit 1 (run beside the tool,
    redirected to a file and its `$?` read bare). No tracked file outside the chunk's three sources and eight new
    files moved, which is the property the entry guards.
  - Treatment: recorded and proceeded on the operator's word — «Record and proceed (Recommended)» (the operator,
    2026-10-16, question dialog at this wrap's P7.1). The probe-form defect is logged as friction.
