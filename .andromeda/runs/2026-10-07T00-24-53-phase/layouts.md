# layouts extract

## Relevance
partial — the chunk creates and modifies no surface, region or component (the diff is an in-process Rust API); layouts binds only as the fixed structure of the stand screens the diff is read on and proven on, plus the CLI contract the new stand check runs under. The plan carries no focus-order, responsive, modal, navigation or empty-state section, and the desktop-native and cli surface headers are marked NOT YET MEASURED for tooling context, expression level and signature placement — nothing is extracted from those.

## Constraints
- The screens the diff is proven on are the stand's lean tasks mounted in TaskShell, with Home skipped; the mount structure (the shell container, its header with back button and title, the body below) is the plan's and is not this chunk's to alter. Whether tracking or the diff needs any change to stand markup is research's question; the scope's answer is `stand.rs` read only. (per layout-templates §Surface: desktop-native → Primary screens)
- The stand's viewport is pinned (size, scale, colour scheme, bundled font); a diff entry that reports bounds is only reproducible under that pin, so the stand check takes the viewport from the stand boot and sets none of its own. (per layout-templates §Surface: desktop-native → Primary screens)
- Every lean task's controls and value displays carry author ids that are their stable element ids, and the diff's subjects on the stand are keyed by exactly those ids; the plan requires that ids add no class, style, wrapper or order and that no task CSS selects by them — a new id introduced for a diff case (in a fixture or otherwise) follows the same rule. (per layout-templates §Surface: desktop-native → Primary screens)
- A CRUD row's stable element id is its author key, derived from its person's model-assigned id, and the plan requires it to follow its person under filter, Create and Delete; a diff that reports a row as appeared or disappeared names it by that id, and a row that merely moves keeps its id. The scope says the lean tasks may lack a node-added and a node-removed case; the plan's CRUD Create and Delete describe rows entering and leaving the list — whether those two actions already serve as the added/removed proof is research's question. (per layout-templates §Surface: desktop-native → Primary screens)
- Every document starts from the fixed skeleton and the app mounts into its main element; a minimal fixture beside the stand mounts the same way, at the harness's default viewport, so the skeleton's nodes are part of every before and after reading and are not subjects a step changes. (per layout-templates §Surface: desktop-native → IA notes)
- The `has_changes` correction edits the windowed shell's window file, where the plan also fixes the viewport as the window surface less the safe-area insets and the pointer-coordinate arithmetic; the change to the accessibility-update gate leaves both as specified. (per layout-templates §Surface: desktop-native → IA notes)
- The new stand check runs under the agent-run contract's `run stand` selection by file name; the plan requires every verb's stdout to be JSON lines only with raw cargo output kept in the run log, so the chunk adds no verb, no selection and no stdout shape, and the check prints no diff. (per layout-templates §Surface: cli → Primary screens)

## Patterns to follow
- Boot each lean task through the stand's TaskShell mount at its pinned viewport and address every node by its author id, as the existing stand checks named in the plan's stand bullet do (layout-templates §Surface: desktop-native → Primary screens).
- Take the plain changed-node case from the counter task's two ids — the increment control acted on, the value display expected in the diff (layout-templates §Surface: desktop-native → Primary screens).
- Take per-row identity from the CRUD row's author key beside its Dioxus key, so appear, disappear and reorder are told apart by id rather than by position (layout-templates §Surface: desktop-native → Primary screens).
- Where a lean task lacks a case, build the fixture on the fixed document skeleton at the harness default viewport rather than extending a stand task (layout-templates §Surface: desktop-native → IA notes).
- Let the new `stand_*.rs` file ride the existing `run stand` selection; results reach the agent as the contract's JSON-line events only (layout-templates §Surface: cli → Primary screens).

## Anti-patterns to avoid
- Adding an element, wrapper, class, style or reordering to a stand task so that a change becomes observable in the diff — the plan requires ids and names to add none of these (layout-templates §Surface: desktop-native → Primary screens).
- Booting Home, or a viewport other than the stand's pin, for the diff check — the stand skips Home and its viewport is fixed (layout-templates §Surface: desktop-native → Primary screens).
- Emitting diff content on a verb's stdout or adding a diff-specific verb or selection to the agent-run script — stdout is JSON lines only (layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts ↔ architecture: the author ids the plan places on stand elements are the stable element ids the diff keys its subjects by (architecture §Standard Contracts → Dioxus DOM bridge).
- layouts ↔ a11y: the chunk adds no focusable element, so the plan contributes no focus-order position (it has no focus-order section); the shell's AccessKit tree update that the truthful flag gates is a11y-plan's to state, not a layout concern. The attribute-only naming of the six inputs is the source of the accessible names a diff may carry.
- layouts ↔ tests: the pinned stand viewport and the agent-run event schema and exit grammar are owned by test-plan §3; the plan's cli entry defers to it.
- layouts ↔ security: ids and accessible names read off the stand layout are content in a diff, bound by security-plan §Input Validation (the `id` row and the rows beside it).

## Acceptance criteria contributions
- (layouts) After the chunk, each of the four lean tasks still mounts in the TaskShell structure with its listed author ids, and no element, class, style, wrapper or order has been added to any stand task or selected by task CSS (per layout-templates §Surface: desktop-native → Primary screens)
- (layouts) The diff stand check boots through the stand at its pinned viewport and sets no viewport, scale, colour scheme or font of its own; every diff subject it asserts on a stand screen is one of the plan's listed author ids (per layout-templates §Surface: desktop-native → Primary screens)
- (layouts) Any fixture added beside the stand mounts into the fixed document skeleton at the harness default viewport, and a no-op step on it reports none of the skeleton's nodes as changed (per layout-templates §Surface: desktop-native → IA notes)
- (layouts) `agent-run.sh run stand` picks up the new check by file name with stdout still JSON lines only, and the usage line's verbs and selections are unchanged (per layout-templates §Surface: cli → Primary screens)
