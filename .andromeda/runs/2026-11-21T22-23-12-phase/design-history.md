@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (file:line citations re-pointed after the CI files moved) re-pointed 4 citations into `ci.yml` and `publish-browser.yml`, among them the ios/android matrix entries that back decision/mobile-targets-built-not-tested. This chunk edits `.github/workflows/ci.yml` (skia's checksum step, the cancelled-runner watch's iOS/macOS jobs), so its line moves owe the same coordinates-only re-point. No claim text may change by it.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — (file:line citations re-pointed after ci.yml moved) moved the ios/android matrix citations 237-252 → 328-343 by a measured line map over `ci.yml`. This sets the precedent: a CI-file chunk re-points the cited mobile-matrix lines by a measured map. If this chunk retires, guards or removes the iOS job from the required set, the mobile-targets row's claim ("CI builds the iOS and Android targets but runs no tests on them") must still hold, or it needs a `~` amendment.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (citations re-pointed by the chunk's line moves) re-pointed design citations into `ci.yml` after its six-line free-disk step. This is the nearest precedent for a CI-step insertion: a new skia fetch/verify step in `ci.yml` moves the cited lines the same way, a citation-only amendment.

@history_item n=4 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — (file:line citations re-pointed after Cargo.toml moved) re-pointed 2 design citations into the root `Cargo.toml` after inserted members and workspace-dependency entries. Dependency moves in this chunk (`rusqlite`, `image`/`ravif`, `accesskit_unix`, `anyrender_serialize`) may shift those manifest lines, which owes the same re-point.

@history_item n=5 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — §Typography → Loading: the seven_guis headless stand registers the bundled DejaVu Sans for every generic, with system fonts off on native, so layout is the same on the dev host and the CI runner. This is the reason text checks do not depend on CI runner fonts. A CI image or dependency change must not break that pinning, and the decision/tests-built-with-system-fonts and decision/test-engine-with-system-fonts rows still need `blitz-dom`'s `system-fonts` feature in the blitz-tests build.

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — introduced the fonts-unavailable refusal (a screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted) and the screenshot answer line / MCP image block. These are the origin of contract/driver-screenshot/fonts-unavailable, contract/driver-screenshot-refusals/fonts-unavailable, decision/screenshot-never-skips-text, contract/escher-session-commands/screenshot and contract/escher-session-mcp/screenshot-result. They are founder-ratified (fork 1, 2026-10-12). Moving the `image`/`ravif`/`rav1e` chain (paste advisory) or the skia/render stack must leave the PNG encoding and the refusal intact.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — fixed the cli surface's output form: one uncoloured JSON line per command, no ESC byte, no terminal test, and exit classes 0/1/2/3, ratified by the founder. contract/escher-session-commands/screenshot rests on it. A dependency bump that pulls a colour or tty crate into `escher-session` would break it.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role (JSON-RPC lines only on stdout, no ESC byte). contract/escher-session-mcp/screenshot-result extends it. It stands as a constraint on any dependency move that reaches the session binary.

@history_item n=9 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's font registration leaves for architecture's Headless stand) moved the stand's bundled-DejaVu registration out of design-system typography to architecture §Standard Contracts → Headless stand. The fact that pins CI-independent font measurement therefore lives in architecture, not in these rows. Read it there when CI's font environment or the `woff` feature path moves.

@history_item n=10 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (F2's stand-row move ratified by the founder's own word) ratified the move of the stand font registration to architecture, so it is no longer PROVISIONAL. Typography's stand rows stay out of design-system.

@history_item n=11 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows (137 rows, verified against the code at edd0cfaea7f0). Every picked row's handle dates from here, among them decision/tests-built-with-system-fonts, decision/test-engine-with-system-fonts (two near-duplicate rows) and decision/mobile-targets-built-not-tested. A change this chunk makes to the system-fonts feature or the mobile CI jobs is an amendment against these handles (`~`/`−`), not a prose edit.

@history_item n=12 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Typography: 2 rows) `+ decision/unavailable-families-use-default-font`, upstream's default-font fallback, ratified as upstream's, guard `font_family_fallback`. The guard test runs only with system fonts in the test build. A CI or feature change that drops system fonts would let it pass vacuously (the trap decision/tests-built-with-system-fonts names).

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list paints last, whole past clips and opacity; no motion; what the tree, the snapshot and a shot read of an open list) these are the origins of decision/select-list-painted-after-root, decision/select-list-opens-without-motion, contract/screenshot/select-list-row and contract/select-accessibility/{chosen-option,open-attribute}. The list is an engine overlay through `paint_scene` alone. A renderer or skia move (verified archive, `SKIA_BINARIES_URL`) must keep its paint order and the screenshots that read it. The rule "no line of upstream's code changes for skia" keeps these intact.

@history_item n=14 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`mix-blend-mode` and `isolation` in the depth rules) extended the open list's paint-last row to "past the blend layers", which is the current text of decision/select-list-painted-after-root. Blending goes through `blitz-paint`'s `blend` module. It has no frame-level witness (an operator CARRY), so a renderer dependency move is not covered by a frame test here.

@history_item n=15 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ contract/select-accessibility/highlighted-row` and `~ contract/select-accessibility/open-select`: the snapshot's `highlighted` bool, and the active descendant naming the highlighted option. These promises are read through AccessKit. The zbus advisory's fix (`accesskit_unix` on zbus 6, coupled to the AccessKit 0.25 pin) must keep the tree's ComboBox/expanded/active-descendant readings.
