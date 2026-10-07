# Cascade dispositions — 2026-10-07-command-and-refusal-schema

Written from the sweep's listing after it ran and before any sidecar entry landed. The sweep ran twice with one
pattern set (`cascade-patterns.toml`, 14 patterns): once after the last body edit, once after the leaves were
re-derived. Baseline `294ff415` (the parent of the pre-CI commit `772c770f`). Every pattern's control fired on the
pre-pass masters. The trail is `cascade-2026-10-07-command-and-refusal-schema.json` in this run dir.

## What was searched
The old wording of every amended passage and the retired claims' own phrasing, over the seven masters, every file
under `.andromeda/registries/`, the leaf bodies (`CLAUDE.md`, `.claude/docs/**`, `.claude/rules/*`), the three
curation homes and the two judgment bases:

| id | pattern | what it looks for |
|---|---|---|
| count13 | `13 (inline )?unit tests\|unit tests 13` | the crate's retired unit-test count, in both word orders |
| threefiles | `in three files` | the retired file count beside it |
| ws598 | `598 passed\|598 · 0 · 8` | the workspace count the chain's last link stated |
| libcite | `escher-driver/src/lib\.rs:2[13]-34` | the two moved citations into the crate root |
| verbset | `a verb set\|verb set\|verb list` | "Not built: a verb set" and any other statement about the verb set |
| privmods | `private modules` | the crate's module list |
| sessiononly | `the session library\|The driver's session —\|driver's session:` | the crate described as the session only |
| actincluded | `` `Session::act` included `` | obs-plan §3's old silence clause |
| reexports | ``re-exports `Session` `` | the old ten-name re-export clause |
| nocommand | `no driver, CLI or MCP command\|no command can\|no driver action\|no command` | every "no command yet" claim the schema could have staled |
| notbuilt | `Not built` | the contract's "Not built" clause |
| cmdspans | `Driver command spans` | every statement about the owed span |
| waitload | `Settle and loads\|waits? on a load\|never waits on one` | every statement about waiting on a load |
| refusalword | `refusal cause\|refused with its cause\|refusal model` (any case) | other statements about refusal causes |

Not looked for: the bare numbers `13`, `598` and `143` (they match line ranges and unrelated counts — sites were
counted by phrase); the word `Command` (it reads `std::process::Command` as well); the dated history links of
test-plan §9's re-count chain other than its last one.

## Zero-row patterns (controls fired)
- **threefiles**, **libcite**, **actincluded**, **reexports** — 0 rows in both runs: the old wording stands nowhere,
  in no master, key file, leaf, curation home or judgment base.

## Master and key-file rows (47 — the same set in both runs)
Lines over 2 000 chars were read by offset, a bounded window around each match, never from the listing.

| row | disposition |
|---|---|
| `registries/contracts/test-plan/session-lifecycle.md:11` count13 · standing edited | amended — the match is this pass's text: "lifecycle unit tests 13 (…; the crate's other 12 pin the command and refusal schema…)" |
| `test-plan.md:324` ws598 · standing edited @c6294 | no change at the match — the settle-detection link is dated history and stays; this pass appended the next link (610) after it |
| `architecture.md:133` verbset · edited @c7837 | amended — the match is this pass's "the verb set is stated and validated in process and executed nowhere" |
| `architecture.md:133` privmods @c264 · `:257` privmods · `obs-plan.md:69` privmods new | amended / this pass's text — the re-export clause, the eight-module scope row, the silence clause naming the three modules |
| `obs-plan.md:69` sessiononly ×2 · `obs-plan.md:105` sessiononly | amended — both lines now name the schema beside the session library |
| `test-plan.md:106` · `test-plan.md:181` · `obs-plan.md:325` sessiononly | no change — each names the crate by its session role and says something of the socket, the state directory or the host's output, all untouched (`host.rs`, `wire.rs` byte-identical) |
| `a11y-plan.md:14` sessiononly | no change — "the session library `escher-driver` … names no feature of `dioxus-native-dom` either — it reads no snapshot" still holds: the manifest is byte-identical and no result shape names a snapshot type |
| `architecture.md:136` nocommand ×3 (@c12554, @c17456, @c19896) | no change — "no driver, CLI or MCP command exposes" the snapshot, its text, the diff or the actionable check "yet": the schema states a `snapshot` verb and diff-shaped result fields, and nothing runs a command or reaches one from a socket, a CLI or an MCP tool |
| `security-plan.md:116` nocommand ×3 (@c3709, @c4745, @c5387) | no change — the same three claims, the same reason |
| `security-plan.md:117` · `:379` · `test-plan.md:102` · `:181` · `obs-plan.md:290` nocommand | no change — "no command can type … yet": `type` is stated and validated, and executed nowhere |
| `architecture.md:133` notbuilt · edited | amended — the clause now opens "anything that runs a `Command`" |
| `obs-plan.md:105` cmdspans · edited | amended — the owed span stays owed; the value domain its cause field reads is recorded |
| `architecture.md:130` waitload @c3100 | no change — the Test harness contract's statement of settle, untouched |
| `security-plan.md:203` waitload ×3 · edited | amended — the row's own text and this pass's addition |
| `architecture.md:141` · `test-plan.md:146` refusalword | no change — the cold-agent stub's own four causes: a different surface, untouched by this chunk |
| `a11y-plan.md:270` refusalword · new | this pass's text |

