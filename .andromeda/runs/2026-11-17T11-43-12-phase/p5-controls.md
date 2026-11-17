# P5 controls — 2026-11-17-upstream-sync-ahead-of-polish-and-ship

Each control fired AS THE ENTRY: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --chunk 2026-11-17-upstream-sync-ahead-of-polish-and-ship --docs-root . --entry {n} --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/82249230-ee3f-4f07-a3ec-f62fd9a45fdd/scratchpad/ctl-clone-1`. The clone is a `git clone --shared` of the project at HEAD `c74c7f3e6750fd51a57f8c0910898b890ae8694c`, with the trial's AnyRender take applied uncommitted (`take-anyrender.patch`: the five commits plus the AnyRender 0.15 move and port, 36 files, +714 −434).

## Must-pass — the take as the plan builds it (the clone, the patch applied)
- C1 · entry 1 (git sources but Taffy's): green · exit 1 · last line 0
- C2 · entry 2 (anyrender 0.15.0 in the lock): green · exit 0 · last line 1
- C3 · entry 3 (zip 9.x in the lock): green · exit 0 · last line 1
- C4 · entry 6 (census, added side): green · exit 0 · last line 3
- C5 · entry 7 (census, removed side): green · exit 0 · last line 3

## Must-pass — inputs the plan's later steps build, made in the clone
- C6 · entry 4, with deny.toml's four zip-ignore lines deleted (step 6): green · exit 1 · last line 0
- C7 · entry 5, with the rename made by `sed` over the files the probe lists (step 5's names): green · exit 1 · no output
- C8 · entry 20, with `.andromeda/cache/chunks/2026-11-17-upstream-sync-ahead-of-polish-and-ship/evidence/shots.md` written (step 9's record): green · exit 0

## Must-fail — inputs each guard must read red on
- C9 · entry 1, with a `[[package]]` `anyrender_probe` whose `source = "git+https://github.com/DioxusLabs/anyrender?rev=97d06af9…"` appended to the clone's `Cargo.lock`: red · exit 0, last line not 0
- C10 · entry 6, with `fn control_probe() { tracing::info!("control"); }` appended to the clone's `packages/blitz-dom/src/util.rs`: red · last line '3' ✗ (last: '4')

The clone is left in the scratchpad (a fresh directory per run; no removal).
