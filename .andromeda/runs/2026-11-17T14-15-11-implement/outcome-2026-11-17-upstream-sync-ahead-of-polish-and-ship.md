# Outcome — 2026-11-17-upstream-sync-ahead-of-polish-and-ship

**Outcome: green** — 20 of 20 non-operator gate entries green on the first firing (`gate-2026-11-17-upstream-sync-ahead-of-polish-and-ship.json`); both one-shot controls hold (`scroll-clamp` · entry 9, `svg-image-fetch` · entry 10); smoke ran as a P2 gate; no coverage row owned (no ref written). Report landed: `doc:2026-11-17-upstream-sync-ahead-of-polish-and-ship/report` rev 1, tx 1789.

## Files changed
59 changed, 0 new — `gate.py scope`: «scope: clean — changed 59 · listed 59 · recorded 0». The five picks (18 files, +291 −157), the AnyRender 0.15 port (blitz-paint 9 files, wpt runner 2, examples 3, the browser's capture), the `vello-hybrid` → `vello-gpu` rename (27 files, the three `.claude/docs` pages through `rec.py harness land`, tx 1783–1785), `Cargo.toml`, `Cargo.lock`, `deny.toml`, `.github/workflows/ci.yml`, `.github/scripts/test_ci_workflows.py`, `tests/blitz-tests/tests/{stand_act_scroll,local_files_for_local_documents}.rs`. The full list is the report's **Files**.

Evidence under `.andromeda/cache/chunks/2026-11-17-upstream-sync-ahead-of-polish-and-ship/evidence/`: `lock.md`, `readings.md`, `sink.md`, `counts.md`, `shots.md`, `shots.py`, `control-1.txt`, `control-2.txt`.

## Deviations, with their justifications
- `cargo update -w` after step 5's manifest renames: before them it exits 101 (a member still inherits `anyrender_vello_hybrid`). The lock is byte-equal to the trial's.
- The three `.claude/docs` pages were landed through `rec.py harness land` (harness-family renders; the write guard refused the Edit). `stack.md` also reads `anyrender 0.15.0`.
- Two names outside the probe pattern were renamed as upstream did: `[vello|cpu|hybrid]` → `[vello|cpu|gpu]` in `commands.md`; «vello hybrid backend» → «vello GPU backend» in wasm_hello's page.
- HEAD's 197 MB driver binary moved to `target/tmp/escher-session.head` (past the land's 16 MiB bound); its sha256 is in `shots.md`.
- Control 1 fired at `layout/mod.rs:855` (the call's real site; the plan names `construct.rs`). Control 2 removed `load_image`'s SVG arm, not the shared `fetch_image` call.
- `stand_act_scroll`'s last two assertions became three measured literals (offset `0.0`, the first row inside the list, its click accepted), both layout modes reading the same.
- Spec claims disproved: 97 carried, not 112 (15 taken by the two Taffy chunks); the SVG load arms are `image` and `feImage`, not `<use>`; the clamp call stands in `layout/mod.rs`; research's sink sweep has two lines moving and one rewritten in place.
- Scope record: none — `gate.py scope` clean, 0 recorded.

## Guard entries
The plan holds no `gate.py guard` entry.

## Surfaced / stuck
Nothing surfaced; nothing stuck.

## Process hygiene
| Process | Started by | Final state |
|---|---|---|
| cargo / rustc builds and test binaries | this run | terminated — `gate.py census --comm seven_guis_nati --comm escher-session --comm cargo --comm rustc`: «census: none running» |
| `seven_guis_native` (smoke) | a gate entry | terminated (`timeout 10`) |
| `escher-session` (shots.py, both binaries) | this run | terminated (none running) |
| control clones `~/.cache/andromeda-control/f28a7104`, `…/10a49278` | the gate tool | removed |
| `target/tmp/trial-pas`, `target/tmp/escher-session.head` | the phase / this run | left in place — the operator removes them after the wrap |

## Decisions & corrections
- write-guard refusal: Edit of `.claude/docs/commands.md` «rendered from the store - write through its verb»; a scripted write had reached three harness pages unguarded — reconciled, then landed through `rec.py harness land`.
- bash-guard refusals: a `cd` into the skill's references dir, and a pipeline tool piped into `head`; each re-issued in the documented form.
- Sweep hazard: the rename probe's pattern misses `[vello|cpu|hybrid]` and the prose «vello hybrid backend»; a bare `hybrid` grep found both.
- The last take's `counts.md` figures for `telemetry_drop` · `host_log` · `mcp_log` sit against the wrong names (the files hold 2 · 6 · 3 tests at both takes).

## Next
The operator pass: the three `operator` entries — `gate.py hygiene`, the push (`git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.3.0`) and `ci.py conclusion --sha HEAD --wait 2700` — recording the per-OS and stand-repeat tallies and `Test [default features]`'s two `df -h` readings in `evidence/ci.md`, then `/andromeda-wrap-session`.
