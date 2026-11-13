# P5 check 4 (9) — must-pass controls, fired as the entries in a scratch clone

Script: `$TMPDIR/{scratchpad}/controls.sh`, run 2026-11-13T14:36Z. The clone is `git clone --shared` of the project at
HEAD `19596fd4b`, in the script's own `$(mktemp -d)`, removed by its `trap`. Each entry was fired through
`gate.py run --chunk 2026-11-13-upstream-sync-determinism-and-exploration --docs-root . --entry {n} --project-root {clone}`.

| entry | input | reading |
|---|---|---|
| 16 `bash .github/scripts/ci-leg.sh audit` | clone unmodified | red · exit 1 (as on the tree: RUSTSEC-2026-0459) |
| 16 | `deny.toml` + one `{ id = "RUSTSEC-2026-0459", reason = … }` before the RUSTSEC-2026-0429 entry | green · exit 0 · `advisories ok, licenses ok` met |
| 17 the pin count | clone unmodified | red · exit 0 · last line 17 (as on the tree) |
| 17 | every live tagged `uses:` of wpt.yml, publish-browser.yml, wpt-post-results.yml set to a 40-hex sha + `# {tag}` | green · exit 1 · last line 0 |
