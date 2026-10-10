# Curation — 0-pending wrap, 2026-10-10

Scope: this session's conversation only (the orientation and this wrap). No chunk report exists on this path.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "An Edit that cuts the tail off a line takes the line's terminator with it"
  Filters: 0 dup · 1 task-specific · 0 conflict · 0 deferred
  No-other-home: "An Edit that cuts the tail off a line takes the line's terminator with it"
  CLAUDE.md size: 142/200 · T1 1.6 KB, 0 over 600 B
```

## Applied

- Tier 3 — "An Edit that cuts the tail off a line takes the line's terminator with it" (confidence 0.8: verified by
  measurement +0.4 · specific technical detail with context +0.2 · reached no other durable home this wrap +0.2, its
  condition held — the other signals total exactly 0.6, the next-entry signal did not fire, and the fact stands on no
  route annotation, in no master and under no hook).
  Proof: this wrap's removal of three freight blocks from the end of the "MCP surface" line of
  `escher-0.1.0/working-route.md` — an Edit with an empty replacement whose matched text ran to the line's end — left
  the file one line short and the separator joined onto the entry; `route.py cursor`, `markerless` and `epoch` each
  printed `UNPARSED: working-route.md:77 — entry line carrying ↓ inside it; read as an ENTRY`, and the scratch
  comparison against the committed route read `lines after MCP identical: False`. One anchored Edit restored the
  newline; all four reads then came back clean (`adaptation-record.md` §Verification).
  Routing: the class is host and tool mechanics, whose Tier-2 home `host-linux.md` carries no `paths:` and loads on
  every turn — judged by Tier 1's bar it is not a universal safety rule, so it went to Tier 3 whole.

## Rejected

- "Where a relay's anchor and route-resolve's insertion rule disagree on a WATCH's place, the rule's reading is put
  to the operator" — task-specific (Filter 2): one relay, one entry; the rule already stands in the wrap's own
  route-resolve reference, and the event is recorded in `adaptation-record.md` row 5.
