# Curation — 2026-10-07-change-tracking-and-diff

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "In a hand-built blitz-dom unit test create elements with qual_name!(.., html) — the one-argument form is a no-namespace element that lays out inline"
  Tier 3 (.claude/docs/session-learnings.md): extended "A chunk that moves cited source lines stales the masters' file:line citations"
  Filters: 0 dup · 2 low-confidence · 0 conflict · 0 deferred · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "In a hand-built blitz-dom unit test create elements with qual_name!(.., html)"
  Extended: T3/session-learnings.md: "A chunk that moves cited source lines stales the masters' file:line citations" + "prove each computed move by content; read by hand only the hunk-touched citations, a net-zero hunk included"
```

## Applied

**Tier 2 · `.claude/rules/testing.md` `## Session Additions`** — the no-namespace fixture rule.
Score: verified by measurement +0.4 · specific technical detail with context +0.2 = 0.6 · reached no other durable home +0.2 = 0.8. Filter 1: a generated-body near-match (the file's Fixtures bullet "Blitz-dom unit tests build DOM by hand through `DocumentMutator`") — an additive facet against a generated body lands as a new Session Additions entry. Tier-2 home by the registry's class: test authoring.
Proof: step 4's control of this chunk first failed on its own fixture assertion — a node dump of the hand-built document read `html` as block and `body` / `div` as inline, with `html`'s layout children empty and no anonymous block; with the elements created in the HTML namespace the same fixture built one (`escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/evidence/control-changed-set.txt`, last paragraph; report.md → Spec claims disproved, fifth bullet). The route carries the consequence (the two existing hover-cursor tests), not this authoring rule.

**Tier 3 · `.claude/docs/session-learnings.md`** — extension, in place, of "A chunk that moves cited source lines stales the masters' file:line citations".
Score: verified by measurement +0.4 · specific technical detail +0.2 = 0.6 · reached no other durable home +0.2 = 0.8. Filter 1: over the similarity bar with the matched entry; the facet it lacks is the content proof.
Proof: this wrap's line map — 181 citations, 169 moves proven by old-lines == new-lines, and the 12 that differed were exactly the hunk-touched ranges, three of them on `window.rs`, whose hunks net to zero so no number moved (`cascade-dispositions.md` → The line map).

## Filtered
- "Removing the last statement between a `let` and its return trips clippy's `let_and_return`" — low confidence: gate failure +0.4 · technical detail +0.2 · one-off −0.3 · could be task-specific −0.2 = 0.1.
- "A case added after the plan goes inside an existing test when a gate pins the count" — low confidence: explicit operator direction +0.4 · technical detail +0.2 · could be task-specific −0.2 = 0.4. The direction was given for this chunk's rule; the generalisation is the wrap's, not the operator's.

## Recurrence-despite-learning (→ handoff, Deferred learnings)
- "The rustfmt write hook leaves a crate root unformatted when its module files do not exist yet" (Tier 3, 2026-10-06; its extension already says an edit made through a Bash script never fires the hook — run `cargo fmt --all` after it). Recurred at this chunk's /implement: the `changed_set_` test module and the engine marks were written by scripts, no `cargo fmt` followed, and the fast leg read red at its fmt step.
- "The project's Bash guards refuse a heredoc written to a file and a leading cd, and read payload prose too" (Tier 3, 2026-10-06). Recurred at this chunk's /implement: one edit script was first sent as a heredoc with a file target and refused; it went through the Write tool on the second try.

## Not curation's
- The two existing `hover_state_tests` in blitz-dom print "skipping: no usable font" and pass with system fonts on (measured at this wrap, `cargo test -p blitz-dom --locked --features system-fonts --lib hover_state_tests -- --nocapture`): a defect of existing tests, routed at P5.
- Telemetry of the pipeline (the evolve envelope's skill spelling, the sidecar entry appended before its check verdict was read) stays in the friction stream.
