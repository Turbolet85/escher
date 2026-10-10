# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-10-09 — In the jobs API a step's seconds and conclusion do not say whether a guarded command ran
A workflow step written `a || b` reads `success` whether `b` ran or not, and where `a` succeeds it reads 0 s. So a count of "the steps that call the script" is not a count of the jobs that ran it: the ci-scripts job's PyYAML guard calls the install script only where `import yaml` fails, and on a runner image that already carries PyYAML the script is never reached. What says whether a command ran is the job log — the command's own output is there or it is not.

Finding the step in the jobs listing has its own trap: a `run:` step has no name of its own, so the API names it `Run {the command}` — select it by a word of its command, and an action step by its `name`. Where a plan or a record states how many jobs ran something, take the number from the logs, and name a guarded call separately.

---

## 2026-10-07 — `tracing-subscriber`'s fmt layer: what a span's close record carries, and the builder's order
Read from the locked `tracing` 0.1 and `tracing-subscriber` 0.3 while building a sink that prints closed spans; each of these cost a rework before it was known.

The builder's order is fixed by its types: `with_span_events` exists only while the layer still holds the stock event formatter, so it is called before `event_format`; and when the event formatter is implemented for one field formatter only, `fmt_fields` comes before `event_format` too. The layer's record of a span closing is an event that carries the span's own metadata (`is_span()` reads true) with the span as its explicit parent, and its own fields are `message`, `time.busy` and `time.idle`. A span's fields reach the field formatter without the span's target, so a formatter that judges a field by target has to store the fields and judge them when the close record arrives; the default `add_fields` also puts a space between two recordings, which a stored form has to override.

Two spellings follow from how a value is recorded: a `&str` recorded on a span or an event prints Debug-quoted and a `format_args!` message prints bare, hence `message="close"` on a span's line beside a bare message on an event's. And the span macros take field names as literal tokens, so a `const` table cannot name them: a macro that takes the list once and emits both the span and the table is how the names are stated in one place.

---

## 2026-10-07 — A fork CI run with one job still open is not green, and how to read and re-run a hung job
A run is the witness of a gate only when its conclusion reads `verdict: green`. A run whose other jobs all read `success` while one is still open reads `in progress`, however long it has stood: fifteen of sixteen is not the acceptance, and it is never recorded as one.

`ci.py conclusion --wait` names the oldest running job and its age, not the step that job is in, so a hang looks the same as a slow build until the bound fires. When a wait returns `in progress` with no failed job, read the open job's steps (`gh run view {run} -R {fork} --json jobs`) before waiting again: a step that normally takes seconds and has stood for minutes is the hang. `gh run cancel {run}` on a run whose only open job is the hung one keeps every concluded job's result, and `gh run rerun {run} --job {databaseId}` then re-runs that job alone as a new attempt of the same run, which the conclusion read counts as one run on the same commit. Bound the re-runs before starting: stop after a second hang in the same step.

A job log fetched with `gh run view --log` carries its colour codes as literal text — `^[[1m` between `Running` and the test target's path — not as escape bytes, so a parser that strips ANSI escapes leaves them in and a pattern anchored on `Running tests/` finds nothing.

---

## 2026-10-07 — `$TMPDIR` is unset in the Bash tool's shell on this host
A redirect to `$TMPDIR/x.log` expands to `/x.log` and fails with "Permission denied", and the command before it in the pipeline still runs — its exit then reads as a failure of the thing being measured. Write scratch output to the session's scratchpad directory by its full path. The gate tool's printed `$TMPDIR/andromeda-gate/…` names the OS temp dir (`tempfile.gettempdir()`), not the shell variable.

---

## 2026-10-07 — A per-crate `cargo clippy` is not the CI lint leg
`cargo clippy -p {crate} -- -D warnings`, with or without `--all-targets`, builds blitz-dom under that one crate's feature set, and a crate that does not turn on blitz-dom's `file-input` feature gets a `needless_return` in blitz-dom's library code: the `return` stands just before a block gated on that feature, so with the feature off it is the function's last statement. The CI form, `cargo clippy --workspace --locked -- -D warnings`, unifies features across the workspace, turns `file-input` on and is green — an upstream lint that is nobody's red. To lint a chunk's new code the way CI will, run the leg itself (`bash .github/scripts/ci-leg.sh clippy`, or `fast`); for a quick compile check of one crate use `cargo test -p {crate} --locked`. A per-crate clippy run is a different check, and its failure says nothing about the gate. [corrected 2026-10-07: the cause is the feature set, not `--all-targets` or test code — the same red reads without `--all-targets`, and blitz-dom alone lints clean with `file-input` on]

