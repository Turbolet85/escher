# layout-templates — amendments

One entry per amendment to `layout-templates.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-06-headless-stand — the headless stand's TaskShell mount; seven_guis citations re-pointed
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** the seven_guis entry adds the headless stand: it skips Home and mounts one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell` — `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body` — at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans; its `app.rs` citations re-pointed to Home, TaskShell and the CSS constants after the chunk's line shift.
**Why:** the headless stand chunk added a second way into TaskShell; the windowed Home → TaskShell flow is unchanged.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-stand-test-contract — agent-run.sh on the cli surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/agent-run.sh` — usage `agent-run.sh <verb> [selection]` (verbs `boot · run · status · cleanup · logs`, selections `stand · all · <blitz-tests file name>`), usage on stderr with exit 2, stdout JSON lines only, raw cargo output kept in `target/agent-run/run.log`; `scripts/agent-run.ps1` its Windows pass-through; the exit grammar and event schema stay in test-plan §3.
**Why:** the stand test contract chunk added a repository CLI entry point beside `paint_bench` and `bump`.
**Kept:** the surface's NOT YET MEASURED marker (tooling context, expression level, signature placement) stands.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/

## 2026-10-06-cold-agent-run-pipe — the cold-agent.sh CLI surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/cold-agent.sh` beside agent-run.sh — usage `cold-agent.sh <verb> [task]`, verbs `run · status · cleanup · logs`, the one task `counter`; a usage error prints to stderr with empty stdout and exits 2 before any precondition; stdout is JSON lines only; the raw session transcript stays in `target/cold-agent/transcript.jsonl`; `scripts/cold-agent.ps1` its Windows pass-through; the exit grammar, schema and stub in test-plan §3.
**Why:** the cold-agent run pipe chunk added an agent-invocable CLI.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/

## 2026-10-06-upstream-sync-element-identity — file:line citations re-pointed after the upstream merge
**Section:** every section citing a merged upstream file's lines
**Change:** 4 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`; the merged files' lines moved.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-stable-element-ids — the lean tasks' controls carry author ids
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** each lean task's controls and value displays carry author `id`s that are their stable element ids — counter-value · counter-increment; flight-one-way · flight-return · flight-start · flight-return-date · flight-book · flight-booked; timer-progress · timer-elapsed · timer-duration · timer-duration-value · timer-reset; crud-filter · crud-list · crud-name · crud-surname · crud-create · crud-update · crud-delete — and each CRUD row the Dioxus key `{i}`; no class, style, wrapper or order added, no task CSS selects by them.
**Why:** the chunk keyed the lean tasks' controls so agents aim at semantic ids (operator's P4 choice); the screens' structure is unchanged (markup-only gate).
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/

## 2026-10-06-id-persistence — CRUD rows keyed by person id
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** each CRUD row's Dioxus key was `{i}`, its index in the people list (per 2026-10-06-stable-element-ids — the lean tasks' controls carry author ids). It is now `{person.id}`, the person's model-assigned id (fixture people 0–2, Create from 3), so a row's id `…/div[{person.id}]` follows its person under filter, Create and Delete. The `crud.rs` citation is re-pointed by the chunk's line map. No class, style, wrapper or order changed.
**Why:** an id an agent holds must name the same person after another person's Delete (operator's P4 choice of person keys over index keys).
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/

## 2026-10-06-accessibility-tree-identity — stand inputs named by attributes alone
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) · citations re-pointed
**Change:**
- The six task inputs are named by attributes alone: `for` on the existing labels of `timer-duration`, `crud-filter`, `crud-name` and `crud-surname`; `aria-label` "Departure date" / "Return date" on `flight-start` / `flight-return-date`. No element, class, style or order changed.
- 4 citations re-pointed by the measured line map: 1 into `flight_booker.rs` (+1 from old line 82, +2 from old 88) and 3 into dioxus-native-dom `dioxus_document.rs` (+2 from old line 18).
**Why:** the accessibility-tree chunk needed a non-empty name on every stand input without touching the stand's structure; its markup probe confirmed the files differ from the base only by those attributes.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-id-stability-across-code-edits — CRUD rows and Home's cards read author keys
**Section:** §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell bullet)
**Change:**
- A CRUD row's id was `…/div[{person.id}]`, from its Dioxus key; now the row carries the author `id` `crud-person-{person.id}` beside that key and reads it (fixture `crud-person-0` · `crud-person-1` · `crud-person-2`, Create from `crud-person-3`).
- Adds Home's seven task-card ids `task-card-{slug}` — `counter`, `temp-converter`, `flight-booker`, `timer`, `crud`, `circle-drawer`, `cells`.
- Both are attributes only: no class, style, wrapper or order, and no CSS selects by them.
- Seven line citations re-pointed (`app.rs`, `crud.rs`); two added.
**Why:** every element an agent can act on reads an author key, the founder's rule (the founder, 2026-10-06); the stand's layout is unchanged.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/
