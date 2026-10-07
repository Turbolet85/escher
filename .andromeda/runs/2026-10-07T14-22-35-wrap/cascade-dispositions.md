# Cascade dispositions — 2026-10-07-act-by-id

Written from the second `cascade.py sweep` of this pass (after every body and every leaf was written;
trail `cascade-2026-10-07-act-by-id.json`, patterns `cascade-patterns.toml`). Baseline `3c58ce5c`, the
parent of the pre-CI commit. The first sweep ran after the bodies and before the leaves; what it found
is under "What the first sweep changed".

## The search

Twenty patterns, each with a known-positive control that fired on the pre-pass masters:

- **the retired claims, by wording and by mechanism** — `exec-nowhere` (executed nowhere · runs a
  `Command` · stated and validated) · `no-drv-command` · `callers-tests` · `or-command` (socket, file or
  command) · `cannot-type` · `no-feature` (names no … feature · no feature named · names none · builds no
  accessibility code · reads no snapshot) · `decides-feature` · `not-wired` (not wired · other six causes) ·
  `held-in-process` (held in process only · no `Session` method · through a `Session` · in process only,
  through `act`) · `tick-handle` · `act-by-id`;
- **the moved counts** — `count-reexport` (32 names · eight private modules) · `count-readers` (eleven ·
  by five checks · four of them and by · of those five) · `count-unit` (25 inline unit tests · count of
  25 · in six files · other 12 pin) · `count-stand` (lists 20 files · 143 result lines, 610);
- **the old coordinates of every moved citation** — `cite-host` · `cite-common` · `cite-driver` ·
  `cite-harness` · `cite-stand`, one alternation per cited file over the 30 old ranges of the report's map.

Read over: the seven masters, every file under `.andromeda/registries/`, the three curation homes, the
two judgment bases, and the leaf bodies. Lines over 2 000 chars were read by the `@c` offset the row
carries.

**Not looked for:** a claim worded with none of these tokens. The detectors found nine such sites the
report's own searches had missed (fanout-results.md, check 4) and those are in the pattern set now; a
tenth wording, if one stands, is not caught here. Not swept: the working route, the master route, the
chunk folders and the sidecars — none is a master, a base, a curation home or a leaf.

## Every row the listing printed

### Masters and key files — 21 rows, none stale

| row | disposition |
|---|---|
| `architecture.md:133` `exec-nowhere` (standing, edited) @c8827 | amended — the hit is this pass's own sentence, "nothing public runs a `Command` that did not come from `validate`" |
| `security-plan.md:76` `exec-nowhere` (new) @c1303 | amended — the same sentence in the schema row |
| `architecture.md:136` `no-drv-command` (standing, edited) @c20851 | no change — the `unkeyed_actionable` clause: no driver command exposes that check; still true |
| `security-plan.md:116` `no-drv-command` (standing, edited) @c4969 | no change — the same clause in the `id` row; still true |
| `test-plan.md:102` `cannot-type` (standing) | no change — "no command can type into the host's instance yet": true as worded |
| `test-plan.md:181` `cannot-type` (standing) | no change — "into the binary's instance yet": true as worded |
| `architecture.md:136` `no-feature` (standing, edited) @c9801 | amended — "seven_guis names none in its own manifest and gets it through its `escher-driver` edge" |
| `a11y-plan.md:14` `no-feature` (standing, edited) ×2 @c622, @c1248 | amended — both hits are this pass's text: the binary "names none in its own manifest", and "seven_guis' own manifest still names none"; the first was reworded after the first sweep (below) |
| `architecture.md:101` `decides-feature` (standing, edited) | amended — "decided by 2026-10-07-act-by-id, the entry that first reads the snapshot through a session" |
| `test-plan.md:26` `count-readers` (new) @c11533 | no change — "eleven driver calls", a different count, this pass's text |
| `test-plan.md:100` `count-readers` (new) | no change — "the eleven readers", the new count |
| `obs-plan.md:69` `count-readers` (new) @c1607 | no change — "eleven of those twelve", the new count |
| `architecture.md:133` `held-in-process` (standing, edited) @c11239 | amended — "a step settles and a call runs in process only, through `act` and `run`" |
| `architecture.md:257` `held-in-process` (standing, edited) | no change to the clause — command, refusal and schema are still data held in process under no `cfg` gate; the row beside it gained `execute` |
| `security-plan.md:333` `held-in-process` (standing) | no change — "held in process only — nothing prints, logs or sends one yet": a `Refusal` is now also returned by `Session::run`, and still printed, logged and sent by nothing |
| `test-plan.md:142` `held-in-process` (standing, edited) @c2336 | amended — "the six `stand_act_*` files drive it through a `Session`" |
| `obs-plan.md:290` `held-in-process` (new) | no change — this pass's text, "an instance held in process only, where no sink is installed" |
| `test-plan.md:21` `count-unit` (standing, edited) @c2573 | no change — "the count of 25 at …command-and-refusal-schema/report.md" is that chunk's dated provenance; the count of 26 stands beside it |
| `test-plan.md:115` `count-stand` (standing, edited) @c2991 | no change — the settle-detection re-count is history in an append-only Proof line; this pass's re-count follows it |
| `test-plan.md:324` `count-stand` (standing, edited) @c6737 | no change — the same, in the baseline line |
| `test-plan.md:87` `cite-harness` (new) | no change — `lib.rs:26` is this pass's own citation of the `Key`, `Modifiers` re-export line |

