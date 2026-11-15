@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — wired the Dioxus `accessibility` feature (workspace takes the Dioxus crates with `default-features = false`, a crate gets it only by naming it; seven_guis builds no platform adapter) and routed tree build/refresh through `Document::accessibility_tree`. Grounds `decision/dioxus-accessibility-feature-by-name`, `decision/a11y-tree-through-document-trait`, `decision/accessibility-tree-through-document-method`: a lock move that pulls a feature or an adapter in through Taffy's graph would break these.

@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally, no `[features]` table, so both seven_guis binaries compile the tree override but neither builds a platform adapter (`accesskit_xplat`/`accesskit_winit` in neither graph, lockfile byte-identical). Source of `decision/driver-requires-accessibility`; the method (dependency-graph reads before and after) is the precedent for measuring this chunk's `Cargo.lock` move.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's third dependency) — re-read the stand graph after a dependency addition: one lockfile line, no package, neither platform adapter joining. Precedent that every lock change re-reads the adapter absence.

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — on a merged lock where the accesskit family moved, re-read that the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit`. Precedent: a dependency move re-reads the adapter probe even when the moved crates are not a11y ones.

@history_item n=5 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — an upstream engine change for every document (the dropped-node hit skip) ratified by the founder's word on the take, and the stand graph re-read on the take ("the adapter's absence from the stand is re-read at each lock change"). Both the ratification precedent for this chunk's layout-change fork (scope: whether «Снять для Taffy» rules every layout change) and the re-read rule `Cargo.lock`'s Taffy move triggers.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read; `:open`; CSS accessors; IME; citations re-pointed) — re-read at `ebacd68bd` with the probe `cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows; upstream engine changes for every document taken on the founder's word on the take, which "ratifies for this take an engine behaviour change for every document"; `decision/script-style-through-inline-attribute` born here; citations into moved upstream code re-pointed by the take's line map. The exact probe this chunk re-runs, and the ratification form its layout-change fork must name.

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand's graph re-read on the lock's move; served pages' Tab, keyboard and contrast rows; `multiple`/`optgroup`; focus fixup; tree focus fallback; lint overlap; non-invertible transform hit; chosen-row pair; a11y leg tally) — re-read the adapter probe on a lock-only move (`pulldown-cmark` 0.13.5, the operator's call), and built many rows this chunk's layout move can disturb: `decision/stated-tab-order-walked` (load page and Cells, arrows bringing a cell into view), `decision/multiple-select-chosen-row-pair`, `contract/snapshot/multiple-select-row-bounds` (own client rects), `decision/focus-fixup-reads-inert-alone`, `decision/disabled-optgroup-one-rule`, `contract/multiple-select/keys-and-row-clicks`; a11y leg 14 files · 112. Layout moves shift bounds and scroll-into-view that these pins read.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list entity; changed set; `expanded`; open list keys, rows, bounds, colours, lint; `STATE_ROWS`; leg counts) — the engine overlay list answered first by hit testing and painted after the root; open rows take one height (`decision/select-list-rows-one-height`), an open row's bounds is its painted row's (`contract/snapshot/open-list-row-bounds`), a driver click under the list is refused `covered`, scroll closing a list marks the changed set (`decision/changed-set-marks`), a long list's rows scroll by `MAX_ROWS`; `select_popup`/`select_popup_painted`/`stand_act_popup` outside the a11y leg, which stands at 14 files · 114. Rows whose geometry and paint a Taffy layout move can shift; the checks the measurement must read green.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (adapter re-read on the take; focus limit; key press past a dropped anonymous block; `text_contrast` `None` past a blend; blending paints; `select_popup_painted` in `embolden`) — re-read probe 0 rows with manifests/lock unchanged; `key_press_after_container_lost_its_box` pins focus past a dropped anonymous block (layout-construction territory a Taffy bump touches); `decision/contrast-unread-past-non-colour-paint` added; `select_popup_painted` also runs in the `fast` leg's `embolden` step this chunk's measurement runs. Kept: the a11y tally 14 · 114.

@history_item n=10 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — another lock-only move re-read for the adapter probe, the owner of the windowed AT witness re-pointed to residual `res-llkgqygv` (`absence/assistive-technology-reach`). The most recent precedent for how a `Cargo.lock` change is recorded in §1.

