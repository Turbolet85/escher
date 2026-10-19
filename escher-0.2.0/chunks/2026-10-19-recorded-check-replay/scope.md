# Scope — 2026-10-19-recorded-check-replay · Recorded-check replay

**Working entry (verbatim):** Recorded-check replay — recorded checks run as ordinary tests under agent-run, CI and
stand-repeat, steps re-validated; red on behaviour change, green otherwise (v020-04)

**Epoch:** Epoch 3 — Determinism and traces (its last entry) · **Version:** escher-0.2.0 · **Previous entry:**
2026-10-19-session-recording (the recording half of v020-04, complete)

## What it builds (stated by the entry)
- Recorded checks run as ordinary tests: under `scripts/agent-run.sh`, in CI, and in the `stand-repeat` leg.
- Each recorded step is re-validated when it is replayed: it goes through `Session::run`, which validates first.
- A replay is red when the app's behaviour changes and green otherwise.
- The matrix capability is v020-04 («Trace to regression check»). Its acceptance: a stand session driven through the
  command line and one driven through MCP are each recorded and replayed as ordinary tests in CI. Each replay passes
  on the unchanged app and fails when the app's behaviour is changed under it, with no hand edit of the recording.

## Freight folded
- CARRY (verbatim, its one citation re-pointed to the other chunk's file at P5): F3 widens to B here, on the founder's
  word (the founder, 2026-10-19, by question dialog in the overseer session, relayed by the operator —
  `escher-0.2.0/chunks/2026-10-19-session-recording/inputs/I4-relay-1.md.txt`): each answer kept whole as the
  driver wrote it, a shot's pixels excepted, as an `answer` key after `digest` — the kept-check form leaves room for it
  and `Check::from_bytes` refuses it until then; a boundary widening (an answer's ids, names, values, snapshot text and
  diff at rest) for this entry's own wrap card
  - The ruling's record: `escher-0.2.0/chunks/2026-10-19-session-recording/inputs/I4-relay-1.md.txt` (a file of this
    repository, cited by path): «Session recording F1 A and F2 B are ratified as his, and F3 widens to B in
    «Recorded-check replay», not in this chunk.» F3 B's option text is in that chunk's founder-forks record
    (`…/inputs/I2-founder-forks.md.txt`, §F3 B).
  - The obligation: every kept step gains an `answer` key after `digest`, holding the answer as the driver wrote it.
    `Check::from_bytes` takes that key, where today it refuses it. (verified at P1:
    `packages/escher-driver/src/recording.rs:392-395`, the closed key list; its test at `:819` refuses a record that
    carries `"answer"`)
  - The operator's take-up directive restates it (inputs#I1): «each kept answer whole, masked as on every carrier,
    except a shot's pixels».
  - «As the driver wrote it» is the line `Outcome::to_json` or `Refusal::to_json` writes, the line the digest
    already hashes (`recording.rs:41-47`). A `snapshot`'s answer is that JSON line, `{"text":…}`, not the bare
    screen text the MCP result carries. (verified at P3: `json.rs:267-273`, `:312`)
  - «Masked as on every carrier» needs no new masking: a password's or a file input's value already reads
    `MASKED_VALUE` in every answer, and a masked `type` already keeps `MASKED_STAND_IN`. (verified at P3: the
    answer is the `to_json` line of the outcome `Session::run` returned, `execute.rs:144-149`, whose readings come
    from the masked snapshot; a refusal's line holds only fixed text, `json.rs:312-324`)
  - «A shot's pixels excepted»: a `screenshot` step keeps its answer without `png`, so its acceptance, width and
    height. This is F3 B's option text («a `screenshot` step keeps whether it was accepted and its dimensions,
    never its PNG»), which also says «pixel comparison stays out of this version». (verified at P3: the line exists
    as `shot_without_image`, `json.rs:256-258`) Whether the digest still hashes the whole line, PNG included, is
    P4's.
  - It is a boundary widening (ids, names, values, snapshot text and diffs at rest), already ratified by the founder.
    It goes on this chunk's wrap card as the ratified widening, citing the ruling.

## Boundaries
- What makes a replay an «ordinary test»: a test target that replays kept checks on freshly booted sessions. A
  target named `stand_*` is picked up by `agent-run.sh run stand` (`scripts/agent-run.sh:181-185`) and by the
  `stand-repeat` leg's glob (`.github/scripts/ci-leg.sh:50-52`), and runs in the CI `test` leg on three OSes.
  (verified at P3) Whether the checks it replays are committed to the repository is founder fork F1 (P4).
