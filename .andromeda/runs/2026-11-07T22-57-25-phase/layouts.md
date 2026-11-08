@extract_section name=relevance
partial — the chunk creates no surface, region or focusable element; it moves the seven_guis stand's signals onto a registry and adds a snapshot field, so the stand's layout invariants and the cli verb table bind it
@extract_section name=constraints
- Moving the stand's signals onto the escher-side registry must add no element, class, style, wrapper, key, id or order to any served screen; layout-templates §Primary screens (sec-dq2cwdsx) requires every task's author ids to stay its stable element ids, with no CSS selecting by them.
- Each served screen keeps its TaskShell mount (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport, in both layout modes, per layout-templates §Primary screens (sec-dq2cwdsx).
- The Tab sequences stay as recorded (CRUD six stops; the flight booker four, five when return is enabled; `CELLS_TAB_ORDER`; `LOAD_TAB_ORDER`), the focus on `crud-list` and never on a row, per layout-templates §Primary screens (sec-dq2cwdsx).
- `cells-grid` must remain the 677th reacting element beside the 676 data cells, and that count is what `stand_non_lean_tasks` reads; the new «reacts to a click» field has to agree with it, per layout-templates §Primary screens (sec-dq2cwdsx).
- The document skeleton `<html><head></head><body><main id="main"></main></body></html>` and the app's mount into `main` stay unchanged by the graph reader, per layout-templates §IA notes (sec-ge4vhhot).
- Any new verb has to go through the twelve-verb table that `escher-session`'s usage line, `help` / `describe()` and the MCP tool listing are all built from; the listed order and the four exit endings must hold, per layout-templates §Primary screens — cli (sec-vtizvuw4).
@extract_section name=patterns
- Stand changes are made by attribute alone (`id:`, `aria-*`, `tabindex`), with no element, class, style or order moving; a signal-registry migration should follow the same discipline, per layout-templates §Primary screens (sec-dq2cwdsx).
- A held instance reads as a fresh boot, and the TaskShell mount and pinned viewport hold after commands; this is the baseline for checking that a graph read changes nothing, per layout-templates §Primary screens (sec-dq2cwdsx).
- The visual lint baseline (`stand_lint` `at_boot` `[0, 0, 0, 0]`, the load page 0) is the layout regression pin, per layout-templates §IA notes (sec-ge4vhhot).
- The verb table drives the usage line, `help` and `tools/list` together, so the new read's verb appears in all three by construction, per layout-templates §Primary screens — cli (sec-vtizvuw4).
@extract_section name=anti_patterns
- Adding a wrapper or a provider element around a task's root, or any class, style or order change, to host the signal registry or context, is banned per layout-templates §Primary screens (sec-dq2cwdsx).
- Making the graph's owner elements or virtual-node sub-targets layout boxes or new Tab stops is banned: the chunk produces no virtual node, and the Tab sequences stay fixed, per layout-templates §Primary screens (sec-dq2cwdsx).
@extract_section name=bindings
- layouts ↔ a11y: the recorded Tab sequences bind to a11y §Focus Order (SC 2.4.3), and the «reacts to a click» field lands beside the role reading (`GenericContainer`) that a11y owns, per layout-templates §Primary screens (sec-dq2cwdsx).
- layouts ↔ tests: `stand_non_lean_tasks`, `stand_lint` and `stand_keyboard` pin the counts and orders above, per layout-templates §Primary screens (sec-dq2cwdsx) and §IA notes (sec-ge4vhhot).
- layouts ↔ cli/carriers: a new verb changes `escher-session`'s usage line, `help` and the MCP listing, per layout-templates §Primary screens — cli (sec-vtizvuw4).
@extract_section name=acceptance
- (layouts) After the stand's signals move onto the registry, no served screen's markup, id, class, style, wrapper or order differs, and `unkeyed_actionable()` still reads 0 on all seven tasks (per layout-templates §Primary screens, sec-dq2cwdsx).
- (layouts) The CRUD, flight booker, Cells and load page Tab sequences read as recorded, in both layout modes (per layout-templates §Primary screens, sec-dq2cwdsx).
- (layouts) `stand_lint` `at_boot` still reads `[0, 0, 0, 0]` and the load page 0 (per layout-templates §IA notes, sec-ge4vhhot).
- (layouts) `escher-session help`, the usage line and `tools/list` list the same verbs in the same order, with the new read's verb among them if one is added (per layout-templates §Primary screens — cli, sec-vtizvuw4).
