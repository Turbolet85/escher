# The founder's forks — 2026-11-18-cold-agent-bar (escher 0.3.0, «Cold-agent bar», v030-15)

The founder is away. The operator directed (inputs#I1) that every fork of this entry that is the founder's is written
here, each with its options, its measured basis and what depends on it, and that none is chosen. The rest of the chunk
proceeds. No option below is marked as recommended. This chunk plans everything that does not hang on these forks.
Its capability v030-15 is claimed **gated** on them: the live run, the bar's number and the docs' claims wait for his
word.

Already decided by the operator (technical, inputs#I2), and the setting for these forks:
- the verdict counts and requires `graph`, `actions` and `layout`, each used at least once;
- `caused` is named out of the run's reach (F5 below asks whether it should not be);
- the prompt asks for the flight type to be chosen by opening its list, and the verdict counts and requires one list
  opening;
- 0.3.0's asks widen the one `stand` prompt (no second task).

## The measured basis, shared by every fork

- **0.1.0's run** (2026-10-15): four lean tasks, eleven tools, 43 calls, 0 wrong (README.md:73).
- **0.2.0's run** (2026-10-25-cold-agent-bar): six apps (counter, flight booker, timer, CRUD, Cells, the load page),
  twelve tools, **66 tool calls, 0 wrong, $0.8244**, `claude` 2.1.308, `claude-opus-5-5`, one run under
  `--max-budget-usd 3`. Judged against the bar the founder set then, «Не больше 5% (Рекомендую)»: `wrong_calls × 100 ≤
  5 × tool_calls` (`scripts/cold-agent.sh:70`, `WRONG_CALL_BAR_PCT = 5`). At 66 calls that bar allows 3 wrong.
- **What a wrong call is today** (`scripts/cold-agent.sh:431`): refused calls + error calls (read from the server's
  `tool_call` span lines) + any tool use outside the escher server + any permission denial.
- **Not the same prompt.** 0.3.0's run widens `STAND_PROMPT` beyond the one 0.2.0's 66 calls were measured on. The
  six apps stay, plus 0.3.0's carried reads (`graph`, `actions`, `layout` on the lean apps) and the flight type
  chosen through its painted list. **0.2.0's baseline numbers were measured on a narrower prompt.** A 0.3.0 run makes
  more calls and meets new ways to be refused. The bar is set knowing that (operator's note, inputs#I2).
- **New ways to be refused**, measured on the built binary (research, `doc:2026-11-18-cold-agent-bar/research`
  §Measured):
  - a click on an option of the closed flight-type select is refused `covered`, an engine limit the founder kept
    (F8/F9, 2026-10-31);
  - a click under an open list is refused `covered`;
  - a `layout` without an id is refused `malformed`.
  The verdict will now count refusals by cause (closed words: `covered`, `disabled`, `off-screen`, `stale`,
  `not-found`, `malformed`, `unknown-verb`, `fonts-unavailable`), so a run's wrong calls can be read by kind.
- **Answer sizes** an agent receives (bytes, at boot): graph 869–5,431 and actions 277–1,572 on the five small apps;
  **Cells graph 293,868 and actions 118,292**. The prompt asks for the new reads on the small apps only.

## F1 — The wrong-call bar (the number)

**The question:** what share of the run's tool calls may be wrong for 0.3.0's run to pass?

**Options:**
- **A — Keep 0.2.0's bar: at most 5 % wrong.** The same rule as 0.2.0. At about 80–100 calls it allows 4–5 wrong. It
  leaves room for the new refusal kinds above.
- **B — 0 wrong.** Both earlier runs measured 0. Any single refusal fails the run, the engine's own `covered` limit on a
  closed option included. The strictest reading.
- **C — Tighter than 5 % but above 0 (for example at most 2 %).** At about 80–100 calls it allows 1–2 wrong.
- **D — At most 5 %, and refusals of a cause the engine states as its own limit do not count.** Only `covered`
  refusals on a closed select's option or under an open list (the founder's kept limits) are excluded. Everything else
  counts. The verdict would report both totals. The pipe needs one more closed-word rule.

**Measured basis:** the shared basis above. 0.2.0: 0 of 66 under 5 %.

**What depends on it:**
- the value of `WRONG_CALL_BAR_PCT` (5 today), plus the cause rule if D;
- the bar-edge contract tests (`test_five_wrong_calls_of_a_hundred_pass_and_six_fail_the_bar` pins 5/100 and 6/100 as
  literals, re-pinned to the new edge);
- v030-15's concretized acceptance (the number written into it);
- the live run's verdict;
- what F2 may say («under a bar of …»).

Until his word, the pipe keeps 5 (0.2.0's) as its standing value, and the live run does not fire.

## F2 — What the run's result may claim (README.md and docs/capabilities.md)

**The question:** after the live run, what do the README and the capability statement say about it, on a pass and on
a fail?

**Today's text:**
- README.md:87 «### 0.3.0 — built, its cold-agent run to come»;
- README.md:98 «16 of its 17 capabilities are verified headlessly; a fresh agent's run over the new capabilities is the
  version's last step»;
- docs/capabilities.md:527–531, the measured-instance paragraph naming the 2026-10-25 run (twelve tools, 66 calls,
  0 wrong, 5 %).

0.2.0's README form, for comparison: «A fresh agent given only the twelve MCP tools drove six apps … in 66 calls, with
0 wrong, under a bar of at most 5 % wrong.»

**Options:**
- **A — State the run as measured, naming its reach.** On a pass: the tools (fifteen), the apps, the call and wrong
  counts, the bar, and that `graph`, `actions`, `layout` and the select list were each used. Also name what the run did
  not reach and where each is proven instead: `caused` (`stand_act_errors`), handler coverage (a command-line role, no
  MCP tool), a fixture state, exploration, a component alone and the seed (in process only), and the bundled fonts.
  On a fail: the same counts and the closed reasons, with «0.3.0 — built, its cold-agent run failed against the bar»
  until a run passes.
- **B — State only the outcome and the counts.** «passed (or failed) against the founder's bar: N calls, W wrong»,
  with no capability list and no reach statement.
- **C — Claim nothing until a second run repeats it.** The status line stays «its cold-agent run to come» until two
  runs agree. This needs a second live run (F3).

**Measured basis:**
- `test_capability_statement.py` pins only README's «the {N} driver commands as tools» phrase and the capability
  page's verb, cause and count names, not the run's numbers. Any text written stays true by hand.
- The secret search over README.md and docs/capabilities.md reads 0 and must stay 0
  (`decision/live-evidence-holds-no-credentials`).
- The capability page states its claims by hand («claims kept by hand»).

**What depends on it:** the two doc edits (gated steps of this chunk's plan), v030-15's evidence line, and whether
the version can be read complete.

## F3 — The word to run a live fresh agent

**The question:** may the one live run fire, once, through the operator's own login, reaching the model provider?

**Options:**
- **A — Run once now, under the same cap ($3).** One `bash scripts/cold-agent.sh run stand`, fired once as an
  operator entry, never in CI or a loop. 0.2.0 cost $0.8244 at 66 calls. The added small-app reads are a few KB each.
- **B — Run once now, with a higher cap (for example $5).** The cap is `--max-budget-usd` on the client
  (`scripts/cold-agent.sh:584`). A higher cap changes the pipe and its pinned argv test.
- **C — Not now.** v030-15 stays claimed and gated. The version cannot close until he gives the word (or moves the
  bar run to 0.4.0, a route change).

**Measured basis:** 0.2.0's run cost and call count above; the client is the operator's own Claude Code login
(`apiKeySource` none, ratified 2026-10-06); the pipe never fires in CI.

**What depends on it:** v030-15's verification and the version's close. F1 is needed before the verdict means
anything. F4 is needed before the widened prompt may run at all.

## F4 — What the live run carries to the model provider (a boundary widening)

**The question:** the widened prompt has the agent call `graph` and `actions`, and their answers then reach the model
provider in the transcript. The provider crossing the founder ratified names the stand's screen (ids, names, values in
snapshot texts and diffs), a lint's findings, one screenshot and, since 2026-11-17, layout answers (security
`decision/cold-agent-screen-to-provider`). It never named `graph` or `actions` answers.

**Content that would newly cross** (both answers hold no control value, typed text or NodeId by construction):
- graph: stable ids, owner ids, component names, registered state names, handler event names, the flag words
  `undeclared` · `unconfirmed`, booleans;
- actions: ids, owner ids, `keyed` booleans, the verb table's words, closed cause words.

**Options:**
- **A — Ratify the crossing for `graph` and `actions` answers on the live run.** The security row is restated at the
  wrap with his word.
- **B — Ratify `actions` only.** The prompt asks for `graph` on no app, and `graph` drops from the required set (the
  operator's technical choice is reopened).
- **C — Ratify neither.** The prompt asks for `layout` only of 0.3.0's reads. `graph` and `actions` drop from the
  required set (reopened).

**Basis:** the playbook's escalate pattern «Boundary widening» (`rul-izni57qv`): «always a human's call». Measured
sizes above. The prompt keeps the Cells answers off the run.

**What depends on it:** the prompt's wording, the required counts, the security-plan row, and whether F3 may fire.

## F5 — A served error page, so a cold agent meets `caused` (operator's addition, inputs#I2)

**The question:** should the stand serve a seventh app whose controls panic, return an error and log, so that a
cold agent's run reaches `caused` (v030-10, the panic, error or log an action caused, returned with its answer)?

**Measured basis:**
- On the built binary, no ordinary step on any of the six served apps answers `caused`. The steps probed were:
  counter's increment, four flight-booker clicks, the timer's `advance`, CRUD's Create, a Cells click and the load
  page's Load.
- `examples/seven_guis/src` holds no log site, `panic!` or `Err(`.
- `caused` is proven today only on fixture pages, in process and through `serve_mcp` (`stand_act_errors`,
  `stand_act_error_text`).

**Options:**
- **A — No error page.** 0.3.0's run names `caused` as out of its reach, with its headless witnesses. This is what
  the plan does now.
- **B — Add one in 0.3.0.** A new chunk, or this one re-planned, adds a seventh served slug, its stand checks, a
  prompt goal and a verdict count. This is a boundary widening: a new served app, and authored error text crossing to
  the model provider in the run. It delays the live run.
- **C — Add one in 0.4.0.** A route entry for the next version. 0.3.0's run stands without it.

**What depends on it:** whether the run's reach includes v030-10, F2's reach statement, and F4's crossing (error text
would join it).

## Where the answers land

Each answer is written verbatim, in its own script, to this chunk's inputs and cited by the plan. F1 sets the
constant and its tests (a gated step). F2 sets the doc text (gated steps). F3 and F4 open the live-run entry. F5's
B or C opens new work outside this plan.
