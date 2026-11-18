# P5 review — 2026-11-18-cold-agent-live-run «Cold-agent live run»

**Plan:** `doc:2026-11-18-cold-agent-live-run/plan` rev 4. It is rendered for your editor at
`.andromeda/cache/docs/2026-11-18-cold-agent-live-run/plan.md` (398 lines, of them the `[[gate]]` fence's 20 entries).
No plan stands in a chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-cold-agent-live-run` is a
diff of the chunk's records.

## Goal
Fire the founder's ONE live cold-agent run at implement (F3, `--max-budget-usd 3`, inputs#I1), with no rehearsal and no
second firing. A second run waits on his new word, and /implement stops and tells you (inputs#I2). The run goes over
the stand's seven apps, judged by the 5 % bar (F1). Keep its evidence without content. Then state it in README.md and
docs/capabilities.md as measured, with its reach (F2). On a pass the README heading reads «built, its cold-agent run
passed» (your P4 answer, inputs#I3); on a fail it reads «built, its cold-agent run failed against the bar» (F2).

## The steps
1. Restate the stale bar comment (`scripts/cold-agent.sh:80-81`) to F1; no code line moves.
2. Readiness, none reaching the provider. A red stops the chunk before the firing:
   - the pipe's 42 contract tests and its bash parse;
   - a comment-only diff probe;
   - the client's 12 flags on 2.1.327;
   - `stand_order_served`;
   - seven_guis `host_log`, `cli_commands` and `mcp_server`;
   - the preservation guard.
3. `claude --version`, the operator probe.
4. **The one run**: `cargo build --locked -p seven_guis --bin escher-session && bash scripts/cold-agent.sh run
   stand`, an operator entry fired once by /implement as soon as steps 2-3 read green, **with no further stop**. It is
   never re-fired, and a red is the result, not a fix-loop target.
5. The evidence, copied into the chunk's store folder:
   - `live-verdict.json` and `live-events.jsonl`, as written;
   - `live-check.json`, as parsed;
   - `live-transcript.jsonl`, with the session dir and home masked;
   - `live-run.md`, the counts table, with `refused_by_cause` (CARRY cry-32fid2ry) and `caused_by_kind`.

   `server.log`, `client.log` and the kept checks are never copied.
6. README.md:87 heading and :98 sentence from the verdict, in the 0.2.0 sentence's form.
7. docs/capabilities.md:532-543 from the verdict: what reached the provider, the counts, and the reach, each item with
   its `stand_*` witness.
8. The docs probes and the statement's pin read green.
9. The operator pass: hygiene, pre-CI commit, push, CI read.

## Touchpoints
- `scripts/cold-agent.sh` (comment only)
- `README.md`
- `docs/capabilities.md`

No new repository file; the evidence lives in the chunk's store folder.

## Test commands (20 entries)
- **Readiness:** entries 1-7, all baselined green at 2026-11-18T19:11Z.
- **Operator:** 8, `claude --version`; 9, THE RUN.
- **Evidence:** 10, the verdict against v030-15; 11, no host path in the transcript copy; 12, no credential or e-mail.
- **Docs:** 13, the heading; 14, the stale lines gone; 15, the verdict's counts on both pages; 16, the statement's
  pin.
- **The rest:** 17, `fast`; then the operator pass, 18-20.

Entries 10-15 read red on the untouched tree, by an absent input or the subject the chunk builds. Each was fired as
itself over a scratch clone on an input it must pass and one it must fail, and all read as designed: 23 clone firings
and one typed (`.andromeda/runs/2026-11-18T18-34-15-phase/p5-controls.md`).

## Checks
- **Mechanical:** pass. The fence parses (20 entries). Planlint reads 0 hits; its one listed «unchanged» claim, on the
  pipe, agrees with step 1's comment-only change. The inputs are cited, 1 unchanged and 2 messages. Research's lists
  parse (modify 3). The graph check is skipped, since no Rust file is touched. No harm to the host is recorded for the
  reused pipe.
- **Rows:** the files total about 324 KB, past 250 000 B, so they were read by search. The 19 rows the plan cites
  were each found in their rows file.
- **Validation-1:** aligned with the scope (rev 3) and the working entry.
- **Escalate patterns:** none is shipped. The run's carriage to the provider is ratified (F4, F5; security ·
  decision/cold-agent-screen-to-provider).

## The concretization preview — v030-15 «Cold-agent bar» (the CARRY's cap; gated flag kept)
**Now:**

> The one live `bash scripts/cold-agent.sh run stand`, fired on the founder's word, reads `"outcome": "passed"` in
> `target/cold-agent/verdict.json`: all six apps met; `graph_calls`, `actions_calls`, `layout_calls` and `list_opens`
> each ≥ 1; lint, screenshot, advance and a part read each used; the recordings kept; the agent's check replayed green
> and able to fail; and `wrong_calls × 100 ≤ B × tool_calls`, B the bar the founder sets from 0.2.0's baseline of six
> apps, 66 calls and 0 wrong (F1). The run's reach statement names `caused`, handler coverage, a fixture state,
> exploration, a component alone, the seed and the bundled fonts as outside a cold agent's MCP carrier, each with its
> headless witness.

**Proposed** (`.andromeda/runs/2026-11-18T18-34-15-phase/acceptance-v030-15.md`):

> The one live run, fired exactly once at implement on F3, reads `"outcome": "passed"` in its verdict, copied to
> evidence/live-verdict.json. That holds:
> - all **seven** apps in `tasks_met`, the order page among them;
> - **`caused_by_kind` ≥ 1 for each of panic, error and log**;
> - the eight counted capabilities each ≥ 1;
> - `isolated` on the fifteen tools, `counts_agree` and `recording_kept`;
> - `check_passed` and `check_can_fail`;
> - `wrong_calls × 100 ≤ 5 × tool_calls`.
>
> README.md and docs/capabilities.md state the run as measured (F2), with its reach: handler coverage, a start in a
> state, exploration, a component alone, the seed and the bundled fonts, each with its named `stand_*` witness.

**Never weakened:** six apps become seven. B becomes the founder's 5. `caused` leaves the out-of-reach list because the
run must now meet all three kinds (CARRY cry-vrneu5x7). F2's docs statement is added.

**Gated:** the cap's `gated` text names F1/F3/F4, which inputs#I1 answers. Its `ref` is written by /implement only with
that clearance recorded (`--cleared-file`). On a fail no `ref` is written, and the cap stays claimed and `planned` for
the wrap.

## Beyond the plan
none
