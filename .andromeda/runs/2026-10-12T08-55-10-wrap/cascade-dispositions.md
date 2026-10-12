# Cascade dispositions — 2026-10-12-self-description (wrap 2026-10-12T08-55-10)

## Step 2 — the sweep

**The search.** The patterns are in `cascade-patterns.toml` in this run dir: 20 patterns, each a retired claim of this pass in every master spelling, its verbs and its phrasings. The claims are: nine verbs/tools/commands/words; two levels; three revisions, answered `2025-06-18` otherwise; twelve node keys or fields; a session error written with kind and message only; 41 re-exports; 59 unit tests; 805 over 170 lines; 127 over 33 files; seventeen readers; twelve modules; seven field kinds; the help the driver serves about itself (to come); a password over MCP not run; `cli_commands` 8, `mcp_server` 2 and `stand_mcp_tools` 2; one command-line comparison; 108 files and 107 targets; `SessionCommand` without `Help`; a tool's description its help alone; the session verbs as the only calls with no `command` child.

The sweep was run after the last body amendment (`test-plan.md:143`, json.rs's pins) and re-run once. The second listing differs from the first only in that line's length.

**Counts** (copied from the listing): `total (20 patterns) · 43 rows over 11 files` · `per class · new 2/2 · standing 12/5 · leaf 29/6 · curation 0/0 · base 0/0`.

Patterns with 0 rows and a fired control (a statement about the pattern only): `reexp41` · `modules12` · `pwmcp` · `onecomp` · `targets` · `listdesc` · `nochild`.

**Master rows (14):**
- `architecture.md:133` rev3 @c37089 — amended in this pass: "until 2026-10-12-self-description one of three, else `2025-06-18`" is the dated history clause the amendment wrote. No change.
- `architecture.md:133` kinds7 @c11827 — amended: the list now goes on "then `Words` · `Entries` … nine kinds". No change.
- `architecture.md:133` frontpage (new) @c41709 — this pass's own text: "built at 2026-10-12-self-description and off it: the help the driver serves about itself". No change.
- `security-plan.md:78` nodef12 @c2163 — amended: "twelve node keys …, thirteen since 2026-10-12-self-description added `differs`". Dated history. No change.
- `security-plan.md:346` errform @c2667 — amended: the old form stands as the dated predecessor of `{"error":{"kind","message","remedy"}}`. No change.
- `test-plan.md:21` errform @c3458 — amended: "… and, since 2026-10-12-self-description, its remedy". No change.
- `test-plan.md:143` errform @c4979 — amended in this pass (the second read of the sweep's first listing found it unamended): json.rs pins "… its message and, since 2026-10-12-self-description, its remedy …, a changed node with `differs`". No further change.
- `test-plan.md:21` unit59 (new) @c31 — this pass's text "(59 in eleven until 2026-10-12-self-description …)". No change.
- `registries/contracts/test-plan/session-lifecycle.md:11` unit59 @c2393 — the as-measured chain's "the crate's 59" at 2026-10-10-mcp-surface. Dated history, true. No change.
- `test-plan.md:330` ws805 ×2 @c12169,12187 — mcp-surface's dated re-count, true as history. This chunk's re-count was appended after it. No change.
- `test-plan.md:116` stand127 ×2 @c5949,5988 — mcp-surface's dated re-count. This chunk's was appended. No change.
- `test-plan.md:330` stand127 @c2837 — "127 result lines" of an earlier chunk's dated reading, a true claim that shares the token. No change.
- `obs-plan.md:70` readers17 ×2 @c5321,9158 — @c5321 is this pass's quotation of the retired wording ("where 'seventeen of the eighteen checks …' stood"). @c9158 reads "the 17 readers of the stand checks' shared module" (`common/mod.rs`, a different count, window read at chars 8893-9293). No change for either.
- `test-plan.md:330` testcounts ×5 @c10499,12273,12306, and @c+12811,12884 — the first three are earlier chunks' dated re-counts (window at @c10499: driver-cli's "`cli_commands` 8, `cli_flow` 2 …"). The last two are this pass's own increments ("`cli_commands` 2, `mcp_server` 2"). No change.

**Leaf rows (29 over 6 files)** — each leaf joins step 3. This wrap runs WHOLE mode (`cascade.py leaves --all`, F252 D2, `inputs#I6`), which re-derives every leaf against its sources whole, so these rows are covered by that set:
- `CLAUDE.md` :37 (verbs9 ×2, levels2, rev3, nodef12) · :101 (verbs9) · :110 (verbs9, levels2, rev3)
- `.claude/rules/security.md` :22 (verbs9, levels2)
- `.claude/docs/commands.md` :59 (verbs9)
- `.claude/docs/security-summary.md` :23 (verbs9, levels2)
- `.claude/docs/services/escher-driver.md` :6 · :24 · :27 (verbs9) · :24 (levels2) · :27 (rev3 ×2) · :20 (nodef12, errform) · :25 (sesscmd) · :60 (unit59)
- `.claude/docs/tests-summary.md` :24 (verbs9 ×2, levels2, unit59, testcounts ×3) · :30 (ws805 ×2, stand127, testcounts ×3)

**Curation homes · judgment bases:** 0 rows. The new playbook rule ("The program's own fixed text inside a ratified carrier") states no retired wording.

**Read beyond the patterns (sections read whole for the amended claims):** architecture §Standard Contracts → Driver session, Dioxus DOM bridge (the diff) and CLIs; §Established Decisions → [Driver session]; §Occupied Resources (the `mcp` role row); §Project Intent → Front page; §Existing Scopes (escher-driver, dioxus-native-dom, blitz-tests); §Conventions → Tests. security-plan §Input Validation (the socket, state directory, schema, CLI, MCP and password rows); §API Security (opening, Settle row); §Error Handling (the refusal bullet). test-plan §1 (seven_guis, escher-driver, dioxus-native-dom, blitz-tests); §2 Directory pattern; §3 (helpers, Proof, Stand log format); §4 escher-driver; §6 (Command line, MCP); §9 baseline; the session-lifecycle key file. obs-plan §3 :70 and the escher-driver census; §4 :106-107; §6 :175, :180-181. layout-templates :74. design-system :396. a11y-plan :275.

Not swept by pattern, and left as read: `obs-plan.md:70` "its second test under its own recorder". Every test of `stand_mcp_tools` installs its own recorder, so the sentence about the second test stays true.

## Leaves

**Mode:** WHOLE (`cascade.py leaves --all`) on the founder's ruling F252 D2 (`inputs#I6`): the first wrap since the cascade deploy, run to clear the leaf drift that had accumulated. The set: 10 groups, 25 leaves, 464,759 B. One leaf is unmapped, `.claude/rules/host-linux.md` (host): it states the host's facts, no master's, so no comparer was spawned for it. The batch ran inside one snapshot pair — before `tree 7c17e87d…`, after `tree bfbe751a…`, index and head unchanged — and `git diff-tree` listed exactly the ten `A …/leaf-{group}.md` lines. The design-system comparer reported two Edit calls on its own findings file after its one Write (against the brief); the path check shows no other file touched.

**Tool total** (copied): `total · blocks 130 · applied 68 · unparsed 1 · held 61` — the 61 held are the 60 conflicts plus `a11y.md:25`, held back by main.

Per group — findings · applied by the tool · by hand · declined:
- architecture-1 — 4 · 1 (`services/dioxus-native-dom.md:19`) · 3 (`commands.md:58` ×2, `:59`, conflicting with test-plan-1's: merged, the bound stated as 94/90 with the 95-byte refusal) · 0.
- architecture-2 — 23 · 22 (`services/escher-driver.md` 20, `services/escher-telemetry.md:33`, `services/seven_guis.md:38`) · 1 (`services/seven_guis.md:11`, conflicting with layout-templates': merged) · 0.
- architecture-3 — 15 · 2 (`CLAUDE.md:37` the `describe` sentence, `:44`) · 13 (`CLAUDE.md` :33, :37 ×3, :38, :80, :85, :100, :101, :110 ×4: merged with the other groups' findings on the same spans) · 0.
- security-plan — 23 · 11 (`rules/security.md` 3, `security-summary.md` 7, `CLAUDE.md:37` `differs` in the closed words) · 12 (`security-summary.md:22` #2 unparsed — guillemets inside its replace-with — written without the quoted label; 11 `CLAUDE.md` conflicts merged) · 0.
- design-system — 5 · 3 (`design-summary.md:28` ×2, `CLAUDE.md:37` the `help` result) · 2 (`CLAUDE.md:100`, `:110`: merged) · 0.
- layout-templates — 13 · 3 (`design-summary.md:27`, `:28`, `CLAUDE.md:37` the `describe()` sentence) · 10 (`CLAUDE.md` 9 and `services/seven_guis.md:11`: merged) · 0. Its applied `CLAUDE.md:37` sentence stated `describe()` a second time beside architecture-3's; the two were merged into one statement by hand.
- test-plan-1 — 20 · 17 (`tests-summary.md` 14, `rules/testing.md` 2, `services/blitz-test-harness.md:19`, its source `test-plan.md:91` read and holding) · 3 (`commands.md`: merged) · 0.
- test-plan-2 — 18 · 2 (`CLAUDE.md:110` the trial's reading, `:13`) · 16 (`CLAUDE.md`: merged) · 0.
- obs-plan — 6 · 6 · 0 · 0.
- a11y-plan — 3 · 1 (`a11y-summary.md:24`) · 2 (`CLAUDE.md:33`: merged; `rules/a11y.md:25`: written by hand without its last clause) · partly 1 — the clause "so a change to how the three are read changes that text too" is declined: no source states it.

**Written by main with no comparer finding** (found by the re-sweep after the apply, or by reading the merged lines):
- `services/escher-driver.md:60`: was "59 unit tests (… `mcp.rs` 9 …)", now 66 by file, with the chunk's eleven controls.
- `services/seven_guis.md:11`: was "compares a snapshot … (that one test unix only)", now two comparisons, the newest revision and `help` before `start`.

**Re-sweep after the leaves:** the same 20 patterns, `total (20 patterns) · 20 rows over 7 files`, leaf rows 6 over 2 files, before the two hand writes above. The remaining leaf rows are dated history clauses ("59 in eleven until …", mcp-surface's 805/127 re-count, "one of three, else `2025-06-18`" until this chunk), plus `escher-driver.md:60`, written after.
