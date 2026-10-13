# layouts extract

## Relevance
partial — the chunk creates or changes no surface, region, focus order or dialog. It measures colours on the headless stand's TaskShell mount, so the stand's layout facts (mount structure, pinned viewport, the stable ids of each lean task's controls, the CRUD list as a nested clipping box) decide where each pair sits and how it is keyed. The colour values themselves belong to the design domain. The surface header's `NOT YET MEASURED` covers only tooling context, expression level and signature placement, and states that the facts below it cover the stand. §Decisions Log reads `NO RECORDED INTENT` and is not cited.

## Constraints
- layout-templates §Surface: desktop-native → Primary screens requires the headless stand to skip Home and mount one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell`. The structure is `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. The contrast harness measures that mount and no other. The three non-lean tasks and Home have no `stand` mount.
- layout-templates §Surface: desktop-native → Primary screens names each lean task's controls and value displays by author `id`, which is their stable element id: `counter-*`, `flight-*`, `timer-*` and `crud-*`. A per-pair record keys its pair by that id. The plan lists controls and value displays together and does not say which is which; the chunk's control set comes from `controls(task)`. Whether a listed value display such as `counter-value`, `flight-booked`, `timer-elapsed` or `timer-duration-value` falls inside "every stand control" is research's question.
- layout-templates §Surface: desktop-native → Primary screens gives each CRUD row the author `id` `crud-person-{person.id}` (fixture `crud-person-0`…`crud-person-2`) with `role="option"` inside `crud-list` `role="listbox"`, a row's selection stated by `aria-selected`. If rows count as controls, their pairs are keyed by those ids. A selected row is a different colour state from a row at rest.
- layout-templates §Surface: desktop-native → Primary screens requires the stand's ids and state attributes to add no element, class, style, wrapper or order. The task CSS selects by none of them. Each held instance reads as a fresh boot, with the TaskShell mount and pinned viewport holding in both layout modes. A measurement harness on this surface stays read-only toward the stand's markup.
- layout-templates §Surface: desktop-native → IA notes requires each document to start with the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, with the app mounted into `main`. Any ancestor chain a computed-background reading walks runs control → TaskShell regions → `main#main` → `body` → `html` → canvas.
- layout-templates §Surface: desktop-native → IA notes requires the CRUD list to be the stand's nested scrolling box: `crud-list` reads (24, 112.796875, 369 × 419.203125) at boot and clips its rows, while `task-body` reads (0, 46, 800 × 554). A reading taken from the painted raster must stay inside the part of the control that can be seen (`visible_region`), not its raw bounds.
- layout-templates §Surface: desktop-native → IA notes sets the test harness's default viewport at 800 × 600, scale 1, light mode, which matches the stand's pinned viewport. The light-only scope rests on that pin.

## Patterns to follow
- Boot each lean task through the one stand mount, the way every existing consumer does: the `escher-session` host role, the `mcp` role and the stand checks. Read the result as a fresh boot in both layout modes (per layout-templates §Surface: desktop-native → Primary screens).
- Key every per-control record by the author `id` the plan lists for that task, never by position or `NodeId`. This includes `back-btn` in `#task-header`, once per task (per layout-templates §Surface: desktop-native → Primary screens).
- Measure a clipped control through the engine's visible-region reader, the reader the driver's `screenshot` crop and `off-screen` refusal already use (per layout-templates §Surface: desktop-native → IA notes).

## Anti-patterns to avoid
- Do not add a stand element, class, style, wrapper, id or reorder to make a pair measurable (per layout-templates §Surface: desktop-native → Primary screens).
- Do not take a layout or colour reading from the windowed stand. The plan records it as not booted or not measured, and only the headless TaskShell mount is covered (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- layouts ↔ a11y: the plan's per-task control ids are the keys of the SC 1.4.3 per-pair record that a11y-plan §6 records as NOT YET MEASURED. Each pair sits on the TaskShell mount (layout-templates §Surface: desktop-native → Primary screens).
- layouts ↔ tests: the plan's id list and the stand checks' `controls(task)` table in `tests/blitz-tests/tests/common/mod.rs` must name the same controls. Whether they agree, CRUD rows and value displays included, is research's question (layout-templates §Surface: desktop-native → Primary screens).
- layouts ↔ design: the text and background colours measured are design tokens (design-system §Color Palette). The layouts domain supplies only where each control sits and which ancestor regions lie behind it (layout-templates §Surface: desktop-native → IA notes).

## Acceptance criteria contributions
- (layouts) Every pair is measured on the lean task mounted in TaskShell (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1, Light viewport, in both layout modes. Each record is keyed by the control's listed stable id, `back-btn` once per task (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The chunk's diff adds no element, id, class, style, wrapper or order to `examples/seven_guis/src/`, and the stand's existing checks read the mount unchanged (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) If the measurement reads the raster, a CRUD row's sample lies inside its visible region within `crud-list` (24, 112.796875, 369 × 419.203125), never in the part `crud-list` clips (per layout-templates §Surface: desktop-native → IA notes).
