# layouts extract

## Relevance
partial: the chunk adds and moves no surface, region or focusable element. Its blitz-dom splits must leave the stand's measured layout, focus order and hit readings as they are. Its README may only quote the `escher-session` command grammar as it reads.

## Constraints
- Splitting the button-focus and keyboard-activation arms out of `handle_click` / `handle_key_or_input_event` must leave each lean task's Tab sequence as recorded, including CRUD's six stops `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. Whether the extracted arms keep the exact gate order is research's question (per layout-templates §Surface: desktop-native / Primary screens).
- `crud-list`'s app-level keyboard region must still receive its keys after `handle_key_or_input_event` is split. Those keys are ArrowDown · ArrowUp · Home · End on the focused list, which select a row and fill `crud-name` / `crud-surname`. The tree's focus stays on the list and never moves to the row. Whether the range-input key arms gate only on `input type="range"` and stay away from these keys is research's question (per layout-templates §Surface: desktop-native / Primary screens).
- Both range inputs must still be driven through the split `handle_pointerdown` / `handle_click` and `parse_float` exactly as before: the Timer's `timer-duration` and the Circle Drawer dialog's `circle-diameter` (present only while the dialog is open). A pointerdown's fraction is read against the input's content box (per layout-templates §Surface: desktop-native / Primary screens).
- The headless stand's mount and viewport must hold after the refactor: `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`, pinned at 800 × 600, scale 1.0, Light, in both layout modes (per layout-templates §Surface: desktop-native / Primary screens).
- The scrolling-box readings that driver hits rest on must not move. `crud-list` reads (24, 112.796875, 369 × 419.203125) at boot and after its content scrolls. `task-body` reads (0, 46, 800 × 554). A driver `click` that names the list lands on the row at its centre at 12 and at 14 Creates (per layout-templates §Surface: desktop-native / IA notes).
- Any `escher-session` command the README's Try-it or prose shows must match the CLI surface's grammar:
  - `escher-session <verb> [arguments] --session <dir>`;
  - `start <counter|flight-booker|timer|crud>`;
  - `help` with no session needed;
  - `escher-session mcp` alone.

  The binary and verb names stay lowercase (per layout-templates §Surface: cli / Primary screens).
- The README's description of `scripts/agent-run.sh` / `scripts/cold-agent.sh`, if it has one, must keep their usage shapes: verbs `boot · run · status · cleanup · logs` and `run · status · cleanup · logs`; cold-agent's one task is `stand` (per layout-templates §Surface: cli / Primary screens).

## Patterns to follow
- The stand's record shape for a change that leaves layout alone: "adding no element, class, style, wrapper, id or order", with the TaskShell mount and pinned viewport holding in both layout modes. The chunk's report states its no-layout-change in this same form (per layout-templates §Surface: desktop-native / Primary screens).
- Engine changes are read against the stand's measured figures, not described in prose. The IA notes cite each `crud-list` / `task-body` bounds reading and the click landing to the chunk that measured it. A behaviour-preserving split is checked against those same readings (per layout-templates §Surface: desktop-native / IA notes).
- Stable element ids are author `id`s that no CSS selects by. The refactor reaches them only through the existing checks and adds or renames none (per layout-templates §Surface: desktop-native / Primary screens).

## Anti-patterns to avoid
- Any edit under `examples/seven_guis/src/` (markup, id, class, style, wrapper, `tabindex` or `onkeydown`) made to make a split pass. The stand's layout and focus order are recorded facts, not something to adjust (per layout-templates §Surface: desktop-native / Primary screens).
- A focus-order change of any kind, such as a button click no longer focusing or the Enter/Space activation gates reordering. It would move the recorded Tab sequences, and the scope makes it the founder's fork rather than a refactor step (per layout-templates §Surface: desktop-native / Primary screens).
- Re-spelling a binary, verb or path in the README to «Escher». `escher-session`, its verbs and the task words stay as the CLI surface spells them (per layout-templates §Surface: cli / Primary screens).

## Contract bindings
- Focus order ↔ a11y SC 2.4.3. The recorded Tab sequences (CRUD's six stops) are the ones `stand_keyboard` asserts through `common::tab_order` / `common::ENABLED_TAB_ORDER`. A split that moves focus moves those tables (layout-templates §Surface: desktop-native / Primary screens ↔ a11y-plan focus order).
- Scrolling-box bounds and hit ↔ the driver's refusal detection (`off-screen` / `covered`) and the `stand_act_scroll` check. The IA notes' `crud-list` readings are the geometry those refusals read (layout-templates §Surface: desktop-native / IA notes ↔ test-plan §3).
- README command lines ↔ the CLI grammar ↔ test-plan §3 Session lifecycle, which the CLI surface names as the contract `escher-session` serves (layout-templates §Surface: cli / Primary screens).

## Acceptance criteria contributions
- (layouts) After the splits, CRUD's Tab sequence still reads `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`, and the list's ArrowDown · ArrowUp · Home · End still move the selection and fill the fields, in both layout modes (per layout-templates §Surface: desktop-native / Primary screens).
- (layouts) `git diff` over `examples/seven_guis/src/` is empty for this chunk: no element, class, style, wrapper, id or order moved (per layout-templates §Surface: desktop-native / Primary screens).
- (layouts) `crud-list` still reads (24, 112.796875, 369 × 419.203125) at boot and `task-body` (0, 46, 800 × 554), and a driver `click` naming the list lands on the centre row at 12 and 14 Creates (per layout-templates §Surface: desktop-native / IA notes).
- (layouts) Every `escher-session` line in `README.md` fits the CLI grammar (`<verb> [arguments] --session <dir>`, `start <counter|flight-booker|timer|crud>`, `help`, `mcp`), with the binary and verbs lowercase (per layout-templates §Surface: cli / Primary screens).
