# Codebase Research — 2026-10-20-load-wait

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full). It has 1 Session Addition (the
  `timeout N` exit-124 trap), which does not apply: no entry here is a boot smoke under `timeout`. The plan's legs are
  `agent-run.sh run stand` and `ci-leg.sh stand-repeat` · `fast` · `doc` · `coverage`.
- **Platform issues consulted:** none. No runner-only bullet, and no CI-reading entry outside the operator leg (Setup
  5a's one non-green row is `in progress`, not red).
- **External inputs:** none. Every fact this chunk turns on lives in this repository.

## Files inspected
- `packages/blitz-test-harness/src/settle.rs` (1-200):
  - `Harness::settle` (:149-186) and `LoadCounter` / `CountedHandler` (:61-128).
  - After a quiet pass, a request in flight (`in_flight > 0`) or a pending critical resource returns
    `NotSettled { busy: Loads }` at once (:163-170).
  - `Loads` is also the class read at the 64-pass bound when the last busy pass only finished a load (:176-181).
  - `CountedHandler::bytes` calls the inner handler first. Its `Drop` then decrements `in_flight` and increments
    `finished` (:113-127). So by the time `in_flight` reads 0, the document's message is already queued.
- `packages/escher-driver/src/execute.rs` (100-135, 440-490):
  - `settled_step` (:453-470) is the one place an acting verb settles: `step` → `harness.settle()` → snapshot → diff.
  - `acted` (:472-485) and the two inline arms at :199 and :214 (`advance`, `scroll`) go through it.
  - The doc line «Settling moves no time and waits on no load» is at :126.
  - `passes` is recorded only on `Ok` (:463).
- `packages/escher-driver/src/session.rs` (1-60, 150-205):
  - `Session::act` (:196-204) is the in-process step: step, then one `settle`, mapped onto `SessionError::NotSettled`.
  - Its doc says «Settling moves no time».
  - `TimeStep` and `with_time` (:166-172) are the app-time mover `advance` drives. The wait must not call it.
- `packages/escher-driver/src/schema.rs` (100-240):
  - `SETTLED` (:117-122, help «whether the app went quiet after the step») and `BUSY` (:124-130).
  - Each acting verb's help, and `BUSY_CLASSES` (:34).
  - The double-click help states its number in text («within 500 ms», :192-194), pinned by `schema_tests.rs:87-88`.
    That is the precedent for a number stated in help.
- `packages/escher-driver/src/lib.rs` (140-158): the re-exports. `host::IDLE_EXPIRY` is a `pub const Duration`
  re-exported beside `serve` (:147), the precedent for a stated time bound as a public constant.
- `packages/escher-driver/src/host.rs` (1-60, 140-215):
  - The host's 2 s `IO_BOUND` is set on the stream for reading the request line and writing the reply (:179-181,
    :182, :211).
  - The call itself runs between them (`session.run(&call)`, :198), so neither 2 s bound spans a call's run.
  - The host already reads `Instant::now()` and sleeps `POLL` 25 ms for its idle expiry (:144-162). It is today's one
    clock reader in the crate's library code outside the client.
- `packages/escher-driver/src/wire.rs` (:15-18): `IO_BOUND` 2 s, `ANSWER_BOUND` 30 s.
- `packages/escher-driver/src/client.rs` (:139-146, :271-305): the client reads a call's answer under `ANSWER_BOUND`.
  The check `an_answer_read_past_its_bound_is_a_timeout` pins it at 30 s.
- `examples/seven_guis/tests/common/mod.rs` (:200, :272-302): `REPLY_BOUND` 30 s on every MCP reply the `Speaker` reads.
- `scripts/cold-agent.sh` (:62, :208-230): `REPLY_BOUND = 30` per reply on the judge's replay children.
- `tests/blitz-tests/tests/fixtures/mod.rs` (1-107):
  - `boot` / `hold` use `stand::options(incremental)` alone, which is offline (`net_provider: None`).
  - `HeldNet` (:64-107) holds each request until the check calls `answer`, from the check's own thread, and never
    overrides `is_noop`.
