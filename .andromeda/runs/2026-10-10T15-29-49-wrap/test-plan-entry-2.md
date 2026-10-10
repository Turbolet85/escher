
## 2026-10-10-scrolling-box-bounds-and-hit — three engine checks, the re-measured counts, and the first conformance reading
**Section:** §1 Coverage scope → blitz-dom (layout) · §1 Coverage scope → tests/blitz-tests · §2 Test levels observed → WPT conformance · §2 Directory pattern · §9 Pipeline facts → Engine features by runner · §9 Pipeline facts → Local baseline
**Change:**
- §1 tests/blitz-tests: gains three engine checks on parsed pages, each in both layout modes and green under the per-package and the workspace build — `scrolled_box_client_rect` 6, `hit_clipped_at_scrolling_box` 9, `scrolled_box_absolute_position` 1 — with what each case reads, and the gaps: no check reads a box both transformed and scrolled or a scrolled box under a vertical writing mode, and a hit at an image's, a sub-document's or a text input's own box is not measured.
- §1 blitz-dom (layout): the `writing_mode.rs` body of the bounds reader is read by `scrolled_box_client_rect` under the workspace build, on horizontal pages only.
- §2 Directory pattern: 106 `.rs` files, 105 test targets and the unbuilt `all.rs` (was 103 and 102).
- §9 Local baseline: 165 result lines, 776 passed · 0 failed · 11 ignored (was 162 and 760); +16 in the three new result lines.
- §9 Engine features by runner: each of the reader's two bodies now holds a check of ours, its two mutation controls each red under its own build only.
- §2 WPT conformance: the first conformance reading taken on the fork's dev host — `css/cssom-view` at the pinned WPT commit, 966 → 967 subtests passed of 2292 run, one moved fail to pass, none pass to fail; report-only.
**Why:** the two engine fixes each needed a regression check under both builds, because the workspace build and the per-package build compile different bodies of the reader, and a fix in one body alone reads green under the other build. The vertical-writing-mode gap was measured at the wrap, not by the chunk. Not re-measured: "no `all` executable among 107" in §2, a 2026-10-10-upstream-sync-agent-surfaces reading.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/
