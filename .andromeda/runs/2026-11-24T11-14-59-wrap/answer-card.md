# Answer card Q1 — 2026-11-24-upstream-sync-ahead-of-inside-the-canvas

## The answers it cites
- D1 and D2 — «Берём?» (drops from other applications, the widening by merge). The founder's own answer, given by question dialog in overseer1's session on 2026-11-24, relayed verbatim by overseer1 and the escher overseer: «Взять вместе с примером (Recommended)» — the forks file's options D1 A and D2 A (`evidence/founder-forks.md`; inputs#I2). Told to him with the question: windowed documents only; a handler can read a dropped file with no path limit; headless is untouched.
- The class: Boundary widening by upstream merge (`rul-72xhfjoz`, escalate) — the OS drop input enters every windowed document with no code of ours; a test of the merged delta covers the dispatch (`drag_and_drop`), none the windowed source or the dropped-file reader; escher routes no agent- or user-supplied input through it today.

## The operations it lists (each read as lying wholly inside D1 A / D2 A)
1. ops-architecture.md · decision/os-drop-input — established-decisions — fit: fits type 4 — the shell's winit drop source turns OS drags into `DragOver` · `DragLeave` · `Drop`, a dropped file arriving only through a file URI
2. ops-architecture.md · contract/blitz-traits-data-transfer/blitz-data-transfer — standard-contracts — fit: fits type 3 — the DataTransfer model's items, effects and store mode
3. ops-architecture.md · contract/blitz-traits-data-transfer/blitz-drag-event — standard-contracts — fit: fits type 3 — the drag payload: coordinates and the shared data transfer
4. ops-architecture.md · contract/ui-event/drag-over — standard-contracts — fit: fits type 3 — dispatches DOM dragenter and dragover
5. ops-architecture.md · contract/ui-event/drag-leave — standard-contracts — fit: fits type 3 — fires DOM dragleave, also on leaving the document
6. ops-architecture.md · contract/ui-event/drop — standard-contracts — fit: fits type 3 — dispatches DOM drop; a drop effect takes only where the source allows it
7. ops-architecture.md · contract/dioxus-drag-events/native-drag-data — standard-contracts — fit: fits type 3 — a Dioxus drag handler reads the data transfer, the dropped files and the modifiers
8. ops-architecture.md · contract/nodes-widgets/widget-drop-acceptance — standard-contracts — fit: fits type 3 — a custom widget accepts a drop only by setting the DragOver event's dropEffect
9. ops-security-plan.md · decision/dropped-file-read-unrestricted — input-validation — fit: fits type 1 — the Dioxus dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction
10. ops-security-plan.md · decision/os-drag-and-drop-untrusted — threat-model-summary — fit: fits type 1 — the desktop's drag and drop is an untrusted input source to the engine and to a Dioxus app's handlers

Check 6 groups with them: none (the refused previews and the println attribution read `not_master` below and stay the report's).

## Named beside the card — escalated, read `not_master` at fit, not asked
- ops-security-plan.md · decision/os-drop-input — fit: not_master type 4 — a coverage line retelling the drop source's dispatch and file-URI conversion; the code holds it (the drop input stands in security-plan through items 9 and 10, in architecture through item 1)
- ops-security-plan.md · decision/root-drop-examples — fit: not_master type 5 — an inventory of the two taken root examples and their previews; the code holds it

## The answer
Given 2026-11-24 in this session's question dialog, as answered:
- Option: «Confirm all 10 (Recommended)»
- Notes: «delegate escher-overseer: this confirm is NOT the founder's own word — he is away; record it PROVISIONAL as the confirmer's, the ratification itself stays his D1/D2 answer.»

Authority of the confirm: delegate escher-overseer, 2026-11-24 — PROVISIONAL; the ratification stays the founder's D1/D2 answer (inputs#I2), superseded by his own recorded word in the batch at the epoch boundary.
