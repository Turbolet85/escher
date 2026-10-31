# Engine debts — the founder's forks

Chunk `2026-10-31-engine-debts`, entry «Engine debts» (`v030-04`). Written at P4 in the away form on the operator's word (inputs#I1): every fork, with its dependences, in one file, **no provisional answer given**. The plan is not written and nothing is built until his word returns.

Each fork below changes behaviour for every document unless it says otherwise. By the playbook's engine-change rule his own answer is the ratification, recorded with his word as he gives it. A delegate's answer would stand PROVISIONAL. Every reading below was measured at `2109f1c2e` in both layout modes unless it says "read from source".

**To answer:** one line per fork, its number and the option's letter, e.g. `F1 A`, or his own words. `F6 A` and `all recommended` are both fine.

## All thirteen at a glance

| # | The question | Recommended | Size |
|---|---|---|---|
| F1 | `visible_region` under a transform | A — fix it | small |
| F2 | A `scale(0)` box answers hits where nothing shows *(found at P3)* | A — fix it | small |
| F3 | `inert` focus fixup | A — clear focus at the mutator's flush | small |
| F4 | The tree names a focus it does not hold *(found at P3)* | A — fall back to the window | very small |
| F5 | Native checkedness in the tree | A — write it on checkable roles | very small |
| F6 | `multiple` select | A — pin as a stated limit | pin small · build large |
| F7 | `optgroup` | A — pin as a stated limit | pin small · build medium |
| F8 | Popup and clicking an option | A — no popup, option stays refused, pin | pin small · popup large |
| F9 | Lint's option overlap | A — exempt a select's options in `overlaps` | small |
| F10 | `change` on checkbox and radio | A — the engine fires it, script's copy removed | small |
| F11 | `change` on text input and range | A — pin as a stated limit | pin small · build medium |
| F12 | Load page's keyboard and contrast rows | A — add them | small |
| F13 | Cells has no keyboard path | A — the grid becomes one Tab stop with keys | medium, stand only |

## What depends on what

- **F9 depends on F8.** F9 A's exemption assumes options have no box of their own. Under F8 C (a painted popup) it must read "closed" only, and F9 is restated with the popup's work. F9 B is incompatible with F8 A and B unless he accepts every closed option reading `zero-size` or `off-screen`.
- **F8 B moves what F9 A leaves.** A driver click on an option changes `covered`'s stated limit and `stand_act_ids`' table; it does not change option bounds, so F9 A still holds.
- **F6 B pulls in F8.** A modelled `multiple` select shows rows, and its rows must take clicks; it needs F8's option click for that select at least. Its options also read real boxes, so F9 A's exemption would not cover them.
- **F7 B touches F9.** The exemption must also read an option under an `optgroup`.
- **F3 and F4 are separate on purpose.** F3 stops an inert node keeping focus. F4 stops the tree naming a focus it does not hold, for every cause, `display: none` and `hidden` included. F3 A alone leaves those; F4 A alone leaves the inert node taking keys. Choosing F3 C makes F4 the only guard.
- **F3 and F11.** Under F3 A the fixup fires no DOM event, so under F11 B or C a dirty text input made inert would lose its `change`.
- **F10 and the script runtime.** The script runtime already synthesises `change` on a checkbox or radio; an engine `change` without its removal fires twice.
- **F1 and F2.** Independent, but F1 A reads a degenerate clip as nothing visible, while F2 is what stops that box taking hits. Taking only F1 leaves a `scale(0)` box with no clip of its own swallowing clicks.
- **F12 and F13 are the ground of Epoch 6's «A11y re-read».** It re-asserts SC 2.1.1 · 2.4.3 · 1.4.3 on the served pages. Under F13 B or C it inherits a red row on Cells.
- **Upstream.** F2's lines sit beside `0980cce5f`'s `hit_inner` hunk, a textual conflict for Epoch 3's sync. Nothing here pre-empts that commit's meaning.

---

## Group 1 — Transforms

### F1 — `visible_region` under a transform

A scrolling box under a transform is read where it stands untransformed, so a driver call on a target that shows can be refused `off-screen`.

**Measured, and it narrows the debt.** The reconnaissance expected pan/zoom to hit this. A plain pan/zoom world (a `transform` on a box of absolutely positioned nodes, under an untransformed `overflow:hidden` viewport) **already reads right** at translate, `translate + scale(2)` and `scale(0.5)`, by harness and by driver `click`. The limit fires only where a box that clips lies under the transform: a node with its own `overflow:auto` body inside a panned or zoomed world. There, `click` is refused `off-screen` and lint reads `off-screen` and `truncated`, while the hit lands on the target. A scale is in reach, as his adaptation asked.

