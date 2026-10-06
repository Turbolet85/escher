# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-10-06 — The project's Bash guards refuse a heredoc written to a file and a leading cd, and read payload prose too
escher's PreToolUse Bash hook blocks a `cat`/`tee` heredoc that writes a file, and a leading `cd` into a subdirectory (it would move the session's working directory for every later call). The heredoc match is a token match over the whole command text, so a heredoc PIPED into a tool — an evolve append, an inline python script — is also refused when its payload's prose quotes the guarded shell form: the record's text tripped it, not the command.

Write documents and scripts with the Write tool — a script into the session scratchpad and run by path, never into a committed run dir, which the hygiene read inspects — reach subdirectories by absolute path or a subshell, and when a payload must mention the guarded form, describe it in words instead of quoting it.

---

## 2026-10-05 — A chunk that moves cited source lines stales the masters' file:line citations
The spec masters cite code as `file:line` throughout. A chunk that inserts or removes lines in a cited file — a profile stanza in `Cargo.toml`, a guard in a workflow, a rewritten `ci.yml` — leaves every citation past the edit pointing at the wrong line, and no drift detector sees it: the detectors read the chunk report alone, and the report carries no map of moved lines. The first such chunk on escher left 114 stale citations across five masters.

At wrap, for every source file the chunk changed, grep the seven masters and the registry key files for `{file}:{N}` citations past the first changed line and re-point them — a fixed offset for a pure insert, a range map for a rewrite — then verify the re-point touched only digits (the masters' text with digits stripped equals the pre-pass text). Do it before the semantic amendments, so their freshly written citations are never shifted twice.

---

## 2026-10-05 — Count from the listing you just read, never from the plan's forecast
A plan's implementation notes often predict a count ("one cache per compiling job, 6 + 4 = 10"). When an evidence record states the measured figure, take it from the tool output read at that moment and count it there; a number carried over from the plan's forecast survives into the record looking measured. On escher the operator-pass evidence first recorded the plan's 10 caches where `gh cache list` listed 11 + 1, caught only when the report re-counted the listing. If the measurement disagrees with the forecast, record both and name the forecast as disproved.

---

## 2026-10-05 — A secret probe over a record that quotes the probe matches itself
A credential or secret grep run over an evidence record (`grep -ciE 'token=|password=|…'` expecting `0`) counts its own pattern when that record lists the probe's command verbatim — a self-match, never a leaked secret. A plan that asks the record to copy "every gate run verbatim" and also runs a secret probe over the same record is jointly unsatisfiable for the probe's own line.

When a record must name such a probe, cite where its command lives (the plan's Test Commands entry) instead of copying the pattern; when authoring a plan, keep the probe's pattern out of the files it scans. Read the hit before treating a non-zero count as a leak.

---

## Entry format

Each entry follows this structure:

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}

See: `.claude/docs/services/{service}.md` (or similar cross-reference)
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.

Other tiers:
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` section (critical, short)
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` sections (loaded when matching files touched)
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read)

wrap-session classifies each learning into its tier automatically during curation (per its curation-tier-decision contract).

## Promotion

When this file grows beyond ~200 lines, `/wrap-session` suggests promoting some entries to topic-specific files (e.g., `.claude/docs/services/{service}.md` if the learning is about a specific service). Promotion is a user action, not automatic — wrap-session never moves entries without approval.

## Demotion from CLAUDE.md

If `CLAUDE.md` `USER:session-learnings` section gets too large (≥ 180 lines total CLAUDE.md), wrap-session suggests promoting old Tier 1 entries down to this file (Tier 3) to keep CLAUDE.md within size budget. This is also a user action.