---

## 2026-10-07 — A sed or grep address that ends at an item's name also selects every item whose name opens with it
A gate that lifts one function or constant out of a file by its opening line — `sed -n '/^fn controls/,/^}/p'` — lifts every item whose name starts the same way: the table `controls` and the test `controls_lie_inside_the_viewport` both match, and the comparison built on it reads red on a correct tree. Close the name with the character that follows it in the source (`fn controls(`, `const INPUT_NAMES:`), and sweep the file for other items opening with the same word before trusting the address. A control built with the same open address proves nothing here: it carries the extra item on both sides and compares equal, so plant the control's difference in the item the gate is for and check that the gate's line count is the item's own.

---

## 2026-10-07 — `str::escape_debug` and `{:?}` on a `str` do not escape alike
Both escape a `"`, a backslash, a line break and a tab, so they read as interchangeable — but `escape_debug` also backslash-escapes an apostrophe (`it's` becomes `it\'s`) and leaves a combining mark in mid-string bare, while the `Debug` form (`{:?}`) writes the apostrophe as it is and escapes the mark. Text that an agent or a person reads — the snapshot's text form, a diff, a CLI's output — wants the `Debug` form.

When a plan names one of the two and its research names the other, nothing has been decided yet: run both on a string holding an apostrophe before writing the code, and say in the plan which one the design means.

See: `.claude/docs/services/dioxus-native-dom.md` (the snapshot text bullet)

---

## 2026-10-06 — A case-insensitive grep for an upper-case acronym matches inside ordinary words
A licence or protocol acronym searched with `grep -i` matches inside common words: `-i 'MPL'` hits every "example", so a per-master site count reads dozens where the acronym itself occurs once or not at all. Sweep for an acronym case-sensitively with word boundaries (`grep -E '\bMPL\b'`), and when an `-i` count looks large, read a few hits before trusting it.

---

## 2026-10-06 — Committed run-dir and evidence text must not spell a host temp path, even as prose
The hygiene read (`gate.py hygiene`) refuses any committed run-dir or evidence file whose text holds an absolute host path, and a temp-dir path counts even inside an explanatory sentence: a phase extract that named the Claude Code session's temp-folder prefix literally, to say the README must not contain it, was refused at the pre-CI commit. When such a file has to talk about a host path, describe it in words ("the session temp-dir prefix") or write it repo-relative, and run the hygiene read before the commit.

---

## 2026-10-06 — A presentation guard that diffs grepped source lines must strip leading whitespace
A gate that proves "no class / style / author-id change" by grepping those lines from the base and the working tree and diffing them reads RED on a re-nest alone: un-nesting an element by one level shifts its `class:` line's indentation while the expression stays byte-identical. The guard's intent is the attribute text, not the indentation — so pipe both grepped sides through `sed 's/^[[:space:]]*//'` before the diff (a CSS-block comparison, whose indentation is content, stays exact). When authoring the guard, add a re-nest control beside the changed-value controls.

---

