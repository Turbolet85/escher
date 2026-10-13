# Cascade dispositions — 2026-10-13-upstream-sync-polish-and-ship

## Step 2 — the sweep
`cascade.py sweep` (cascade v1.8) over `cascade-patterns.toml`, baseline a7031368 (the pre-CI parent), listing `cascade-sweep.txt`:
`total (15 patterns) · 83 rows over 20 files` · `per class · new 22/5 · standing 37/7 · leaf 22/12 · curation 0/0 · base 2/1`.

The search: one pattern per amended claim, each spanning the claim's own verbs and its retired mechanism, not only its name. The 15 patterns:
- `taffy-git`: is a git / from git / at a pinned git / git dependency / pinned by rev / the taffy git rev.
- `beta2`: the old version.
- `dom-removal`: not cleared on removal / DETACHED / only detaches / `remove_node_if_unparented` / keeps resolving until.
- `ua-default-css`: `DEFAULT_CSS` / always added / appends.
- `falsy-list`: `BOOLEAN_ATTRIBUTES` / `isBoolAttr` / 27 boolean / 27 names / 27-name / `is_bool_attr`.
- `uievent-variants`.
- `publish-false`.
- `sync-pin`: the four shas.
- `front-page`: still to come / ten or eleven driver commands / commands as tools.
- `cursor-icon` with the dev-dependency list's tail.
- `test-tallies`: 61/67 unit tests, eight files, 78/79 tests, Ran N, the area-changes test count, 112/113 files, among 107/128, 849 passed, 174/179 result lines.
- `obs-census`.
- `relayed-short`.
- `doubled-word`.
- `pipe-span`.

Not run: `ua-sheets-type` (`Option<Vec<String>>` / `add_user_agent_stylesheet` / `UserAgentStylesheet`). The first firing refused it with `control: ua-sheets-type never fired over the seven masters' pre-pass text (a7031368)`, so no master stated the API before this pass. The field's type now stands at architecture.md:121 alone, and nothing older restates it.

Two patterns read 0 rows with their control fired, both directive item 1 fixes:
- `doubled-word` — architecture.md:257 no longer holds it.
- `pipe-span` — security-plan.md:101 no longer holds it; rows :100–:102 read 4 pipes each.

The sections read: every master row below, read by offset where the line is over 2 000 chars.

### Master rows (new 22 · standing 37)
- `new` (22): this pass's own text — each one re-read on its line, no intra-line duplicate of a retired mechanism left. The `until then` / `where until that chunk` clauses carry the retired claim as history on purpose.
- taffy-git — architecture.md:73 (edited mixed ×3) and security-plan.md:237 (edited ×2): amended. The hits are the amendment's own history clause ("a git dependency pinned by `rev` until then"), and no standing claim remains.
- beta2 — obs-plan.md:148: no change. A verbatim sample log line, dated to 2026-10-07-driver-command-spans; the version it carried then is true of that capture.
- dom-removal:
  - architecture.md:91 (edited): amended (history clause).
  - test-plan.md:99 (edited): amended (history clause).
- ua-default-css — design-system.md:339 (edited): amended.
- falsy-list — test-plan.md:26 (edited mixed ×2): amended. The check's own 27 names stay, true of the unchanged file, beside the bridge's 28.
- uievent-variants — architecture.md:123 (edited mixed ×3): amended (`PointerLeave` added).
- publish-false:
  - architecture.md:106: no change ("example and app crates set `publish = false`", true).
  - architecture.md:154 (edited — digits only, the citation sweep; @c767, @c961): no change. escher-telemetry and escher-driver as `publish = false` libraries, true. blitz-vibey-script is named there as a crate, with no publish claim.
  - architecture.md:224 (edited): amended.
  - security-plan.md:248: no change (the blitz-tests crate, true).
- sync-pin:
  - architecture.md:87 ×2 (@c2271, @c3141): no change. "cut against upstream `main` at 7832c177" is the dated ref of the flagged-upstreamable diffs, true of their measurement.
  - test-plan.md:116, :317: no change — dated history of the 2026-10-10 sync's run.
  - test-plan.md:145, :330 (edited): amended, the 7832c177 / 9462a7e4 mentions left as dated history.
