# /andromeda-implement outcome — 2026-10-18-upstream-sync-determinism-and-traces

**Outcome: green + smoke.**

The gates went green on the first firing, in 1 iteration: 30 entries, 25 green · 0 red · 1 recorded (entry 14,
report-only) · 4 not run (`leg operator`, entries 27–30). The trail is
`.andromeda/runs/2026-10-18T06-16-11-implement/gate-2026-10-18-upstream-sync-determinism-and-traces.json`.

## Files changed
- `packages/blitz-dom/src/document.rs` — the take: `git cherry-pick --no-commit fb8d6c15e9ce062b3b41e71cd48988f0093506cc`.
  - It exited 0, staged this file alone (+17 −3) and left no `CHERRY_PICK_HEAD`.
  - It holds upstream's two hunks exactly: the `get_cursor` fallback to `Some(CursorIcon::Default)` and the
    `hovering_below_content_reports_default_cursor` test. Entries 1–3 prove it: patch present, exact take, preservation.
- Evidence (the chunk's records, outside the scope read), each under
  `escher-0.2.0/chunks/2026-10-18-upstream-sync-determinism-and-traces/evidence/`:
  - `before.md`
  - `take.md`
  - `counts.md`
  - `readings.md`
  - `sink.md`
  - `lock.md`
- `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical to `fb842671`. Taffy stays the registry 0.15.0, and
  the root manifest holds no git dependency.

## Readings
- **Workspace tally:** 192 lines · 991 passed · 0 failed · 12 ignored, against 192 · 990 · 0 · 12.
  - The +1 is attributed by a per-target pairing against the engine-debts wrap's fast-leg log. One line of 192 moved:
    blitz-dom's lib, 97 → 98, which is upstream's `hovering_below_content_reports_default_cursor`.
  - That is the prediction exactly.
- **Agent runs:** `run stand` 170 · 0 · 7 and `run all` 562 · 0 · 12, both equal to baseline. The stand's figures did
  not move.
- **Other legs:**
  - a11y: 6 lines · 43 passed · 0 failed.
  - CI scripts: Ran 100, OK.
  - Audit: `advisories ok, licenses ok`.
  - `doc` green.
  - blitz-tests: 132 files, 131 targets, 0 absent by name.
  - `stand_*` files: 41.
- **Sink:** the taken commit adds 0 log, print, env or listener sites. `host_log` and `mcp_log` pass under both
  builds, and `packages/escher-telemetry/` is unchanged.
- **CARRYs:**
  - CARRY 1: upstream still pins a git Taffy, `ea07c4c3`. The four Taffy-main commits wait.
  - CARRY 2: `keyboard_compat.rs` is still listed upstream, and both coverage exclusions stay.
  - CARRY 3: F1–F7 untouched. F8 is the same file in other functions, and its five functions only shift +4 lines. No
    upstream PR.
- **Line map of `document.rs`**, for the wrap's citation re-pointing (`evidence/readings.md`): old 1–2263 unchanged ·
  old 2269–3127 +4 · old 3128–3575 +14.
- **Matrix:** this chunk claims 0 capabilities (`matrix.py show`), so no ref was written.

## Smoke
- The plan's `role = 'smoke'` entry 26 ran green as a P2 gate. It was the windowed `seven_guis_native` with
  `WAYLAND_DISPLAY` set, and it logged `service.name=seven_guis` up to the 10 s bound.
- `agent-run.sh status` after `run all` read `all · passed 562 · failed 0 · ignored 12`, and `cleanup` read `done`.
- Recorded limit: the smoke cannot see a cursor's visibility. The windowed effect of `fb8d6c15e` is witnessed by
  upstream's unit test alone.

## Deviations from the plan
- None. The steps ran in order as written. The scope read is `scope: clean — changed 1 · listed 1 · recorded 0 …
  excluded 94`, and no `scope-record.md` line was needed.
- Two readings went beyond the letter:
  - The per-target pairing of the +1, against the engine-debts wrap's gate log, which proves no offsetting move.
  - The take's measured line map.
- Step 1's "clean tree apart from this chunk's own folder and run dir" did not hold literally. The tree also carried
  the pipeline's own post-wrap bookkeeping: three modified `.andromeda/` ledgers, `.claude/session-handoff.md`,
  `escher-0.2.0/working-route.md`, and four untracked run dirs or files under `.andromeda/runs/`. All are excluded
  from the scope read, none is source, and each is recorded in `evidence/before.md`.

## Surfaced
- nothing

## Process hygiene
`gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args agent-run.sh`:
`census: none running`.

| process | started by | final state |
|---|---|---|
| `seven_guis_native` (entry 26) | a leg (this run's gate block) | terminated (`timeout 10`, census none running) |
| cargo / rustc (entries 9–25) | this run | terminated (census none running) |
| agent-run.sh boot / run | this run | terminated; `cleanup` done |

## Next
→ **the operator pass**, plan step 4, then `/andromeda-wrap-session`:
1. Entry 27 — `gate.py hygiene`, fired as written.
2. The pre-CI commit (`git add -A`):
   - Its subject is exactly `chore(2026-10-18-upstream-sync-determinism-and-traces): operator pre-CI commit, for the
     run this chunk's verdict reads`.
   - Its body carries `(cherry picked from commit fb8d6c15e9ce062b3b41e71cd48988f0093506cc)`.
3. Entry 28 — the fast leg, the clean-tree guard and the push, through `gate.py run --plan … --operator 28`.
4. Entry 29 — `ci.py conclusion --sha HEAD --wait 1800`, fired as written.
5. Entry 30 — the per-OS tally read with `<id>` = the CI run id, into `evidence/ci.md` against CI#39100630123.

Each line goes to `evidence/operator-pass.md`.

## Decisions & corrections
- A guard refusal and a recurrence of a standing learning, host-linux «Paths & the working directory»: bash-guard
  refused a bare `cd` into the skill's references dir (`Blocked: cd (…/andromeda-implement/references) moves the
  session cwd`). It was re-issued once with absolute paths and ran.
