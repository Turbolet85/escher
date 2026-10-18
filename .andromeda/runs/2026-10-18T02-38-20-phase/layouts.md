# layouts extract

## Relevance
partial — no surface is created. The chunk touches the desktop-native stand's TaskShell header (debt 9's `task-header-spacer`), the CRUD list's focus model (debt 3), the focusability that sets Tab stops (debts 1, 7), the recorded scrolling-box readings (debts 4, 5) and the cli surface's `help` text (debts 5, 9, wherever `covered`'s meaning moves).

## Constraints
- layout-templates §Surface: desktop-native › Primary screens requires that every stand markup change states a role, state or name through attributes alone, "adding no element, class, style, wrapper, id or order", and that no task CSS selects by the author ids. A debt-9 change to `task-header-spacer`, or any stand markup a fix needs (debts 2, 3), is held to that rule.
- layout-templates §Surface: desktop-native › Primary screens records the TaskShell as a header (back button, title, spacer) over a scrolling body, mounted as `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport. If debt 9 is "fixed" by changing the box, that structure and the viewport must hold. Whether a change to the spacer's height moves `#task-body`'s reading is research's question.
- layout-templates §Surface: desktop-native › Primary screens records CRUD's Tab sequence as `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` (six stops). It also records that "the tree's focus is the list, never the row, no `aria-activedescendant` being read". A debt-3 fix changes that stated reading but must keep the six stops: the list holds focus, and its rows take no Tab stop.
- layout-templates §Surface: desktop-native › Primary screens records that CRUD's ArrowDown · ArrowUp · Home · End select a row and fill `crud-name` / `crud-surname` as a row's click does. A debt-3 fix that follows `aria-activedescendant` has to keep that key model. Whether the list sets `aria-activedescendant` today is research's question.
- layout-templates §Surface: desktop-native › IA notes records the nested-scroller readings: `crud-list` (24, 112.796875, 369 × 419.203125) at boot and after its content is scrolled, and `task-body` (0, 46, 800 × 554). A fix to `Node::absolute_position` (debt 4) or to the hit's stop at `contain: paint` (debt 5) must leave these readings unchanged, as it must the driver `click` on the list landing on the row at its centre at 12 and at 14 Creates.
- layout-templates §Surface: desktop-native › Primary screens records Home's seventh card, Cells, at 615 to 684, below the 600-high `#home` scrolling box: it shows, and a hit lands on it, only once it is scrolled into view. Any change to a hit or position reader (debts 4, 5, 8) must keep that reading.
- layout-templates §Surface: cli › Primary screens fixes the shape of `escher-session help`: one uncoloured line of JSON with seven keys (`verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`), every text a fixed string of the schema's tables. If `covered`'s meaning text changes (debts 5, 9), that shape must stay, along with the MCP tool descriptions built as "help + `. Returns: ` + result fields".

## Patterns to follow
- A minimal fixture beside the stand where the stand lacks the case: the in-file fixture taller than the viewport for a viewport scroll, per layout-templates §Surface: desktop-native › IA notes. This applies to debts 2 (no stand task states `aria-checked`), 5 (no stand file sets `contain`), 7 and 8, whose cases the stand markup does not hold.
- Attribute-only state on stand controls: `aria-pressed` on `flight-one-way` / `flight-return`, `aria-invalid` on the date inputs, `role="listbox"` + `aria-label="People"` on `crud-list`, `role="option"` + `aria-selected` on rows. Each was added with no element, class, style, key, id or order moving, per layout-templates §Surface: desktop-native › Primary screens. Any `aria-activedescendant` or `aria-checked` statement added to the stand follows the same form.
- Every stand reading is pinned to the headless mount through `task_in_shell` at the pinned viewport with the bundled DejaVu Sans, in both layout modes, per layout-templates §Surface: desktop-native › Primary screens. A rewritten pin reads the same mount.

## Anti-patterns to avoid
- Adding an element, wrapper, class, style or order to a stand task, or a CSS selector by author id, to carry a debt's fix. layout-templates §Surface: desktop-native › Primary screens bans each.
- Giving CRUD rows their own Tab stops, or moving focus off `crud-list`, as a way of exposing the current option. That would break the six-stop sequence and the list-holds-focus model recorded in layout-templates §Surface: desktop-native › Primary screens.

## Contract bindings
- Focus order ↔ a11y (SC 2.4.3): debts 1 (`inert`) and 7 (`disabled` narrowed) change focusability. Debt 3 changes what the tree reports as focused. The CRUD Tab sequence in layout-templates §Surface: desktop-native › Primary screens is the same table as `stand_keyboard`'s `common::tab_order` / `common::ENABLED_TAB_ORDER` (CLAUDE.md invariant), so a focus-order change moves both together. Whether any stand element carries `inert` or a `disabled` attribute outside form controls is research's question.
- The cli surface ↔ the driver schema: `covered`'s meaning and remedy text (debts 5, 9) is fixed `'static` table text. It appears in the `help` line and the MCP tool listing described in layout-templates §Surface: cli › Primary screens.

## Acceptance criteria contributions
- (layouts) CRUD's Tab sequence reads `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`, six stops, after any debt-1, debt-3 or debt-7 fix. If debt 3 is fixed, the recorded "the tree's focus is the list, never the row" reading is rewritten to the fixed reading (per layout-templates §Surface: desktop-native › Primary screens).
- (layouts) If debt 9 changes `task-header-spacer`, the TaskShell still mounts `#task-header > #back-btn + #task-title` above `#task-body` at 800 × 600, with no element, class, style, wrapper or order added. If debt 9 is answered by words, no stand markup moves (per layout-templates §Surface: desktop-native › Primary screens).
- (layouts) After a debt-4, debt-5 or debt-8 fix, the following still hold, in both layout modes (per layout-templates §Surface: desktop-native › IA notes):
  - `crud-list` reads (24, 112.796875, 369 × 419.203125) at boot and after its content is scrolled;
  - `task-body` reads (0, 46, 800 × 554);
  - a driver `click` naming the list lands on the row at its centre at 12 and at 14 Creates.
- (layouts) `escher-session help` still answers one uncoloured JSON line with exactly the seven keys after any change to `covered`'s text (per layout-templates §Surface: cli › Primary screens).
