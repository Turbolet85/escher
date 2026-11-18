# Cascade dispositions — 2026-11-17-stand-requirement-sweep (wrap 2026-11-18T00-09-04-wrap)

## Step 2 — the sweep
Search: `cascade.py sweep --patterns-file cascade-patterns-test-plan-1.toml` (one pattern, `tests1-1` fixed `MASKED_VALUE`,
the term test-plan `decision/typed-password-never-returned` r1 held and r2 lacks; control fired at architecture.md:616).
The other landings' dry runs wrote no file (`patterns: none: 1 set(s) and retire(s) changed prose alone`): architecture
tx 1902 (guard), security-plan tx 1903 (guard), test-plan tx 1905 (guard) — their deltas are step 3's.
Not looked for: the guards' old lists (an added name, no wording retired); the widened rule's new reach (graph,
actions, layout, kept and explored checks, a file input's path) — every other master states the mask per carrier and
none states the narrower password-only claim (read in the pre-pass `mask-carrier` row and these rows).

Counts (the listing's `total` and `per class`): 20 rows over 13 files · standing 10/4 · leaf 10/9.

- standing 10 (architecture 3 · security-plan 2 · test-plan 1 · a11y-plan 4): no change — each names the mask constant
  in a true claim of its own (the snapshot's value, the constant's contract, a carrier's value); the test-plan row
  `ctr-c72gcrmi` is the write contract, true. `dec-owtricwg` flagged `edited` is the guard set of tx 1902; its rule
  text is unchanged and true.
- leaf 10 (9 files): no change at step 2 — each names the mask in a true statement; the leaves are re-derived at step
  3 where a source changed.

## Leaves
Set: `cascade.py leaves --run-dir … --add .claude/docs/services/escher-driver.md --add .claude/docs/tests-summary.md`
(the two `--add`s: CARRY `cry-b3h6o7do`'s leaves, directive `inputs#I2` (2)) — epoch open, pass mode, baseline 66fbe33b;
`total · pass · 13 group(s) · 22 leaves · 663722 B · unmapped 1`. `unmapped .claude/rules/host-linux.md (host)`: the
host leaf, no source states it — no step-3 edit. Batch `leaf`: 13 comparers, 13 pass.
Apply: `leaves --apply` (all 13 files) dry run `total · blocks 7 · applied 7`, every receipt ok; written by `--only`:
`total · blocks 6 · applied 3 · held 3` (the three files holding findings; `testing.md`'s file not passed).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| added-1, added-2, architecture-1/2/3, a+s+t-1, a+s+t-3, security-plan-1/2 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan-2 (`services/escher-driver.md`) | pass | 2 | `:63#1` (the mask check missing from the MCP checks' list) | `:60` (CARRY) | `:61#1` — it hangs the function-name pin on the older `WITNESSES` table; only the 0.3.0 table holds it |
| architecture+test-plan (`commands.md`) | pass | 2 | `:61#1` (the enumerated stand command line missing the new target) | 0 | `:61#2` — a target's description, a detail its master holds; no reader acts against the source without it |
| test-plan-1 (`tests-summary.md`) | pass | 2 | `:24#1` (the mask check missing from the driver tests' inventory) | `:24` (CARRY) | `:21#1` — it replaces a true clause: a `Unit` witness is run by the workspace `test` leg outside `stand-repeat` |
| test-plan-2 (`rules/testing.md`) | pass | 1 | 0 | 0 | `:22#1` — the rule already sweeps a renamed witness; the pin's detail is its master's |

**The CARRY, by hand (one exception, the operator's word `inputs#I3`, 2026-11-18).** Both leaves were compared, and
neither comparer flagged the stale per-file counts. The pass reads a leaf against its masters' changes, and no master
row states a per-file test count (`rec.py find` over test-plan and architecture for the totals: 0 hits). The operator
ruled to remove the counts, not re-measure them, keeping what no command prints. Drafted whole by a script of
exact-substring replacements, each asserted to match once on its line and nowhere else in the file, and landed by
`rec.py harness land` (tx 1910, 1911); `git diff` reads one line changed in each.
- `.claude/docs/services/escher-driver.md:60` — 8 edits, 3352 → 2825 chars: the total (104 in seventeen files) and its
  history, every `{file}` count, the off-unix count, and the inline / `#[path]` split with its history removed; `json.rs`
  renamed `json_tests.rs` (in `json.rs` until 2026-11-17-answer-writer-by-family), and `json::tests` named among the
  `#[path]` children — without that, the line would put the tests in a file that holds none once its count is gone.
- `.claude/docs/tests-summary.md:24` — 41 edits, 22314 → 21174 chars: the same for the driver's unit tests, plus every
  per-target test count the line gives for integration targets (`cli_commands`, `cli_flow`, `host_timer`,
  `mcp_server`, `mcp_log`, `stand_mcp_tools`, `stand_mcp_masked`, the `stand_act_*` files, `scroll_into_view_nested`,
  `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box`, `transformed_client_rect`,
  `scrolled_box_absolute_position`, `stand_session_recording`, `stand_replay`, `host_binary`, `recording`, `host_log`),
  each with its «N until …» history. What each change added is kept, rewritten as «since {chunk}». Kept as is: the
  numbers inside test content (table rows, exit statuses, part numbers, needle and call counts).