- `examples/seven_guis/src/stand.rs` (:57-65): `options(incremental)` holds `net_provider: None`. Replace fields with
  `..` to inject one.
- `tests/blitz-tests/tests/stand_settle.rs` (1-30, 180-240):
  - The load arm drives `Harness::settle` on an HTML page with `HeldNet`, never a `Session`.
  - Its session arms (`act`) meet no counted load (`restless_fixture` reads `Render`).
- `packages/blitz-dom/src/mutator.rs` (395-420, 1036-1075, 1189-1300):
  - A `link` added to the document queues `LoadStylesheet`, and an `img` queues `LoadImage` (:1063-1064).
  - An `href`/`rel` write on a `link` reloads it (:411-414).
  - `load_linked_stylesheet` marks the request critical only inside `head` and with a non-noop provider (:1225-1229).
  - It then fetches through `net_provider` (:1232).
- `packages/blitz-dom/src/net.rs` (:99-124, :143-144): `ResourceHandler` parses the body and posts
  `DocumentEvent::ResourceLoad` on the document's channel (`tx.send`, :123) from whatever thread calls `bytes`.
- `packages/blitz-dom/src/resolve.rs` (:50): `resolve` handles the queued messages. Settle's pass (poll, then
  resolve) applies a load answered between passes.
- `packages/blitz-dom/src/document.rs` (1342-1440, 1481-1511, 1054-1066):
  - `load_resource` → `add_stylesheet_for_node` for CSS, `apply_loaded_image` for an image.
  - Neither inserts into `changed_nodes`. The set is marked at :1661 and by the mutator alone.
- `packages/blitz-traits/src/net.rs` (:19-36): `NetProvider` and `NetHandler` are both `Send + Sync + 'static`, so a
  handler can be answered from a thread the stand-in spawns.
- `packages/escher-driver/src/describe.rs` (via the tests extract): `describe()` writes every text from the schema's
  tables, and its count row is `[11, 13, 12, 3, 8, 4, 15]`. A bound stated inside an existing help text moves no count.
- `.github/scripts/ci-leg.sh` (:48-59): `stand-repeat` globs `tests/blitz-tests/tests/stand_*.rs`, so a new
  `stand_*` file joins it with no script change.
- `tests/blitz-tests/Cargo.toml` (:48-57): every file under `tests/` is its own target. `all` is declared and never
  built, so a new file needs no manifest line.
- `.github/scripts/test_ci_workflows.py` (:358-359): the leg's word pin names `stand_settle` and others, not a file
  count.

## Graph impact
- **`settle`** (rust plane): 21 call rows, of which 2 are production (re-derived: `calls WHERE callee_name IN
  ('settle','settled_step','act','acted') AND callee_kind = 'fn'`, 147 rows in
  `.andromeda/runs/2026-10-20T01-41-31-phase/tree-query-2026-10-20-load-wait.json`).
  - `escher_driver::execute::settled_step` @ `packages/escher-driver/src/execute.rs:461`.
  - `escher_driver::Session::act` @ `packages/escher-driver/src/session.rs:202`.
  - The rest are blitz-test-harness's own unit tests (7), `stand_settle` (7), `stand_act_ids` (3) and
    `font_face_sniffed_source` (2). None of these drive a `Session` with a counted load.
  - So the two driver sites are the whole production blast radius of a wait over settle.
- **`settled_step`**: 3 callers, all in `execute.rs` (`acted`, and the `advance` and `scroll` arms). One change there
  reaches all five acting verbs.
- **`act`**: 94 rows, nearly all session_common's own `act(session, call)` builder (a test function of the same name,
  `tests/blitz-tests/tests/session_common/mod.rs:183`) and stand checks. `Session::act` is the in-process API those
  checks and `stand_settle` call. A wait added to it is inherited by every caller, and none of them holds a counted
  load (sweep below).
- **`acted`**: 22 rows. The rows in `json.rs` and `recording_tests.rs` are the test-side `Outcome::Acted` builders of
  the same name, not callers of `execute::acted`.

