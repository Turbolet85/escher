# layouts extract

## Relevance
partial — the chunk renders no surface and places no component; layout-templates binds it only through the stand screens it serializes (their population, shell nesting, pinned viewport and author ids) and through the absence of any snapshot screen on the cli surface.

## Constraints
- layout-templates §Surface: desktop-native / Primary screens (the seven_guis Home and TaskShell bullet) requires the headless stand to skip Home and mount one lean task in TaskShell through `task_in_shell`; "the whole stand screen" is therefore one lean task inside TaskShell, four screens in all (Counter, FlightBooker, Timer, Crud), and Home's task cards are not part of any serialized stand screen.
- layout-templates §Surface: desktop-native / Primary screens requires the TaskShell nesting it names (the shell under `main`, a header holding the back button and title, above the task body); the serialized parent/child structure must not contradict that nesting. Which of those containers the snapshot keeps as nodes is research's question.
- layout-templates §Surface: desktop-native / Primary screens requires the stand's pinned viewport, scale, colour scheme and bundled font; every serialized `bounds` value is a function of that pin, so the size budget and the determinism check are stated for that one viewport and no other.
- layout-templates §Surface: desktop-native / Primary screens requires each lean task's controls and value displays, and each CRUD row, to carry the author `id` that is its stable element id; a serialized screen is complete for this domain only if every id that bullet lists for the task appears in it. Whether the snapshot today keeps every such element as a node is research's question.
- layout-templates §Surface: desktop-native / Primary screens requires that ids and input names add no class, style, wrapper or order to the stand; the chunk may not buy compactness or completeness by adding, wrapping, restyling or reordering a stand element.
- layout-templates §Surface: desktop-native / Primary screens requires TaskShell's body to be a scrolling region; "whole screen, no truncation" must hold for the body's content whether or not it fits the pinned viewport. Whether the snapshot keeps nodes scrolled out of view, and in which coordinate space their bounds are read, is research's question.
- layout-templates §Surface: cli / Primary screens lists no snapshot command or screen; the chunk adds none, and a snapshot CLI surface is a layout-templates amendment owned by the later driver entries, not a side effect of this serializer.

## Patterns to follow
- Reach each screen through the stand's headless TaskShell boot at its pinned viewport, never through Home navigation (per layout-templates §Surface: desktop-native / Primary screens).
- Address and assert serialized nodes by author id, not by tree position or sibling index (per layout-templates §Surface: desktop-native / Primary screens).
- Treat a CRUD row's id as following its person under filter, Create and Delete; a serialized row is identified by that id, not by its place in the list (per layout-templates §Surface: desktop-native / Primary screens).
- Every Dioxus document starts from the same fixed skeleton with the app mounted into `main`; the serialized tree's root-side structure is uniform across the four screens (per layout-templates §Surface: desktop-native / IA notes).
- The agent-read CLI screens already speak one JSON object per line on stdout with raw output kept in a log file; this is the existing agent-facing output shape to weigh at the format fork, not a mandate on an in-process text form (per layout-templates §Surface: cli / Primary screens).

## Anti-patterns to avoid
- Adding a class, style, wrapper element or reorder to a stand task or to TaskShell to make its screen serialize smaller or more completely (banned per layout-templates §Surface: desktop-native / Primary screens).
- Selecting or styling by the stand's author ids, or deriving a serialized node's identity from layout position instead of its id (per layout-templates §Surface: desktop-native / Primary screens).
- Printing the serialized screen onto the agent-run or cold-agent stdout stream, which the plan requires to be JSON lines only (per layout-templates §Surface: cli / Primary screens).

## Contract bindings
- layouts ↔ tests: the pinned viewport and TaskShell boot the plan requires are the fixture the new stand check runs on, and the check runs under the agent-run contract whose event schema lives in test-plan §3 (per layout-templates §Surface: cli / Primary screens).
- layouts ↔ security: the author ids and input names the plan places on the stand are two of the three content classes the serialized text carries; the crossing is security-plan §Input Validation's, escalated per the scope's CARRY.
- layouts ↔ a11y: the six task inputs are named by attributes alone, and those names are what the serialized `name` field reads; the naming rule and its criterion are the a11y extractor's to anchor.
- layouts ↔ a11y (focus order): none — the chunk adds no focusable element and changes no order.
- layouts ↔ design: none — the chunk touches no token, spacing or breakpoint.

## Acceptance criteria contributions
- For each of the four lean tasks booted in TaskShell at the pinned viewport, the serialized text carries every author id the plan lists for that task, each exactly once (per layout-templates §Surface: desktop-native / Primary screens).
- The CRUD screen's serialized text carries one node per fixture row under its `crud-person-{id}` id, nested under the list's node (per layout-templates §Surface: desktop-native / Primary screens).
- The chunk's diff adds no element, class, style, wrapper or reorder under `examples/seven_guis/src/` (per layout-templates §Surface: desktop-native / Primary screens).
- Two fresh boots of the same task at the pinned viewport serialize to identical text, bounds included, and the recorded size budget names that viewport (per layout-templates §Surface: desktop-native / IA notes).
