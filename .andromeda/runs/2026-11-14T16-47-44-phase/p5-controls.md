# P5 controls — 2026-11-14-remote-pages-local-files (validation check 4 (9))

Clone: scratch clone `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/ab31177a-2288-481c-aa3b-096510715751/scratchpad/ctl-clone`,
`git clone --shared` of the project, HEAD `e73c5ff8cede598c43f583d5b56910652bc011f9`. Each entry fired as itself:
`gate.py run --chunk 2026-11-14-remote-pages-local-files --docs-root . --entry {n} --project-root {clone}`.

| entry | input | reading |
|---|---|---|
| 4 `git diff e73c5ff8c -- apps/browser \| grep -cE …` | unmodified clone | green · exit 1 · last line 0 (as on the tree) |
| 5 `git diff --quiet e73c5ff8c -- Cargo.lock` | unmodified clone | green · exit 0 (as on the tree) |
| 4 | `apps/browser/src/favicon.rs` + an added comment line (must pass) | green · exit 1 · last line 0 |
| 4 | + an added `println!` line (must fail) | red · exit 0 (count 1) |
| 5 | `Cargo.lock` + an appended line (must fail) | red · exit 1 |
