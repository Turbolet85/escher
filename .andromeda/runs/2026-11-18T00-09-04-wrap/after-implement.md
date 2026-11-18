## After implement
- **Report:** 2026-11-17T22-32-51-implement — the newest implement run of 2026-11-17-stand-requirement-sweep (`rec.py
  runs … --file '*-2026-11-17-stand-requirement-sweep.json'`, its last run line), its outcome `green`.
- **Commits:** since last_wrap (2026-11-17T19:37:35Z, the `state` view): `feaa41386 chore(2026-11-17-stand-requirement-sweep):
  operator pre-CI commit, for the run this chunk's verdict reads` (parent `66fbe33be`), the pass's only commit; no fix
  commit followed it.
- **Operator pass:** `{work_dir}/evidence/operator-pass.md`.
  - Entry 12 (`gate.py hygiene`): `hygiene: clean — read 200 … 0 host paths kept`; `rec.py land` → `unchanged · 7 file(s)`.
  - The fast leg on the committed tree: exit 0.
  - Entry 13 (the push): `66fbe33be..feaa41386 build/escher-0.3.0 -> build/escher-0.3.0`, exit 0.
  - Entry 14, attempt 1: CI#44197885534 on `feaa41386` — `verdict: red · checks 17/17 · first-fail +1406 s Coverage
    report`; 16 of 17 jobs `success`, `Coverage report` (job 134449443048) `failure`: every test read `ok`, then
    `llvm-profdata merge` refused a corrupt profile (`escher-18135-…profraw: invalid instrumentation profile data (file
    header is corrupt)`, `error: no profile can be merged`). No job cancelled.
  - Entry 14, attempt 2 (`gh run rerun 44197885534 --failed`, on the operator's direction): `verdict: green · checks
    17/17 · attempt 2 · wall 2955 s · runs CI#44197885534 completed/success`; the re-run job 134461293346 merged with no
    corrupt-profile line and read escher-driver 5477/5607 = 97.68 % · dioxus-native-dom 4328/4767 = 90.79 % ·
    blitz-test-harness 1068/1116 = 95.70 %, `coverage floor: 3 of 3 crates at or above their floors`.
  - The pass's end: green on `feaa41386` at attempt 2, the attempt-1 red recorded.
