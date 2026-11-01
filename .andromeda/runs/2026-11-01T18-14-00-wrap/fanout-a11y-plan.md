proposals:
  - detector: D-a11y-surface
    severity: warning
    section: §5 Keyboard Navigation
    change: "In Focus order, beside the fixup sentence at a11y-plan.md:107, add a new sentence: the fixup reads `is_inert()` alone, so a focused control that no longer shows, by `hidden`, an inline `display:none` or CSS alone, keeps focus and takes keys, as `key_press_after_container_lost_its_box` asserts in both layout modes; the limit owned by the `CARRY:` on the entry «A11y re-read»."
    sidecar: "§5 Focus order gains the limit the panic's fix leaves: a focused control that no longer shows keeps focus and takes keys, its owner the `CARRY:` on «A11y re-read»."
    rationale: "report.md:58 says the limit stands on no master row and owes a new sentence in a11y-plan §5; report.md:23 has `DocumentMutator::flush` unedited, still reading `is_inert()` alone, focus staying and the key still acting; report.md:81 names the owner; report.md:140 states the limit that stays."
    basis: "packages/blitz-dom/src/document.rs:1493-1503"
  - detector: D-a11y-surface
    severity: warning
    section: §6 Visual Design Verification
    change: "In Color contrast, the `text_contrast` parenthesis at a11y-plan.md:160 now reads: `None` under opacity below 1, a filter, a background image or a `mix-blend-mode` other than `normal`."
    sidecar: "§6 Color contrast: `Harness::text_contrast`'s `None` list gains a `mix-blend-mode` other than `normal`, since blending now paints; no stand row moves."
    rationale: "report.md:24 has `text_contrast` read `None` past an element on the walk with a `mix-blend-mode` other than `normal`; report.md:44 marks a11y-plan.md:160 changed where the line states its `None` conditions; report.md:99 carries the amendment."
    basis: "packages/blitz-test-harness/src/contrast.rs:154-156"
note: "D-a11y-surface is the nearest detector for both proposals, though the chunk adds no interactive UI element: the first is the report's owed focus sentence and the second its owed `None` condition. D-a11y-obs-schema did not fire, report.md:31 moving no violation schema or scrub set. D-retired did not fire, no retired item standing in this body or the key file. The plan's §3 takes no proposal: its contrast paragraph and the key file name the reader and state no `None` conditions, and report.md:44 holds such rows no-change. For the cascade: the detail's `None` list stands in the §6 opening contrast paragraph, not on the row report.md:44 names (visual-design-verification.md:13)."
