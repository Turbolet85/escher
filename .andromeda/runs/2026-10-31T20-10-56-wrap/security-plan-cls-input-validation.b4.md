# Classified — security-plan · input-validation.b4

## Answers
input-validation#1.68 · yes · stale · keyboard.rs and pointer.rs gained lines above the cited spans (keyboard.rs:124-372, pointer.rs:832 and :1033-1181), so the citations moved; the activation claims for buttons, links and inputs still hold.
input-validation#1.69 · yes · stale · pointer.rs gained lines from 220 onward, so the cited :865-908 moved; the checkbox and radio change claim is unchanged.
input-validation#1.70 · yes · contradict · the stated limits end: a single select now paints its list, Enter and Space open it, and a multiple select's size is read, so "no popup is painted", "Enter and Space open nothing" and "size is not read" no longer hold.
input-validation#1.71 · yes · stale · keyboard.rs (:285-316, :431-468) and pointer.rs (:1121-1162) moved by the added lines; the multiple select's key and click rules hold.
input-validation#1.72 · yes · contradict · a group's non-empty label is now painted as a header, so "unpainted" no longer holds and the label-as-name reservation ends on inputs#I4.
input-validation#1.73 · no · the set's writes and reads sit in dioxus_document.rs, which is unreached, and the select model's claims are re-read unmoved with apply_select_choice still the one path.
input-validation#1.74 · yes · stale · focus_multiple_select and Target.multiple are renamed focus_select and Target.select, focus_select also closes any list, and the reached schema.rs restates type's help.
input-validation#1.75 · yes · stale · closed_select_of answers None for an open list, is_select is added and called by locate alone, clear_select_typeahead's caller is focus_select, snapshot.rs citations moved, and the new public readers of F12 (open_select, select_list_bounds, select_list_view, select_row_layout, close_select_list, SelectListView, SelectListRow) join this list.
input-validation#1.76 · yes · stale · render.rs gained lines at 7 and 246-247, moving the cited :978-1052; snapshot.rs gained lines above :266-293 and pointer.rs moved :657-723; the mask itself is unchanged.
input-validation#1.77 · no · layout/replaced.rs is unreached and its clamps are unchanged.
input-validation#1.78 · no · layout/inline/layout.rs is unreached and unchanged.
input-validation#1.79 · no · text_extent.rs is unreached and the session adds no text_extent caller; the lint change reads visible_region, not text_extent.
input-validation#1.80 · yes · stale · visible_region now answers the list's scrollport for an open list's row or group, and document.rs gained lines above :2913-2917, so the citation moved.
input-validation#1.81 · yes · stale · render.rs gained lines at 7 and 246-247, moving the cited :407-422; the list now paints last and a non-finite box is unpainted.
input-validation#1.82 · yes · add-to · F5: the open list is held by the same check set against a non-invertible transform; node.rs and hit_inner are unmoved, so the hit_inner claim stands.
input-validation#1.83 · no · gradient.rs is unreached and unchanged.
input-validation#1.84 · no · kurbo_css/css_box.rs is unreached and unchanged.
input-validation#1.85 · yes · stale · render.rs gained lines at 7, moving the cited :309-313; the hidden-input rule holds.
input-validation#1.86 · yes · stale · scrolling.rs is reached and scroll_into_view on a row and the wheel now reach the list's scroll_y, so the cited :164-167 and :566-568 can have moved.
input-validation#1.87 · no · form.rs is unreached and unchanged.
input-validation#1.88 · no · form.rs is unreached and unchanged.
input-validation#1.89 · no · form.rs is unreached and unchanged.
input-validation#1.90 · yes · stale · pointer.rs gained lines from 220 onward, moving the cited :1050-1052; the accept rule holds.
input-validation#1.91 · no · dioxus-native-dom events.rs is unreached and unchanged.
input-validation#1.92 · no · dioxus_document.rs is unreached and unchanged.