## Patterns detected
- **One settle seam per surface** (`execute.rs:453-470`; `session.rs:196-204`): every acting verb settles in
  `settled_step`, and the in-process step settles in `Session::act`. A wait written once as a private escher-driver
  function and called from both covers every carrier (socket, command line, MCP, in process).
- **A stated time bound is a `pub const Duration` re-exported at the crate root** (`host.rs:11`, `lib.rs:147`). Its
  number is also said in help text, as the double click's 500 ms is (`schema.rs:192-194`), and a unit test pins text
  against value (`schema_tests.rs:87-88`).
- **A load is counted only through a non-noop `HarnessOptions::net_provider`** (`settle.rs:61-128`;
  `stand.rs:57-65`). The stand, every `fixtures::boot`/`hold` and so every existing session check is offline, so a
  wait keyed on `Busy::Loads` from a counted request costs them nothing.
- **A load answered off-thread is applied by the next settle pass** (`net.rs:123`; `resolve.rs:50`;
  `settle.rs:155-161`). `CountedHandler`'s `Drop` follows the inner `bytes`, so `in_flight` reads 0 only after the
  message is queued, and the next pass applies it (`loaded` true → one more pass).

## Mechanism equalities (verified at HEAD)
- **E1: a counted load answered from another thread inside the bound ends the wait settled, with the loaded content
  on screen.**
  - The chain: a click adds a `<link rel=stylesheet href=…>` in the body. `process_added_subtree` queues
    `LoadStylesheet` (`mutator.rs:1063`), and `load_linked_stylesheet` fetches through the harness's
    `LoadCounter` (`mutator.rs:1232` → `settle.rs:94-103`).
  - Settle reads `Loads` (`settle.rs:163-170`). The stand-in's thread then calls `bytes`:
    `ResourceHandler<StylesheetHandler>` parses and `tx.send`s (`net.rs:143-144`, `:123`), and the handler's drop
    makes `finished` +1.
  - The next settle's resolve applies the sheet (`resolve.rs:50` → `document.rs:1395-1398`), and layout moves the
    styled element's box. A wait that re-settles until it reads anything but `Loads` therefore returns `Settled`,
    with the box change in the snapshot after.
  - Proven only by the plan's check: no test today answers a load from another thread.
- **E2: a counted load never answered keeps every settle at `Loads`.** `HeldNet` keeps the handler, so
  `in_flight` stays 1 and every settle reads `NotSettled { busy: Loads }` (`settle.rs:163-170`;
  `stand_settle.rs:187-209` reads the same on a harness). The wait therefore ends only at its bound, and the outcome
  is today's `settled: false, busy: "loads"` (`execute.rs:461-466`, `BUSY_CLASSES[2]`).
- **E3: the wait moves no virtual time.**
  - `settle` reads `self.time()` and never ticks (`settle.rs:149-152`).
  - A loop of settles separated by a wall-clock sleep leaves the harness's `VirtualClock` where it was.
  - The time step (`session.rs:24-26`, `with_time`) is run only by `advance`'s own step, never by settle.

## Conventions to follow
- **Failure messages name a mode, a task and an index, never an id, a screen reading or a URL** (session checks; per
  the tests history, 2026-10-07-act-by-id).
- **Both layout modes** in every held-session check (`for incremental in [false, true]`, `stand_settle.rs:188`).
- **A page or stand-in with one reader stays in its check's file** (`tests/blitz-tests/tests/fixtures/mod.rs`, module
  doc and test-plan §3).
- **`.test` URLs, offline, in-process stand-ins only** (`stand_settle.rs` `http://settle.test/`).

## New files to create
- `tests/blitz-tests/tests/stand_load_wait.rs` — the held-session check, both layout modes, both arms:
  - a load answered inside the bound by a stand-in that answers each request on its own after a short delay
    (in-file: one reader);
  - a load never answered (`fixtures::HeldNet`).

  Each arm runs through `Session::run`, an acting `click` on a fixture button that adds a body stylesheet link.

