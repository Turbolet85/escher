@history_item n=1 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the engine's changed set truthful and the tree refresh read it; the diff became one more reader of the snapshot model, no second role or name mapping. Matters: any highlight or Cells selected reading added to the tree/snapshot reaches the tree only if its move marks the changed set, and the diff reads it only through the model.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set; a freshly booted document holds a non-empty set, so a check reading the set as "what one step changed" drains first (`decision/harness-settle-leaves-changed-set`). Trap for any new check that a highlight move or a Cells selection marks the set.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand checks' expected roles and names stated once in `tests/blitz-tests/tests/common/mod.rs` tables, never a second mapping (`decision/role-name-tables-stated-once`). A Cells grid role or a selected cell reading extends those tables, not a local copy.

@history_item n=4 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid) — the builder writes `selected` from `aria-selected` on every element, trimmed and lower-cased, and the snapshot takes `selected` from the tree, never the element (`decision/aria-states-on-every-element`, `decision/aria-state-lower-case-closed-values`, `decision/snapshot-reads-the-tree`); ratified by the founder. The path a Cells selected cell reading would take.

@history_item n=5 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the stand states a listbox, two toggle buttons and two invalid inputs) — CRUD states `role="listbox"`/`role="option"` with `aria-selected` by attribute alone, no element, key, id or Tab order moving. The precedent the scope names for Cells' selected cell (debt 5): selected stated by attribute, no focusability change.

@history_item n=6 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the snapshot text crossing to MCP ratified by the founder (2026-10-10); MCP results read the snapshot model alone. The founder's 2026-10-10 field rule the scope invokes for debt 3: a closed-value field inside a ratified carrier is no widening, a new key or open value still is.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — the node keys' help states each reading and `DiffNode::differs` names readings in node-key order, closed words only. A new highlight or selected-cell reading must be stated in the help and named by `differs`.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a screenshot is one more reader of the model's bounds and changes nothing: no focus moves, `has_changes` false, next diff empty (`decision/screenshot-changes-nothing`). Today the highlighted row shows only in a shot; any fix to debt 3 must keep the shot inert.

@history_item n=9 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the CRUD list's keys) — the CRUD list became a Tab stop whose arrows move the selection, rows stay unfocusable options, tree focus on the list, no `aria-activedescendant`. The listbox-with-unfocusable-options pattern a select highlight reading or a Cells grid selection would mirror.

@history_item n=10 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` (WCAG 2.x, 4.5 / 3.0 large, composited to the first opaque layer, `None` under opacity, filter, background image) and `stand_contrast`'s table (`decision/wcag-contrast-thresholds`, `decision/text-contrast-required-ratio`). The one reader a highlighted-row or chosen-row contrast fix must go through.

@history_item n=11 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met, three contrast tables) — stand accent darkened (`decision/stand-accent-colour`), `TEXT_ROWS` and `STATE_ROWS` read by one shared reader, SC 1.4.3 asserted on every measured non-exempt row (`decision/stand-contrast-asserted`). Any new highlighted/chosen-row contrast row joins these tables.

@history_item n=12 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named, three gaps re-homed) — the missing `aria-activedescendant` (SC 4.1.2 work outside tier 0) re-homed to a residual by the founder's F2 «Оставить записанным». Bears on debt 3: a highlight reading via active descendant is outside the tier-0 reach unless the founder widens it.

@history_item n=13 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§2–§4, §7: the toggle per role and the snapshot's sources) — the unread `aria-activedescendant` kept and pinned by `accessibility_active_descendant_unread` (`decision/active-descendant-unread`, `decision/active-descendant-kept-unread`), ratified 2026-10-18. A debt-3 fix reading active descendant would flip that pin.

@history_item n=14 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled`) — disabled narrowed to form controls by presence, `inert` read in focus order (`decision/disabled-by-presence`, `decision/focusable-elements`). Gates any new select/Cells key or focus path.

@history_item n=15 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's six builder reads kept, `aria-expanded` among them, with one read per ARIA attribute (`decision/one-read-per-aria-attribute`). A new select or grid reading must not add a second read of an attribute.

@history_item n=16 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — `:focus-within` matches, `:focus-visible` never does (`decision/focus-visible-never-matches`). Cells' focus outline is drawn with `:focus` rules; a selected-cell cue must too.

@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the lint's `contrast` reads through `text_contrast` with the stand's exempt rule, no second formula or threshold (`decision/lint-contrast-single-formula`, `decision/lint-contrast-shares-one-reader`); stand at boot reads no contrast finding. Debt 1 must extend the lint through this one reader.

