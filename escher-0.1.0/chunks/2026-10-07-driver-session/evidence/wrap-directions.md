# Directions for the wrap — 2026-10-07-driver-session

Recorded by /andromeda-implement, 2026-10-07T06:17Z. /implement authors no amendment, route line or key-file
edit: these are the directions it was given for the wrap, with their source. The text as received, and the
confirmation round asked before acting on it, are `inputs#I3`.

## What was ruled on
/implement's report surfaced a gap: the plan's "nothing of the screen leaves the session process" criterion is
green on a host with no log sink, while the `escher-session` binary, which installs the sink, reads stable ids on
stderr at `RUST_LOG=debug` and accessible names at `trace` (0 at the default level and at `info`). The
measurement is `host-stderr-by-level.md`.

## The three directions (as received, `inputs#I3`)
1. **Scope the security and obs wording to what was measured**: the session host and the windowed stand are
   quiet at the default level and at info; at debug third-party targets print ids, at trace also names; typed
   text is not measured.
2. **Add a route entry NEXT, ahead of Settle detection**: the telemetry sink drops records from targets outside
   its allowlist at every level, proven on the real `escher-session` binary and the windowed stand at trace, with
   a typed sentinel once a command can type.
3. **Reword the CLAUDE.md grep bullet**: inside a Bash tool call `grep` is a shell function running Claude Code's
   embedded ugrep, `/usr/bin/grep` is GNU grep, use `command grep`.

## Whose word (gate-contract.md §Scope, *Whose word*)
- Directions 1 and 2 — the operator's answer, verbatim: "The founder decided directions 1 and 2 this morning
  (2026-10-07) in the overseer session, by choosing "record it honestly and put the fix in a chunk right after"
  from three options; the wording of the directions is the overseer rendering of that choice, not his text."
  So: the choice is the founder's own, 2026-10-07; the wording reached this session relayed by its operator, as
  a rendering and not verbatim.
- Direction 3 — the operator's answer, verbatim: "Direction 3 (the grep bullet) is the operator technical
  correction." Given in this session, 2026-10-07.
- /implement's reading, labelled as such and not a ruling: direction 2 moves the route, and the rule names a
  relayed rendering on a trajectory fork PROVISIONAL until the founder's own word — while the choice it renders
  is stated to be his. Which of the two the record carries is the wrap's to write, with the operator.

## What /implement measured that bears on them
- Direction 1: only the `escher-session` binary was measured by level (`host-stderr-by-level.md`). The windowed
  stand was NOT measured by level in this chunk: its one reading here is the plan's 10 s boot smoke at
  `RUST_LOG=info`, whose captured lines held no stand id. That it prints ids at debug and names at trace is an
  inference from the shared sink and components, not a measurement.
- Direction 3, measured in this session's Bash tool (2026-10-07T06:17Z): `type grep` reads `grep is a function`;
  bare `grep --version` reads `ugrep 7.8.4`; `command grep --version` and `/usr/bin/grep --version` both read
  `grep (GNU grep) 3.12-modified`.