## Leaf rows
First run: 14 leaf rows. Each added its leaf to step 3's set; after the re-derivation the second run reads:

| leaf | disposition |
|---|---|
| `.claude/docs/services/escher-driver.md` (count13 `:35`, verbset `:6`) | re-derived — responsibility, consumes, publishes (the verb table, `validate`, `Refusal`), conventions, gotchas, tests 25; both rows gone |
| `.claude/docs/tests-summary.md:24` count13 | re-derived — 25 in six files; row gone |
| `.claude/docs/tests-summary.md:30` ws598 | re-derived — the 610 link leads; the row that remains is the settle-detection link kept as history |
| `CLAUDE.md:37` sessiononly | re-derived (modules block) — the crate's bullet names the schema; row gone |
| `CLAUDE.md:12` · `:80` · `:104` | re-derived — the overview's `packages/` line, a pointer-table row for the schema, the architecture paragraph (the schema as built); the `nocommand` row at `:104` is "no driver action returns settled yet", still true |
| `.claude/rules/observability.md:18` sessiononly · `:36` cmdspans, refusalword | re-derived — the driver library named with its schema; the span rule carries the cause-name domain and the `Debug` hazard |
| `.claude/docs/commands.md:58` sessiononly | re-derived — "the driver crate's unit tests", with the three module selectors; row gone |
| `.claude/docs/obs-summary.md:49` cmdspans | re-derived — the schema's silence and the cause-name domain |
| `.claude/docs/security-summary.md:16` waitload · the new highlight | re-derived — the no-wait answer; a highlight for the command schema's validation |
| `.claude/docs/a11y-summary.md:22` | re-derived — the `disabled` cause on the presence reading |
| `.claude/rules/security.md` §Surfaces | re-derived — one bullet for the validation and the refusal's no-content rule |
| `.claude/docs/services/escher-telemetry.md:31` sessiononly | no change — "the session library (`escher-driver`) install[s] no subscriber" still holds of the whole crate |
| `.claude/docs/services/blitz-test-harness.md:20` waitload | no change — the harness's own statement of settle |

Leaves read and left as they are, no row: `.claude/docs/stack.md:44`, `.claude/docs/conventions.md:19`,
`.claude/docs/services/seven_guis.md:11` · `:15`, `.claude/rules/verification-harness.md:33`, `.claude/rules/a11y.md`,
`.claude/rules/testing.md` — none states an amended clause.

## Curation homes and judgment bases
0 rows in both runs: `CLAUDE.md`'s session learnings, the rule files' `## Session Additions`,
`.claude/docs/session-learnings.md`, `.andromeda/playbook.md` and `.andromeda/drift-base.md` quote none of the
retired wording.

## Lateral binds
`test-plan §3 ↔ obs-plan §3` — neither side's harness commands, status shape or log format changed. `a11y-plan`
schema ↔ `obs-plan` schema — no violation schema and no log schema changed.