His adaptation's proof, the pan/zoom fixture, is written under every option. It adds no capability.

- **A — Fix it (Recommended).** `visible_region` maps each clipping box's padding box through the transforms, using the mapping the client rect already has. Exact under translate and scale. Under rotation it over-reads, since the region is the bounding box of the rotated clip. A page with no transform reads as today. It moves the act gate, the shot's crop and the lint together, drops `off-screen`'s limit sentence (and `docs/capabilities.md`'s, by hand), and renames the limit's pin. A node graph with scrolling node bodies then works through the driver. Cost: one method, one visibility.
- **B — Pin it as a stated limit.** No engine change. Plain pan/zoom is proven and the inner-scroller reading pinned; the text stays. A scrolling node body stays refused until someone fixes this.
- **C — Keep it as a residual.** No fix and no new proof beyond the fixture.

### F2 — A `scale(0)` box answers hits where nothing shows *(found at P3, not a recorded debt)*

A box under a non-invertible transform (`scale(0)`, `matrix(0,0,0,0,0,0)`) with no clip of its own takes a hit at points far from it: measured at (300, 200) and (590, 390) inside a 600 × 400 clip, though nothing is painted there. The inverse leaves NaN coordinates, and the hit's bounds test reads true on NaN. No panic. Any remote page can do this, and a click at such a point would land on the control that does not show.

- **A — Fix it (Recommended).** A non-invertible transform takes no hit for its subtree, as paint shows nothing there. A few lines in `hit_inner`, with a check of its own. It sits next to `0980cce5f`'s hunk: a textual conflict for Epoch 3.
- **B — Keep it as a residual.** A new `.andromeda/residuals.md` line owns it; no check. The behaviour stands.

---

## Group 2 — Focus

### F3 — `inert` focus fixup

Today, when `inert` lands on a focused node's box: the engine still says it is focused, the snapshot reads nothing focused, and no `blur` runs. **It still takes keys**: Enter clicked the inert button (count 1 → 2) and a focused inert text input took a typed character. Tab leaves it normally.

