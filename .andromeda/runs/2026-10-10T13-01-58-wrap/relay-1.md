# The first live agent trial of `escher-session` (2026-10-10, run by the operator on the founder's word)

## What was run

The founder ruled on 2026-10-10, by question dialog in the overseer session, that a real agent trial runs now and
small («Сейчас, малое» — the overseer's label for "now, small"). A fresh agent with no knowledge of the project was
given one thing: the path of the built `escher-session` binary (commit `177d1652`), behind a wrapper that logs every
call. It was forbidden to read any file and told to learn the tool from what it prints. Three tasks: make the Counter
read 3 · book a one-way flight for 24 December 2026, then try a return earlier than the departure, then type a
non-date · add three people in CRUD and delete the second.

## Measured, from the wrapper's call log (not from the agent's report)

- 49 calls in 64 s (12:54:53Z–12:55:57Z): 39 exit `0` · 2 exit `1` · 1 exit `2` · 7 exit `3`.
- By verb: `click` 13 (2 refused) · `type` 13 · `snapshot` 9 (1 session error) · `status` 6 (5 session errors) ·
  `start` 4 (1 session error) · `stop` 3 · one bare call (usage).
- **Unintended failures: 3, all one cause** — `state-dir-too-long` on an absolute `--session` path (one `start`, and a
  `status` and a `snapshot` sent in the same batch). The agent recovered by guessing a relative path.
- The other 7 non-zero exits were deliberate: the bare call for usage, two clicks on the disabled Book button the
  task asked it to probe, four `status` calls to prove each session was gone.
- **0 `unknown-verb`, 0 `malformed`, 0 unintended usage errors**: the one usage line taught the whole syntax.
- No host was left running (0 `escher-session` processes after it).

By the agent's own report, with quoted output lines: all three tasks done.

## What the agent could not learn from the tool — items for the route's disposition

These are an agent's observations after one run; each is a hypothesis about the product until measured in it.

1. **`state-dir-too-long` names no bound and no way out.** The only wrong calls of the run. (Already pinned on
   "Self-description" by the Driver CLI wrap; this is its first measured cost: 3 of 3 unintended failures.)
2. **No selected or pressed state reaches a snapshot.** Which flight mode is active and which CRUD row is selected
   were inferred from side effects (a field turning enabled, Update and Delete turning enabled). The question for the
   route: is it the stand tasks that expose no such state, the snapshot that reads none, or both.
3. **Invalid input has no positive signal.** A non-date and an earlier return date showed only as Book turning
   disabled; the stale line "Booked one-way flight on 24.12.2026" stayed on screen over an invalid form.
4. **Diff noise.** One click on Book answered 8 `changed` elements that had only moved (their bounds shifted); the one
   meaningful change was the element in `added`.
5. **Words never explained by the tool:** `settled`, `served`, `idle_expiry_s`, the four exit codes, and that `type`
   replaces. The agent inferred each. ("Self-description" serves help; this is what its help has to cover.)
6. **Guessed and right:** `DD.MM.YYYY` from the pre-filled value; that list rows with role `GenericContainer` and no
   state can be clicked to select.

Not exercised, so not measured: `press`, `--shift`, `advance`, `scroll`, the `timer` app, the idle expiry.

## The adaptation asked of the route (a 0-pending wrap; no chunk is pending)

**Context correction.** None beyond the trial itself: it ran on the binary built from `177d1652`, outside the
repository, and changed nothing in it. Its call log and the agent's report stay with the operator; this file is the
record to cite.

**Items for your route-resolve dialogue** — the six above, each for DISPOSITION, none pre-placed. For each: is it
already owned by a route entry (then it gains this trial as its measured cost, in one clause); does it belong on an
entry as a new CARRY; is it a property of the stand's tasks rather than of the framework; or is it nothing. Where an
item is still a hypothesis, the disposition says which entry MEASURES it first — an agent's inference is not a
measurement of the product. Items 2 and 3 touch what "state" means in v010-04, which reads verified: if the dialogue
finds the requirement's wording is not met by what a snapshot reads, say so plainly rather than place a CARRY.

**Anchors.** "Scrolling-box bounds and hit" stays the route's head and takes none of these unless the dialogue shows
one is the same defect. The order of Epoch 5 and Epoch 6 stays. No source, master or requirement changes in this wrap.
