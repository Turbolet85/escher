proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 169
    section: "## Standard Contracts"
    change: "`off-screen`'s meaning now reads \"the element lies outside the viewport, or outside the visible part of a scrolling box that holds it; under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view\": its first clause the operator's, its second restated at 2026-10-31-engine-debts in place of the transformed-box clause, whose limit the engine fixed on the founder's word (2026-10-31, «Починить (Рекомендую)»); the remedy, the eight causes and their names unmoved."
    sidecar: "contract-driver-session: `off-screen`'s transformed-box clause retired; its meaning states the rotation reading, the remedy, causes and names unmoved."
    rationale: "The report's Retired bullet names `off-screen`'s transformed-box clause, and Symbols / APIs (driver and stand) gives the new text at `refusal.rs:83-85`; the `visible-region` sweep marks this line changed."
    basis: packages/escher-driver/src/refusal.rs:83-85
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 171
    section: "## Standard Contracts"
    change: "In the crate docs' three readings as off, the third, a scrolling box under a transform read where it stands untransformed, so an element inside it can read `off-screen` though it shows, is restated since 2026-10-31-engine-debts as the rotation reading: under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view; the word `untransformed` no longer stands in packages/escher-driver."
    sidecar: "contract-driver-session: the crate docs' untransformed-box reading retired for the rotation reading."
    rationale: "The report's Retired bullet says the word `untransformed` leaves `packages/escher-driver` (entry 8), and Symbols / APIs lists the crate docs at `lib.rs:77-81` among the fixed text that moved; the `visible-region` sweep marks this line changed."
    basis: packages/escher-driver/src/lib.rs:77-81
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 171
    section: "## Standard Contracts"
    change: "`type`'s help says that on a select the text chooses the first option whose label starts with it and, since 2026-10-31-engine-debts, carries its reading on a `multiple` select, where the text chooses its match alone."
    sidecar: "contract-driver-session: `type`'s help gains its `multiple` select reading."
    rationale: "Symbols / APIs (driver and stand) lists `type`'s help at `schema.rs:239-240` among the fixed text that moved, and the `type-on-select` sweep marks this line changed; the choosing of the match alone is the report's F6 keyboard rule and the target `stand_act_select`'s `type_on_a_multiple_select_chooses_its_match_alone_with_no_stray_click`."
    basis: packages/escher-driver/src/schema.rs:239-240
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 181
    section: "## Standard Contracts"
    change: "After this line, in the group's record shape: ` `type` on a `multiple` select (since 2026-10-31-engine-debts, «Построить сейчас»): `Target` gains `multiple`, and `type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead, without which a second `type` joins the first's text; a text opening on a space types nothing, since no label can match and the Space would flip a row; no verb, argument, key name, result field, cause, session error or exit is added.`"
    sidecar: "contract-driver-session: `type` on a `multiple` select focuses with no click and clears the typeahead, no closed set moved."
    rationale: "Report Symbols / APIs (driver and stand), F6, and Deviations 4 and 6c; this line states that `type` clicks its target, which no longer holds of a `multiple` select."
    basis: packages/escher-driver/src/execute.rs:557-563
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 182
    section: "## Standard Contracts"
    change: "After this line, in the group's record shape: ` `press` carries no Ctrl, so a set of rows of a `multiple` select that are not adjacent cannot be built through the driver, and since 2026-10-31-engine-debts its help states it: \"a set of rows that are not adjacent needs Ctrl, which this verb does not carry\" (the founder, 2026-10-31, «Ограничение и residual (Рекомендую)»).`"
    sidecar: "contract-driver-session: `press` carries no Ctrl, a limit its help states on a `multiple` select."
    rationale: "Report Symbols / APIs (driver and stand), `press`'s help at `schema.rs:261-263`, and Decisions & corrections for the founder's word; the `press-ctrl` sweep finds no site stating this help, so the fact is new here."
    basis: packages/escher-driver/src/schema.rs:261-263
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 202
    section: "## Standard Contracts"
    change: "After this line, in the group's record shape: ` Since 2026-10-31-engine-debts the `overlap` reading leaves out a closed select's options: `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so CRUD's `role=\"option\"` rows are not read as a select's, and `overlaps` skips such an entry on either side; a `multiple` select's rows are linted as usual (the founder, 2026-10-31, «Исключить закрытые option (Рекомендую)»).`"
    sidecar: "contract-driver-session: the lint's `overlap` skips a closed select's options on either side, read by the element."
    rationale: "Report Symbols / APIs (driver and stand), F9; this line defines `overlap` as any two nodes neither an ancestor of the other whose visible parts intersect, and that definition now has this exception. Its `visible_region` statement stands: the `visible-region` sweep reads it as naming the reader, and the callers are unchanged."
    basis: packages/escher-driver/src/lint.rs:337-338
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 209
    section: "## Standard Contracts"
    change: "The `overlap` reading of a select's options: the snapshot gives a closed select's options its client rect, and since 2026-10-31-engine-debts the lint's `overlaps` skips them, so on the stand all four lean tasks read 0 `overlap` at boot, the flight booker's `flight-type-one-way` × `flight-type-return` gone and `stand_lint`'s `at_boot` reading `[0, 0, 0, 0]` in both layout modes; the open defect and its `.andromeda/residuals.md` owner end; a `multiple` select's rows read their own client rects, and `stand_lint`'s planted pair of overlapping rows reads exactly `fx-row-a` × `fx-row-b`."
    sidecar: "contract-driver-session: a closed select's options no longer read `overlap`; `stand_lint`'s `at_boot` moves from `[0, 1, 0, 0]` to `[0, 0, 0, 0]` and the open defect ends."
    rationale: "Report Counts / qualifiers moved, `lint-overlap`, which marks `standard-contracts.md:209` changed, F9 under Symbols / APIs (driver and stand), and Deviations 8 for the planted pair."
    basis: tests/blitz-tests/tests/stand_lint.rs:282-332
note: F1, F2, F4, F5, F8 to F11, F13, F15 and F20 name contracts outside this group's driver-session entities and are left to their own groups or the new-detail pass; a `multiple` select's snapshot rows that `click` lands on are the Dioxus DOM bridge contract's; the report's `press-ctrl` says the limit lands as a residual line, P5's, beside F17.
