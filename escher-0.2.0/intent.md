# escher 0.2.0 — intent

escher is a fork of Blitz (DioxusLabs/blitz) turned into an AGENT-FIRST UI framework: a UI an agent can see, drive
and verify headless. 0.1.0 shipped the first wave on the internal stand: verified 17/17 at `af1a00ad`, the cold-agent
baseline 0 wrong calls in 43 over four tasks, and the version-end audit at 0 surviving mutants over 1,558. 0.2.0 makes
the codebase ready to grow, pays the engine's recorded debts, and adds the next capabilities by value. It leads the
first real consumer, Viola's UI.

Assembled 2026-10-16 by the escher overseer from the founder's words, given in the escher overseer session and quoted
verbatim below. Sources: `capability-list.md`, `deferred-from-0.1.0.md` and `scope-choices-2026-10-16.md` beside this
file; `.andromeda/residuals.md`; the version-end audit `.andromeda/runs/2026-10-15T10-44-59-code-audit/proposals.md`.

## Principles for this version
- **escher leads Viola.** The founder, 2026-10-16: «ну получается что нам надо чтоб эшер шел в переди виолы, чтоб мы
  когда планировали юай виолы примерно было понятно что и как умеет эшер так что давай делать интент и роут а уже
  виолу будем подстраивать под эшер». Viola's UI is planned against what escher can already do, so escher's
  capabilities come first and are stated plainly enough for that planning to read them.
- **The codebase first.** The founder, 2026-10-15: «… после разбора первым чанком 0.2.0 будет улучшение кодовой базы
  на основе аудита». The first entry improves the code on the audit's findings, before any new capability.
- **Then the engine's debts.** The founder, 2026-10-16, «Один пункт «долги движка» (Рекомендую)»: the engine
  deferrals recorded in 0.1.0 are one entry right after the improvement. Each one changes the engine for every
  document, so each comes to the founder as its own fork (the playbook's engine-change rule).
- **Kept from 0.1.0.** Infrastructure ahead of the queue. The stand is the proof, with a minimal fixture beside it
  where the tasks lack a case. Upstream is merged at each epoch boundary while our changes stay mostly additive. The
  API explains itself.

## Findings (OBSERVED → EXPECT)

1. **Codebase health.**
   OBSERVED (version-end audit, 2026-10-16): functions over the complexity ceiling went 74 → 80 → 83 across the last
   two epochs. The escher-authored growth is in blitz-dom's event handlers (`handle_click` 74, `handle_pointerdown` 40,
   `handle_key_or_input_event` 56) and the range input's `parse_float` (19). The largest file, `blitz-dom/src/document.rs`,
   holds 2,793 code lines, 688 of them in five inline test modules. Files over 800 code lines went 12 → 15 → 16;
   escher's own are `escher-driver/src/mcp.rs` (1,140) and `schema.rs` (840). Six mutants are caught only by the
   300 s timeout, never by a failing test: three in `TimerTicks::next`, two in `mcp.rs` `read_request`, one in
   `MutationWriter::create_text_node`.
   EXPECT: escher's over-ceiling handlers are split into named functions along their range and activation branches.
   escher's inline test modules leave `document.rs`, and upstream's body stays as it is. `mcp.rs` and `schema.rs`
   are split along their existing seams. Each of the six hanging mutants fails a test fast. Every check stays green,
   and the mutation score holds at 0 survivors.

2. **The engine's recorded debts.**
   OBSERVED (`deferred-from-0.1.0.md` §2): the engine ignores `inert`. It does not read `aria-checked`, nor follow
   `aria-activedescendant` to the current option. `Node::absolute_position` reads a self-scrolled box shifted by its
   own scroll. The hit does not stop at a `contain: paint` box. A clicked caret in a password field reveals the
   hidden characters' width class (measured: index 5 against 2). `disabled` removes focusability on any element, not
   only form controls. A transformed box's client rect reads its untransformed position. `covered` is ambiguous on a
   zero-height box.
   EXPECT: each debt is taken to the founder as a fork, then fixed, pinned as a stated limit, or kept as a residual by
   his word. Every fix is proven on the stand or a fixture, and its upstream merge surface has an owner.

3. **A trace becomes a regression check (F1).**
   OBSERVED: the cold-agent pipe replays one agent-written JSON check on a fresh session with no model
   (`scripts/cold-agent.sh`). No general command turns a driver session into a check.
   EXPECT: any driver session, through the CLI or MCP, can be recorded as a replayable check. The check is ordinary
   test input that runs in CI. It fails when the UI's behaviour changes and passes while it holds, so an agent's
   exploration becomes a regression test without being rewritten by hand.

4. **Visual lint (E1).**
   OBSERVED: `covered` and `off-screen` are named per action only. Text contrast is checked on the stand by one test
   (`stand_contrast`), not by a command.
   EXPECT: one command reports, for the current screen, each element that overlaps another, is clipped, has truncated
   text, lies off-screen, has zero size, or fails text contrast. Each finding names the element's stable id and its
   measured reason.

5. **One virtual clock (C1).**
   OBSERVED: time under a driver session moves only by `advance`, but the overlay scrollbar fade reads the wall clock
   (`document.rs`, `Instant::now()`), so two screenshots within 0.7 s of a scroll can differ.
   EXPECT: every time the engine and the app read in a headless run comes from one virtual clock that only the driver
   moves. Two shots of the same state read the same pixels.

6. **Ready for a real app.**
   OBSERVED: every 0.1.0 proof runs on four small 7GUIs tasks. No command waits out a network load (residual,
   2026-10-07-command-and-refusal-schema). There is no `select` interaction model (residual, the founder 2026-10-09,
   «Остаток до первой нужды»). A screen over the 10,000-byte snapshot budget (Cells) cannot be read whole.
   EXPECT: a command that meets a load in flight waits it out within a stated bound. A `select` is driven by id like
   any control. A screen over the budget is read in parts an agent can walk. Each is proven on fixtures first. The
   proof on Viola's real UI comes when Viola 0.2.0 plans its frontend: Viola's 0.1.0 is CLI-only and its frontend
   moved to its 0.2.0 (the founder, 2026-10-16, relayed by overseer1). escher is ready before that planning starts.

7. **The cold-agent bar.**
   OBSERVED: 0.1.0's run is the baseline: 4 tasks, 43 calls, 0 wrong, $0.32 (residual: "the bar starts in 0.2.0",
   the founder, 2026-10-06).
   EXPECT: 0.2.0 ends with a cold-agent run over its new capabilities, judged against a bar the founder sets from that
   baseline.

## Out of this version
Not chosen on 2026-10-16: errors returned with the action (D1), "what can I do here" (B3), seeded randomness (C2),
bundled fonts as a cross-host guarantee (C3), start in a state (C4). The deeper layer stays later: the UI graph (D2),
layout explained (D3), auto-exploration (E2), handler coverage (E3), component isolation (F2). The residuals not
named above stay residuals until asked: opt-in OTel export, MCP revision `2026-07-28`, the actionable-key check as a
command, Cells' selected-cell state, and the paste and memmap2 advisories.
