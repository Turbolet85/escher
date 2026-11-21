# Driver debts — the founder's forks

Chunk `2026-11-20-driver-debts`, entry «Driver debts» (`v040-05`). Written at P4 in the away form on the operator's
word at this take-up: every fork, with its dependencies, in one file, **no provisional answer given**. The plan is not
written and nothing is built until his word returns.

The capability's acceptance is: «Each of the three debts reaches the founder as a fork; the graph is read in parts as
the founder ruled, every part within the answer bound; each debt fixed is proven on the stand through the driver's
carriers, and each one kept stands as a residual with his reason.» The readings below were taken at `1455525f` from
source and from the built `escher-session` binary on the stand, unless a line says otherwise.

**To answer:** one line per fork, giving its number and the option's letter (`F1 A`, `F3 B`), or his own words.
`all recommended` is fine. **F2 needs no answer.** His earlier ruling already settles it (quoted below). Write `F2 B`
only if he wants to set that ruling aside.

## All three at a glance

| # | The question | Recommended | Size |
|---|---|---|---|
| F1 | `press` with Ctrl, so an agent can choose rows that are not next to each other in a list | A — add `--ctrl`, held as ⌘ on macOS and Ctrl elsewhere | small |
| F2 | The UI graph read in parts | **already ruled**: `graph --part N`, paged as `snapshot` is | small to medium |
| F3 | Handler coverage when a handler stops the event from bubbling | A — when a handler stops the event, count only the innermost listener as reached | small |

## What depends on what

- **None of the three depends on another.** Each changes its own verb or reader, and any mix of answers can be built.
- **They share three files.** The capability statement `docs/capabilities.md`, the self-description `describe.rs` and
  the pinned help strings move with every fix. A kept debt keeps its stated limit in all three places.
- **F1 A and F1 B differ only on macOS.** Today the seven served tasks read the same answers on linux, windows and
  macOS (`stand_cross_os_readings`). F1 A keeps that true. F1 B breaks it for any recorded check that uses `--ctrl`
  in a text field.

---

## F1 — `press` with Ctrl

**The debt** (`res-hfvjj77l`). `press` takes a key and an optional `--shift`, and nothing else. In a list where more
than one row can be chosen (a `multiple` select), the engine already understands Ctrl: an arrow with Ctrl moves the
current row without changing the choice, and Space then flips the current row in or out. A click with Ctrl flips a
row too. But the driver cannot hold Ctrl, so an agent cannot choose, say, rows 1 and 3 without row 2. This limit is
stated in `press`'s help (`schema.rs:315-320`) and in `docs/capabilities.md:408-409`.

**His earlier word.** At 2026-10-31-engine-debts he kept it as a limit: «Ограничение и residual (Рекомендую)»
(`chunks/2026-10-31-engine-debts/inputs/I12-engine-debts-founder-2026-10-31.md.txt`, line 31). Version 0.4.0's intent
(§4) brings every debt of 0.1.0–0.3.0 back to him as a fork, so that word answers the earlier version and not this
one. Option D below is that same word, repeated.

**Measured.**
- The engine's two readers treat Ctrl differently. The list reads Ctrl and ⌘ alike (`keyboard.rs:283`). A text field
  reads one "action key": ⌘ on macOS, Ctrl elsewhere (`util.rs:8-10`). With the action key, the arrows move a word at a
  time, Home and End go to the start or end of the whole text, and Delete and Backspace delete a whole word
  (`text.rs:287-375`).
- The driver already holds that same per-host action key. It uses it to select a field's text before `type`
  (`execute.rs:800-802`).
- A new flag costs little: the command line, MCP's tool listing and the `actions` reader all take it from the verb
  table (`cli.rs:303`, `mcp/listing.rs:79`, `actions.rs:125`). The work is the schema row, the typed command, the key
  press, the help text and the checks.