## 2026-10-06 — Inside a Bash tool call `grep` is ugrep, and a long bounded repetition can print nothing
Inside a Bash tool call `grep` is a shell function that runs Claude Code's embedded ugrep; the host's own `/usr/bin/grep` is GNU grep, and `command grep` reaches it [corrected 2026-10-07: this entry said the host's grep is ugrep — measured, `type grep` reads "grep is a function", bare `grep --version` reads ugrep 7.8.4, `command grep --version` and `/usr/bin/grep --version` read GNU grep 3.12]. A pattern carrying a long bounded repetition — a context window like `.{0,200}` around the match — exceeds ugrep's complexity limit: the error goes to stderr, and in a call that pipes or alternates several patterns stdout stays empty, which reads exactly like zero hits. Two site sweeps over the masters returned nothing that way while the same patterns in Python found four sites. For a site sweep or an absence claim, use Python's `re` (print a window around `match.start()`), or keep ugrep patterns free of long `{m,n}` counts, and read stderr before trusting an empty result.

---

## 2026-10-06 — A grep hit seen through a clipped view is not read
The `.andromeda/` masters carry lines of 1–5 KB, so a grep row viewed through `cut -c1-N`, `head -c` or a truncating tool shows a few hundred characters of a line whose fact may sit thousands of characters further in. A report that calls such a hit "unrelated" or claims "no master states X" from the clipped view is guessing; the hit is read at its match offset (a script printing a window around `match.start()`, or `cascade.py window --at`) before it is dispositioned. Absence and caught-all claims carry the full hit list, each row with its offset and its disposition.

---

## 2026-10-06 — Grepping `agent-run` also finds the cold-agent marker and "agent-runnable"
A fixed-string grep or sweep pattern `agent-run` matches inside the chunk marker `cold-agent-run-pipe` (every evidence path and sidecar citation under it) and the prose word "agent-runnable". So a hit count of the agent-run contract's sites overcounts. Anchor the pattern on the contract's own forms (`scripts/agent-run`, `target/agent-run`, `agent-run contract`) or read every row before counting it as a site.

---

## 2026-10-06 — A probe's write-up claims only what the probe read
A probe that reads an attribute and a selector match says nothing about a third consumer of the same attribute. blitz-dom reads `disabled` two ways — presence for the element state, `:disabled` and click targeting, a parsed bool for focusability — so a probe that saw `disabled="false"` match `:disabled` did not show the control leaving the focus order, yet the evidence file, the implement report, the operator question and the regression test's doc comment all said it did; two drift detectors reading the cited source caught it at the wrap. When a write-up names a mechanism, either the probe measures it or the sentence cites the code line that decides it — and a claim about one consumer of an attribute is not a claim about the others.

---

## 2026-10-06 — The rustfmt write hook leaves a crate root unformatted when its module files do not exist yet
The PostToolUse hook formats each file as it is written, but rustfmt resolves `mod` declarations: a new crate's `lib.rs` written before its `format.rs` / `panic.rs` exist fails to resolve its modules and is left as written, while the module files written after it are formatted. The miss surfaces only at the `fast` leg's `cargo fmt --check`. When creating a crate, write the module files first, or run `cargo fmt -p {crate}` once the burst of writes is done. Extended 2026-10-06: an edit made through a Bash script (`python`, `sed`) never fires the hook at all — write Rust through the Write/Edit tools, or run `cargo fmt --all` after a scripted edit.

---

## 2026-10-06 — The Bash guard's heredoc arm reads a payload's prose too
The guard's `cat`/`tee`-heredoc match is a token match over the whole command text, so a heredoc PIPED into a tool — an evolve append, an inline python script — is refused when its payload's prose quotes the guarded shell form: the record's text trips it, not the command. When a payload must mention the guarded form, describe it in words instead of quoting it. The two guard rules themselves — a heredoc with a file target, a `cd` that moves the working directory — are in `.claude/rules/host-linux.md`. [corrected 2026-10-10: trimmed to the one fact the host rule file lacks — that file has carried both rules on every turn since 2026-10-07, and the match over payload prose read true in the guard's source on 2026-10-09]

---

## 2026-10-05 — A chunk that moves cited source lines stales the masters' file:line citations
The spec masters cite code as `file:line` throughout. A chunk that inserts or removes lines in a cited file — a profile stanza in `Cargo.toml`, a guard in a workflow, a rewritten `ci.yml` — leaves every citation past the edit pointing at the wrong line, and no drift detector sees it: the detectors read the chunk report alone, and the report carries no map of moved lines. The first such chunk on escher left 114 stale citations across five masters.

The wrap's citation sweep owns the repair: it opens the wrap's reconcile phase, re-points the digits of every citation whose cited line moved, and lists each row it leaves to a read — which is read at the cited lines and dispositioned before the drift fan-out. Re-point no citation by hand outside a row the sweep lists. [corrected 2026-10-10: this entry told a wrap to grep the masters and re-point by hand — the sweep, first written on escher at the 2026-10-09-audit-corrections-agent-surfaces wrap, does it at every chunk wrap]

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
