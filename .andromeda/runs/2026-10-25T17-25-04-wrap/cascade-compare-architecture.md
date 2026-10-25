# Cascade compare — architecture · wrap · 2026-10-25-cold-agent-bar
E items 35 · held 16 · lost at classify 0 · lost at strong 0 · beside a placed fact 1 · cascade only 5 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### established-decisions#11.3 · lost at classify · beside a placed fact F8 (near 76, established-decisions#11.2) · .andromeda/architecture-detail/established-decisions.md:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "New item after the select model's merge-surface line, inside the `dec-scrolling-and-input` group: - Right after a scroll the overlay scrollbar is shown, and a main-button pointerdown in its band hits the scrollbar (`hit_with_scrollbar` while `scrollbar_opacity > 0`) and is taken as a thumb drag, dispatching nothing to the element under it; on a harness's `VirtualClock` it stays shown until the document's clock passes its fade, and a pointer resting on the thumb keeps it shown — so a driver `click` at an element in that band right after `scroll` lands on the scrollbar, and an `advance` past the fade comes first; an existing engine behaviour, unchanged (packages/blitz-dom/src/events/pointer.rs; tests/blitz-tests/tests/stand_cells_served.rs:176-192 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "established-decisions: [Scrolling and input] gains the measured overlay-scrollbar hazard — a pointerdown in the shown scrollbar's band right after a scroll is a thumb drag that dispatches nothing, kept shown until the clock passes its fade (2026-10-25-cold-agent-bar, F21)"
    rationale: "The report's F21 names two new facts with no master site; one is an engine scrolling-and-input behaviour this category's [Scrolling and input] group holds the kin of (drag threshold, overlay scrollbar fade on the document's clock at line 76) — measured in Decisions & corrections (report.md:355-357) and pinned by the new `stand_cells_served::a_click_right_after_the_scroll_lands_on_the_shown_scrollbar` (New text, by line: stand_cells_served.rs 176-192)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:355
    fact: F21

## Held

- conventions#8.1
- existing-scopes#1.31
- existing-scopes#1.32
- occupied-resources#2.1
- occupied-resources#3.1
- occupied-resources#5.1
- occupied-resources#6.1
- stack-and-technologies#1.22
- standard-contracts#13.1
- standard-contracts#15.1
- standard-contracts#17.1
- standard-contracts#18.3
- standard-contracts#18.4
- standard-contracts#21.1
- standard-contracts#23.1
- standard-contracts#26.1

## Cascade only

- established-decisions#11.2
- established-decisions#14.1
- existing-scopes#2.2
- standard-contracts#18.1
- standard-contracts#22.2