- **It crosses a boundary.** `--ctrl` would be a new argument on a verb every carrier already carries (the session
  socket, the command line, MCP, kept checks). The playbook's escalate pattern `rul-izni57qv` ("a validated surface
  admits a new input class") names this, and `snapshot --part` was ratified as a widening the same way
  (`decision/large-screen-in-parts`). The flag's value is a closed true/false and carries no text. **His answer is
  the ratification.**

**Options.**
- **A — add `--ctrl`, held as the host's action key (⌘ on macOS, Ctrl elsewhere). (Recommended)** A non-adjacent set
  becomes reachable by keys: `press --ctrl --key arrow-down` as often as needed, then `press --key space`. Text fields
  gain word moves and word deletes. Every answer stays the same on all three operating systems, because the engine
  reads the host's action key in both places. The help would state that the flag means ⌘ on macOS.
  *Cost:* small — one argument on `press`, its help, the checks (stand `act_select` over the carriers, the command line,
  MCP), and the capability statement. The limit and residual `res-hfvjj77l` close.
- **B — add `--ctrl` as the literal Ctrl key on every host.** The list behaves as in A, since it reads Ctrl and ⌘
  alike. But in a text field on macOS, Ctrl does nothing the engine reads, so a recorded check that uses it there
  reads differently on a Mac than on linux. *Cost:* the same as A, plus a new stated limit (an answer that differs by
  operating system), which breaks the rule that the seven tasks read the same on every host.
- **C — A, and also `click --ctrl` (and `click --shift`).** An agent can also flip or span rows with the pointer, as
  a person does, not only with keys. *Cost:* small to medium — two more arguments on `click`, a modified pointer press
  in the harness (`input.rs` today builds every press with no modifiers), and their checks. It is a second widening, on
  `click`, needing its own ratification in the same answer.
- **D — keep the limit, as he ruled at 0.3.0.** Nothing is built. `res-hfvjj77l` stays open with his reason, and the
  help and the capability statement keep saying it. *Cost:* none now. Agents still cannot choose non-adjacent rows.

**Why A.** It closes the debt with the smallest widening (one true/false argument on one verb), and it keeps the
property most expensive to lose: the same recorded check gives the same answers on every operating system. C adds
the pointer path, which is not needed to reach the set, and costs a second widening.

---

## F2 — the UI graph read in parts (already ruled)

**The debt** (`res-7cc5bi4k`). `graph` takes no argument and answers the whole screen at once. On Cells at boot that
is **293,869 bytes**: 740 elements, about 423 bytes each, and 6 states taking 1,058 bytes. That fits the answer
bound of 1,048,576 bytes, but it is large for an agent to read. Every other served task answers in 870 to 5,432 bytes
(measured on the stand: counter 1,150 · timer 1,992 · crud 5,432 · load 870 · order 1,733).

**His ruling, which answers this fork.** The residual's own record (`res-7cc5bi4k` rev 3) says: «its form is
ruled: `graph --part N`, paged as `snapshot` is, the argument on the ratified crossing — the founder's «A. graph
--part N (Рекомендую)», given by dialog in the overseer session and relayed by the overseer at the
2026-11-08-handler-coverage wrap» (`chunks/2026-11-08-handler-coverage/inputs/I7-relay-1.md.txt`). That ruling
decides the form and ratifies the widening, so this chunk builds it as he ruled.

**What "paged as `snapshot` is" settles.** A graph within the page size answers whole, as today. A larger one answers
the requested part, or part 1 when none is named, with its number and the total. A part past the last answers empty.
`--part` is bounded 1 to 1000, like `snapshot`'s. Each part stays far below the answer bound.

**Settled by the operator (technical, by dialog, `inputs#I1`).** The page size is `snapshot`'s 10,000 bytes, so
Cells reads in about 30 parts of about 23 elements. The state list rides in part 1 only, and the verb's help says in
so many words that a later part's empty state list means "read part 1", not "no states". A part is the whole
answer's form plus two counts: `{"nodes":[…],"states":[…],"part":n,"parts":m}`.

