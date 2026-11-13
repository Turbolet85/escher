# Implement outcome — 2026-11-13-upstream-sync-determinism-and-exploration

**Outcome: green + smoke**, with one open item: the widened pin check's red-before-green control is unproven (below).

## Files changed (45 — `gate.py scope`: changed 45 · listed 45 · recorded 0, `scope: clean`)
- **The take** (step 1): 29 no-commit cherry-picks, each clean in the plan's order. 38 files, +1157 −182, the research
  figure exactly. The seven upstream targets are among them. Cargo.lock and every Cargo.toml are byte-identical to
  `19596fd4b` (gate entry 1 green).
- `tests/blitz-tests/checks/counter-cli.json`, `counter-mcp.json` — re-recorded by `record_checks.sh` (step 2).
- `deny.toml` — the `RUSTSEC-2026-0459` ignore with its written reason (step 3).
- `.github/workflows/wpt.yml`, `publish-browser.yml`, `wpt-post-results.yml` — 18 live `uses:` pinned (step 4).
- `.github/scripts/test_ci_workflows.py` — new `WorkflowPinTest` over every workflow (step 5).
- `tests/blitz-tests/tests/blend_mode_frame.rs`, `inline_box_decorations_frame.rs` — new (steps 6, 7).
- `tests/blitz-tests/tests/all.rs` — the two `mod` lines (step 8).
- Evidence in `.andromeda/cache/chunks/2026-11-13-upstream-sync-determinism-and-exploration/evidence/`:
  `lock.md` · `checks.md` · `pins.md` · `tests.md` · `sink.md` · `readings.md` · `counts.md` · `control-1.txt`.

## Gates (trail `.andromeda/runs/2026-11-13T14-48-05-implement/gate-2026-11-13-upstream-sync-determinism-and-exploration.json`)
`entries 20 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 3`, the three `leg operator` (18 hygiene, 19 push,
20 CI read). Everything was green on the first firing, with 0 fix iterations.
- The `cargo +1.100.0` clippy lines (7, 8) are green, so the CI clippy red is cured by `da271cde1`.
- `fast` (2): green in 284 s; ci-scripts `Ran 112 tests` OK; `host_log` 6 · `mcp_log` 3 · `telemetry_drop` 1 passed.
- `run stand` (4): 262 · 0 · 7, unchanged. `run all` (5): 785 · 0 · 12 (+17, `evidence/counts.md`).
- Audit (16): `advisories ok, licenses ok`. Pins probe (17): exit 1, last line 0.
- `doc`, `a11y`, `msrv`, `stand_replay`, `recording` and both frame tests: green.
- No deferral: the chunk has Rust delta, so every gate ran.
- No matrix write: the chunk owns no `coverage` row (0 rows, as the plan's "no capability claimed" reads).

## Controls (`gate.py control`, `evidence/control-1.txt`)
`control: 2 holds · 0 timeout · 0 does not hold · 1 unreproduced · clone removed`
- `blend-normal` holds: `blend_mode_frame` reads red with `convert_blend_mode` forced to `BlendMode::default()`.
- `own-box-always` holds: `inline_box_decorations_frame` reads red with `paints_own_box` forced `true`.
- **`wpt-pin-tag` UNREPRODUCED.** `fast` read red IN PLACE in the tool's clone. The clone root
  (`~/.cache/andromeda-control/escher-2026-11-13-upstream-sync-…`) pushes `stand_session_lifecycle`'s socket
  past the 94-byte bound (`StateDirTooLong`), the contract's named limit "a witness keyed on the tree's own path
  reads the clone's". A hand control on a scratch copy of `.github/` (the tree untouched) was refused by the
  session's permission classifier and not run. **So the plan's acceptance "reads red on a tag before green"
  (step 5, `cry-zexngiys`) is unproven. The operator decides:**
  - run that hand control: copy `.github/`, set one `wpt.yml` pin back to `@v5`, run `python3 -m unittest discover -s .github/scripts -p test_ci_workflows.py -k WorkflowPinTest` in the copy, and expect FAIL;
  - or accept it on its construction.

## Smoke
The smoke ran as P2 gate entry 6: stayed up its 10 s and logged `service.name=seven_guis`. Smoke ✓.

## Deviations from plan, with justifications
- Step 4 pinned **18** `uses:` lines, not 17. The take's `f2065fd48` added `cargo-bins/cargo-binstall@v1.27.0` to
  `wpt.yml`, and the step says to pin over the taken text.
- Step 4 resolved `actions/checkout@v5` and `Swatinem/rust-cache@v2` to their own SHAs today, which differ from
  `ci.yml`'s pins (`checkout@v4`, an older rust-cache commit), as the step reads (`evidence/pins.md`).
- Step 5 moved `test_j` and `test_k` out of `CiWorkflowTest` into a new `WorkflowPinTest` (one `TestCase`). It
  added two cases: one that the check reads all four workflows, and one for the trailing ref comment, read as text
  because YAML drops comments. The ci-scripts count moves by +2.
- Step 5's control: unreproduced, above.
- Step 8: six of the seven upstream targets run one layout mode. Each is recorded as an exception and not edited
  (`evidence/tests.md`).
- The scope record: none needed, no out-of-list edit.

## Findings for the wrap
- **The hit arm's returned coordinates** (`evidence/readings.md`): after `7ea0a2a5a`, escher's `clips_to_padding_box`
  early return in `Node::hit_inner` returns `x`/`y` in border-box coordinates for an inline root. The final self
  hit now returns content-box ones (`x - content_box_offset`). The stop itself reads `own_x`/`own_y` and is
  unmoved, and no check reads the returned coordinate. It is escher's own arm, not upstreamable.
- `cry-mjwr45xr`'s record pointer (`escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/lock.md`) names
  no disk path. Its file is in the store (`rec.py files 2026-11-01-upstream-sync-ui-graph`).
- The trial worktrees `target/tmp/trial-det` and `target/tmp/trial-a` can now be removed, on the operator's call
  (plan, Implementation notes).
- Scratch reads under `target/` (gitignored): `target/tmp/ui-graph-files/`, `target/tmp-*.log`, `target/tmp-take.diff`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none | — | `census: none running` (comm seven_guis_native, escher-session, cargo · args agent-run.sh) |
The smoke's `seven_guis_native` ended by its own `timeout 10`. The control's clone was removed by the tool.

## Decisions & corrections
- The bash guard refused a `rec.py files` output piped into `tail`, and a `cd` into a subdirectory. Each was
  re-issued once in the form it allows.
- The auto-mode permission classifier refused the hand pin control on a scratch copy of `.github/` as a control run
  outside the gate tool ("Undermining Oversight"). It was not worked around, and is left for the operator.
- A standing learning recurred (host-linux.md 2026-10-13, the 94-byte socket bound), this time under the control
  tool's clone root rather than the session scratchpad.

## Next
The OPERATOR PASS: entry 18 (hygiene), entry 19 (the push, after the pre-CI commit), entry 20 (the CI read). Before
it, the operator's word on the unproven pin control. Then `/andromeda-wrap-session`.
