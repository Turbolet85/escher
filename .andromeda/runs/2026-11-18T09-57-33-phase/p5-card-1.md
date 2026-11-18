# P5 review — 2026-11-18-served-error-page «Served error page»

The plan: `.andromeda/cache/docs/2026-11-18-served-error-page/plan.md` (rev 4, 382 lines), open it in an editor. No plan
stands in a chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-served-error-page` diffs the
chunk's records.

## Goal
A seventh served stand app, the order page (slug `order`), on the founder's F5 (inputs#I2), built as you chose at P4
(inputs#I3). It has a quantity field (boots at `0`), Submit and Export. A Submit outside 10–20 returns an `Err` that an
error boundary of the page's own takes: the screen shows only "Order failed" and Retry, so the rule is read in
`caused` alone. A good Submit logs once and shows "Order placed", the done state. Export panics. The pipe gains a
seventh goal, a `DONE` row and `caused_by_kind`, each kind required. The live run stays «Cold-agent live run»'s.

## What an agent reads in `caused`, per control (inputs#I1)
Every text is the page's own fixed text. None writes a field, so neither scrub redacts any of it.
- **Submit**, quantity outside 10–20 (`click order-submit`, or `press enter` on it) — one item:
  `kind: error` · `level: error` · `message: "quantity must be a whole number from 10 to 20"` · `boundary: own` ·
  `chain` as the error holds it (measured at implement) · `id: order-submit` · its `owner` · `event: click`. A new
  item after each Retry.
- **Retry** (`click order-retry`) — no `caused` key; the Submit button returns.
- **Submit**, quantity 10–20 — one item: `kind: log` · `level: info` · `source: app` · `target:` the page module's
  path · `message: "order placed"` · `file` and `line` of the log call · `fields: []` · `id: order-submit` · its
  `owner` · `event: click`; "Order placed" shows.
- **Export** (`click order-export`) — one item: `kind: panic` · `level: error` · `message: "export is not built yet"`
  · `file` (the page's source, its separator per OS) · `line` (`EXPORT_PANIC_LINE`) · `column` · `id: order-export`
  · its `owner` · `event: click`. The call is accepted, so it is no wrong call. The payload also prints raw on the
  host's stderr through std's chained panic hook (the standing except); the stderr sink prints none of the three texts.
- **Quantity** (`type order-quantity …`) — no `caused` key.

## Top acceptance criteria
- The page boots in both layout modes under the snapshot budget, with 0 lint findings and every control keyed.
- Each control answers exactly the items above, the same over `serve_mcp`, and a kept check replays green.
- The binary starts all seven served tasks. A host serving the page at `trace` writes neither the log message nor the
  rule to stderr.
- The served-page tables and `stand_cross_os_readings` (48 → 54 rows) take the seventh app.
- The prompt names seven apps and goal 7 with no stand id. The verdict carries `caused_by_kind`, and a run where any
  kind reads 0 fails `capability-unused`.
- `fast` and `stand-repeat` pass, and CI reads green on the pushed HEAD.

## Touchpoints (20)
New: `examples/seven_guis/src/tasks/order.rs` · `tests/blitz-tests/tests/stand_order_served.rs`.
Modified: `tasks/mod.rs` · `stand.rs` · `session_host.rs` · `tests/{cli_commands,host_log}.rs` (seven_guis) ·
`tests/blitz-tests/tests/{common/mod,stand_actions,stand_cross_os_readings,stand_explore,stand_graph,stand_handler_coverage,stand_layout,stand_keyboard,stand_contrast}.rs`
· `scripts/cold-agent.sh` · `.github/scripts/test_cold_agent.py` · `docs/capabilities.md` · `README.md`.

## Test commands — 15 entries, baselines on the untouched tree
1 seven_guis lib, green (7) · 2 `stand_order_served`, red, its target not yet written · 3 the eight served-table stand
checks, green (78, 152 s) · 4 `cli_commands` (the smoke), green (19) · 5 `host_log`, green (6) · 6 `bash -n` the pipe,
green · 7 cold-agent unittest, green (41) · 8 ci-scripts, green · 9 secret search, green, and its planted `/home/` line
reads red in a clone · 10 guard, green, its clone pair reading clean on a README edit and outside on a
`cold-agent.ps1` edit (`p5-controls.md`) · 11 `fast`, seen green at the last wrap · 12 `stand-repeat`, green, 3 of 3
rounds (948 s) · 13–15 operator: hygiene, push, CI read (`--wait 2700`, last wall 1955 s).
Implement's one-shot controls: (a)–(c) on the pipe's tests, (d) the boundary removed on the page's check.

## Capabilities
None claimed. The claimable pool is empty: v030-15 reads `planned, unclaimed, gated`, and «Cold-agent live run» carries
`CARRY: proves v030-15`. No concretization preview.

## Notes for the wrap
- The founder's F5 widening (the order page's authored `caused` text to the model provider) lands at the wrap as
  Expected amendments to `decision/cold-agent-run-carries-stand-screen` and `decision/cold-agent-screen-to-provider`.
  The wrap halts on them once, as the answer card, on his recorded word (inputs#I2).
- CARRY cry-72hc55ey (the `is_select` row) is an Expected amendment.
- CI#44279555352 on `e4c8e3047` (the last wrap) is green, 17/17, 1955 s.

## Beyond the plan
- CARRY onto «Cold-agent live run» (`chk-bx26t3rg`): at its claim, v030-15's acceptance takes the seventh app met and
  `caused_by_kind` with each of panic, error and log at least 1, beside the counts it already requires. This chunk
  makes the pipe require them; the cap's acceptance text is the live run's to concretize.

Apply? (yes / review / cancel)