**Options.**
- **A — build `graph --part N` as ruled. (His ruling; no answer needed.)** *Cost:* small to medium — the argument,
  cutting the graph's elements into parts in the driver, the paged answer's JSON, and the checks (Cells read in parts
  with every part within the bound, over the socket, the command line, MCP and a kept check). `res-7cc5bi4k` closes.
- **B — set his ruling aside and keep the graph whole.** Only if he wants to reverse himself. Cells' answer stays
  293,869 bytes.

---

## F3 — handler coverage when a handler stops the event from bubbling

**The debt** (`res-4dr3wt62`). When the engine delivers a click, Dioxus calls the click handlers along the way up:
the target's own first, then each ancestor's, until a handler stops the event (`stopPropagation`). Our reached
record, which `escher-session coverage` and each acting answer's `caused` read, writes down every element on the way
up whose listeners hold that event. It does so **before** any handler runs, and it never sees the stop. So a parent's
handler that the event never reached can read "exercised". This is stated in `docs/capabilities.md:384-386` and
pinned by dioxus-native-dom's reached tests.

**Measured** (dioxus-core 0.7.10, the version `Cargo.lock` holds).
- The bubbling happens inside Dioxus. `Runtime::handle_event` calls the listeners innermost first and returns as soon
  as one has stopped the event (`runtime.rs:455-463`).
- Our bridge (`dioxus_document.rs:465-505`) hands Dioxus the event once and gets back one flag: stopped or not. It
  cannot see **which** handler stopped it. The per-listener state is private to dioxus-core (`EventMetadata` is
  `pub(crate)`).
- So, from our side alone: *not stopped* → every recorded listener ran, so the record is exact. *Stopped* → at least
  the innermost one ran, and we cannot tell how far up the event went.

**Options.**
- **A — when the event was stopped, count only the innermost listener as reached. (Recommended)** The bridge still
  works out the ids before the handlers run (a handler may remove its own element), but writes them after the
  dispatch: all of them when nothing stopped the event, only the first when something did. This is exact in the
  usual case, where the clicked element's own handler stops its own event. When an ancestor stops it instead, the
  listeners in between may read "unexercised" although they ran. So the error flips direction: coverage can now
  under-count, never over-count. The stated limit is rewritten to say so. *Cost:* small — the bridge and the reached
  record in dioxus-native-dom, two reached tests (a stop by the target, a stop by an ancestor), one stand coverage
  check, and the capability statement. It touches no carrier, verb, field, span or engine behaviour: the record is
  switched on only in driver sessions and coverage runs. `res-4dr3wt62` closes and a narrower stated limit takes its
  place.
- **B — keep the limit as it stands.** Nothing is built. Coverage may still over-count an ancestor a stop never
  reached. *Cost:* none now. `res-4dr3wt62` stays open with his reason.
- **C — make the record exact by changing Dioxus itself.** Each listener would report that it ran. That needs a
  change inside dioxus-core. Today's rules allow no git dependency and no patched crate (only crates.io versions,
  Taffy excepted), and Dioxus is held at the 0.7 line (the Carry on «Upstream sync ahead of the canvas»). The ways
  there are: (C1) vendor a copied dioxus-core into the workspace — an amendment to the workspace's no-patch rule,
  which is his call, plus a fork to keep in step; or (C2) propose the hook upstream to Dioxus and wait for a release
  that has it, keeping the debt open until then. *Cost:* C1 large and lasting; C2 small for us but open-ended, and
  outside this chunk.

**Why A.** For a coverage tool, saying a handler was tested when it was not is the harmful error, and A removes it in
every case. What remains is the cautious error ("this may be untested"), and only where an ancestor stops the event.
C buys the last case at the price of a vendored framework or an open wait.

---

## What his answers set in motion

- Each **fix** is proven on the stand through the driver's carriers. Its stated limit leaves `press`'s help (F1), the
  graph's help (F2) and the capability statement, and its residual closes at the wrap.
- Each **keep** stays a residual with his reason, in his words, and its stated limit stays in all three places.
- F1 A/B/C and F2 A are boundary widenings on ratified carriers. His own answer is the ratification, and the wrap
  records it with the pattern `rul-izni57qv`.
