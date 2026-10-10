# Curation — 0-pending wrap, review mode

No chunk was wrapped. The candidates are the five Tier 3 items the handoff carried for the founder's review and the
nine notes of the project's auto-memory store, each put as a question dialog. The founder's word on the five is
`relay-1.md` (given 2026-10-10 in the overseer session, relayed by the operator); the operator answered the nine and
the cap question by dialog in this wrap's conversation, 2026-10-10.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A per-crate `cargo clippy` is not the lint leg" (confidence 0.9)
                                              + "At a wrap's route-resolve every open finding gets a route owner" (confidence 0.9)
  Tier 2 (.claude/rules/*):                   + host-linux.md: "`pgrep -x` matches nothing for a name over 15 characters"
  Tier 3 (.claude/docs/session-learnings.md): no new entry · 2 corrected in place · 2 kept unedited
  Filters: 3 dup (notes already homed) · 0 task-specific · 0 conflict · 1 deferred (→ handoff)
  Relayed, nothing written: 3 notes · named for relay beside a write: 2
```

## Applied

- **Tier 1 — the clippy sentence** (the founder: "Promote to Tier 1"). The Tier 3 entry stays as the detail.
  Proof: four recurrences past the Tier 3 entry, recorded in the handoffs of commits `d606e718` (three, 2026-10-07)
  and `72149121` (the fourth, at the 2026-10-07-driver-command-spans implement); cause and control in the entry's
  own `[corrected 2026-10-07]` tag.
- **Tier 1 — the route-owners sentence** (the operator: "Tier 1 sentence"; note: the pipeline gap is already with
  the pipeline owner, the Epoch 4 diagnosis item on route-resolve having no delegated placement form — no second
  relay). Proof: the direction restated at three wraps, per the memory note
  `escher-wrap-findings-get-route-owners` read whole in this wrap (2026-10-06-snapshot-state-fidelity,
  2026-10-07-refusal-detection, 2026-10-07-driver-command-spans).
- **Tier 2 — host-linux.md, the `pgrep` line** (the operator: "Host rule line and relay"). The file has no `paths:`
  and loads on every turn; the line is under 600 B. Proof: three hits in one session on 2026-10-07, per the memory
  note `process-census-by-ps-comm`; the file's Processes bullet read in this wrap lacks the 15-character fact.

## Corrected in place (exempt from the cap)

- **Tier 3, 2026-10-06 — the Bash guards entry** (the founder: "Trim and relay"), retitled "The Bash guard's
  heredoc arm reads a payload's prose too". Proof: `.claude/rules/host-linux.md` carries both guard rules in its
  body since commit `9b758f6c` (2026-10-07); the guard's heredoc arm tests the whole command text — read in
  `andromeda-tools/hooks/bash-guard.sh`, arm (b), on 2026-10-09.
- **Tier 3, 2026-10-05 — the citations entry** (the founder: "Correct to a pointer"). Proof: escher's first
  citation sweep was written at the 2026-10-09-audit-corrections-agent-surfaces wrap (the handoff of commit
  `b03a5fc7`: 40 re-pointed by the tool, 4 by hand, 20 rows dispositioned).

## Kept unedited, off the carried list

- Tier 3, 2026-10-06 — "A grep hit seen through a clipped view is not read" (the founder: "Keep at Tier 3").
- Tier 3, 2026-10-05 — "Count from the listing you just read, never from the plan's forecast" (the founder: "Keep
  at Tier 3").
  Basis for both: the deferred sections of the four handoffs since they were carried name no recurrence of either.

## Deferred (Filter 5, the cap of three; the operator: "The testing.md line waits — the letter holds")

- For `.claude/rules/testing.md` `## Session Additions`: "A script that attributes `cargo test` results to a test
  target runs cargo with stderr merged into stdout and parses the one stream — `Running tests/{name}.rs` is stderr
  and the `test … ok|FAILED` lines stdout, so two captured streams attribute nothing; print one mutation's
  per-target table before the full batch." Proof: the 2026-10-07-refusal-detection chunk, plan step 11 — a
  nine-mutation control run repeated, per the memory note `cargo-test-merge-streams-for-per-target-parse`.

## Named for the operator to relay to the pipeline owner

1. **Pipeline — the Bash guard's heredoc arm reads payload prose** (the founder's word, item 1): a heredoc piped
   into a tool, with no file target, is refused when its payload's prose quotes the guarded form — the guard
   refuses what its own rule allows.
2. **Host — the 15-character process name** (note `process-census-by-ps-comm`): `pgrep -x` matches nothing for a
   name over 15 characters and says so on stderr only; for the host template's Processes bullet.
3. **Host — the stamp-ahead hook and a bare time-of-day** (note `stamp-ahead-hook-bare-times`): the operator's
   global hook places a time with a `Z` and no date on the side of midnight nearer the clock, so a recorded past
   time is refused as a stamp ahead; its message does not say to date the time. Nothing written in the project.
4. **Pipeline — the WATCH clause's anchor** (note `route-watch-clause-closes-block`): `route.py` anchors the ruled
   clause at the block's end; route-resolve's letter shows the form with the clause last and does not say that
   nothing may follow it, and the tool's `clause UNPARSED` row does not say it either. Nothing written in the
   project.
5. **Pipeline — a directive item outside the 0-pending doors** (note `andromeda-letter-over-directive`): the wrap
   letter's 0-pending step does not say what becomes of an item a directive places there — the operator's ruling
   (2026-10-09): the letter wins, the item is carried to the chunk wrap, asked once, before any costly read.
   Nothing written in the project.

Not relayed, on the operator's word: the route-owners gap in route-resolve (already with the pipeline owner).

## The memory store

All nine notes are deleted and the index is emptied. Already homed, nothing written (the operator: "Remove —
already homed", three times): `bash-cd-guard` and `bash-heredoc-file-guard` (the host rule file's Paths and
Transports bullets, and the trimmed Tier 3 entry) · `escher-clippy-per-crate-trap` (the new Tier 1 sentence and the
Tier 3 entry). The other six: the dispositions above.
