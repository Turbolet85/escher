@history_item n=1 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` (WCAG 2.x ratio from computed styles, large text by font metrics at 24 px or 14 pt bold) and `stand_contrast`'s table with a painted-pixel witness within 1 per channel. Matters: bundling the font set changes glyph shaping under that painted witness and the large-text reading rests on font metrics; the stand-contrast rows (decision/wcag-contrast-thresholds, decision/text-contrast-required-ratio, decision/stand-contrast-measured) must read unchanged on every CI OS.

@history_item n=2 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — (SC 1.4.3 met on the stand, three contrast tables) the stand accent darkened to `#3a5ce5` / `#2a4cd3` / `#1f3fc1` (white 5.47), `TEXT_ROWS` and `STATE_ROWS` added, `stand_contrast` put in the a11y leg. Matters: decision/stand-accent-colour and decision/stand-contrast-asserted are the readings a cross-OS comparison must hold equal; a font change must move no stand colour and no table row.

@history_item n=3 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the lint's `contrast` reads through `text_contrast` with the stand's exempt rule, no second formula or threshold, and the stand at boot reads no `contrast` finding on the four lean tasks in both layout modes. Matters: lint findings are one of the readings the scope names for cross-OS equality (decision/lint-contrast-single-formula, decision/lint-contrast-shares-one-reader, decision/stand-lint-clean-at-boot); also the gated part of the same chunk (`lint` verb) fixed the finding as `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only (contract/visual-lint-finding/content, contract/lint-contrast-finding/*) — `bounds` and `ratio` are font-sensitive numbers a cross-OS comparison will read.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password paints bullets laid out by `layout_as_text_input`; the driver's `screenshot` is one more reader of the model's bounds (element shot = bounds rounded outward, cropped) and a shot changes nothing. Matters: the scope rules out comparing shot bytes across OSes; contract/driver-screenshot/element-shot and decision/screenshot-changes-nothing stay, while bullet glyph widths now come from the bundled font.

@history_item n=5 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the password click caret measured by real glyph widths (index 5 vs 2), its fix a residual; later 2026-10-18-engine-debts placed the caret by the bullets. Trap: caret placement and text extents are font-dependent readings that bundling DejaVu Sans on every path must keep stable across hosts.

@history_item n=6 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the flight booker's select joins the stand tables (`flight-type` `#1a1a1a` on `#ffffff`, 17.40, 15px 400); a separate entry records the served pages (Cells, load page), the lint's option overlap at boot and the overlay scrollbar band. Matters: the shared fixture pages and the session's served tasks are exactly the run paths the scope must move onto the bundled set; font size/weight readings in the contrast rows are stated against them.

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (several sub-entries) the load page's and Cells' contrast rows (`the_load_page_meets_its_ratios`, `cells_meets_its_ratios`) and Tab orders; the lint's closed-select overlap ended so the stand reads no finding at boot `[0, 0, 0, 0]`; a `multiple` select's rows read their own client rects in the snapshot; `optgroup` modelled; the default sheet's chosen-row pair `#000000` on `#c8c8c8` hardcoded. Matters: decision/stand-contrast-measured guards, decision/stand-lint-clean-at-boot, contract/snapshot/multiple-select-rows-and-value, contract/snapshot/multiple-select-row-bounds and decision/multiple-select-chosen-row-pair all read bounds or text laid out by the font set this chunk bundles; the load page and Cells are served pages whose font context must be the bundled one.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the engine overlay select list on every document, remote pages included (decision/single-select-engine-overlay-list); rows one height with the select's font on one line, a larger authored font clipping (decision/select-list-rows-one-height); a group header heavier by synthesized bold on plain and emboldening builds; an open row's bounds its painted row's in the snapshot and diff; labels leave only as pixels, a row shot crops to it; the lint reads a highlighted row's plain pair (decision/highlighted-row-contrast-unread); engine colour constants measured by `select_popup_painted`; no motion (absence/select-list-motion); a click under the list refused `covered`; the changed set marks list open/close. Matters: row height, header weight and row bounds are font-derived readings; the boundary keeps remote pages' fonts unchanged, yet the overlay paints on them.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (sub-entries) `select_popup_painted` also runs in the `embolden` leg built with `blitz-paint/font-embolden`, macOS exercising the feature in CI; `text_contrast` reads `None` past a `mix-blend-mode` (decision/contrast-unread-past-non-colour-paint); blending paints with no frame test. Matters: the embolden leg is the existing cross-OS font-path precedent — a header heavier "on every build" must hold with the bundled font on Linux, macOS and Windows.

