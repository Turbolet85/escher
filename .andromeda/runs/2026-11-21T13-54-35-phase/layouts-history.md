@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list, the engine's select list in every document) — added the open popup list for a closed select in every document: opened by click/Enter/Space, painted by `paint_scene` with no row a layout box, up to twenty rows scrolling by key and wheel, group `label` a painted header, Enter/Space/Tab/click choosing, Escape/outside click/page scroll closing, focus staying on the select; while open each row reads its own `bounds` in snapshot and diff, after a close the rows read back at the select's client rect (row decision/open-list-rows-own-bounds). The forks were the founder's (F1–F4, F7). This chunk turns a sized single select away from that popup (no popup opens, rows real boxes), so the popup's gate and reads must stay as they are for an unsized single select. Changing any of them is out of scope, and a reading that departs from them is a fork.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — added the exceptions now on decision/scroll-into-view-moves-every-holding-scroller and decision/visible-region-through-clipping-boxes: a row or group of an open select list is seen through the list's scrollport alone, and `scroll_into_view` on a row writes the list's `scroll_y` at once. The entry records what the code does, with no ruling behind it. A sized single select's list-box rows are in-flow boxes like a `multiple` select's, so they should fall under the general rule (clipping boxes, scroll chain) and not under this popup-only exception. The exception's wording ("open select list") must not be stretched to cover them.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_select_box`) — `overlaps` skips entries with `Entry.in_select_box` (a closed select's option or an open list's row/group). `closed_select_of` answers `None` for an open list, reads the element and never the role, and a `multiple` select's rows are linted as usual. The founder's F10. A sized single select must be treated like a `multiple` list box here (rows linted, not skipped as a closed select's options), or `closed_select_of`/`in_select_box` will misclassify it. This bears on decision/stand-lints-clean-at-boot.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading, a closed select's options skipped) — moved `stand_lint` `at_boot` from `[0, 1, 0, 0]` to `[0, 0, 0, 0]` in both layout modes by skipping a closed select's options (`Entry.in_closed_select`, later retired for `in_select_box`). The founder's F9. The snapshot still gives each closed option its select's client rect. This is the source of the flight booker's clean boot lint and of the "multiple rows linted as usual" split. The chunk's change to which selects count as closed must not reintroduce the `flight-type-one-way` × `flight-type-return` overlap.

@history_item n=5 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — replaced the flight booker's toggle buttons with the mode select `flight-type` and options `flight-type-one-way` · `flight-type-return`, chosen by `selected` (rows contract/stand-element-ids/flight-type*). It recorded the boot snapshot giving each option the select's client rect and served six tasks, the flight booker among them (contract/escher-session-served-tasks/flight-booker). This is the stand's only select: it is unsized, so its boot reading (decision/flight-type-select-boot-reading) must not move, and any sized-select check needs a fixture outside the stand markup.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines) — the flight booker's mode-region bullet: select styled 15px weight 400, and its two options show the chosen one alone "by the engine's select rule". This is the closed-box rendering that stays for an unsized single select. A list-box face for a sized one is a new rendering path beside it and must not alter this one.

@history_item n=7 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — kept the stand's select row as written (`#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, `#3a5ce5` focus border) while moving the stand value rows in. These are the stand select's own CSS values. A UA stylesheet look for a list box must not override the stand's declared select styling, and the stand select stays unsized.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — origin of decision/scroll-into-view-moves-every-holding-scroller and decision/visible-region-through-clipping-boxes: an into-view scroll moves every scrolling box holding its target, then the viewport, and `visible_region` narrows through clipping boxes. The founder ratified it for every document (2026-10-07), so this is an engine-wide rule. A sized single select's scrolling rows are the new case this rule covers. Keys moving the choice should bring the row in through it, as `crud-list` does, and departing from it is the founder's call.

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — what a scrolled in-flow list reads: `crud-list` keeps the same bounds unnamed in a scroll's diff, and a `click` naming the list lands on the row at its centre once scrolled. These bounds and hit fixes are what a list box's scrolled rows will be read by (snapshot, diff, click on a row). They are the precedent for real-box rows inside a scrolling select.

@history_item n=10 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — the CRUD list (a `role="listbox"` in-flow list) took focus and keys: ArrowDown/ArrowUp/Home/End move the selection, an end key changes nothing, the tree's focus stays the list and never the row (no `aria-activedescendant`). This is the stand's precedent for a list box's key model and its focus/current-row gap, which a sized single select's keys moving the choice will mirror at engine level.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — stand states set by attributes alone: `crud-list` `role="listbox"`, rows `role="option"` with `aria-selected`, adding no element, class, style, key, id or order (row decision/stand-a11y-by-attributes-alone). The chosen-row marking a list box reads (`selected`/`aria-selected`, ListBox vs ComboBox role) must agree with this. Stand markup is attribute-only, so a sized-select fixture must not add structure to the stand tasks.