Zero rows for `callers-tests`, `or-command` (masters), `not-wired`, `count-reexport`, `tick-handle`,
`cite-host`, `cite-common`, `cite-driver`, `cite-stand`: each control fired on the pre-pass text, so the
zero is that pattern's — every old wording and every old coordinate it names is gone from the masters and
the key files.

### Leaves — 5 rows, all this pass's text

| row | disposition |
|---|---|
| `.claude/rules/a11y.md:26` `no-feature` | re-derived — "seven_guis names none and gets it through escher-driver" |
| `.claude/docs/obs-summary.md:49` `held-in-process` | re-derived — "types into an instance held in process only" |
| `.claude/docs/tests-summary.md:24` `held-in-process` | re-derived — "drive it through a `Session` over the stand" |
| `CLAUDE.md:81` `act-by-id` | re-derived — the new pointer-table row "Driver executor (… act by id …)" |
| `CLAUDE.md:105` `act-by-id` | re-derived — "act by id (built — …)" in the Architecture paragraph |

### Curation homes and judgment bases — 0 rows

`CLAUDE.md` `USER:session-learnings`, the rule files' `## Session Additions`, `docs/session-learnings.md`,
`playbook.md` and `drift-base.md` hold none of the twenty patterns. One stale entry stands in a curation
home under none of them and was found by reading: `.claude/rules/testing.md` Session Additions,
2026-10-06 — "the delete arrives there as an Apple standard key binding the harness does not synthesize".
The harness synthesizes it since this chunk. The cascade never edits that home; it goes to curation (P3)
as an in-place extension.

## What the first sweep changed

Run after the bodies, before the leaves. It printed the 21 master rows above with one difference and 10
leaf rows:

- `a11y-plan.md:14` read "a crate gets the feature only by naming it — the seven_guis stand binary names
  none", standing beside this pass's new sentence that the feature reaches seven_guis through
  `escher-driver`: one line saying both. Reworded in the same amendment to "only by naming it or by
  depending on a crate that names it — … names none in its own manifest".
- Leaf rows, each a stale leaf and each re-derived: `CLAUDE.md:37` and `:104` (`exec-nowhere`),
  `.claude/docs/services/escher-driver.md:6` (`exec-nowhere` ×3, `held-in-process`, `act-by-id`), `:11`
  (`no-feature` ×2, `decides-feature`) and `:41` (`not-wired`), `.claude/docs/security-summary.md:29`
  (`or-command`), `.claude/docs/tests-summary.md:24` (`held-in-process`, `count-unit` ×2).

## Leaves re-derived (step 3), by provenance

Recomputed from the amended sections, not from the swept wording — so several carry no swept token:

- from **architecture** — `CLAUDE.md` `GENERATED:setup` blocks (the `escher-driver` and `seven_guis`
  module lines; the pointer table, one row added, "Driver executor"; the Architecture paragraph: the
  diff, settle and schema clauses, and act by id as built) · `.claude/docs/services/escher-driver.md`
  (regenerated whole) · `.claude/docs/services/blitz-test-harness.md` (the exports, the input helper) ·
  `.claude/docs/services/seven_guis.md` (the host's time step, `timer_step`, the feature's reach, the
  driver-action checks) · `.claude/docs/conventions.md` (feature gating) · `.claude/docs/commands.md`
  (the executor's commands);
- from **security-plan** — `.claude/docs/security-summary.md` (the snapshot-content row) ·
  `.claude/rules/security.md` (the driver-call bullet);
- from **test-plan** and its key file — `.claude/docs/tests-summary.md` (the harness-stand line, the
  blitz-tests line, the escher-driver line, the baseline) · `.claude/rules/testing.md` (Integration) ·
  `.claude/rules/verification-harness.md` (input helpers, `Session::run`, the timer);
- from **obs-plan** — `.claude/docs/obs-summary.md` · `.claude/rules/observability.md`;
- from **a11y-plan** — `.claude/rules/a11y.md`; `.claude/docs/a11y-summary.md` read and left: it carries
  no feature-reach statement and its keyboard-harness line is still true;
- from **design-system** — `.claude/docs/design-summary.md` read and left: its motion line states that no
  project-wide motion token exists, and carries no animation-runtime fact;
- from **layout-templates** — citation-only; no leaf cites `session_host.rs` by line (0 line citations of
  any of the eight modified files in the leaves, by the citation map run over them).

`USER:*` blocks and every `## Session Additions` section are untouched.

## The lateral binds

`test-plan §3 ↔ obs-plan §3` (harness commands): the agent-run contract, the status shape and the log
format are unchanged — `scripts/` and `.github/` are byte-identical — and both plans' §3 now name the six
`stand_act_*` checks consistently (test-plan as covered, obs-plan as sink-less). `a11y-plan schema ↔
obs-plan schema`: neither changed; no a11y violation schema exists yet.
