proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "Replace the line's claim, its owner phrase dropped, with this: The engine paints a single select's list over the page, its detail at the `desktop-native-select-list` group: a click, Enter, and a Space with no typed text standing open it; a group's `label` is named in the accessibility tree and painted as a header row; a `multiple` select's `size` is read, a single select's stays unread"
    sidecar: "surface: the select-popup limits retired, the list now painted, opened by a click, Enter and Space, a group's label painted, a multiple select's size read; owner phrase dropped."
    rationale: "Report Retired names «no popup is painted», «Enter and Space open nothing on a single select», «a group's label is unpainted» and «a `multiple` select's `size` is unread» as limits ended; Counts / qualifiers lists surface:13 under `no-popup`, `enter-space`, `size-unread` and `group-label`, and Sweep reach lists surface:13 ×2 under `owner-title`, dropping the owner phrase."
    basis: .andromeda/design-system-detail/surface.md:13
  - detector: D-design-tokens
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: An open select list's highlight, disabled text and border are engine constants on `SelectListView`, not rules of that sheet and no design token, on the founder's F6, the engine holding no tokens: `HIGHLIGHT_TEXT` and `HIGHLIGHT_BACKGROUND` on the highlighted row, `DISABLED_TEXT` on a disabled row, `BORDER` and `BORDER_WIDTH` on the list's box; their literals at §Color Palette (packages/blitz-dom/src/node/select_list.rs:110-123; packages/blitz-paint/src/render/select_list.rs:54-83; packages/blitz-paint/src/render/select_list.rs:85-128)"
    sidecar: "surface: token map gains the select list's engine constants, hardcoded on the founder's F6, literals at Color Palette."
    rationale: "Report Coverage for `select list, engine overlay` reads `tokens hardcoded✗ — engine constants on his F6, the engine holding no tokens`; Symbols / APIs names the constants; `select-colours` lists surface:9's neighbour as a changed site."
    basis: .andromeda/design-system-detail/surface.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 10
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: In that sheet `select[multiple][size]` takes `overflow-y: auto` and its rows `line-height: 1.2`, beside the engine's height hint of `size` × 1.2em on a `multiple` select, the parse capped at 65534 and `0` or a bad value hinting nothing, so it stands that many rows tall — engine defaults, no design token (packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/stylo.rs:1124-1139)"
    sidecar: "surface: token map gains the sheet's sized multiple-select rules and the size height hint."
    rationale: "Report Symbols / APIs, `size`: `stylo.rs:1124-1139` hints the height and `default.css:183-193` adds the overflow and line-height rules; Coverage `size on a multiple select`."
    basis: packages/blitz-dom/assets/default.css:183-193
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New group after this line, `<!-- detail: values · desktop-native-select-list -->` closed by `<!-- /detail -->`, its first item: An open single select's list is painted by the engine over the page, its rows no boxes of the page's own: `paint_scene` paints it after the root's render and before the devtools overlays, whole past an `opacity` ancestor and a box that clips its select, with no motion; only rows in the scrollport are laid out, a non-finite box goes unpainted, and a shot of an open list shows an option's label and a group's `label` as pixels (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40)"
    sidecar: "surface: new desktop-native-select-list group, the list painted last and unclipped, with no motion."
    rationale: "Report Symbols / APIs, Paint; Schema / config on the list's pixels; Reverted names rows as boxes a rejected approach; Outcome (design) rows on clips, `opacity` and no motion; Expected amendments name §Surface: desktop-native."
    basis: packages/blitz-paint/src/render.rs:246-247
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New item in the `desktop-native-select-list` group: Placement: the list is at least its select's width and at most the viewport's, stays inside the viewport, and opens where its rows fit, else on the side with more room; a row is the select's font on one line tall; `MAX_ROWS` = 20 rows show, the rest reached by key, wheel or the driver's `scroll`; a thumb is painted under the `scrollbars` feature, takes no hit and is not draggable (packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:265-293; packages/blitz-paint/src/render/select_list.rs:130-154)"
    sidecar: "surface: the select list's placement, row height, twenty-row limit and undraggable thumb."
    rationale: "Report Symbols / APIs names `place`, `geometry`, `MAX_ROWS` and `paint_list_thumb`; Deviations give the row height; New text names the placement, long-list and thumb tests; Beyond the plan states the thumb not draggable; Expected amendments carry placement to §Surface: desktop-native."
    basis: packages/blitz-dom/src/node/select_list.rs:218-263
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New item in the `desktop-native-select-list` group: Row states: a group's `label` is a header row, its options under it, heavier than a row by a synthesized bold on every build, and it chooses nothing; the highlight moves by key, and by pointer to an enabled row; a disabled row changes nothing and leaves the list open; the highlight is in no snapshot and no tree, and a highlighted row's contrast is unread by the lint (packages/blitz-paint/src/render/select_list.rs:85-128; packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689)"
    sidecar: "surface: the select list's header, highlighted and disabled row states, and the highlight's stated limits."
    rationale: "Report Symbols / APIs, Paint, on `RunPaint` and the synthesized bold; Deviations carry the operator's «a header heavier than a row on every build»; New text names the group, highlight and disabled-row tests; Beyond the plan and Coverage state the highlight's limits."
    basis: packages/blitz-paint/src/text.rs:586-593
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New item in the `desktop-native-select-list` group: Opening and closing: a click, Enter, and a Space with no typed text standing open the list on its chosen option, changing nothing; a disabled, inert or hidden select, or one with no row to show, opens nothing, and a label's click focuses a bound single select and opens nothing; Enter, Space, Tab and a click on a row choose and close, Tab before focus moves either way; Escape, a second click, a click outside, the page scrolling and a wheel outside the list close it with the value unchanged; a select made inert, disabled, hidden or removed closes its list; a close clears the typed text; the driver's `type` focuses with no click and leaves no list open (packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/document.rs:891-907)"
    sidecar: "surface: the select list's opening, choosing and closing states."
    rationale: "Report Symbols / APIs, Pointer, Keys, The resolve and Driver; Deviations on a select with no row to show and a close clearing the typed text; Decisions on the page's scroll closing the list and a label opening nothing; New text names the opening, choosing, closing and `type` tests."
    basis: packages/blitz-dom/src/events/keyboard.rs:215-250
note: "surface:13's retired claims and owner phrase share one line, so they are one proposal; the new `desktop-native-select-list` group needs that subject added to the subjects line at .andromeda/design-system.md:247, which this detector may not propose; past the one Write, this file took two Edits, a misquote corrected in the first proposal's rationale and this clause."