@history_item n=12 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs named by attributes alone (`for`, `aria-label`), no element, class, style or order changed. This is the first half of decision/stand-a11y-by-attributes-alone, the constraint on any stand-side touch this chunk makes.

@history_item n=13 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by `aria-label` alone. The current-option gap stays stated (no `aria-activedescendant` read). That gap carries over to a sized single select's list box in the accessibility tree.

@history_item n=14 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author ids as attributes alone, with no class, style, wrapper or order and no CSS selecting by them. This is the origin of decision/stand-ids-as-attributes-alone, which binds any id this chunk would add to the stand.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed the three non-lean tasks so `unkeyed_actionable()` reads 0 on all seven tasks (decision/every-stand-control-keyed). If list-box rows of a sized select become actionable boxes, they must not read unkeyed on any stand task.

@history_item n=16 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand skips Home and mounts one task in TaskShell at the pinned 800 × 600, scale 1.0, Light, DejaVu Sans viewport (decision/headless-stand-pinned-viewport). A list box `size` × 1.2em tall is measured against this viewport and font, in both layout modes.

@history_item n=17 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host is a further consumer of the TaskShell mount, and a held instance reads as a fresh boot with the mount and pinned viewport holding after commands (decision/held-instance-reads-as-fresh-boot). Sized-select readings through the session must hold this.

@history_item n=18 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads 0 lint findings at boot in both layout modes, and the gated part exposes this as `escher-session lint` (contract/escher-session-lint/answer). This is the base of decision/stand-lints-clean-at-boot. A list box's rows (clipping, overlap, off-screen) are new lint inputs.

@history_item n=19 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native) — `~ decision/stand-lints-clean-at-boot` now names the order page alongside the counter, flight booker, timer, CRUD and load page. That is the current list of pages this chunk must keep at zero findings at boot.

@history_item n=20 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-layout/answer` (placing words, bounds, width/height), and `~ decision/no-caller-input-echoed` gained the layout answer's id exception. A sized single select's in-flow list box changes what `layout` reports for the select and its rows. Per scope, a reading differing from a `multiple` list box's is a fork, and no new field or word is allowed.

@history_item n=21 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — ratified by the founder's own word the delegate confirm behind 2026-11-17-layout-on-the-carriers, which added contract/escher-session-layout/answer and the no-caller-input-echoed exception. Those rows stand as ratified and this chunk must not widen them.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added `actions` (contract/escher-session-actions/answer): ids, owner, keyed and verbs with cause, never a value. A sized single select stops offering the popup's open and starts offering row choice, so its actions answer changes within this closed form, with no new verb or cause word unless the founder rules.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role) — `coverage <app> <check>...` replays kept checks on fresh boots and answers `{"unexercised","checks","held"}` (contract/escher-session-coverage/answer). Ratified at 2026-11-15T05-21-42-wrap. Kept checks replayed against a select whose rendering changes must still replay, and the role must not grow a new field.

@history_item n=24 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot` with `--id` cropping to visible bounds, the `off-screen` refusal for a shot of no pixel (contract/escher-session-refusals/off-screen, contract/escher-session-screenshot/answer), and the MCP image block (contract/escher-session-mcp/screenshot-result). A list box's scrolled-out rows crop by visible region, so rows outside the box's scrollport must read off-screen consistently with this.

@history_item n=25 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part`: `{"text"}` whole or `{"text","part","parts"}` in parts (contract/escher-session-snapshot/answer). The list-box rows' snapshot reading rides this unchanged form.

@history_item n=26 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role and scoped "no ending writes back anything the caller typed" to the commands (decision/no-caller-input-echoed). The carriers the sized select's readings cross are already ratified, and the scope forbids any new carrier crossing.

@history_item n=27 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` keeps an owner-only replayable check (decision/session-recording-replayable-check), ratified by the founder (F1 A, F2 B). Recorded checks touching a select replay against the engine this chunk changes.

@history_item n=28 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check keeps each answer, masked, with its digest, and `Check::replay` reads it back (decision/session-recording-replayable-check). A replayed check whose recorded answer covers a select's reading would diverge if a sized single select's reading changed under it.

@history_item n=29 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master turned into rows, each verified against the code. This is the origin of every picked row's handle and current wording. Amendments for this chunk name rows by handle (`+`/`~`/`−`), e.g. `~ decision/open-list-rows-own-bounds` or the scroll/visible-region exceptions, if the list-box case changes their wording.