- «No hand edit of the recording»: under a committed check, the check is what the binary wrote, one through the
  command line (`escher-session start … --record`) and one through the `mcp` role (`mcp --record`). A seven_guis
  process check records the same scripts again and reads the committed bytes. (verified at P3: the two roles already
  keep byte-identical checks of one script, `examples/seven_guis/tests/recording.rs:143-144`; the freshness check is
  the plan's)
- [premise-corrected: `TaskShell` is private (`examples/seven_guis/src/app.rs:163`), so no test can mount a changed
  task in the stand's chrome today; a changed stand task would move the a11y and layouts records] «Fails when the
  app's behaviour is changed under it»: the witness replays a kept check against a test-only variant of a stand task
  mounted in the stand's own chrome. It needs `TaskShell` reachable from a test (a visibility change, no markup), or
  the shell's markup replicated in the test (technical fork T2). The replay reads red at the step that exercises the
  change and names that step and the answer keys that differ. Today's in-process round trip
  (`tests/blitz-tests/tests/stand_session_recording.rs`) replays on the unchanged app only.
- Where the committed checks live and what they hold at rest in the repository is the founder's question. A file
  in git is not `0600` in a `0700` directory, and it is pushed to the GitHub remote. Today the security plan keeps
  every kept check owner-only. The playbook's «Boundary widening» pattern names it (a kept check's content on a
  medium never ratified). It goes to the founder-forks file as F1 (inputs#I1). (verified at P3: the security, obs
  and arch extracts and histories all leave it open)
- v020-04 is claimable by this chunk: of the markerless entries, only this one names it
  (`escher-0.2.0/working-route.md:29`; it is otherwise named only at `:27`, the frozen recording entry). (verified at
  P1: a grep of the working route)
- Not in scope: pixel comparison of shots; the cold-agent pipe's own check (`scripts/cold-agent.sh`); a change to the
  eleven verbs' behaviour; the engine.

## Surfaces and contracts it touches
- `packages/escher-driver/src/recording.rs`: the record writer (`record_of`), the reader (`step_of`,
  `Check::from_bytes`) and `Step`'s accessors.
- A replay runner: today's helper is `replay` in `tests/blitz-tests/tests/session_common/mod.rs:242-251`, read by
  `stand_session_recording` alone. Whether replay becomes a public escher-driver API is technical fork T1 (P4).
  (verified at P3: the code graph's callers)
- `tests/blitz-tests/tests/` (a new replay target) and its target pin `.github/scripts/test_blitz_tests_targets.py`;
  `examples/seven_guis/tests/recording.rs` (the binary's recording checks, CLI and MCP).
- `scripts/agent-run.sh`, `.github/scripts/ci-leg.sh` (`stand-repeat`), `.github/workflows/ci.yml`.
- Masters at the wrap: `.andromeda/security-plan.md` (§Surfaces, the kept check's content; §Data Protection),
  `.andromeda/architecture.md` §Standard Contracts (Driver session), `.andromeda/test-plan.md` §3 and its
  session-lifecycle key file, `.andromeda/obs-plan.md` (the never-logged rule).
- Platform-gated code: `recording.rs`'s `imp` is `cfg(unix)` / `cfg(not(unix))`. A push that touches it runs
  `cargo check --tests --target x86_64-pc-windows-gnu` first (inputs#I1).

## Inputs and verdicts
- The operator's directive for this take-up (inputs#I1):
  - the F3 → B widening, ratified;
  - every fork that is the founder's goes into ONE founder-forks file in the run dir, with its options whole, its
    dependences and a marked recommendation, and the least-widening option named; the turn then ends;
  - technical forks stay in the dialog;
  - the pre-CI commit subject is the guide form exactly;
  - before a push that touches platform-gated code, `cargo check --tests --target x86_64-pc-windows-gnu`.
- Founder fork F1, answered by the founder himself (inputs#I2): **A**, «Коммитить две проверки (Рекомендую)». Two
  checks of scripted stand sessions are committed to the repository as the binary wrote them, one recorded through
  the command line and one through the `mcp` role. Ratified, not PROVISIONAL.
- Technical forks, the operator's answers in the P4 dialog: T1, the replay runner is a public escher-driver API
  (`Check::replay`); T2, `TaskShell` is made `pub` so the red witness mounts a variant task in the stand's chrome.
- CI since the last wrap's flip: `6fd1d3daf6a1`: verdict not yet available (in progress · checks 17/17 ·
  CI#39212950293 in_progress; the oldest running check is Test (windows), at 460 s). Not folded and not read as
  green. Re-read at P5: green · checks 17/17 · wall 982 s · CI#39212950293 completed/success.
