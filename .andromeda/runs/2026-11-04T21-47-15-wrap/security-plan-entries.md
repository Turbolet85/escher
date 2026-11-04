@amendment marker=2026-11-04-dioxus-runtime-probe
title: the zip ignores gain RUSTSEC-2026-0429
section: Dependency Security (dependency-security: the Audit tool); Supply chain integrity (dependency-security: Licence compliance)
change:
  - The ignore count reads nine (was «Eight ignores stand»).
  - The zip 2.4.2 passage names four IDs (was three): RUSTSEC-2026-0367, 0368 and 0369 since 2026-10-16-fast-failing-mutants, and RUSTSEC-2026-0429, `ZipArchive::extract` aborting the process on a symlink entry with a large declared size; each fixed only in zip 9.0 (was «each fixed in zip 9.0»).
  - The reach reads `anyrender_serialize 0.8.0`'s `deserialize` opening a `ZipArchive`, reading entries `by_name` into memory and never calling `extract` (was «reads archive entries `by_name` into memory and never extracts to disk»); the founder's ruling is scoped to «the three».
  - A second external decay is recorded: RUSTSEC-2026-0429 with RUSTSEC-2026-0433 turned the Dependency audit red (CI#41911351081) over a lock the green CI#41346357110 read, the ignore reading green (CI#41914044942).
  - The `deny.toml` citations after the added ignore line read one line on: the `[advisories]` and `[licenses]` tables, the zbus-family ignores and the NCSA warning, in the Audit tool and in Licence compliance.
why: the report's Schema / config fact: `deny.toml` gained the RUSTSEC-2026-0429 ignore with the zip IDs' reason, no fixed release being reachable (anyrender_serialize 0.8.0 the latest, requiring zip ^2.1), and the advisory turned CI red on an unchanged lock; the ignore is the operator's own fix directive in session (2026-11-04), under the project's security rule that an advisory is fixed by a semver-compatible update or ignored by ID with a written reason. The count is read from `deny.toml`, which holds nine IDs; the report's «eight» is its own miscount.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the audit's residual owners as store rows
section: Dependency Security (dependency-security: the Audit tool)
change:
  - The zip ignores are owned by the store's `residual` row `res-h7acz56c` (target: next) (was «a `.andromeda/residuals.md` line (target: next; `.andromeda/residuals.md:29`)»).
  - The paste ignore, the zbus-family ignores and the memmap2 measured limit are each owned by «a `residual` row in the store (target: next)» (was «a `.andromeda/residuals.md` line», the zbus one citing `.andromeda/residuals.md:30`).
why: the report's facts: the residuals live as `residual` rows in the store since the bases door and no `.andromeda/residuals.md` stands, a path grep there finding no file; `res-h7acz56c` was revised to name the fourth zip ID.
kept: the `.andromeda/residuals.md` owners outside Dependency Security (the Input Validation witnesses) stand unchanged; a rename of that path alone is not this chunk's work, and the family is routed whole to a carry.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: pulldown-cmark 0.13.6 fixes RUSTSEC-2026-0433
section: Dependency Security (dependency-security: the Audit tool)
change:
  - After the RUSTSEC-2026-0414 sentence, the lock resolves pulldown-cmark 0.13.6 by a `Cargo.lock`-only update, semver-compatible with no coupled pin and reached only through `rdme`, fixing RUSTSEC-2026-0433 (quadratic parsing of nested image wikilinks under `ENABLE_WIKILINKS`, which `rdme` never sets) (was the lock at 0.13.5).
  - The audit leg reads `advisories ok` and `licenses ok` locally and in CI#41914044942.
why: the report's Dependencies fact: pulldown-cmark bumped 0.13.5 to 0.13.6 in `Cargo.lock` alone, no path this repo uses affected; the update is the operator's own fix directive in session (2026-11-04).
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: advisories published after a green audit
section: Dependency Security (dependency-security: a new Audit tool bullet)
change:
  - New: a CI Dependency audit can go red on advisories published after the base's green audit with `Cargo.lock` and `deny.toml` unchanged (CI#41911351081 against the green CI#41346357110 on the same graph); the local `fast` leg runs no audit, and `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push.
why: the report's measured fact for the next audit: the first CI read went red on two new advisories over an unchanged lock, and the local audit leg read their fix green before the push. A standing rule for later chunks: run the audit leg locally before a push.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the MountedData hazard re-measured and pinned
section: Error Handling (error-handling: mounted handles from a spawned task)
change:
  - The record reads its mechanism at HEAD: `DioxusDocument::poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, where dioxus-core 0.7.10 first polls a task spawned from a key handler that wrote a signal (`handle_task_wakeup`), so that task's `set_focus`, borrowing through `doc_mut`, panics `RefCell already borrowed` (was «first polled inside `render_immediate`, which holds the document's `RefCell`», a `MountedData` call in that task).
  - Measured headlessly in both layout modes (was «through the harness's `poll(None)`»), the windowed path read, not run.
  - Pinned by `dioxus_runtime_probe`'s two `should_panic` tests, one per layout mode, its fix routed to the route entry «MountedData buffered render».
  - New: the same handler calling `set_focus` at once with no task moves focus and does not panic; `get_scroll_offset` / `get_scroll_size` take a shared borrow through `doc`, read from the code, not run.
  - Its source reads the 2026-11-04-dioxus-runtime-probe report, the Cells sentence still the 2026-10-31-engine-debts report's (was the engine-debts report alone).
why: the report re-measured the record at HEAD and pinned the hazard with no fix applied; the fix goes to a chunk of its own on the founder's word, given by dialog in the overseer session and relayed by the operator, porting upstream's buffered render onto the 0.7 bridge to invert the pin.
kept: Cells' synchronous handle calls stand, the native handle acting at the call.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
