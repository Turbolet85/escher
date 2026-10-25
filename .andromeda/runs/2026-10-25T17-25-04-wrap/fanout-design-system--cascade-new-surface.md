proposals:
  - detector: D-new-detail
    severity: warning
    fact: F10
    file: .andromeda/design-system-detail/surface.md
    near: 35
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New group after line 35 — `<!-- detail: values · desktop-native-stand-load-page -->` holding one bullet: `- The stand's load page (since 2026-10-25-cold-agent-bar, `seven_guis::stand::boot_load`, mounted in `TaskShell` titled \"Load\") holds two keyed elements, the Load button `load-fetch` and the Loaded paragraph `load-done` (`display: none` until its sheet lands); no CSS selects by id — its classes are `load-root`, `load-body`, `load-btn` (with `:hover` and `:active`) and `load-done` (examples/seven_guis/src/stand.rs:185-200; examples/seven_guis/src/stand.rs:203-243)`, then `<!-- /detail -->`"
    sidecar: "design-system surface.md: the stand's load page — its two keyed elements and its id-free classes — added under Surface: desktop-native › Component Patterns (2026-10-25-cold-agent-bar)"
    rationale: "The report's Changes › Symbols / APIs states `boot_load` mounts the load page in `TaskShell` titled \"Load\" with `LoadPage` at stand.rs:185-200 and its CSS `.load-root` · `.load-body` · `.load-btn` (+`:hover` `:active`) · `.load-done` at 203-243, and that the page's ids are `load-fetch` and `load-done` with no CSS selecting by id; Coverage of new surfaces reads the page's two elements keyed; no group of the file holds the stand's served pages, so a new values group under the desktop-native component patterns, after the devtools-overlay group, takes it"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:39
note: the Load button's colours (F2) and the stand-in's 200 ms answer (F8) are color-palette's and motion's facts and are left out of this group