@history_item n=10 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally, no `[features]` table of its own, reaching both seven_guis binaries; no platform adapter built, lockfile byte-identical. Matters: decision/driver-requires-accessibility; the scope forbids a new dependency crate, and any feature change for fonts (e.g. `system-fonts`) must keep this reach and re-read the adapter absence.

@history_item n=11 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the stand's graph re-read) a lock move re-read with no platform adapter joining the stand graph, the adapter owned by residual `res-llkgqygv`. Precedent: every `Cargo.lock` or feature move re-reads `accesskit_(xplat|winit)` at 0 rows (as 2026-10-24-upstream-sync-polish-and-ship states: re-read at each lock change); applies if this chunk touches a font feature.

@history_item n=12 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the accessibility tree with no second mapping. Matters: decision/snapshot-model-single-source and decision/readers-of-one-snapshot-model — the cross-OS readings (snapshot text, bounds) must come from the one model, with no font-specific second reader.

@history_item n=13 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the tree's name sources: text runs label their parent, `aria-label`, `<label>`; the 15 stand controls named. Matters: decision/text-runs-label-their-parent — a run's text and its TextRun nodes are what the bundled font shapes; names must not move with the font.

@history_item n=14 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree refreshed on change; the changed set excludes layout; the diff reads the snapshot model. Matters: decision/a11y-tree-refresh-on-change and decision/changed-set-marks — a font-driven relayout marks nothing, so a cross-OS reading of a diff must not expect font changes to surface as changes.

@history_item n=15 marker=2026-10-12-self-description
**2026-10-12-self-description** — `DiffNode::differs` names readings in node-key order; the Flight Booker's Book names eight nodes that only moved, each `[bounds]` (2026-10-13-stand-keyboard-harness later: seven `[bounds]` plus `flight-book` `[focused, bounds]`). Trap: bounds-only diffs are font-sensitive and pinned by count; a text-extent shift under the bundled font could move them.

@history_item n=16 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the snapshot's text parts, one more reader of the model over `SNAPSHOT_TEXT_BUDGET`. Matters: contract/snapshot-text/parts; the snapshot text is the first reading the scope compares across OSes.

@history_item n=17 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a select's option excepted from `display: none` in the tree; the user-agent select box's contrast not measured. Matters: decision/hidden-content-excluded, contract/accessibility-tree-hidden-content/display-none, contract/accessibility-tree/hidden-and-inert-nodes and contract/snapshot/closed-select-option-bounds (options reading the select's client rect, a font-laid-out box).

@history_item n=18 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — (the enabled-state walks, the list named) every interactive-role node of the four lean tasks named; the `&dyn Document` tree equal to the document's. Matters: decision/adapter-tree-equals-harness-tree; the harness-built document is one of the run paths moving to the bundled font context.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the read-only UI graph reader `DioxusDocument::graph` gated behind `accessibility`, `signal_registry` ungated. Matters: decision/agent-readers-behind-accessibility; with 2026-11-08-what-can-i-do-here (actionable-node reader gated) and 2026-11-08-handler-coverage (reached record ungated), the gate list a font change must not disturb.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the driver's `type` on any select) `type` focuses any select with no click and closes any list, adding nothing to the driver's closed sets. Matters: decision/driver-type-focuses-select-no-click; the scope likewise forbids any new verb, argument, field, cause or log field.

@history_item n=21 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (sub-entries) the stand accent's home ratified at layout-templates (decision/stand-accent-colour); citations into the shell's page zoom re-pointed. Matters: decision/viewport-scheme-and-zoom-level and decision/hidpi-or-zoom-change-reshapes-text sit beside text reshaping the bundled font set feeds.

@history_item n=22 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files (`stand_contrast`, `stand_keyboard`, `select_interaction` among them), tallied by test count (later 114 at 2026-10-31-select-popup). Matters: the contrast and select checks the bundled font affects run in this leg; its tally is the measure a font change must leave unmoved.

@history_item n=23 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence), each verified against the code. Matters: every picked row's handle dates from this conversion; an amendment from this chunk is written as `+` / `~` / `−` on those handles.

@history_item n=24 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/changed-set-marks`: a scroll that closes a list now marks; any other scroll and layout do not. Matters: the latest wording of a picked row; a font-driven layout change still marks nothing.
