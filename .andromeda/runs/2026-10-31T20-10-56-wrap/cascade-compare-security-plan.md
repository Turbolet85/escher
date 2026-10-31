# Cascade compare — security-plan · wrap · 2026-10-31-select-popup
E items 34 · held 14 · lost at classify 0 · lost at strong 0 · beside a placed fact 1 · cascade only 8 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### input-validation#1.52 · lost at classify · beside a placed fact F11 (near 39, input-validation#1.10) · .andromeda/security-plan-detail/input-validation.md:176
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 176
    section: "## Input Validation"
    change: "Add after this line a new row. Boundary: Markup numeric attributes. What to validate: `size` on a `multiple` select. How (stack-specific): In every document, remote pages included: on a `multiple` select `size` is read through `size_rows` (packages/blitz-dom/src/node/select_list.rs:125-130), `parse_unsigned_integer` capped at 65534, `0` or a bad value no hint, a parse into a capped whole number with no text carried out; a `height` of `size × 1.2em` is hinted (packages/blitz-dom/src/stylo.rs:1124-1139) and the default sheet gives `select[multiple][size]` `overflow-y: auto` and its rows `line-height: 1.2` (packages/blitz-dom/assets/default.css:183-193); `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size` (packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459); a single select's `size` stays unread; pinned in `select_popup` by the `size_rows` unit, a hostile `size` and `size=3` showing three rows — an engine change for every document and a new attribute read, carried to P2's answer card on the founder's recorded word, inputs#I4 of escher-0.3.0/chunks/2026-10-31-select-popup — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "New row: `size` on a `multiple` select, a capped whole-number parse, a single select's still unread."
    rationale: "The report's Symbols / APIs `size` and Coverage `size on a multiple select` bring a new attribute read, validated by a capped parse; the plan's expected amendments owe a row for it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:21

## Held

- error-handling#1.1
- error-handling#5.1
- input-validation#1.60
- input-validation#1.62
- input-validation#1.68
- input-validation#1.70
- input-validation#1.72
- input-validation#1.74
- input-validation#1.75
- input-validation#1.80
- logging-monitoring#1.1
- threat-model-summary#1.1
- threat-model-summary#3.1
- threat-model-summary#4.1

## Cascade only

- input-validation#1.10
- input-validation#1.51
- input-validation#1.54
- input-validation#1.55
- input-validation#1.65
- input-validation#1.71
- input-validation#1.81
- input-validation#1.86
