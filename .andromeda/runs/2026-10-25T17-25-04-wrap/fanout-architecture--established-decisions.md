proposals:
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
note: D-arch-decisions and D-arch-resources read clean for this category (Dependencies none, LoadNet a hand-written NetProvider over blitz-traits, no driver schema, carrier or dependency moved; the retired aria-pressed buttons are named nowhere in this file, line 64's engine aria-pressed read stands); F21's other fact, the driver lint reading a select's two options as one `overlap`, belongs to Standard Contracts → Driver session, not here.
