# Scope — 2026-10-19-shared-fixtures · Shared fixtures

**Working entry (escher-0.2.0/working-route.md:36, verbatim):** Shared fixtures — fixture pages beside the stand and
one held network stand-in, stated once in blitz-tests' shared modules, run through agent-run

**Origin:** the route run `.andromeda/runs/2026-10-16T06-47-57-route/` — the tests validator's insert
(`validation-tests.md` lines 4-5: test-plan §3's «NOT YET MEASURED — the test-data bootstrap mechanism of the
5-command contract» and §8, where the held network stand-in is repeated file-private), placed at the head of Epoch 4
after its sync as infrastructure ahead of Load wait, Select by id and Large screen in parts (`merge-decisions.md`
line 16; the engine-debt fixtures stayed per-file). The entry names no matrix capability; it is infrastructure for
v020-05 · v020-07 · v020-08 · v020-09, each «proven on a fixture».

**Freight:** none — the entry carries no annotation (`route.py pins`: line 36 holds no block).

## What it builds
- **Fixture pages beside the stand, stated once.** A fixture is a minimal page (a Dioxus component or an HTML string)
  booted with the stand's own options (`seven_guis::stand::options(incremental)` — pinned 800×600, bundled font,
  offline) where the four lean tasks lack a case. Today each check states its own: a fixture held in a driver session
  is booted by a file-private helper (`stand_act_obstructed::hold`, `stand_act_scroll::hold_fixture`,
  `stand_act_transformed::hold`, `stand_session_recording::hold_fixture`, `stand_act_screenshot::typed_shot`'s boot),
  and a password fixture component is written again in seven files (`stand_act_diff`, `stand_act_filled`,
  `stand_act_screenshot`, `stand_diff`, `stand_mcp_masked`, `stand_session_recording`, `stand_snapshot_text`). The
  chunk states the boot of a fixture held in a session once in blitz-tests' shared modules, and the fixtures more than
  one check reads once beside it. [premise-corrected: research.md §Patterns detected — the boot is five file-private
  helpers and 16 session holds of one shape; the seven password fixtures are four pages, two of them shared (labelled
  `Passphrase`: `stand_act_diff`, `stand_diff`, and `stand_snapshot_text`, whose checks read only the `fx-secret` line;
  named `Secret`: `stand_mcp_masked`, `stand_session_recording`), `stand_act_filled`'s and `stand_act_screenshot`'s one
  reader each; a third shared page is the simple covered page of `stand_act_spans` and `stand_mcp_tools`; a page read by
  one check, or whose ids differ (the two `restless_fixture`s), stays in its file]
- **One held network stand-in.** A `NetProvider` that keeps every request with its handler until the check answers it
  (its url read, the request answered with bytes, the held count read), stated once in blitz-tests' shared modules,
  in process. It replaces escher's file-private copies — `stand_settle::ManualNetProvider` and
  `font_face_sniffed_source::HeldNetProvider` — with no change to what either check asserts.
  Upstream's own `render_blocking_stylesheet::ManualNetProvider` (4322e86f0, upstream #695, in `upstream/main`) and
  `link_rel_attribute::RecordingNetProvider` (e255bf7df, upstream #846) stay as upstream wrote them — intent
  §Principles «Upstream is merged at each epoch boundary while our changes stay mostly additive», and
  2026-10-16-file-splits' precedent «upstream body untouched» (verified: both upstream-authored, research.md §Sweeps).
  [premise-corrected: research.md §Mechanism equalities — `stand_boot`'s escher-authored `RecordingNetProvider` reads
  only the list of urls asked, which the held stand-in records the same; it joins, three escher copies to one]
- **Run through agent-run.** The checks that read the shared fixtures run under `scripts/agent-run.sh run`, and the
  agent-run contract's «NOT YET MEASURED — the test-data bootstrap mechanism» (test-plan §3;
  `.claude/rules/verification-harness.md`) gets a measured statement. Verified (research.md §Files inspected): today's
  selections are `stand · all · <file>`, `run stand` takes every `stand_*.rs`, fixture-held ones included, and
  `font_face_sniffed_source` runs under `run all` or by name; `boot` compiles every target (`--no-run`). Whether a
  fixture selection is added or the existing selections with the shared module are the mechanism stays a P4 fork.

## Boundaries
- In process only: the held network stand-in opens no listener, port, socket or file, reads no environment variable
  and fetches nothing — no crossing, nothing for arch §Occupied Resources. Verified (research.md §Patterns detected):
  each copy is a `Mutex<Vec<…>>` filled by `fetch`, with no env, file, socket or print site.
- No engine, driver, harness or app change: test code and, where the agent-run fork lands there, `scripts/agent-run.sh`
  with its contract tests (`.github/scripts/test_agent_run.py`). No new workspace crate; the shared modules stay no
  test target (`tests/all.rs` untouched, `BlitzTestsTargetsTest` pins it).
- Behaviour unchanged: every check moved asserts what it asserted, in both layout modes; the blitz-tests target count
  and test count hold (no test lost — test-plan §9's tallies).
- No new user-content log field, no id, name or typed text in an assertion message (security-plan §Input Validation's
  rule on test names and messages).

## Surfaces and contracts touched
- `tests/blitz-tests/tests/common/mod.rs` · `tests/blitz-tests/tests/session_common/mod.rs` or a third shared module
  beside them (a P4 fork: research.md §Open questions; a shared module names no other, research.md §Conventions) and
  the checks that today state their own copy (research.md §Files to modify, 19 files).
- test-plan §1 (blitz-tests' shared modules), §3 (agent-run · the bootstrap line), §7 (Test Data & Fixtures), §8 (the
  Network stand-in rows); `.claude/rules/testing.md` §Framework/§Fixtures and `verification-harness.md` follow by the
  wrap's cascade.

## CI read at the take-up (Setup 5a)
- `5174f92a7335` (the last wrap's commit, 2026-10-19-test-splits; the only sha from that flip through HEAD) — CI
  CI#39331047864: verdict not yet available at P1 (in progress, 17 checks; the oldest running check at 64 s).
  Re-read at P5: green, 17/17 checks, wall 1087 s — nothing to fold.
