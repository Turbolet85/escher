# Curation — 2026-10-06-headless-stand

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A probe's write-up claims only what the probe read" (confidence 0.8)
  Filters: 3 dup · 1 task-specific · 0 conflict · 0 deferred
  No-other-home: "A probe's write-up claims only what the probe read"
  Extended: T3/session-learnings.md: "The rustfmt write hook leaves a crate root unformatted when its module files do not exist yet" + "an edit made through a Bash script never fires the hook" (confidence 0.8)
  CLAUDE.md size: 128/200 · T1 0.4 KB, 0 over 600 B

## Applied
- T3 "A probe's write-up claims only what the probe read" — signals: verified by measurement +0.4 (the claim was falsified), specific technical detail +0.2, no other durable home +0.2 (the engine fact landed in the masters; the write-up practice is in no master, route annotation or playbook rule) = 0.8.
  Proof: `escher-0.1.0/chunks/2026-10-06-headless-stand/evidence/disabled-false-probe.txt` (first version; corrected) and the report's first-draft Spec claim 3, against `packages/blitz-dom/src/node/element.rs:629` (`attr_parsed::<bool>`); caught by the a11y-plan and architecture detectors — `fanout-results.md` §Report correction.
- T3 extension of "The rustfmt write hook leaves a crate root unformatted …" — facet: a Bash-scripted edit never fires the hook. Signals: verified by a real gate failure +0.4, specific detail +0.2, no other durable home +0.2 = 0.8.
  Proof: /implement's first gate block (run dir `.andromeda/runs/2026-10-06T02-13-34-implement`), entry `bash .github/scripts/ci-leg.sh fast` red · exit 1 — `Diff in ./examples/seven_guis/src/tasks/timer.rs:52` — the timer seam was written with a python replace through Bash; `cargo fmt --all` fixed it.

## Rejected
- dup (generated body): Dioxus boolean attributes other than `checked` / `disabled` still write `"false"` — now in `.claude/rules/a11y.md` and `services/dioxus-native-dom.md` via this wrap's cascade.
- dup (generated body): `type_text` into a Dioxus controlled input reaches `oninput` — now in `services/seven_guis.md`.
- dup (masters): every stand boot installs a `TimerTicks` source so an undriven timer cannot tick between pumps — in test-plan §8 and `services/seven_guis.md`.
- task-specific: "a plan step naming a type as a public constant must check the crate can name it" (`ColorScheme` reached seven_guis only transitively) — plan-authoring detail of this chunk.

## Recurrence-despite-learning (→ handoff Deferred learnings)
- "The project's Bash guards refuse a heredoc written to a file and a leading cd" (T3, 2026-10-06) — two blocked calls this session: a `cat` heredoc into a throwaway probe test file (implement P2) and a leading `cd` into `.claude/docs` (this wrap's cascade).