@history_item n=11 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the AT witness at the platform adapter owned by a re-carried residual; `docs/capabilities.md` `## Accessibility reach` publishes tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, the a11y leg's files, named limits, hand-maintained. If the Taffy move changes any stated limit or the leg's counts, that document is edited by hand.

@history_item n=12 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — SC 1.4.3 met on the stand (accent `#3a5ce5`, `decision/stand-accent-colour`), three contrast tables, a11y leg widened to six files on the founder's F4 (no new job, no required status check), the enabled-state Tab walks. Sources of `decision/stand-contrast-asserted`, `decision/a11y-ci-leg`, `decision/stated-tab-order-walked`: the stand checks this chunk's measurement must read unchanged after the layout move.

@history_item n=13 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files under `cargo test --workspace --locked`, eight also in `stand-repeat`; "the stand's Tab order, Enter and Space and contrast hold after 0.2.0's engine changes, nothing re-baselined". Precedent: after an engine-wide change the stand's a11y readings are re-confirmed, not re-pinned; `--locked` on the leg must stay with a git Taffy.

@history_item n=14 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the stand a11y files also run three rounds in `stand-repeat` under the zero-failure budget (`decision/stand-checks-repeated-three-rounds`, `decision/a11y-test-leg`); a layout-induced flake fails the budget.

@history_item n=15 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created the `a11y` CI job "Accessibility (a11y) tests" running `ci-leg.sh a11y` with `--locked` in the slow tier behind the fast jobs (`decision/a11y-ci-gate`, `decision/a11y-ci-leg`); the leg a git-sourced lock must keep passing.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages; the scrollbar's band; the flight booker's select) — after a `scroll` a click in the shown overlay scrollbar band is a thumb drag (`decision/driver-click-in-shown-scrollbar-drags`); the flight-type select's contrast and Tab rows; the snapshot's closed-select option bounds (`contract/snapshot/closed-select-option-bounds`). Layout-dependent geometry the Taffy move may perturb.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb; the actionable counts at boot) — `contract/actions-answer/causes`: `crud-person-14` reads `off-screen` until a `scroll`, Cells 461 of 678 `off-screen`, `ACTIONABLE_AT_BOOT` pinned in both layout modes. Off-screen causes are layout-derived, so a Taffy layout change can move these pinned counts.

@history_item n=18 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's element shot is the node's bounds rounded outward and cropped to the visible part (`contract/driver-screenshot/element-shot`); bounds are Taffy layout output, so a layout move changes shots.

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space; `disabled` by presence) — Enter/Space activation, click focuses a button, `stand_keyboard`; the diff's Flight Booker Book names bounds-only nodes (`[bounds]`), `contract/snapshot-diff/differs` — a pinned diff that moves if layout moves node bounds.

@history_item n=20 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` (WCAG 2.x, 4.5 / 3.0 large at 24 px or 14 pt bold) with a painted-pixel witness within 1 per channel (`decision/wcag-contrast-thresholds`, `decision/text-contrast-required-ratio`); large-text classification and the painted witness read laid-out text, so the measurement must read `stand_contrast` green.

@history_item n=21 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses) — `res-llkgqygv` (stand platform adapter and its windowed AT witness) re-carried at 0.3.0, target next: the owner `absence/assistive-technology-reach` names; the Taffy chunk does not discharge it.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record) — states that the wasm build compiles dioxus-native-dom without `accessibility`, why the record reads `element_listener_ids` not `element_facts`; bears on the scope's wasm-leg measurement (`examples/wasm_hello`'s own lock) and on `decision/agent-readers-behind-accessibility`.

@history_item n=23 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the a11y plan became rows (241 rows), each verified against the code; the picked rows' handles and guards come from it, so the chunk's amendments are written as `+`/`~`/`−` row entries.

@history_item n=24 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy; 5. Keyboard Navigation; 7. Screen Reader Support) — `~ decision/changed-set-marks` (a scroll that closes a list marks), `+ decision/hidden-checkable-label-toggle` and `+ decision/empty-alt-image-decorative`, each an upstream behaviour taken on a take and ratified («Утвердить все 19»). The latest precedent for recording a take's every-document behaviour changes as rows, which this chunk's upstream Taffy chain may also bring.
