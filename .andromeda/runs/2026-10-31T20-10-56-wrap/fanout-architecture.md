proposals:
  - detector: D-retired
    severity: warning
    section: §Established Decisions → Scrolling and input
    change: "In the select model, replace «no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select, under a group too, takes no click and stays refused `covered`, `size` is unread and Ctrl+A is not modelled» with: a click, Enter and a Space with no typed text standing open a single select's list, which the engine paints over the page with a group's `label` as a header; a label's click focuses the select and opens nothing; an option of a closed single select, under a group too, takes no click and stays refused `covered`; a `multiple` select's `size` hints a height of that many rows, a single select's stays unread, and Ctrl+A is not modelled."
    sidecar: "Select model: the limits no popup, Enter and Space opening nothing, and `size` unread retired; a single select's list opens by click, Enter and Space and is engine-painted."
    rationale: "Report Retired names «no popup is painted», «Enter and Space open nothing on a single select», «a group's label is unpainted» and «a `multiple` select's `size` is unread»; Counts rows `no-popup`, `enter-space` and `size-unread` each list architecture:62 as changed, and `label-click` holds that a label opens nothing."
    basis: ".andromeda/architecture.md:62"
  - detector: D-retired
    severity: warning
    section: §Standard Contracts → Document core
    change: "In the document's clock entry, public `clear_select_typeahead`'s one non-test caller is the driver's `focus_select`, in place of `focus_multiple_select`."
    sidecar: "Document core: `clear_select_typeahead`'s caller renamed to the driver's `focus_select`."
    rationale: "Report Retired names `focus_multiple_select`; Symbols / APIs → Driver: `focus_multiple_select` → `focus_select`, and `clear_select_typeahead` has one non-test caller, `focus_select`; `focus-select` lists architecture:135 as changed."
    basis: ".andromeda/architecture.md:135"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Document core
    change: "After the `select_of` readers, add: `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout` are public and read-only; public `close_select_list` closes an open list, its callers keyboard, pointer, reconcile and the driver's `focus_select`; the rest of the list's machinery on `BaseDocument` is crate-private or private."
    sidecar: "Document core: registers the five new public `BaseDocument` select-list methods."
    rationale: "Report Symbols / APIs → `BaseDocument` lands four public read-only methods and one public writer, and Coverage names them as a new surface; the body registers none of them."
    basis: "packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:990-1064"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Nodes and widgets
    change: "After «the select's rules live in the crate-private `node::select`, with no public type», add: the open list's state lives in `node::select_list`, crate-private but for two public read types, `SelectListView` with the list's colours as associated constants and `SelectListRow`, re-exported from the crate root."
    sidecar: "Nodes and widgets: registers the `node::select_list` module and its public read types `SelectListView` and `SelectListRow`."
    rationale: "Report Crates / modules names the new module blitz-dom `node::select_list`; Symbols / APIs lists `SelectListView` and `SelectListRow` as public, re-exported from the crate root, with the colours as associated constants; Deviations records two public read types."
    basis: "packages/blitz-dom/src/node/select_list.rs:78-108; packages/blitz-dom/src/lib.rs:108-111"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Dioxus DOM bridge
    change: "The public `closed_select_of`, which answers `None` for a select whose list is open, and the new public `is_select`, read by the driver's `locate` alone, are the select tests the driver reads, naming the element and never the role; `is_multiple_select` stays re-exported, read within the workspace by `closed_select_of` alone; a select's options are its children in the snapshot, a `multiple` select's rows and groups reading client rects of their own and an open list's rows reading `select_list_bounds` before the client rect."
    sidecar: "Dioxus DOM bridge: `closed_select_of`'s open-list answer, the new `is_select`, `is_multiple_select`'s remaining reader, and an open list's row bounds in the snapshot."
    rationale: "Report Symbols / APIs → Bridge and → Driver's remaining callers state each of these; `closed-option` and `focus-select` both list architecture:183 as changed."
    basis: ".andromeda/architecture.md:183; packages/dioxus-native-dom/src/snapshot.rs:152-155"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Paint and shell
    change: "After «`paint_scene` paints a document into a `PaintScene`», add: an open select's list is painted last within it, by `paint_select_list` after the root's render and before the devtools overlays, read from `SelectListView`, past any clip or `opacity` on its select's ancestors, and with no shell or renderer changed."
    sidecar: "Paint and shell: `paint_scene` paints an open select's list after the root's render, read from `SelectListView`."
    rationale: "Report Symbols / APIs → Paint places `paint_select_list`; `paint-order` lists architecture:157 as changed; Outcome's (arch) and (design) rows hold paint through `paint_scene` alone and a list whole past `opacity` and a clip."
    basis: ".andromeda/architecture.md:157; packages/blitz-paint/src/render.rs:246-247"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: "In the `visible_region` entry, add: for a row or a group of an open select list, `visible_region` answers the list's scrollport, not its clipping boxes."
    sidecar: "Scrolling, selection, tree: `visible_region` answers the list's scrollport for an open list's row or group."
    rationale: "Report Symbols / APIs → Seen through the list: `visible_region` answers the list's scrollport for a row or a group of an open list; `visible-reg` lists architecture:145 as changed."
    basis: ".andromeda/architecture.md:145; packages/blitz-dom/src/scrolling.rs:754-756"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Mutation, query and CSSOM
    change: "After «a native control's checked state on a checkable role», add: and a single select's native `expanded` from its list being open, no `aria-expanded` read on one, every other element keeping that read."
    sidecar: "Mutation, query and CSSOM: the tree writes a single select's native `expanded` and reads no `aria-expanded` on one."
    rationale: "Report Symbols / APIs → Tree: a single select writes `set_expanded(open)` and reads no `aria-expanded`, every other element keeping that read; Expected amendments lists §Standard Contracts → Mutation."
    basis: "packages/blitz-dom/src/accessibility.rs:238-242"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Events
    change: "In «a focus change as `blur`, `focusout`, `focus`, `focusin`, but for the focus fixup under `inert`», add a second exception: the driver's `type` on any select, which focuses with no click and fires no DOM focus events."
    sidecar: "Events: the driver's `type` on any select named as a focus change with no DOM focus events."
    rationale: "Report Symbols / APIs → Driver: `type` on any select focuses with no click, so it fires no DOM focus events on a single select either; the body names the inert fixup as the one exception."
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → Scrolling and input
    change: "In the [Scrolling and input] entry, qualify the chain and hit claims: where a select list stands open, a hit answers the list first, with no scrollbar, so a row lies past any box that clips its select; a wheel over the list scrolls the list; `scroll_into_view` on a row writes the list's `scroll_y` at once and moves no chain or viewport."
    sidecar: "Scrolling and input: an open list's exceptions to the hit, wheel and `scroll_into_view` claims."
    rationale: "Report Symbols / APIs → The hit, → Pointer and → Seen through the list: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, `handle_wheel` scrolls the list over it, and `scroll_into_view` on a row returns with no chain or viewport moving; the entry states these without the exception."
    basis: "packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/scrolling.rs:669-672"
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts → Scrolling, selection, tree
    change: "Qualify «`scroll_into_view` scrolls every scrolling box on the target's chain» and «a hit stops at the padding box of a box paint clips»: `scroll_into_view` on an open list's row writes the list's `scroll_y` at once and moves no chain or viewport; `hit_with_scrollbar` answers the open list first, `hit_inner` unedited, and `hit`, `set_hover_to` and `handle_pointerdown` all read it."
    sidecar: "Scrolling, selection, tree: the same open-list exceptions restated at the contract."
    rationale: "The same claims restated in the contract; report Symbols / APIs → The hit and → Seen through the list."
    basis: "packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/scrolling.rs:669-672"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → Scrolling and input
    change: "After the entry naming the eight further direct engine changes, add: the select list is one more direct engine change for every document, remote pages included, its merge surface the sixteen files of the chunk's `evidence/merge-surface.md`, a line owned by Epoch 3's «Upstream sync ahead of the UI graph»."
    sidecar: "Scrolling and input: the select list added to the record of direct engine changes for every document, with its merge surface."
    rationale: "Report Coverage names the `select list, engine overlay (every document, remote pages included)`; Expected amendments carries the playbook's *An engine behaviour change for every document*; P5's Merge surface gives sixteen files and the line's owner."
    basis: "escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md"
  - detector: D-arch-decisions
    severity: warning
    section: §Cross-cutting Patterns → Invalidation and state integrity
    change: "The changed set is written at five kinds of site, the new one a select list's open and close marking its select; with no list open it is written by no node creation, hover, scroll, resize, load, animation or layout; where a list stands, a page scroll or a move that closes it marks its select through `reconcile_select_list` at the next resolve and a wheel elsewhere through `close_select_list`, while a highlight or a wheel move inside the list marks nothing."
    sidecar: "Invalidation: a fifth kind of changed-set site, and «no scroll or layout» narrowed to where no select list stands."
    rationale: "Report Spec claims disproved, `changed-set`: architecture:275 is false where a list stands; Counts row `changed-set`: open and close mark the select, a new kind of site, architecture:275 changed."
    basis: ".andromeda/architecture.md:275"
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Runtime threading and event flow
    change: "In the `resolve` order, add `reconcile_select_list` after layout and before the hover refresh."
    sidecar: "Runtime threading and event flow: the `resolve` order gains the select list's reconcile."
    rationale: "Report Symbols / APIs → The resolve: `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`; the body enumerates the order without it."
    basis: "packages/blitz-dom/src/resolve.rs:133-135"
note: "No port, listener, env var, crate or dependency landed, so §Occupied Resources, §Stack and §Inherited Defaults body lines hold; the test-only `cc-popup` and `hl-popup` state directories are new detail for the existing res-filesystem category, the cascade's."