- **Changes after implement:** none — no commit after the pre-CI one, no directed edit of the wrap's.
- **Inputs:** `inputs.py verify` (this wrap's run dir):
  - I1 · message: the operator, /andromeda-phase invocation, 2026-11-17 · copy message · n/a — cited (plan, report).
  - I2 · message: the operator, /andromeda-wrap-session invocation, 2026-11-18 · copy message · n/a — snapped `--step
    wrap` by this wrap and cited here (`inputs#I2`): three directives — (1) re-point the three verified ledger refs
    naming renamed tests (v030-01, v030-09, v030-10) or state why not, P7.3's; (2) correct CARRY `cry-b3h6o7do`'s two
    stale test-count leaf lines through the leaves step's comparers, the operator told before any door if the pass
    prints them `not compared`; (3) a `leaves --apply` receipt refusal re-spawns by `--respawn {name} --failed RULED
    --rows {name}={the tool's line}`, a refusal of that stopping the wrap; no hand edit of a leaf.
  - `inputs: 2 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 1
    · unparsed 0` — the uncited one is I2, read before this close cited it. No DRIFT.
- **Scope:** none — `scope: clean — changed 3 · listed 3 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 ·
  widening 0) · absorbed 0 · excluded 199` (base 66fbe33b, the parent of the oldest pre-CI commit feaa4138).
- **Sweep reach:** `cascade.py sweep --pre-pass`, patterns `{run_dir}/report-patterns.toml`, baseline feaa4138; the first
  firing refused five patterns' controls (`session-common`, `agent-run`, `doc-loader`, `silent`, `coverage` — no master
  holds them; record `{run_dir}/cascade-2026-11-17-stand-requirement-sweep-pre-pass.json`), the re-fire ran the other
  five.
  - `stand_requirement_sweep`'s test count 3 → 4, tables 2 → 3 — `sweep-tests`: standing 1 (a11y-plan 1:
    `dec-3qlfzc22`'s guard names the target, a true claim, no count — no change). No test-plan site: the expected
    test-plan §1 count has no master home (0 hits) — no master site.
  - `STAND_REPEAT_NAMED` 10 → 17 — `named-17`: standing 1 (test-plan 1: `dec-aovxp22d` «the named blitz-tests targets»,
    no count — no change).
  - blitz-tests targets +1, `stand_*.rs` 63 → 64 — `stand-files`: standing 2 (test-plan 2: `ctr-exq4f34j` and
    `dec-aovxp22d`, the glob with no count — no change).
  - `mod session_common;` readers 42 → 43 — `session-common`: 0, the control's refusal its basis.
  - agent-run `run stand` 332 · 0 · 10 and `run all` 879 · 0 · 15 — `agent-run`: 0, the control's refusal its basis.
  - `document_loader::tests` 3 → 8 — `doc-loader`: 0, the control's refusal its basis.
  - Spec claims disproved: the three ledger refs — no master site, ledger entries (P7.3, directive I2 (1)); the guard
    entry's form — no master site, a plan entry absorbed by plan rev 6.
  - Expected amendments: test-plan §9 (the arm read) — `arm-read`: standing 1 (test-plan 1: `dec-bk5ehcmd`, the audit
    leg's «leg check», a different fact sharing the token — no change), owner none; test-plan §1 — as `sweep-tests`,
    owner none; obs-plan §3 silent checks — `silent`: 0, the control's refusal its basis, owner none; security-plan
    `decision/mask-holds-on-every-carrier` — `mask-carrier`: standing 1 (security-plan 1: `dec-hhwm3cz7`, its guard
    `stand_act_filled, stand_mcp_masked` lacks `stand_masks_on_new_answers` — stale), owner security-plan; test-plan
    agent-run Proof readings and coverage figures — `agent-run`, `coverage`: 0, the controls' refusals their basis,
    owner none; matrix#v030-04…17 notes — no master site, ledger-note, owner P7.3.
  - The leaves hold the counts the masters do not (leaf 29 rows over 10 files; `tests-summary.md:18`/`:21`/`:30` «ten
    named» · «`STAND_REPEAT_NAMED` (10)», `services/escher-driver.md:61` and `tests-summary.md:21`
    «`stand_requirement_sweep` (3 …»): the cascade's.
  - Sweep reach: 5 patterns · 14 moved facts, 11 with a pattern (the report's own count: 6 counts, 5 expected
    amendments; the 2 disproved claims and the ledger-note entry no master site) · by line 0 lines · 0 offsets.
- **Outcome, final:**
  - Acceptance (ci, fast, guard, CI): supersedes «the CI read is the operator pass's (entry 14), not run here»: met —
    CI#44197885534 on `feaa41386` `verdict: green · checks 17/17 · attempt 2`.
  - Acceptance (ci, stand-repeat and coverage): implement's stands; the CI figures beside its local ones — escher-driver
    97.68 % · dioxus-native-dom 90.79 % · blitz-test-harness 95.70 % (CI job 134461293346), each above its floor.
  - Gates `gate.py hygiene` · the push · `ci.py conclusion`: supersedes «not run — leg operator»: hygiene clean · pushed
    · green at attempt 2 (operator-pass.md).
  - Recorded red, not a gate red: `Coverage report` attempt 1 (job 134449443048) red on a corrupt `.profraw` at the
    profile merge after every test read `ok`; the identical-tree re-run green; no cause measured — routed by P5's
    Recurrence-watch row (the operator's candidate WATCH).
  - Watches: a CI job cancelled with no runner acquired and no step run · 1 green run [CI#44197885534] (no job
    cancelled in either attempt).
  - Outcome basis: Setup 4's commit list (`feaa41386` alone), CI#44197885534 in `{work_dir}/evidence/operator-pass.md`,
    the pass record; implement's report for the rest.
- **Decisions & corrections after implement:**
  - The operator's ruling (2026-11-18, operator-pass.md): the `index out of bounds` panic at
    `tests/blitz-tests/tests/stand_act_error_text.rs:129:17` in the coverage log is the fixture's own deliberate
    out-of-bounds read (`index_past_end()`, `INDEX_LINE`) — no defect; the open question is the corrupt `.profraw` alone.
  - The operator's direction (operator-pass.md): let the run finish, re-run `Coverage report` once; a corrupt-profile
    red green on its re-run is a candidate WATCH for the wrap.
  - The operator's wrap directives (`inputs#I2`): the three listed under **Inputs**.
- **Process hygiene, re-measured:** `gate.py census … --run-dir {run_dir}` → `census: none running`. The two scratch
  clones implement left (`…/scratchpad/guardclone`, `…/scratchpad/guardclone-impl`) are directories outside the tree,
  for the operator to remove.