@history_item n=18 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — a `contrast` finding is `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only, ratified as the `lint` verb on every carrier (`contract/lint-contrast-finding/*`, `contract/visual-lint-finding/content`). A highlighted-row finding stays in this shape or is a widening.

@history_item n=19 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a select's changed choice marks the changed set; a select's option excepted from `aria-selected` (its `selected` is its chosen state) and from `display: none`; select keyboard, typeahead and `input`/`change` per change (`contract/accessibility-tree-states/select-option`, `contract/accessibility-tree-hidden-content/display-none`). Base of every select reading this chunk touches.

@history_item n=20 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `select_interaction` and three a11y witnesses run in `stand-repeat` by its named `--test` list. A new select check must join the list to repeat.

@history_item n=21 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files, `select_interaction` and `stand_act_select` among them, on the founder's word. New tests in those files move the leg's tally, which masters record.

@history_item n=22 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3) and named limits, edited by hand (`decision/accessibility-reach-tier-0`, `decision/a11y-tier-0`). Each debt fixed retires a named limit there by hand; each kept stays.

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select) — `select#flight-type` joined the stand: `ComboBox` named `Flight type`, options `ListBoxOption`, chosen one `selected`; its `#1a1a1a` on `#ffffff` pair 17.40 in control and state rows. The stand select every both-modes proof of debts 1–3 runs on.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the capability statement's named limits) — limits now include a `multiple` select's `size` unread; `size` sent to the popup's chunk on the founder's word. History of debt 4.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree, the snapshot and the form values) — `set_multiselectable`, a multiple's rows read their own client rects, value its first chosen option (`contract/accessibility-tree/multiple-select`, `contract/snapshot/multiple-select-rows-and-value`, `contract/snapshot/multiple-select-row-bounds`, `contract/form-event-values/multiple-select`). Debt 4 (single select with `size` as a list box) would take these readings.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — `optgroup` → `Role::Group` named by `label`; options in tree order; a disabled group's rows disabled by one rule `option_disabled` (`contract/accessibility-tree/optgroup`, `decision/disabled-optgroup-one-rule`). Any highlight reading or size-list must keep groups and this rule.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys and row clicks) — a multiple takes its own keys and row clicks apart from the single select (`contract/multiple-select/keys-and-row-clicks`); `select_interaction` 18 tests. A single select with `size` > 1 (debt 4) must choose which key/click model it takes.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and chosen-row pair) — a multiple's chosen rows `#000000` on `#c8c8c8`, Chromium's inactive pair, hardcoded as the engine holds no tokens, 12.56:1 measured by `select_interaction` (`decision/multiple-select-chosen-row-pair`). The pair the working entry ties to debt 1; same values as the open list's highlight constants.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard) — `cells-grid` named `Cells` by `aria_label` with no role, one Tab stop, arrows move `current`, Enter/Tab editor rules, `:focus` outline on the current cell (`contract/cells-grid/*`); role and cell semantics kept out by residual `res-wvatzhpe`. Debt 5's base and its residual.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows) — Cells' stated `CELLS_TAB_ORDER`, keyboard rows and `cells_meets_its_ratios` (`decision/stated-tab-order-walked`). A Cells role or selected change must keep its walked order and ratios.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — tree focus only on a built node, else `u64::MAX` (`contract/accessibility-tree/focus`). A highlight reading must not move tree focus off the select.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — the lint leaves a closed select's options out of `overlap`, reading the element through `closed_select_of`, never the role. Precedent for how the lint treats select rows.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity) — the list is an engine overlay on every document, remote pages included, first hit and painted after the root (`decision/single-select-engine-overlay-list`); AT reading not measured. Every debt-1–3 fix is an engine change for every document.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — `size` read on a `multiple` select alone on the founder's F8; a single select's `size` stays unread, a named limit. Debt 4's origin.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a single select's list opening or closing marks the set; a highlight or wheel move inside the list marks nothing (`decision/changed-set-marks`). Trap for debt 3: a highlight reading in the tree would not refresh unless the highlight move marks the set.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — a single select is a `ComboBox` reading no `aria-expanded`, writing `expanded` from its list's open state, a closed bool of engine state inside the states class, no widening (`decision/single-select-expanded-from-list`, `contract/aria-property-reads/aria-expanded`, `contract/accessibility-tree/single-select`, `contract/accessibility-tree/select-expanded`). The precedent for a closed-value highlight reading without widening.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group's label painted as a header) — header heavier by a synthesized bold on every build, chooses nothing (`contract/select-open-list/optgroup-header`). A larger-font row-height fix (debt 2) must keep headers' height and weight.

@history_item n=38 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — leg 14 files · 114 tests, `stand-repeat` 17 words. New tests in leg files move these recorded counts.

@history_item n=39 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup`, `select_popup_painted`, `stand_act_popup` in `test` and `stand-repeat`, not in the a11y leg; list coverage headless, windowed list read and not run (`decision/select-list-coverage-headless`). Where debt 1–3 proofs land.

@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list) — Enter / Space open on the chosen option, nothing fires; disabled, inert, hidden or empty select opens nothing (`contract/select-closed-list/*`). A single select with `size` > 1 shown as a list box (debt 4) changes what opens.

@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's keys, choice and close) — move keys carry the highlight over enabled rows choosing nothing; Enter/Space/Tab/click choose through one write path, one `input` then one `change`; Escape and outside closes, a close clearing typed text (`contract/select-open-list/*`, `decision/select-list-choice-one-write-path`, `decision/tab-chooses-and-closes-select-list`). The highlight debt 3 would read.

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a click, the pointer and focus on an open list) — focus stays on the select while open; a pointer over an enabled row moves the highlight; a label's click focuses and opens nothing (`decision/focus-stays-on-select-with-list-open`, `decision/label-click-focuses-select-only`). A highlight reading must hold tree focus on the select.

@history_item n=43 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a long list and a sized `multiple` select) — rows take one height, the select's font on one line, so a larger authored font clips, a stated limit (`decision/select-list-rows-one-height`); `MAX_ROWS` 20; on a multiple with valid `size` a key scrolls the row into view via `carries_valid_size`. Debts 2 and 4.

@history_item n=44 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `type` focuses any select with no click and closes any list (`decision/driver-type-focuses-select-no-click`). A size-list single select must keep this.

@history_item n=45 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — no `expanded` node key (`NODE_FIELDS` then 13), the highlighted row in no snapshot and no tree, a stated limit; an open row's own `bounds` ruled inside the crossing by the 2026-10-10 field rule; AT reading `unrunnable-here` (`contract/snapshot/select-list-rows`, `contract/snapshot/open-list-highlighted-row`, `contract/snapshot/open-list-row-bounds`, `absence/snapshot-list-open-state`). Debt 3's stated limit and its crossing rule.

@history_item n=46 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how an open list's labels leave) — option and group labels leave only as pixels and the snapshot's `name`, no log site, ratified (`decision/option-labels-leave-only-as-pixels`, `contract/driver-screenshot/row-shot`). A highlight reading must not carry a label anywhere new.

@history_item n=47 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`STATE_ROWS`' list-open rows) — `STATE_ROWS` 20, list-open rows `flight-type-one-way` / `flight-type-return` at 17.40. Where a highlighted-row contrast row would join.

@history_item n=48 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list) — `Entry.in_select_box`; the lint's `contrast` reads a row's plain pair even when highlighted, so a highlighted row's contrast is unread by the lint, a stated limit (`decision/highlighted-row-contrast-unread`). Debt 1's origin.

@history_item n=49 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's colours and its measured pair) — `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT`, `BORDER` engine constants, not tokens; `select_popup_painted` reads highlight and disabled rows against them (`decision/select-list-colours-engine-constants`). Debt 1's measured side.

@history_item n=50 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list has no motion) — a list opens and closes with no motion, no `advance` needed (`absence/select-list-motion`). A fix must keep it.

@history_item n=51 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — the list paints whole past an `opacity` ancestor and clipping boxes; `:open` does not match it. A row-height or highlight paint change keeps these.

@history_item n=52 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (select_popup_painted also runs as embolden) — `select_popup_painted` also runs in the `embolden` step of the `fast` leg. A larger-font paint proof there runs on both builds.

@history_item n=53 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — reader and lint read `None` past a non-normal `mix-blend-mode` (`decision/contrast-unread-past-non-colour-paint`). A lint over highlighted rows inherits this.

@history_item n=54 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blending paints with no frame test) — the open list paints last, past blend layers, read and not measured. Bears on any list paint change.

@history_item n=55 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the «reacts to a click» reading measured by link 2) — on Cells 676 cells react to `click` and `cells-grid` to `keydown`, none stating a role. Debt 5's measured Cells base.

@history_item n=56 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed to residual rows, `res-wvatzhpe` (Cells' role and cell semantics) among them. Debt 5's owner on the masters.

@history_item n=57 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses re-said) — `res-wvatzhpe` clauses read «re-carried at 0.3.0 (target: next)». A ruling on debt 5 must re-say these clauses.

@history_item n=58 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — a new snapshot field added: `NodeState.clickable`, text token, JSON field, `Reading::Clickable`; `NODE_FIELDS` 13 → 14, `Reading::ALL` 12. The template, and the counts to move, if debt 3 or 5 adds a reading.

@history_item n=59 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — `actionable_nodes` lists graph nodes, owner set for an option's select (`contract/actionable-nodes/listing`). A size-list single select's rows change this listing.

@history_item n=60 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — `actions` writes ids, words and booleans only; an open list stays open; flight-type options read `covered` until the list opens (`contract/actions-answer/fields`). A debt 3 or 4 fix must keep these readings.

@history_item n=61 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks`: a scroll that closes a list marks its select; any other scroll does not. The row any highlight-marking change extends.

@history_item n=62 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy) — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed in a redraw after style resolve only when the changed set is non-empty, through the document trait's method; a poll pushes none. A tree highlight reading refreshes only via this path.

@history_item n=63 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (4. ARIA Patterns & Roles; 6. Visual Design Verification) — `~ decision/hidden-content-excluded`, `~ contract/accessibility-tree/hidden-and-inert-nodes`, `~ decision/hidden-content-excluded-from-a11y-tree`: `visibility: collapse` excludes as `hidden` does, a select's option still excepted from `display: none`. A size-list single select's options must keep the exception.