## Files to modify
- `packages/escher-driver/src/execute.rs` — `settled_step` settles through the waiting settle. Its doc moves from
  «waits on no load».
- `packages/escher-driver/src/session.rs` — `Session::act` settles through the same waiting settle, its doc restated.
  The private wait function lives here, or in `execute.rs`.
- `packages/escher-driver/src/schema.rs` — the stated bound: a `pub const` beside the tables, and the `settled`
  field's help stating it.
- `packages/escher-driver/src/schema_tests.rs` — pins the help text against the constant, as the double click's
  text is pinned.
- `packages/escher-driver/src/lib.rs` — re-exports the bound constant (55 → 56 re-exports).

## Sweeps
- `net_provider` over `tests/blitz-tests/tests` and `examples/seven_guis`: 6 hits. 1 is no-change (`stand.rs`, the
  offline stand options). 5 are no-change because they are harness-only checks: `font_face_sniffed_source`,
  `link_rel_attribute`, `render_blocking_stylesheet`, `stand_boot`, `stand_settle`. None holds a `Session`, so no
  existing driver path meets a counted load (re-derived: `grep -rln net_provider`, then
  `grep -n 'Session::start|fixtures::hold'` on the two escher-authored ones, 0 hits).
- `waits on no load|never waited|wait on a load|waits on a load|waited on|no load` over `packages/escher-driver/src`,
  `packages/blitz-test-harness/src` and `examples/seven_guis/src`: 2 hits.
  - `execute.rs:126` changes (its doc moves).
  - `settle.rs:141` is no-change: settle itself still never waits, so its doc stays true.
- `changed_nodes.insert` over `packages/blitz-dom/src`: 6 sites (`document.rs:1661`, `mutator.rs:213, 248, 325, 462,
  538`). None is in `load_resource`. A CSS or image load marks nothing in the changed set, so an «`has_changes()`
  reads true after the load» criterion would read only the click's own mutation and cannot witness the load.

## Scope premise closure
All [inferred] bullets in `scope.md` were re-read against the above:
- The bound is a constant: VERIFIED. The validated surface is closed (`schema.rs` verb table; arch and security
  extracts), and the `IDLE_EXPIRY` precedent stands.
- The existing result already says it: VERIFIED (`execute.rs:461-466`, `BUSY_CLASSES`).
- A stand-in that answers on its own: VERIFIED (`fixtures/mod.rs:88-97`; the check's thread is inside
  `Session::run`). Corrected: its placement is the check's own file, by the one-reader rule.
- Wall time, no virtual time: VERIFIED (E3). The wall read lives in escher-driver: the host already reads
  `Instant` there, and blitz-dom gains none.
- `snapshot` / `screenshot` do not wait: VERIFIED (`execute.rs:108-116`: no step, no settle).
- The recorded decisions moved: VERIFIED. They also include the settle-detection standing rule «a driver command
  that would wait on a load with a clock is a new decision for this plan» (the security history).
- The guarantee is kept by the bound: VERIFIED (E2 ends at the bound).
- Harness settle stays: VERIFIED. Its production callers are exactly the two driver sites (graph).
- The span is unchanged: a P4 lean, not a code fact. `passes` semantics is an open question.
- Bounds above the stated bound: CORRECTED. The host's 2 s `IO_BOUND` does not span a call's run (`host.rs:179-211`).
  The binding bounds are the client's 30 s `ANSWER_BOUND`, the `Speaker`'s 30 s and the cold-agent `REPLY_BOUND`
  30 s.
- No real network: VERIFIED (`.test` URLs, in-process stand-ins).

## Open questions
- What is the stated bound's value? It has to stay strictly below 30 s, and the never-answered arm pays it per mode
  per stand-repeat round → blocks: plan-decision.
- What does `passes` count on a waited step: the last settle's passes, or the sum across the wait? → blocks:
  plan-decision.
- How does the wait space its re-settles while a load is in flight (a short wall sleep between settles; its length)?
  → blocks: implementation-scope.
