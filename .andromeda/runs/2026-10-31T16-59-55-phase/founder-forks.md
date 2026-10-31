# Select popup: the founder's forks

Chunk `2026-10-31-select-popup`, `v030-17`, last of Epoch 2. Answers are his to give, by dialog, relayed verbatim (inputs#I1). Each option's first line is its label. **Every fork here changes what the engine does for every document, remote pages included.** His own answer ratifies it; a delegate's would stand provisional. All together they are a *Boundary widening* (a validated surface admits new input and new pixels: an option's label and a group's label, painted, in every shot), so this chunk's wrap halts once as his answer card, as the route entry already says.

**What is common to every option below.** The list is drawn by the engine, inside the window, on top of the page and past any box that would clip it. Its rows are the select's own `option` elements, so each keeps its stable id open and closed. No new verb, cause, key name, carrier, port or crate is needed by any recommended answer.

**How far the browser comparisons go.** What Chromium, macOS and native menus do is from memory, not measured in this run. The code's behaviour is read from the tree (research.md).

---

## Group 1: focus and keys

### F1: What opens the list, and what a closed select's keys do

Today Enter and Space open nothing. The arrows, Home, End and typing change the choice directly, with no list. A click only focuses.

- **A: Click, Enter, Space open; the rest stays (Recommended).** A click on the select opens it and a second click closes it. Enter and Space open it (a Space while typed text still stands joins the text, as now). The arrows, Home, End and typing keep choosing with the list closed. This is how Chromium behaves on Windows and Linux. The stand's keyboard route to the return flight stays as it reads, so its Tab tables hold.
- **B: The arrows open too.** The arrows, Home and End open the list and no longer change a closed select's choice. This is the W3C pattern and macOS. It ends a ratified reading and moves `stand_keyboard`'s route and `ENABLED_TAB_ORDER`'s way in.
- **C: Only a click opens.** Enter and Space stay as they are. A keyboard user keeps choosing through the arrows and never sees the list, so the acceptance's "chosen by key" holds only for a closed select.

A disabled or inert select opens nothing under any option.

### F2: Keys while the list is open

- **A: Highlight, then confirm (Recommended).** The arrows, Home, End and typing move a highlighted row, skipping disabled ones; nothing is chosen yet. Enter or Space chooses it and closes. Escape closes with no choice. Tab chooses the highlighted row, closes and moves on, as Chromium and the W3C pattern do, and as Cells' open editor already does. One `input` then one `change` on a real change, none otherwise. Cost: the highlight is not in the snapshot, so an agent pressing keys sees it only in a screenshot. An agent can `click` a row by id instead, and the tool's help says so.
- **B: The same, but Tab closes with no choice.** Tab loses nothing typed by accident, but a keyboard user who tabs away discards the highlighted row.
- **C: The arrows choose at once.** Each step fires `input` and `change` with the list still open; Escape restores the value at opening, firing them again. Nothing is hidden from an agent. Apps hear every row passed over, a flight booker recalculating at each step.

### F3: Where focus is while the list is open

- **A: Stays on the select (Recommended).** Rows never take focus, so the Tab order does not change. The accessibility tree gains a native "expanded" on the select, the shape of F5 in Engine debts. The tree's focus stays on the select and `aria-activedescendant` stays unread by his «Оставить записанным (Рекомендую)» (2026-10-14), so an assistive technology hears that the list is open but not which row is highlighted. That is a stated limit. The list closes whenever the select stops holding focus, by any route, including `inert` (which fires no `blur`), `disabled`, `hidden` and removal.
- **B: Focus moves onto the rows.** Assistive technology can follow the row. Options become focusable, which moves every stated Tab table's meaning and the actionable-key check, and it reopens his word on `aria-activedescendant` for the same purpose by another road. Large.

### F4: A click outside the open list

- **A: Closes, and the click goes on (Recommended).** The list closes with no choice and the click acts on whatever it landed on, in one step. A driver's accepted `click` always lands, so its promise holds.
- **B: Closes, and the click is swallowed.** The target gets nothing, as some native menus behave. A driver's `click` on a visible button would be accepted and do nothing, which an agent cannot tell from a dead button.

---

## Group 2: the look

### F5: Reduced motion

The engine has no reduced-motion handling (measured absent).

- **A: No motion (Recommended).** The list appears and goes in one frame, so there is nothing to reduce and no OS setting to read. A long list's scrollbar fades as every scrollbar does today.
- **B: A short fade, with reduced-motion handling.** Needs a new read of the OS setting, registered as a new crossing, and a driver step that returns before the fade has run until an `advance`.

### F6: The engine's colours for the list

The engine holds no tokens, so these are new hardcoded defaults, as the `multiple` row's pair was.

- **A: Follow the select, one fixed highlight (Recommended).** The list takes the select's own text and background colours and its `1px solid #999` border (black on white by default; the stand's `#1a1a1a` on `#ffffff` carries over). The highlighted row is `#000000` on `#c8c8c8`, 12.56:1, the pair `multiple` rows already use. A disabled row is `#6d6d6d` on white, 5.17:1 by my arithmetic, to be measured. No shadow, no radius, no separate mark for the chosen row: the highlight starts on it, as in Chromium. Every pair is measured by a test.
- **B: A stronger highlight.** White on `#3a5ce5`, 5.47:1. More visible, but it makes the stand's accent an engine default, against the rule that stand values and engine defaults are kept apart.
- **C: Dark and Light palettes, chosen by the scheme, as scrollbars are.** More pairs to measure, and it mismatches the select itself, which stays white under Dark because the default sheet is Light-only.

