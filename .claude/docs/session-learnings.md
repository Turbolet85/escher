# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-10-20 — A case-blind sweep pattern turns a code token into its plain-word homonym
A site-sweep pattern written `ignore_case = true` that holds a code token alongside its prose wording (`\bVERBS\b`
beside «eleven verbs») matches every plain word the token spells, so a table-name alternative becomes a sweep of
every mention of the word, and its standing figure counts rows that never name the table. Put a code token in a
case-sensitive pattern of its own, and keep `ignore_case` for the prose wording alone.

Where it applies: P1's site sweep and P2's cascade sweep, whenever a moved fact is named both by an identifier and by
an ordinary word.

---

## 2026-10-20 — A write-set probe must allow the files a new enum variant's exhaustive matches live in
A plan gate that asserts "no tracked file changed outside the write set" (a `git diff --name-only` filtered by an
alternation of allowed paths) reads red when a step adds an enum variant: every exhaustive `match` on that enum gains
an arm, and those files are often outside the step's own list. A must-fail control that plants some other file never
exercises this companion. When a step adds a variant to a public enum, list the files holding its exhaustive matches
(search for `match` on the enum's name) in the probe's alternation, or record them as companions in the scope record
before the gate runs.

Where it applies: phase P4/P5 when a plan's Test Commands carry a write-set probe, and implement when such a probe
reads red on a file that holds only a new match arm.

---

## 2026-10-20 — escher-driver names no engine type, so a driver type cannot hold one
escher-driver depends on blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` and `png` and on neither blitz-dom nor stylo, so a type it exports cannot carry blitz-dom's `BoundingRect` or read stylo's `Contain` or `Overflow` without a manifest and lockfile change. A plan that types a driver field with an engine type, or reads a computed style as an engine enum inside the driver, should first read the crate's `[dependencies]`. If the manifest is out of scope, the driver holds its own rectangle and reads computed styles through `BaseDocument::resolved_style_value` as closed keyword sets.

Where it applies: phase planning and implement steps that add a public escher-driver type built from engine data.

---

## 2026-10-20 — A count stated in more than one form needs every form in its sweep pattern
The masters state one reader count several ways on the same line: `25 readers`, `by 25 checks`, `by 25`, and in words, `twenty-five checks`. A sweep pattern for the digits beside one noun misses the other forms. A site that matched only on the path beside the count then reads as a path citation, and the count stays stale. Write the pattern for a moved count with every form: the digits before each noun the masters use, `by N`, and the number in words.

Where it applies: P1's site sweep and P2's cascade sweep, whenever a chunk moves a shared module's reader count or any count the masters also spell out.

---

## 2026-10-19 — A new name that is a prefix of the name it replaced counts the old sites
When a chunk renames a symbol to a shorter name that the old one begins with (`HeldNet` replacing `HeldNetProvider`), a bare grep or sweep pattern for the new name also matches every surviving site of the old one, and a count reads the retired name's sites as the new one's. Bound the new name at its end (`HeldNet\b`) and sweep the old name as its own alternative, so each site is counted once and as what it is.

---

## 2026-10-19 — An evidence time is read, never estimated
A time written into chunk evidence (a run's start or end, a stamp beside a reading) is copied from a recorded event's own stamp or from `date -u` at the moment of writing. An estimated time ahead of the clock is refused by the stamp-ahead hook, and one behind it still misdates the reading.

---

## 2026-10-19 — An in-line edit beside a provenance parenthesis reads as new provenance
On architecture, design-system and obs-plan, `classify.py check --mode standing` keys each provenance span inside a
parenthesis on the 8 tokens of the claim just before its `(`. An in-line edit that rewrites those tokens — a value
changed in place, a word added to the claim — makes the standing ratification, date or chunk marker in the parenthesis
read as new on the line, and the provenance arm fails though nothing was added. Write the new words after the
parenthesis (`… (the founder, …), and kept checks the same way (…`), or keep the claim's last 8 tokens as they were. A
superseded earlier-reading parenthesis («(10 verbs and 8 causes until …)») is history: drop it from the body rather
than re-shaping around it.

---

## 2026-10-18 — An atomic read-modify-write that builds on the MSRV and on the host toolchain
`AtomicU64::fetch_update` reads as deprecated on the dev host's toolchain ("renamed to `try_update`"), so under clippy's `-D warnings` it fails the lint leg, while `try_update` is newer than the workspace's MSRV (1.91) and fails the MSRV build. Write the update as a hand `compare_exchange_weak` loop — load, compute, `compare_exchange_weak(current, next, …)`, retry on `Err(actual)` — which builds clean on both; blitz-dom's `VirtualClock::advance` is written that way, its saturating add pinned by a unit test.

---

## 2026-10-18 — A count the plan names may stand in no master
A plan's baseline count is often taken from the previous chunk's gate log or CI run, and that chunk's wrap may never have written it to a master — a wrap that carries counts without re-reading the tallies leaves the masters on an older figure. A site sweep for the plan's own numbers then finds nothing. Before amending a tally, sweep for the masters' latest recorded reading, move the record from that figure, and name the reading in between as skipped.

---

## 2026-10-18 — A sweep pattern's clause in a master's stock wording widens it past its fact
A site-sweep pattern built from a fact's wording can carry a clause the masters also use as stock phrasing for unrelated facts: a CARRY pattern's `observed absent` clause matched 71 unrelated «observed absent» lines across five masters, and a `0 high` alternative matched «600 high». The pattern's count then measures the phrase, not the fact, and every extra row costs a disposition. Anchor such a clause to the fact's own name token (`observed absent: a test naming the builder's merged`) or drop it — a bare stock phrase belongs in no pattern.

---

## 2026-10-17 — A leaf comparer cannot see a count its group's sources do not state
`cascade.py leaves` groups a leaf by the masters its provenance names, and the comparer it spawns is told that a
statement no source states is not its own. `.claude/docs/services/escher-driver.md` names architecture alone, yet it
restates test-plan §1's per-file unit-test counts for escher-driver (`73 unit tests (… mcp_tests.rs 11 …)`). A wrap
that moves those counts amends test-plan, and the escher-driver leaf's comparer, reading architecture, raises
nothing. The stale count shows up only as the step-2 sweep's `leaf` row on that file.

Where it applies: a wrap that moves a per-crate test count. Dispose the sweep's `leaf` rows on
`services/escher-driver.md` by hand, and on any other services leaf that restates test-plan text, because no comparer
reaches them. A count pattern that carries the leaf's own spelling (the backticked file names) finds the rows.

---

## 2026-10-16 — Moving Rust items between modules strands two kinds of import
Splitting a module along its seams leaves two imports behind that the compiler only reports one at a time. An import
whose only use is an intra-doc link (`[`VERBS`]` in a doc comment) reads as `unused_imports` to rustc, so clippy
`-D warnings` fails on it, while dropping it breaks the link under rustdoc `-D warnings` — name the target by path
instead (`[`VERBS`](schema::VERBS)`, `[`NAME`](super::NAME)`), or keep the import only where code uses it. And a
`#[cfg(test)]` child module reaches its parent's private `use` imports through `use super::*`: when the code that
needed an import moves out, the parent stops importing it and the tests lose it too, moved item or not — the test
module then names it with its own `use` line.

Where it applies: any split of a Rust file into child or sibling modules, and any test module moved out of line
through `#[path]`; plan the test file's own `use` lines for every name the parent no longer imports, not only for the
moved items.

---

## 2026-10-15 — A saved `cargo build` capture in a run dir holds the project root
A phase or implement step that saves a `cargo build` run's output into its run dir commits the project root's absolute path: cargo's `Compiling {crate} v{version} ({root}/{path})` lines name it. `gate.py hygiene` refuses the file as an in-root host path. Because the path is under the root, `gate.py respell` fixes it, dropping the root prefix and nothing else, and the respell is recorded. Where it applies: any build or test capture kept as evidence. Keep the raw capture in the session scratchpad with its figures in a record, or respell it before the hygiene read.

---

## 2026-10-15 — In a master sweep, `flake` reads Nix and `repeated` reads everything
A site-sweep pattern for the flakiness budget built on `flake` matches the Nix `flake` / `flake.nix` sites of the masters — 13 of 14 standing rows at the 2026-10-14-quality-gates wrap — and one built on `repeated` matched 11 master rows, none about the stand-repeat leg. Spell the budget by `flakiness` or `zero-failure`, and the leg by its name `stand-repeat`, and disposition every row that is left.

Where it applies: P1's site sweep and P2's cascade sweep over the masters, whenever a chunk touches the flakiness budget or a repeated run.

---

## 2026-10-15 — `git fetch upstream main` moves FETCH_HEAD; read upstream's head live
`git fetch upstream main` prints `main -> FETCH_HEAD`, which says nothing about whether `upstream/main` names upstream's current head. Before a record states that upstream stands at a commit, read it live with `git ls-remote upstream refs/heads/main` and compare it with `upstream/main` (at 2026-10-14-quality-gates both read `e72650a6`).

Where it applies: a merge-surface read, an upstream-sync take-up, or any claim about upstream's `main` that a later sync will rely on.

---

## 2026-10-14 — An operator gate entry's output survives a context clear; a bare-fired one does not
`gate.py run --operator {n}` writes each entry's whole output to `$TMPDIR/andromeda-gate/{marker}/run-{UTC}/{n}.log`, outside the session and the repository, and that log outlives a pause-clear of the window. So an operator entry whose printed result was lost with the window can be read back from there instead of re-run. An operator entry the operator fires bare — `gate.py hygiene`, say — leaves no such log, and its result then rests only on the word of whoever saw it.

Where it applies: resuming a chunk after a clear. Look in that directory before calling an operator entry's output lost. For an entry whose result must survive, fire it through `gate.py run --operator` rather than bare. Extended 2026-10-18: an earlier wrap's gate log in that directory is also a per-target baseline — pairing its `test result:` lines against a new run's attributes a moved tally to the one target that moved.

---

## 2026-10-13 — An added-line census reads inline test modules too
A plan's security census over the chunk's added engine lines (`git diff … | grep -c '\.expect\('` and its kin) counts every added line, including the `#[cfg(test)]` module inside a source file. A `.expect(…)` in a new inline test therefore reddens the census though no engine line carries one. In inline test modules write `let Some(x) = … else { panic!(…) };` (or an assertion) instead of `.expect`. Then the census reads 0 without a carve-out, and the plans' own note — inline test modules use assertions — holds.

---

## 2026-10-13 — obs-plan words its silent-check census "installs no subscriber"
The census of checks that install no subscriber, read no env var and print nothing (obs-plan §3, the stand bullet) is never called a "silent census" in the master. A site-sweep pattern built on the plan's shorthand "silent census" finds no master row and is refused as an unfired control (`cascade.py sweep` exit 3). Sweep the master's own wording, `installs no subscriber`.

---

## 2026-10-13 — A phase control that reads a scratchpad file lands a host temp path in the run dir
A phase or implement control that records the command it ran, where that command reads a file in the session's scratchpad under the OS temp dir, writes the absolute host path into the committed run dir. `gate.py hygiene` refuses it (`· tmp`), and `gate.py respell` cannot fix it because the path is not under the project root, so the operator pass has to mask it by hand.

Where it applies: authoring a control. Point the recorded command at a file inside the run dir, or record it with the temp prefix already written as `$TMPDIR/…`, so the hygiene read stays clean.

---

## 2026-10-13 — `gate.py run … --operator {n} --dry-run` writes the run dir's gate trail
A dry run of an operator entry is not read-only: it appends to the run dir's `gate-{marker}.json` trail. Once that trail is tracked (after the operator pass's pre-CI commit), a dry run fired before the push entry dirties the tree, and the push entry's own `git diff --quiet` guard then fails. Set the trail aside before the dry run and restore it before the push entry runs, or skip the dry run once the trail is tracked (as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/evidence/operator-pass.md, entry 20).

Where it applies: an operator pass whose plan carries a push entry guarded by `git diff --quiet`.

---

## 2026-10-13 — The token `screenshot` in the masters mostly names something other than the driver's verb
Before 2026-10-12-headless-screenshot the masters held `screenshot` 32 times, 31 of them the upstream `examples/screenshot.rs` example, the browser's `capture` (`blitz-screenshot-<unix-secs>`) or the `screenshot` Cargo feature. A grep or sweep pattern for the verb reads all of them as the verb. Key a pattern on the verb's own context (`` `screenshot` `` beside `verb`, `--id`, `Outcome::Shot`, `fonts-unavailable`) and disposition every row that names the example, the capture or the feature as a true claim sharing the token (as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md, Counts).

Where it applies: a site sweep or a cascade sweep for the `screenshot` verb, its counts or its result fields.

---

## 2026-10-12 — `cascade.py sweep --pattern` takes its ids after one flag
`--pattern` is a list option: `--pattern A B C` runs the three named patterns, while `--pattern A --pattern B --pattern C` keeps only the last one given and silently runs one. A re-fire that should run only the patterns whose control never fired therefore names them all after a single `--pattern`. Read the count line of each pattern the listing prints before trusting a re-fire: the number of patterns run is in its `total (N patterns)` line.

Where it applies: a wrap's P1 site sweep and P2 step-2 sweep, whenever only part of a patterns file is re-fired.

---

## 2026-10-12 — A sweep pattern for a bare count also matches a citation's digits
A pattern for a count such as `\b805\b` matches every master citation whose range holds the same digits (`render.rs:805-812`), so the sweep lists a true claim beside the count it was written for. Anchor a count pattern on the words that state it (`805 passed`, `805 · 0 · 11`, `over 170 result lines`) rather than on the bare number, and when a bare number cannot be avoided, read every row it prints before dispositioning any as the count.

Where it applies: writing `report-patterns.toml` and `cascade-patterns.toml` for a moved test count, re-export count or line count.

Extended 2026-10-19: a count pattern keyed on a target's name followed by a number (`recording\W{0,3}\d+`) also matches a chunk marker that ends in that word followed by a history count («…2026-10-19-session-recording, 73 until…»), so read every row before counting it.

---

## 2026-10-11 — `gh api` refuses a job's log that holds terminal escapes
A CI job log read through `gh api repos/{owner}/{repo}/actions/jobs/{id}/logs` is refused when its text carries terminal escape sequences — and a runner's log usually does (coloured cargo and rustup output) — unless the call passes `--allow-escape-sequences`. Read a failed job's log with that flag (still with `-R` on the fork, per the Tier 1 note), and read the step's failure from the text, never from the refusal: the refusal says nothing about the job.

---

## 2026-10-10 — A grep for a struct literal cannot tell a spread from a full literal
`grep 'TypeName {'` lists every site that opens a literal of a struct and says nothing about how each one closes. A literal that ends with `..base()` or `..Default::default()` compiles unchanged when the struct gains a field; a full literal does not. A sweep that counts the hits and reads them as "closes with a spread, no change needed" is a forecast: before a plan says a set of literals compiles unchanged after a field is added, each hit is read to its closing brace, or the claim is left to the compiler and said so.

Where it applies: any research or plan step that sizes the fallout of a new struct field from a grep — the snapshot model's state struct is the measured case.

---

## 2026-10-10 — A green test run prints no figure the test only formats into its failure message
A size, a count or a list that a test computes and puts only inside an assertion's message never reaches the log of a passing run: the entry's log holds the test's `ok` line and nothing else. A plan that says a reading "is taken from the listed entries' logs" needs an entry that prints the figure on success — or names the instrument that does (a kept measurement script, a one-shot restored mutation that makes the assertion fail and show its message, a run of the binary by hand).

Where it applies: authoring a plan's readings and its Test Commands block — name the producer of every figure the report must state before the gates run.

---

## 2026-10-10 — An Edit that cuts the tail off a line takes the line's terminator with it
On a one-entry-per-line file — the working route, a master record — an Edit whose replacement is empty and whose matched text runs to the end of its line also removes the newline after it, so the next line is joined onto the entry: on the route that is the `↓` separator, and the file still reads plausibly by eye. The route tool says it (`UNPARSED: … entry line carrying ↓ inside it`) and the file is one line short of the count the edit should leave. When cutting freight off the end of a line, carry the newline and the head of the next line in both strings of the Edit, and after any such cut re-run `route.py markerless` and compare the line count before going on.

---

## 2026-10-10 — A hand re-point of a citation can match a longer citation that starts the same
At a wrap's citation sweep a master citation is re-pointed by an anchored replace of its own text, and a citation such as `tests/common/mod.rs:1-4` is also the head of `tests/common/mod.rs:1-40`: a replace keyed on the bare citation matches both, and under replace-all it would write the longer one's digits into a range nobody read. Anchor the replace on the citation plus the character that closes it — `)` or `;` — and after any replace-all search the file for the new text followed by a digit before moving on. The same holds for a single number: `host.rs:35` heads `host.rs:350`.

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

A job log fetched with `gh run view --log` carries its colour codes as literal text — `^[[1m` between `Running` and the test target's path — not as escape bytes, so a parser that strips ANSI escapes leaves them in and a pattern anchored on `Running tests/` finds nothing. Extended 2026-10-10: the Windows job prints the path with a backslash, `tests\{name}.rs`, so a `/`-only pattern misses that job even with the colour codes handled — count cargo lines in a CI log only with a pattern that has first matched a known positive from that same log, such as the bare target name.

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