HTML (inputs#I5) runs the fixup in "update the rendering", and its note says that "will usually fire blur events, and possibly change events"; only the fixup on removal fires none.

- **A — Clear focus at the mutator's flush, for `inert` alone (Recommended).** Focus goes to the viewport through `clear_focus` (focus states, `FOCUS_WITHIN` chain, IME off, redraw, changed set), exactly what removing a focused node does today. **No DOM `blur` or `focusout` fires**, which departs from HTML's note: the mutator holds no route to dispatch events. Consequences: the next Tab starts from the document's first stop, not after the box; `inert_subtree`'s pin is restated; `disabled` and `hidden` are left as they read.
- **B — HTML's fixup at resolve, with `blur`.** Read `is_focussable` in the rendering step and fire `blur`. Closest to HTML, and it also covers a control disabled after taking focus, which **moves a pinned reading** (`a_button_disabled_after_it_took_focus_takes_no_key` says it keeps focus). It needs a new route from resolve to event dispatch, and page script then sees a new event trigger. `hidden` and `display: none` still need `is_focussable` to read them. Larger, and more for Epoch 3's merge.
- **C — Keep the limit.** Pin and text stand; an inert node keeps taking keys until focus moves.

### F4 — The tree names a focus it does not hold *(found at P3, not a recorded debt)*

`build_accessibility_tree` writes `focus` from the focused node with no check that the node was built. So an inert focused node, measured, leaves `focus` naming an absent id; `display: none` and `hidden` go through the same lines. AccessKit's consumer **panics** on that: `"Focused ID {:?} is not in the node list"` (inputs#I6, `src/tree.rs:89-94`). **Read from source, not run:** it needs a platform adapter and an active assistive technology. No stand binary builds an adapter. `rdme` and any embedder on the `blitz` crate's default features do (`blitz-shell`'s default `accessibility`), and the browser does with its optional `accessibility` feature; there, a page's own markup could trigger it.

- **A — Fall back to the window (Recommended).** The builder writes the window's id when the focused node was not built. About one line; a check through `&dyn Document`. No snapshot reading moves. The engine's focus is untouched; F3 deals with that.
- **B — Keep it as a residual.** A new line owns it, its proof waiting on the adapter witness that this version does not reach.

---

## Group 3 — Native checkedness

### F5 — A native checkbox or radio states its checked state in the tree

HTML-AAM (inputs#I5) maps checkedness to `aria-checked`. Today nothing is written. His «Исправить позже (Рекомендую)» (2026-10-18) left it for "a later chunk or the nearest accessibility work". The value is closed, on a crossing already ratified, so this is no widening (the playbook's field-inside-a-ratified-carrier rule).

**Measured from the code:** the snapshot's `checked` reads the element first and `pressed` is filtered off the five checkable roles, so **no snapshot, text or diff reading moves on a native control's own role**. The stand holds no checkbox or radio, so no committed check moves. Only `accessibility_states`' fourth test is restated. Clicks already mark the changed set. Proof stops at the tree handed to the adapter; what an assistive technology reads stays unmeasured. There is no `indeterminate` model, so `mixed` has no source.

- **A — Write it where the role is checkable (Recommended).** `toggled` true or false from the element's state when its role is one of the five. A native control whose `role` attribute names something else stays silent, so `pressed` never begins to read on it. About three lines.
- **B — Write it on every native checkbox and radio.** Closest to HTML-AAM's letter. `<input type=checkbox role=button>` would begin to read `pressed`, a reading that moves; it would be stated, not hidden.
- **C — Keep it as a residual.**

---

## Group 4 — The select model

### F6 — `multiple` select

It lays out as an empty bordered box and takes no key.

- **A — Pin it as a stated limit, with a check of its own (Recommended).** The stand holds none and the 0.4.0 inputs ask for none. The limit stays in the masters and is asserted by a check, not only stated.
- **B — Build it now.** A chosen set, list layout, rows, ctrl and shift selection, form entries, a select value beyond one, `type`'s meaning restated, and a default-sheet change for every document. Its own chunk in all but name, and it needs F8's option click.
- **C — Keep it as a residual, unpinned.**

### F7 — `optgroup`

Options inside an `optgroup` are not listed, so such a select holds no choice and shows nothing. Real pages use this.

- **A — Pin it as a stated limit, with a check of its own (Recommended).** The stand and the 0.4.0 inputs need none. The cost of leaving it is on remote pages only.
- **B — Build it now.** Walk one level in the option list, across `is_select_option`, the builder, `form.rs`, the mutator's selectedness, the snapshot's option parent and the default sheet's child combinator, with a disabled group's rule. About seven files; every document. No new key or verb.
- **C — Keep it as a residual, unpinned.**

### F8 — A painted popup, and clicking an option

No popup is painted; a driver `click` on an option is refused `covered`. Today an agent chooses through `type` or the arrows.

- **A — No popup; the option stays refused; pin it (Recommended).** `covered`'s reading, and Enter and Space opening nothing, get a check of their own. Nothing moves.
- **B — No popup; a driver `click` on an option chooses it.** Through `apply_select_choice`, keeping one write path. Convenient for agents, but a click then does something no pointer can, and `covered`'s text and `stand_act_ids`' table (68 accepted · 9 refused) move. Driver only; no engine change.
- **C — Paint a popup.** A new overlay in paint, hit and keys; a focus and key contract he would state; reduced-motion handling; colours as a new engine default, since the engine holds no tokens; `default.css` for every document. Option bounds then read real boxes when open, restating F9. Needs a route entry of its own: too large for this chunk. *Playbook: Boundary widening* — a validated surface would admit new input and pixels, so the wrap would still halt on it once, as his answer card.

### F9 — Lint reads a select's options as overlapping *(asked with F8)*

Every option is given its select's rect, so `lint` reports an `overlap` in every document; the flight booker reads one at boot.

- **A — Exempt a select's options in `overlaps` (Recommended).** A driver change only. Bounds stay; `stand_lint` becomes `[0, 0, 0, 0]`. It tells real select options from CRUD's `role="option"` rows by the element, not the role. No new field; a finding's form is unchanged.
- **B — Move the option bounds.** The snapshot gives each option a box of its own. It moves every reader: closed options would read `zero-size` or `off-screen`, the shot's crop and the refusal change, and the owner moves from the driver to the snapshot.
- **C — Keep it, pinned as measured.**

### F10 — `change` on checkbox and radio

The engine fires `input` alone, so a Dioxus app's `onchange` never runs. The script runtime already synthesises its own `change`.

- **A — The engine fires it; the script runtime's synthesis is removed (Recommended).** One event for both kinds of app: `change` after `input`, per HTML. Dioxus handlers gain a trigger they do not have today. A radio's `change` fires only on an actual change, which also narrows what script sees today (its synthesis fires on every `input`); a radio's unconditional `input` stays as it reads.
- **B — Pin it as a stated limit.** Dioxus `onchange` stays silent on these; script keeps its own.

### F11 — `change` on text input and range

HTML fires it on commit, not per keystroke, which needs per-input "value at focus" state and a route through blur.

- **A — Pin it as a stated limit, with a check of its own (Recommended).** The stand uses `oninput` throughout (`git grep` over `examples/seven_guis/src`), so nothing waits on it.
- **B — Build it for text input only.** A password's or file input's value must stay masked on every carrier, proven by a typed-sentinel check at `RUST_LOG=trace`. A file input's value holds a host path. Range stays `input` alone, stated.
- **C — Build it for text input and range.** As B, and range's founder-ratified rule of one `input` per change, with no `change`, is restated.

*Playbook, for B and C: Boundary widening* — the security plan calls a further exit for a control's value a boundary question, so the wrap would still halt on it once, as his answer card.

---

## Group 5 — The served pages' rows

### F12 — The load page's rows

**All read green when measured, so no colour or engine move is in play.** Tab `back-btn` → `load-fetch`, wrapping. Enter and Space each start the load. Contrast: `load-fetch` 5.47 · hover 6.83 · active 8.27, `back-btn` 11.09, `task-title` 18.88, `load-done` 15.96, each against 4.5.

- **A — Add the rows (Recommended).** In `stand_keyboard` and `stand_contrast`, beside the tables keyed by `LeanTask`, not inside them. Those two files already run in both legs, so **the `a11y` leg's fourteen-file list, which is his word, does not move**. Closes the OPEN gap in a11y-plan §5 and the load page's half of residual line 40.
- **B — Leave the gap open.** It stays with its residual line for «A11y re-read».

### F13 — Cells has no keyboard path

**Red on SC 2.1.1, measured, and it is the further fork the scope foresaw.** Tab stays on `back-btn`; **0 of 676 cells are focusable**; a cell opens only by pointer, and its editor then holds no focus. Contrast read so far is green: `back-btn` 11.09, `task-title` 18.88, `cell-input` 15.19; an empty cell holds no text; **a filled cell is not yet measured**, and any option below measures it.

- **A — The grid becomes one Tab stop with keys (Recommended).** The shape `crud-list` already has on the stand: one stop, arrows move the current cell, Enter opens the editor focused, Enter commits. A stand change, no engine change. It adds a stand element or attribute and new key handling, so the Tab table, the measured 5 parts and the lint's 490 may move and are re-measured. The current cell stays unstated to assistive technology, as `crud-list`'s row does; residual line 24 stays out.
- **B — Every cell a Tab stop.** 676 stops. Meets the letter of 2.1.1 and is poor to use; moves the screen most.
- **C — Pin the measured reading.** A row asserts Tab reaching `back-btn` alone; the gap stays owned, and «A11y re-read» inherits a red.

---

## Not asked — decided from the record, for his information

- **CARRY 2's three checks are written regardless.** `:open` read right on every path (boot, write, removal, `summary` click and Enter; `details` and `dialog`), so its check is new and moves nothing. A wrong reading would have come here as a fork.
- **The redraw route's check reaches its first hop only.** With a stand-in for winit's proxy provider, `send_event` is read through to its receiver and one wake-up, with no window. `BlitzShellProvider` and `application.rs` need a real window and stay with the windowed boot smoke; the check says so. It lives in blitz-shell's unit tests, so blitz-tests gains no dependency.
- **`pseudo_element_flow_classification` runs in both layout modes.** An edit to an upstream test; its first test, sampled, read alike in both. The additivity note records it, and the both-modes exception's text moves at wrap.
- **CARRY 3's stale `remove_attribute` is a wrap amendment**, never a phase edit.
- **No option above adds a verb, node key, cause, listener, env var or dependency.** A fix's merge surface goes to Epoch 3's «Upstream sync ahead of the UI graph». Nothing is sent upstream.

---

## His answers (2026-10-31, by dialog in the overseer session, relayed by the operator)

- F1 A «Починить (Рекомендую)» · F2 A «Починить (Рекомендую)» · F3 A «Снимать фокус без blur (Рекомендую)» · F4 A «Откат на окно (Рекомендую)» · F5 A «Писать на checkable-ролях (Рекомендую)»
- **F6 B «Построить сейчас» · F7 B «Построить сейчас»**
- **F8: «Давай нарисовать в отдельном чанке»** — option C, in a chunk of its own, not this one. Placed by «Сразу после Engine debts (Рекомендую)»: the last chunk of Epoch 2, before «Upstream sync ahead of the UI graph».
- **The click until that chunk, «Только строки multiple (Рекомендую)»:** a `multiple` select's rows take a click from both the pointer and the driver, having real boxes. An option of a closed single select stays refused `covered`.
- F9 A «Исключить закрытые option (Рекомендую)», restated: `overlaps` skips the options of a closed single select, those under an `optgroup` included. A `multiple` select's rows, with boxes of their own, are linted as usual.
- F10 A «Шлёт движок (Рекомендую)» · F11 A «Закрепить ограничение (Рекомендую)» · F12 A «Добавить строки (Рекомендую)» · F13 A «Одна Tab-остановка (Рекомендую)»
