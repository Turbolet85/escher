# P5 review — 2026-11-08-handler-coverage «Handler coverage» (card 2, after the review's fold)

The plan is at `.andromeda/cache/docs/2026-11-08-handler-coverage/plan.md`, rendered from the store at rev 4. Open it in
an editor. No plan stands in a chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-08-handler-coverage` shows a
diff of the chunk's records.

## What the review changed (the operator's review on card 1, folded)
- **The byte-equality reason is gone.** `mutation_writer.rs` staying byte-equal to upstream is no longer given as a
  reason anywhere in the plan or research: not in step 2, the Test Commands prose, entry 8's note, the (arch)
  criterion, the rejected approaches, or research's «The P4 fork, decided».
- **The record's reason is now its own merit.** It is a root context beside the `SignalRegistry`, read by the same
  `has_context` lookup at `ScopeId::ROOT` that the bridge already makes. It exists only on a document the coverage
  reader switched on. A `DioxusState` field would stand on every document, coverage asked or not.
- **Entry 8 no longer lists `mutation_writer.rs`.** Re-baselined green on the tree. In the clone, a `Cargo.toml` edit
  reads red and a `mutation_writer.rs` edit reads green, now outside the guard
  (`.andromeda/runs/2026-11-08T20-10-44-phase/controls.md`).
- **Checks re-run.** The whole mechanical set ran again after the edit: the fence parses, planlint reports 0 hits, the
  lists parse, the baseline listing is unchanged, and the citations verify.

## Goal
- **The command.** One command lists the handlers no recorded check exercised:
  `escher-session coverage <app> <check>…`, with `escher_driver::handler_coverage` under it.
  - It replays each named kept check on a fresh session of the app.
  - It reads the UI graph at boot and after every step.
  - It answers one JSON line of (id, owner, event) handlers that no replayed dispatch reached, plus two counts
    (`checks`, `held`).
  - No verb is added; `VERBS` stays at fourteen.
- **The reached record.** A new record of reached handlers is a root context in dioxus-native-dom beside the
  `SignalRegistry`, written on `handle_event`'s path. It holds on every app, registry or not, resolves the stable id
  at dispatch, and is off unless coverage switches it on.
- **The owed replay test.** The chunk writes the replay test v030-06 owes (CARRY `cry-sxrivdk3`): a recorded `actions`
  step, kept, replayed, and a tampered one caught.

## Top acceptance criteria
1. **matrix#v030-07:** on the six served tasks in both layout modes:
   - one `snapshot` check lists exactly `HANDLERS_AT_BOOT`, equal to the graph's handlers on a fresh boot;
   - each task's `GRAPH_DRIVES` check lists exactly `UNEXERCISED_AFTER_DRIVES`;
   - the record holds on the load page (no registered state);
   - a refused step exercises nothing, and a keyboard activation exercises its click;
   - a handler its own step removes reads exercised;
   - canvas handlers are not counted, and 0.4.0 extends this.
2. **matrix#v030-06:** a recorded Counter session holding two `actions` steps replays holding (4 steps run), and a
   tampered kept `actions` answer stops it at step 3 naming `nodes` (`stand_replay`).
3. **(security)** The answer holds ids, owner ids, event names and two counts only. A bad, oversized or other-app
   check file is refused `malformed`, status 1, holding no byte of its path or contents.
4. **(arch)** No verb is added. `graph.rs`, `signal_registry.rs`, the verb table, every manifest and the committed
   checks are byte-identical to the base (entry 8). The record takes no document borrow across `render_immediate`.
5. **(obs)** There is no new log site, span or print, `info_span!` stays at 2, and `host_log` reads 0 coverage needles
   at `trace`.

## Touchpoints
- **New:** `reached.rs` (dioxus-native-dom), `coverage.rs` (escher-driver), `stand_handler_coverage.rs`.
- **Modified:**
  - `dioxus_document.rs` and its tests, both `lib.rs`;
  - `replay.rs` (one path with a per-step hook), `json.rs`, `cli.rs` and `cli_tests.rs` (the `coverage` role, the
    usage line);
  - `session_host.rs` (doc), seven_guis `cli_commands`, `host_log` and `common`;
  - blitz-tests `stand_replay` and `common`;
  - `docs/capabilities.md`.

## Test commands
19 entries. Eight are new, each baseline-run on the untouched tree:
- 2 and 4 are red only because the subject is absent;
- 5, 6, 8, 9, 11 and 16 are green;
- 8 and 9 carry clone controls.

The rest:
- 1, 3, 7, 10, 12-15 were seen green at the last wrap;
- 17-19 are operator entries (hygiene, push, CI read).

## Concretization previews (never weaker)
- **v030-07** — current: «After a set of recorded checks has run on the stand, one command lists exactly the handlers
  none of them exercised.» → proposed: the role `escher-session coverage <app> <check>…` over
  `escher_driver::handler_coverage`, with these assertions on the six served tasks in both layout modes:
  - one `snapshot` check lists exactly `HANDLERS_AT_BOOT`, equal to the graph's handlers on a fresh boot;
  - each task's `GRAPH_DRIVES` check lists exactly `UNEXERCISED_AFTER_DRIVES`;
  - two checks list only what neither exercised;
  - the record holds on the registry-less load page;
  - a refused step exercises nothing, and a keyboard activation exercises its click;
  - a handler its own step removes reads exercised;
  - handlers inside a canvas are not counted, and 0.4.0 extends it (`inputs#I2`, `inputs#I3`).

  Witnesses: `cargo test -p blitz-tests --locked --test stand_handler_coverage` and `cargo test -p seven_guis --locked
  --test cli_commands`. Method `integration`. The outcome is unchanged; it is made concrete.
