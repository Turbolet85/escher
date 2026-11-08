@carried n=1 label=architecture/established-decisions fact=fct-7rnkaga4
why: the read's covered and off-screen causes must come from the same client-rect and hit rule the executor's call meets, an open select list answered first, so the shared refusal path keeps this rule unchanged.
quote: a box's client rect is the box's own — the shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own — and, an open select list answering first above that walk, a hit stops at the padding box of every box paint clips — one that clips by `overflow`, on one axis or both, or one whose computed `contain` includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted
@carried n=1 run=2 label=architecture/established-decisions fact=fct-7rnkaga4
quote: `scroll_into_view` writes every scrolling box on its target's containing-block chain, innermost first, and then the viewport, a row of an open select list excepted
@carried n=1 run=3 label=architecture fact=fct-6ketzni4
quote: a box's client rect never subtracts its own scroll offset, and a hit stops at the padding box of every box paint clips; where a select list stands open a hit answers the list first, with no scrollbar, so a row lies past any box that clips its select
@carried n=2 label=architecture/established-decisions fact=fct-j4ls6r2g
why: the read's covered cause must keep covered's stated meaning, a box with no area tested at its centre, so its answer agrees with the call's.
quote: `aria-activedescendant` stays unread and a box with no area stays hit at its centre, stated in `covered`'s meaning (§Standard Contracts → Driver session) — neither an engine change.
@carried n=3 label=architecture/established-decisions fact=fct-i7wbrsm5
why: off-screen is read through `visible_region` and covered through the hit, so the read must decide both through the same transform-aware readers the call uses.
quote: `visible_region` reading a clipping box through its transforms and a non-invertible transform taking no hit (§Standard Contracts → Scrolling, selection, tree)
@carried n=3 run=2 label=architecture fact=fct-vmq4u633
quote: `visible_region` through transforms, a non-invertible transform's no hit
@carried n=4 label=architecture/established-decisions fact=fct-2fe67avo
why: the owner-served case today is a select serving its options, so the read must report an option of a closed single select as refused covered, under a group too, and a disabled select as refused before anything else.
quote: Its limits: a single select's `size` is unread, and an option of a closed single select is not clicked (a driver `click` on one is refused `covered`, under a group too).
@carried n=4 run=2 label=architecture/established-decisions fact=fct-2fe67avo
quote: the walk's disabled gate stops a disabled select first, so it neither focuses nor acts
@carried n=4 run=3 label=architecture fact=fct-hnlfyl4f
quote: an option of a closed single select, under a group too, takes no click and stays refused `covered`
@carried n=5 label=architecture/established-decisions fact=fct-zquehlnc
why: proving an owner-served option accepted on the stand or a minimal fixture needs its single select's list opened first, by one of these three acts, before the read is compared with a real call.
quote: a single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing
@carried n=6 label=architecture/established-decisions fact=fct-jyueaf6l
why: a `multiple` select's rows are owner-served nodes a click acts on through the select's option arm, so the read must list each row with a click its call would accept, a disabled select's rows refused.
quote: A `multiple` select's pointer: `click_select_row`, behind an `option` arm of `handle_click`, stops at a disabled select, focuses the select and clears the typeahead and the text selection; a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, Shift with no cursor yet spans from the last chosen row, and a disabled row changes nothing
@carried n=7 label=architecture/established-decisions fact=fct-d6vzo5so
why: a call into a just-scrolled box's scrollbar band is accepted yet dispatches nothing, so a check comparing the read with a real call inside a scrolled box (Cells) must advance past the fade first, and the read's accepted answer must not be read as a dispatch.
quote: right after a `scroll`, a main-button pointerdown at a point in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`events/pointer.rs`, `hit_with_scrollbar` while `scrollbar_opacity > 0`); on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown — so a driver `click` inside a just-scrolled box needs an `advance` past the fade first, as `stand_cells_served` walks Cells to `Z25` with `advance 5000`
@carried n=8 label=architecture/established-decisions fact=fct-nl4nv5ny
why: the read's disabled cause must come from the one disabled reading, and its actionable set reads the snapshot's clickable and `unkeyed_actionable`'s focusability, so a disabled button with a listener is listed refused disabled while a `<div disabled>` with one is not.
quote: `disabled` is read one way, by its presence — bare, `"false"` or `"true"` — on a form control alone (ratified by the founder, 2026-10-18): `ElementData::is_disabled` is `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and a `disabled` attribute present, and focusability (`ElementData::flush_is_focussable`), the pointer-move gate, `ClickTarget::Disabled`, `handle_click`'s walk gate and the Enter and Space activation gates and the snapshot's `NodeState.clickable` (true where the bridge's vdom walk holds a current `click` listener and the element is no form control `ElementData::is_disabled` reports) read it, so a disabled `button` with a `click` listener reads not clickable and a `<div disabled>` with one reads clickable
@carried n=8 run=2 label=architecture/established-decisions fact=fct-nl4nv5ny
quote: the Tab walk (`focus_next_node` / `focus_prev_node`), autofocus and `DioxusDocument::unkeyed_actionable` move with focusability
@carried n=8 run=3 label=architecture fact=fct-fe5adstg
quote: `disabled` is read by its presence on a form control through one reading — focusability, the pointer and click gates and the Enter and Space activation read it — and an option's through `option_disabled`, its own attribute or its group's
@binding n=1
**established-decisions (hit, client rect, `visible_region`) ↔ plan (the shared refusal path)**: the read and the executor must decide off-screen through `visible_region` and covered through the hit that answers an open select list first and stops at clipping boxes, both through one path in `execute.rs`, so the read's cause equals the call's; the read's half of that path must write no scroll, never calling `scroll_into_view`, since the read changes nothing (architecture/established-decisions fct-7rnkaga4) (architecture/established-decisions fct-i7wbrsm5) (architecture/established-decisions fct-j4ls6r2g).
@binding n=2
**established-decisions (select model, `multiple` select pointer, select list) ↔ plan (owner-served nodes)**: a select's options and groups, listed with the select as owner, must read refused covered while a single select's list stands closed and acceptable once it stands open, a `multiple` select's rows acceptable by click, and every option of a disabled select refused disabled; the minimal fixture or stand check proving it opens the list by click, Enter or Space first (architecture/established-decisions fct-2fe67avo) (architecture/established-decisions fct-zquehlnc) (architecture/established-decisions fct-jyueaf6l).
@binding n=3
**established-decisions (DOM semantics: disabled) ↔ plan (the actionable set and the disabled cause) ↔ the body (Dioxus DOM bridge)**: the disabled cause must read `ElementData::is_disabled` on a form control and `option_disabled` on an option, never a second reading, and the actionable set must be bounded by the snapshot's `clickable` and `unkeyed_actionable`'s focusability, so the list and `unkeyed_actionable` answer from one reading (architecture/established-decisions fct-nl4nv5ny) (architecture fct-fe5adstg).
@binding n=4
**established-decisions (overlay scrollbar hazard) ↔ plan (stand checks in both layout modes)**: a check comparing the read with a real call inside a just-scrolled box must `advance` past the scrollbar fade before the call, since an accepted call there can dispatch nothing (architecture/established-decisions fct-d6vzo5so).
