proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 51
    section: "## Typography"
    change: "In the font-emboldening item, the clause «with hinting off when emboldening» now reads «with hinting off on an emboldening build and on on a plain one; a synthesized bold adds one embolden weight to the build's own, so on a plain build it is emboldened with hinting on, and on an emboldening build it takes twice the stated strength»; the rest of the item stands."
    sidecar: "typography: emboldening no longer implies hinting off or the stated strength for a synthesized bold, since 2026-10-31-select-popup."
    rationale: "Report, Spec claims disproved `stroke-text`: typography:51 «with hinting off when emboldening» no longer holds for a synthesized header on a plain build, and an emboldening build takes twice the strength; Symbols / APIs, Paint: `hint` stays `!FONT_EMBOLDEN_ENABLED`; Counts `stroke-text` marks typography:51 changed."
    basis: packages/blitz-paint/src/text.rs:695
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 51
    section: "## Typography"
    change: "New item after the emboldening item: - An open select list paints a group's `label` as a header in synthesized bold, `RunPaint { color, synthesize_bold }` through `stroke_text_as`, heavier than a row on every build; every other caller passes `RunPaint::default()` and a run with no synthesized bold paints as before (packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689)"
    sidecar: "typography: new item, a select list's group header in synthesized bold, from 2026-10-31-select-popup."
    rationale: "Report, Symbols / APIs, Paint: `stroke_text_as` and `RunPaint`, a synthesized bold adding one embolden weight, every other caller on the default; Counts `group-label`: a group's `label` painted as a header; Reverted: one vector for both arms read a header no heavier than a row, replaced at `715798ce2`. No line of the file states this weight."
    basis: packages/blitz-paint/src/text.rs:683-689
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 17
    section: "## Typography"
    change: "New table row after the blitz-dom default code/pre row: | blitz-dom default `select[multiple][size]` rows | not stated | not stated | rows line-height 1.2; the select's `height` hinted at `size` × 1.2em, `size` capped at 65534, `0` or a bad value no hint | not stated | (packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/stylo.rs:1124-1139) |"
    sidecar: "typography: new row, the line height of a sized multiple select's rows and its height hint, from 2026-10-31-select-popup."
    rationale: "Report, Symbols / APIs, `size`: `stylo.rs:1124-1139` hints `height` of `size × 1.2em` on a `multiple` select and `default.css:183-193` sets `line-height: 1.2` on its rows; Retired names «a `multiple` select's `size` is unread». A new value in the engine's default sheet, one row per value."
    basis: packages/blitz-dom/assets/default.css:183-193
note: "D-design-tokens raises nothing here: the select list's `hardcoded✗` names engine constants with rows owed in §Color Palette and no typographic token, and typography:72's absence of font tokens stands; the list's row height (the select's font on one line, report Deviations) is left to Spacing."