---

## Group 3: the two limits the route gave this chunk

### F7: A group's label

Named in the tree and unpainted today. Its name there rests on an operator lean, not on his F7 of Engine debts, so this answer settles both.

- **A: Painted as a header row (Recommended).** In the list's colours, bold, unselectable, its options under it. A click on it does nothing and leaves the list open. It is page text in pixels and stays inside the ratified name crossing.
- **B: Left unpainted.** The rows run flat with no headers, and the limit stays stated.

### F8: `size`

Sent here by his «В чанк с попапом (Рекомендую)».

- **A: Read on a `multiple` select when written (Recommended).** A valid `size` makes it that many rows tall and the rest scroll. With no `size` it stays all rows, as built, so no existing page moves. A single select's `size` stays unread, a stated limit: it is always a closed box and a list.
- **B: HTML's whole rule.** A `multiple` select defaults to 4 rows, and a single select with `size` above 1 becomes an open list box. Correct, but it moves every `multiple` page and adds a second way a single select reads.
- **C: Left unread.** Pinned as a stated limit, with its owner a residual line. Cheapest.

---

## Group 4: what the driver reads

### F9: A driver `click` on an option of a *closed* select

The route says `covered`'s closed-option reading ends here. An option of the *open* list is accepted under every option; this asks about the closed one.

- **A: Still refused `covered`, its meaning restated (Recommended).** The fixed text gains that such an option lies under its select: click the select first. An agent reaches it in two calls, each something a pointer can do. Eight causes kept.
- **B: The click opens and chooses in one call.** Convenient, but a driver click then does what no pointer can, the objection in the text of his F8 B.

### F10: The lint on an open list (his F9, restated)

A list sits over the page by design. The rows have real boxes while open.

- **A: Rows read no `overlap`, the rest as usual (Recommended).** `overlaps` skips an open list's rows on either side, and they keep `clipped`, `off-screen` and `contrast`. A closed select reads exactly as today, and `stand_lint` stays `[0, 0, 0, 0]`.
- **B: Rows read `overlap`.** One finding per page element under the list, on every page with a list open: true and of no use.

---

## What he is not being asked

These are my leans, so a reader can object. Each stands unless he or the operator says otherwise.

- **Placement.** Below the select, or above where there is more room; at least the select's width; at most 20 rows, scrolling within. The list ignores a transform on the select's ancestors beyond starting at its on-screen box. It stays inside its own document, so inside an iframe it stays in the iframe.
- **Driver.** `click` on a select opens it. `type` on a select stays what it is today, choosing by its text with no list left open, and focuses with no click. `scroll` on a row scrolls the list. A click on a disabled row is accepted, changes nothing and leaves the list open, as a `multiple` select's row does.
- **A label's click** focuses the select and opens nothing, as browsers do.
- **The page scrolling** closes the list with no choice.
- **Windowed.** The same list reaches the window through the one paint path. No test launches a window, so it is read, not run; the existing boot smoke is all that runs it.
- **Reading whether it is open.** No new node key: an open list's rows read bounds of their own, a closed one's read the select's, and `differs` names them. By the closed-field rule (2026-10-10) this is not his to rule, and the operator is asked it separately.

---

## His answers (2026-10-31, by dialog in the overseer session, relayed verbatim)

From `~/dev/projects/additional/escher-overseer/relays/select-popup-founder-2026-10-31.md`. All ten are option A.

| Fork | His word |
|---|---|
| F1 | «Клик, Enter, Space (Рекомендую)» |
| F2 | «Подсветка, затем выбор (Рекомендую)» |
| F3 | «Остаётся на select (Рекомендую)» |
| F4 | «Закрывает и срабатывает (Рекомендую)» |
| F5 | «Без анимации (Рекомендую)» |
| F6 | «Как у select (Рекомендую)» |
| F7 | «Рисовать заголовком (Рекомендую)» |
| F8 | «Читать на multiple (Рекомендую)» |
| F9 | «По-прежнему covered (Рекомендую)» |
| F10 | «Строки без overlap (Рекомендую)» |

The operator's word: no new `expanded` node key, and the other leans stand.