- front-page — architecture.md:210 (edited ×2): amended ("ten since …" kept as history, "eleven since …" added).
- cursor-icon:
  - architecture.md:52: no change (the workspace's `cursor-icon "1"`, true).
  - architecture.md:64 (edited mixed ×2): amended.
  - design-system.md:317: no change (cursor icons use the crate, true).
- test-tallies — test-plan.md:66, :145, :330 (edited): amended. The old figures stand as dated history beside the new.
- obs-census — obs-plan.md:70 (edited): amended.
- relayed-short (12 standing at the listing): the eight sites the founder's confirmed list leaves, each with its reason in `fanout-results.md`:
  - architecture.md:87 @c1884, :101 @c4160 (the orphaned-host answer), :124, :133 @c31172, :136 @c13924.
  - security-plan.md:25, :78 @c3523, :226.

  Four further sites copy one of the ten amendments and are folded into this pass under the playbook rule A the founder added at this wrap (inputs#I5): obs-plan.md:107 (the `tool_call` span, «Span с tool и ending»), a11y-plan.md:46 (the adapter states, «Ратифицировать»), a11y-plan.md:275 (the MCP crossing, «Ратифицировать как есть»), `.andromeda/registries/contracts/test-plan/session-lifecycle.md`:7 (the socket crossing, «Сокет несёт команды»). Each one is respelled to the amendment's verbatim form; `registry.py check` reads 0 defects.
### Judgment bases (base 2)
- `.andromeda/playbook.md`:42 ×2 and :62 — no change. Both are notes of rules recording other rulings («Внутри ратифицированного» / «Подтвердить всё», the snapshot-states fork; «Добавить», 2026-10-10), not one of the ten amendments. A base edit would take the propose → approve channel; none is owed.

### Leaf rows (22 over 12 files) → step 3's set
CLAUDE.md:51 · `.claude/rules/security.md`:28 · `.claude/rules/a11y.md`:32 · `.claude/docs/gotchas.md`:7, :16 · `.claude/docs/stack.md`:7, :21 · `.claude/docs/services/blitz-dom.md`:25 · `.claude/docs/services/dioxus-native-dom.md`:23, :24, :33, :35, :41 · `.claude/docs/a11y-summary.md`:14, :24 · `.claude/docs/tests-summary.md`:21, :30 · `.claude/docs/workflow.md`:15 · `.claude/docs/conventions.md`:19 · `.claude/docs/services/blitz-traits.md`:11. Every one joins step 3, re-derived there and never patched.

### Curation homes
0 rows.

## Leaves
Set: `cascade.py leaves` (pass mode — the chunk does not close Epoch 6, `route.py epoch`: 7 markerless left), the step-2 leaf files added, listing `cascade-leaves.txt`: `total · pass · 11 group(s) · 25 leaves · 281077 B · unmapped 1`. One comparer per group, one batch inside one snapshot pair (before `tree 9fc95ac3…`, after `tree ff0dc6ac…`, index and head unchanged; `git diff-tree` printed the eleven `A …/leaf-{group}.md` lines and nothing else). Apply: `cascade-leaves-apply-dry-run.txt` then `cascade-leaves-apply.txt` — `total · blocks 22 · applied 22`.
- architecture-1 · pass · 10 findings · applied 10 · by hand 1 · declined 0 — the hand write: `.claude/docs/services/dioxus-native-dom.md:41`, the step-2 `test-tallies` leaf row "60 unit tests in eight files", stale against test-plan §1 (67 in nine), which the group's one source, architecture, does not state; written to the test-plan figure with its six upstream additions named.
- architecture-2 · pass · 4 · applied 4 · 0 · 0.
- masters (CLAUDE.md) · pass · 1 · applied 1 · 0 · 0 — the coupled-pins warning no longer names a taffy git rev.
- architecture+layout-templates · pass · 0.
- architecture+test-plan · pass · 0.
- architecture+obs-plan · pass · 0.
- security-plan · pass · 1 · applied 1 · 0 · 0.
- design-system+layout-templates · pass · 0.
- test-plan · pass · 3 · applied 3 · 0 · 0.
- obs-plan · pass · 1 · applied 1 · 0 · 0.
- a11y-plan · pass · 2 · applied 2 · 0 · 0.
- unmapped `.claude/rules/host-linux.md` — the host leaf; no source states it, so no comparer and no edit.
- The step-2 leaf rows each read against the result:
  - `stack.md:21` · `services/blitz-traits.md:11` (`cursor-icon`, the workspace dependency, true): no change.
  - `a11y-summary.md:14` ("still to come", the keyboard harness, true): no change.
  - `tests-summary.md:30`'s 7832c177 mention: dated history, stands beside the new tally.
  - Every other row was rewritten by a finding above.
