# escher — the founder's fork for «Single select sized as a list box» (2026-11-21)

Chunk `2026-11-21-single-select-sized-as-a-list-box`, route entry `chk-iqimao3h`, residual `res-ysz67imk`. He ruled
that this is built (his F4 C at 2026-11-20-select-list-and-selection-debts, «Отдельной записью маршрута
(Recommended)»). What stays his, by the operator's directive at this take-up: any choice that turns what an agent or
a page sees beyond HTML's own rendering of a sized single select, and the reversal of his F8.

Research found **one** fork of his: how much of F8 falls. Every other choice the chunk makes follows HTML's own
rendering of a sized single select and is listed at the end, so he can object to any of them.

---

## F1: How much of F8 falls

**His F8** (2026-10-31-select-popup, «Читать на multiple (Рекомендую)», option A), as the forks file of that chunk
gave it:

> Read on a `multiple` select when written. A valid `size` makes it that many rows tall and the rest scroll. With no
> `size` it stays all rows, as built, so no existing page moves. A single select's `size` stays unread, a stated
> limit: it is always a closed box and a list.

His F4 C (2026-11-20) already rules that a sized single select is built, so the last sentence falls whichever option
he takes: «A single select's `size` stays unread, a stated limit: it is always a closed box and a list.» What is left
to decide is whether the rest of F8 falls too. F8's option B, which he did not take then, was HTML's whole rule: a
`multiple` select with no `size` is 4 rows tall and the rest scroll, and a single select with `size` above 1 is a
list box.

**Measured** (`git grep` over `tests`, `packages`, `examples` and `apps`):
- The 7GUIs stand holds no `multiple` select and no select with `size`. Its one select, the flight booker's
  `flight-type`, has no `size` and keeps its popup either way, so no stand screen, cross-OS digest or committed check
  moves under either option.
- 17 `multiple` selects stand in checks and examples: 12 with no `size` (in 8 check files and `examples/form.rs`) and
  5 with one.
- A single select with `size` above 1 appears in no check today. The one sized single select, `select_popup`'s
  hostile `size="many"`, is invalid, so it reads display size 1 and keeps its popup.

**Options**

- **A: Only the last sentence falls (Recommended).** A single select whose `size` parses above 1 becomes a list box
  `size` rows tall. It opens no popup, and its keys and clicks choose a row. A `multiple` select stays exactly as F8
  built it: all rows with no `size`, `size` rows with one. *Cost:* no existing page or check moves. The `multiple`
  default stays a stated limit against HTML (4 rows), as it is today. This option is the chunk as scoped, about one
  implement window.
- **B: F8 falls whole: HTML's rule.** Everything in A, and a `multiple` select with no `size` becomes 4 rows tall,
  the rest scrolling. *Cost:* every `multiple` page with no `size` moves, remote pages included. Here that means the
  12 measured sites: their rows past the fourth scroll out of view, their snapshot bounds and diffs move, and the
  lint reads those rows `clipped`. Their checks and `stand_lint`'s planted `multiple` fixture are re-pinned. The
  chunk grows by roughly a third and may need a second implement window. The stated limit on the `multiple` default
  ends.
- **C: Keep F8 whole.** The sized single select stays unbuilt. *Cost:* it contradicts his F4 C, which gave the entry
  to be built. It is listed only so the choice is complete.

**What his answer ratifies.** Under A or B, the engine reads a single select's `size` in every document, remote pages
included. It is read as a capped number, carrying no text out, through the same `size_rows` parse a `multiple`
select's `size` already goes through. This is an engine change for every document and a boundary widening of the
select's attribute reads (the playbook's *Boundary widening* pattern). His word on A or B is its ratification. Under
either option no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span is
added.

---

## What he is not being asked: the chunk follows HTML here

These are the builder's leans. Each stands unless he or the operator says otherwise.

- **Which `size` counts.** HTML's display size: `size` parsed as a non-negative integer where that is above 0,
  else 1. So `size="1"`, `size="0"` and an invalid value such as `size="many"` leave a single select a drop-down with
  its popup.
- **Nothing chosen is allowed.** HTML's selectedness setting algorithm chooses a first option only where the display
  size is 1. A list box with no `selected` option therefore starts with nothing chosen: its value is empty, no form
  entry is sent for it, and the snapshot reads no `value`. A list box turned back into a drop-down chooses its first
  enabled option again.
- **How it reads.** HTML-AAM maps such a select to `listbox`. The tree and the snapshot read it `ListBox` with no
  `expanded`, as a `multiple` select reads, but not multiselectable. Its options read their own bounds. The chosen
  row reads `selected` and is painted in the engine's chosen-row pair, black on `#c8c8c8`, the same pair a `multiple`
  select's chosen rows take. No `highlighted` or active descendant is written, since it has no open list.
- **Keys and clicks.** The arrows, Home, End and typeahead choose a row at once and scroll it into view, firing one
  `input` then one `change`. Enter and Space open nothing. A click on a row chooses that row alone; Ctrl, Meta and
  Shift change nothing, since a single select holds one choice. A click on a label focuses the select, as today.
- **For an agent.** A `click` on a row is accepted and chooses it; a `click` on the select lands on the row at its
  centre. `type` chooses the first row whose label starts with the text, as on any select. `actions` lists the rows
  with no `covered` cause, the lint reads them like a `multiple` select's rows, and `layout` places them as
  ordinary block rows. The `click` and `press` help texts are restated for the list box under the ratified rule for
  the program's own fixed text. No new word, field or cause is added.
- **Stated limits, carried by the tool.** Ctrl+click toggling and Ctrl+A are not modelled on a list box, as on a
  `multiple` select today.