- **v030-06** — current: its `stand_actions` acceptance (unchanged, kept whole) → proposed: that acceptance plus one
  clause. A Counter session recorded through `Session::recording_into` with two `actions` steps, read by
  `Check::from_bytes`, holds when replayed through `Check::replay` on a fresh session, all 4 steps run. A copy whose
  last kept `actions` answer is changed stops at step 3 naming `nodes`, in both layout modes (`cargo test -p
  blitz-tests --locked --test stand_replay`). It is claimed by this chunk, and reads verified only on that test's pass
  (`inputs#I1`). Strictly stronger.

## Decisions taken
- **Q1, the carrier:** a command-line role. This is the founder's own word, relayed by the overseer: «A. Роль
  командной строки (Рекомендую)» (`inputs#I4`). Pattern `rul-izni57qv`; the wrap halts on it once as the answer card.
- **Q2, the screens:** every screen the replays read. This is the overseer's technical call as delegate (`inputs#I4`);
  its line is PROVISIONAL, discharged at the epoch boundary.
- **Leaned per artifact:** `mounted` is not counted; the id is resolved at dispatch; a refused step reaches nothing;
  the record is witnessed on a registry-less app.

## Notes
- **sampler: off.**
- Aggregate B: 0 `@none`.
- One strong reader was re-spawned (`a11y-cascade-g7`, `CLEAN_EXTRACT`) and passed.
- `inputs#I4`'s recorded origin reads 20:58Z; the dialog was answered about 21:38Z.
- The evolve diagnosis of 0.3.0's finished epochs waits for Epoch 3's end, which is this chunk (`inputs#I1`).

## Beyond the plan
1. **Deferral:** counting the handlers a canvas dispatches inside one listener goes to 0.4.0 (the incubator's canvas
   plans, `inputs#I2`, `inputs#I3`). This chunk states the limit only.
2. **Owner for a part left out:** the limit «a bubbling dispatch counts every ancestor listener it passed, a
   `stopPropagation` unread» is proposed as a new residual row the wrap opens (target: next).
3. **Stated limit with no owner:** «a handler on no screen a replay reached is not listed» holds by construction. No
   reading can see such a handler, so no owner is proposed.

Apply? (yes / review / cancel)
