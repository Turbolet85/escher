# Engine debts — the founder's forks

Chunk `2026-11-21-engine-debts` (route entry «Engine debts», capability v040-06). Written at P4 on the operator's word:
research first, then every fork of his in one file, each with its measurement, every option with its cost, the
recommended option first, the dependencies named. **No provisional answer is given.** The plan is not written and
nothing is built until he answers.

Four debts from 0.1.0–0.3.0. The capability says each one is fixed and proven headless in both layout modes, or kept
as a residual by his word with his reason. A fix to F1 or F4 changes behaviour for every document. Under the playbook's
engine-change rule, his own answer is the ratification and is recorded with his word as he gives it. A delegate's
answer would stand PROVISIONAL. None of the recommended options widens a boundary: no new carrier, verb, argument,
field, cause, log field or span.

All readings below were taken by reading the code at `5cf8878da` and the pinned library sources (dioxus-core 0.7.10,
boa_engine 0.22). **No trial was built or run.** Research: `doc:2026-11-21-engine-debts/research`.

**To answer:** one line per fork, its number and the option's letter (e.g. `F1 A`), or his own words. `all
recommended` is fine.

| # | The question | Recommended | Size |
|---|---|---|---|
| F1 | A focused control that stops showing keeps focus and still takes keys | A — it gives focus up after the next layout, firing no blur | small to medium |
| F2 | (only with F1 A or B) Which "stops showing" counts | A — the same set the accessibility tree already drops | none extra |
| F3 | A task run during a render "cannot focus a node that render created" | A — that case cannot happen; re-state the limit and pin the real one with a check | small, tests only |
| F4 | A page script's `Math.random` ignores the run's seed | A — a seed the embedding code can give a script document, off by default | small |
| F5 | The error-boundary registry only grows | A — drop a boundary when its component is gone | small |

**Dependencies:**
- F2 is asked only if F1 is A or B.
- F3 A rests on a reading that a check must confirm. If the check shows the old limit can happen after all, F3 comes
  back to him and nothing is built for it without his word.
- F1, F3, F4 and F5 are otherwise independent: any answer to one goes with any answer to another.
- Whatever he answers, each stated limit in `docs/capabilities.md` is rewritten to match.

---

## F1 — A focused control that stops showing keeps focus and takes keys

**What happens today.** A control holds focus. Then it is hidden: by the `hidden` attribute, by
`style="display:none"`, or by CSS alone, on itself or on a container. It keeps focus, and keys still act on it.
Measured at 2026-11-01: Enter on a hidden `summary` opened its `details`.
- The engine gives focus up only when a control comes under `inert` (`mutator.rs:1259`).
- The accessibility tree and the snapshot already drop the hidden control and read focus as "the window".
- So today the engine's focus and the agent-visible focus disagree in exactly this case. The capability statement
  names it a SC 2.4.3 gap.
- Any remote page can reach it with CSS alone.
- The stand never reaches it: Cells' editor is removed, not hidden.

**His earlier words that bear on it:**
- 2026-11-01, F1 «Убрать падение, фокус остаётся (Рекомендую)». That fork was about a crash. Option A ended the crash
  and left focus where it was.
- He was offered then, as option B, exactly the fix below ("after the next layout, a focused node with no box loses
  focus, firing no `blur`"). He did not choose it. That B was never built or measured.
- This fork asks again. Option A here reverses nothing that was built: it adds the B he was shown then.
- 2026-10-31, F3 «Снимать фокус без blur (Рекомендую)». The `inert` case gives focus up with no `blur`/`focusout`.
  Option A follows that word for the new trigger. Option B departs from it.

### Options

**A — After each layout, a focused control that no longer shows gives focus up, firing no `blur` (Recommended).**
- Where it runs: in the resolve pass, after layout, beside the existing step that closes an open select list when its
  select is hidden. It reuses the engine's existing "has a box" reader (`layout_reason.rs:345`) instead of writing a
  fourth reading.
- What changes:
  - Keys go to the document.
  - The engine's focus, the tree's focus and the snapshot's `focused` agree again.
  - The next Tab starts from the top.
- Cost:
  - About 40–60 lines in blitz-dom, an engine change for every document, remote pages included, flagged upstreamable.
  - A new check in both layout modes: `hidden`, inline style, a CSS rule, a hidden container, and Enter after the hide
    acting on nothing.
  - `key_press_after_container_lost_its_box` turns from "focus stays" to "focus given up, no panic".
  - About eight master rows are amended: the focus-fixup rows, the focus-events exception list, and the changed set,
    whose first mark caused by a layout this would be.
  - Two capability-statement limits are rewritten, and the SC 2.4.3 gap is retired.
- Risks, each to be checked:
  - Every option of a closed select computes `display: none`, but options never take focus.
  - A `display: contents` control has no box, yet browsers keep it focusable. It must be kept as "shows".
  - A hidden checkbox toggled through its label is not a focus path, so it is unaffected.

**B — As A, but firing `blur` and `focusout` like any other focus loss.**
- An app's `onblur`/`onfocusout` handler hears the hide, as it does when the control is removed.
- Departs from his 2026-10-31 word for `inert`, so the two fixups would differ.
- Same size as A, plus one more check: the events fire once and in order.
- Whether HTML's current fixup fires `blur` was **not** read in this research (no fetch). It is stated as unread.

**C — Keep the limit.**
- No engine change.
- The residual stays, with his reason.
- The SC 2.4.3 gap stays named.
- Cost: none.

---

## F2 — (only with F1 A or B) Which "stops showing" counts

**A — The set the accessibility tree already drops (Recommended).**
- The set is: `display: none` on the control or any container (the `hidden` attribute is one), `visibility: hidden`
  and `visibility: collapse`.
- The engine's focus then never names what the tree and the snapshot cannot see.
- No extra cost over F1.

**B — `display: none` alone (on the control or a container).**
- This is HTML's "not being rendered". A `visibility: hidden` control keeps focus and keys, while the tree and the
  snapshot read the window.
- So that one disagreement stays, and is stated as a limit.

---

## F3 — "A task run during a render cannot focus a node that render created"

**What the residual says.** It dates from 2026-11-07 (`res-433ttjb2`). A task the app spawns from a key handler runs
during the next render, while that render's changes are still buffered. So, by construction, it sees the document as
the previous render left it, and `MountedData::set_focus` on a node the same render created "does nothing". This was
written by reasoning, never run.

**What research found.**
- An app can only reach a node through the handle Dioxus gives it in the node's `mounted` event.
- That handle is built after the buffered render is applied, and only for a node the document holds
  (`dioxus_document.rs:254-273`).
- So no task can hold a handle to a node the same render is still creating, and the stated case cannot happen.
- What does stand, by the same construction, though still not run:
  - A task holding a handle to a node the same render removes acts on that node just before it is removed (a focus set
    there is cleared when the node goes).
  - A size or position read during the render sees the previous layout.

**His earlier word that bears on it:** «Порт, отдельный чанк (Рекомендую)», 2026-11-04. That chose the buffered render
as its own chunk. The limit itself was recorded as a residual on the operator's word, not his.

### Options

**A — Re-state the limit to what can happen, and pin it with a check (Recommended).**
- A runtime-probe check, both layout modes, pins two things:
  - The ordinary pattern works: a key handler shows a new input, a task awaits its handle, and focus lands on it.
  - What happens to a handle of a node the same render removes.
- The residual closes as "re-measured, the remnant stated". The capability statement's limit is rewritten to the
  remnant.
- Cost: tests only, about 80 lines. No engine change, nothing to ratify as behaviour.
- If the check shows the old case can happen after all, this fork comes back to him.

**B — Defer what a task writes during a render until the render is applied.**
- Focus, scroll-into-view and scroll requests made during a render are queued, then applied after it.
- Reads (a size, a position) cannot be deferred and still see the previous layout.
- Cost: medium, about 100 lines in dioxus-native-dom, a behaviour change for every Dioxus document.
- It adds merge surface where upstream's Dioxus 0.8 move later replaces the whole buffer file. That move is held while
  Dioxus stays at 0.7.3.
- Not recommended: it fixes a case research could not reach.

**C — Keep the residual as written.**
- It keeps a statement research found unreachable, so a keep would need its reason rewritten.
- Cost: none.

---

## F4 — A page script's `Math.random` ignores the run's seed

**What happens today.**
- A run's randomness comes from one seed, but only Dioxus components draw from it. A page script's `Math.random`
  (Boa's) draws from the host.
- No driver run, stand app, harness boot or kept check runs page scripts: every one of those documents parses HTML
  with scripting off.
- So no agent-facing run is affected today. Only code that builds a script document itself is: the browser app behind
  its non-default `javascript` feature, the WPT runner, the examples.
- Boa 0.22 has no hook for randomness, so a seed means replacing `Math.random` on the page's global once the script
  engine is built.

**His earlier word that bears on it:** 2026-11-14, «A: нет, сид только при загрузке». No seed crosses the driver's
carriers. A seed reaches a run only from the in-process code that builds it. Options A and C keep that word. B does not
touch it either, but changes every page.

### Options

**A — A seed the embedding code can give a script document, off by default (Recommended).**
- A `with_seed(u64)` builder on the script document, beside `with_virtual_time`. Given a seed, `Math.random` draws from
  the same splitmix64 generator the run's seed uses. Not given, nothing changes: the browser and every remote page draw
  from the host as today.
- Cost:
  - About 40 lines and one test file in blitz-vibey-script: two documents of one seed read one sequence, two seeds
    differ, an unseeded document is unchanged.
  - No new dependency, no carrier.
- Honest limit: it reaches no driver run, because none runs scripts. It closes the debt for code that embeds a script
  document.

**B — Seed every script document by default (seed 0).**
- Every page's randomness repeats on each load, remote pages in the browser app included.
- Not recommended.

**C — Keep the limit.**
- The residual stays, its reason "no driver run reaches page scripts".
- Cost: none.

---

## F5 — The error-boundary registry only grows

**What happens today.** Every error boundary an app builds is registered with its document so that `caused` can read
its errors. It is never removed. An app that remounts its parts keeps adding boundaries for as long as the session
lives (`boundary.rs:46-57`). The recorded reason was that "a dropped boundary cannot be told" behind dioxus-core's
private fields.

**What research found.**
- That reason does not hold. Each boundary is built inside its own component, so its component id can be kept beside
  it.
- Dioxus' public API then tells whether that component is still there (`VirtualDom::get_scope`,
  `Runtime::has_context`, dioxus-core 0.7.10).
- The private fields are not needed.

**No earlier word of his bears on it.** The residual was recorded on the operator's word (2026-11-16).

### Options

**A — Drop a boundary from the registry once its component is gone (Recommended).**
- Checked each time a new boundary registers, so the registry holds live boundaries and at most those dropped since the
  last registration.
- Cost: about 30 lines in `boundary.rs`, escher's own file (no upstream merge surface).
- Proof: a unit check that N remounts keep the registry bounded (a measured number pinned), and `caused` still reads a
  new error.
- The architecture contract row «the registry grows for the document's life» is rewritten. Nothing an agent sees
  changes.

**B — Wait on a dioxus-core release that exposes it.**
- None is announced, and Dioxus is held at 0.7.3. It would be named a wait on another project's release, with no end
  date.

**C — Keep it.**
- The residual stays, but needs a new reason, since the recorded one is false.
- Cost: none.

---

## For the operator, not the founder (technical, asked separately)
- With F4 A: where splitmix64 lives for the script document. Either a second copy in blitz-vibey-script, or moved to
  a crate both use.
- With F5 A: when the dead boundaries are dropped. Either at each new registration (recommended, so a read during a
  step never changes) or at each read.
