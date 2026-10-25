proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 13
    section: '## Motion (calibrated to expression level `unrecorded`)'
    change: '- While the overlay scrollbar shows after a scroll (opaque for `FADE_DELAY`, then fading over `FADE_DURATION`, on the document clock), a main-button pointerdown in its band is taken as a thumb drag and dispatches nothing, and a pointer left resting on the thumb keeps it shown; so a driver `click` at a point under the shown scrollbar right after `scroll` lands on the thumb, and an `advance` past the fade is needed before it (tests/blitz-tests/tests/stand_cells_served.rs:176-192 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)'
    sidecar: 'motion.md: the shown overlay scrollbar takes a click right after a driver scroll as a thumb drag, a resting pointer keeps it shown, an advance past the fade is needed first (F21, 2026-10-25-cold-agent-bar)'
    rationale: 'The report Changes F21 names this as a new fact with no master site (the scrollbar fade facts it rests on stand true), and Decisions and corrections measures it (a pointerdown in the band with scrollbar opacity above 0 is a thumb drag; on the virtual clock it stays shown until advance passes its fade; a pointer resting on the thumb keeps it shown); Motion holds the overlay scrollbar fade timings at line 13, so this consequence of the fade is its detail.'
    basis: tests/blitz-tests/tests/stand_cells_served.rs:176-192
    fact: F21
note: D-design-tokens holds — the two new UI surfaces (the load page, select#flight-type) read design-token✓ and the report names no transition or animation they add; the Retired flight-booker buttons appear nowhere in motion.md.
