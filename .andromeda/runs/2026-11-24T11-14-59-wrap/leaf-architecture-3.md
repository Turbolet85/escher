@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-har
@leaf_edit n=1 leaf=.claude/docs/services/blitz-traits.md line=16 class=omission
leaf-text: `EventState`, `BlitzPointerId`, per-event
source: ctr-yeshsmcq «contract/blitz-traits-data-transfer/blitz-data-transfer · blitz-traits DataTransfer model · BlitzDataTransfer — Holds string and file items with their types and files, effectAllowed and dropEffect, and a store mode»
replace-with: `EventState`, `BlitzPointerId`, the DataTransfer model — `BlitzDataTransfer` (string and file items with their types and files, `effectAllowed` and `dropEffect`, and a store mode, the operations named by `BlitzDragOperation` and `BlitzDragOperations`) — per-event
@leaf_edit n=2 leaf=.claude/docs/services/blitz-traits.md line=16 class=omission
leaf-text: `BlitzImeEvent`.
source: ctr-przrppgk «contract/blitz-traits-data-transfer/blitz-drag-event · blitz-traits DataTransfer model · BlitzDragEvent — Carries the pointer coordinates and the shared data transfer of a drag.»
replace-with: `BlitzImeEvent`, `BlitzDragEvent` (the pointer coordinates and the shared data transfer of a drag).
@leaf_edit n=3 leaf=.claude/docs/services/blitz-traits.md line=16 class=omission
leaf-text: - `UiEvent`, `DomEvent`,
source: ctr-gvddx42n «contract/ui-event/drag-over · UiEvent · DragOver — Dispatches DOM dragenter and dragover to the node under the drag.»
replace-with: - `UiEvent` (its drag variants `DragOver` — DOM dragenter and dragover to the node under the drag — `DragLeave` — dragleave, also when the drag leaves the document — and `Drop` — drop to the node under the drag, a drop effect taking only where the drag source allows it), `DomEvent`,
@leaf_edit n=4 leaf=.claude/docs/services/dioxus-native-dom.md line=22 class=omission
leaf-text: listener registration sets a `"<rust func>"` placeholder;
source: ctr-5netrz26 «contract/dioxus-drag-events/native-drag-data · Dioxus drag events · NativeDragData — Implements Dioxus's NativeDataTransfer, HasFileData, HasDataTransferData, HasDragData and ModifiersInteraction, so a Dioxus drag han»
replace-with: listener registration sets a `"<rust func>"` placeholder; drag events reach the app as `NativeDragData`, which implements Dioxus's `NativeDataTransfer`, `HasFileData`, `HasDataTransferData`, `HasDragData` and `ModifiersInteraction`, so a drag handler reads the data transfer, the dropped files and the keyboard modifiers;
