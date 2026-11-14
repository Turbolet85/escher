@history_item n=1 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (Enter and Space activate a focused control, and a click focuses a button) set the engine rows behind `contract/keyboard-activation/enter`, `contract/keyboard-activation/space`, `decision/keyboard-activation-enter-space` and `decision/click-focuses-buttons`: a pressed Enter on an `a` with `href` dispatches a synthetic Click (a link takes no Space), the app's handlers then `handle_click`, in every document; a click on a button focuses it and submits a submit button's form owner. Founder-ratified («Клавиши нажимают», «Клик даёт фокус»). It matters here because a keyboard-activated link or submit on the top document reaches `BrowserNavProvider::navigate_to` through this path; the chunk's refusal of a remote page's `file:` navigation must sit in the provider and leave the activation and focus behaviour (and `stand_keyboard`, `stand_act_keys`) untouched.

@history_item n=2 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — measured that a click on a plain button left nothing focused, deferring whether it should; the precedent `decision/click-focuses-buttons` later superseded. Matters only as the row's history: the chunk changes no click focus.

@history_item n=3 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the founder's «Клик даёт фокус» first ratified there for the range input, the same ruling the button-focus row rests on; nothing in this chunk moves pointer focus.

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§5, §6: `inert` and `disabled` in focus order, keys and the hit) narrowed `disabled` to form controls (`ElementData::is_disabled`) for the click, activation and focus gates, and made a `disabled` attribute on a `summary` or an `a` gate neither Enter nor Space — the `except` clause of `decision/keyboard-activation-enter-space`; then (the PROVISIONAL batch ratified, Epoch 2 close) the activation keys' disabled gate ratified. Matters: a disabled-attributed link on a remote page still activates and reaches `navigate_to`, so the chunk's `file:` refusal must hold for it too.

@history_item n=5 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — added the scope entity behind `decision/accessibility-reach-tier-0` / `decision/a11y-tier-0`: `docs/capabilities.md` publishes `## Accessibility reach` (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, the AT reading unmeasured, named limits), its pin reading none of these claims, so a change is made there by hand. Matters: this chunk edits `docs/capabilities.md` (dropping two provisional clauses at lines 344 and 346–347); the edit must leave the Accessibility reach section and its named limits unchanged.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the capability statement's named limits) re-stated the limits `docs/capabilities.md` publishes under `## Accessibility reach`. Matters as a register in the file this chunk edits: no limit moves with the two dropped clauses.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the capability statement's named limits) narrowed the published limits to the focus fixup firing no `blur`, a single select's `size` unread and `aria-activedescendant` unread, and stated the list's thumb and row height as limits in `docs/capabilities.md`. Matters: the current text of the accessibility section in the file the chunk edits; leave it as it stands.

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — autofocus read by presence (`!= "false"`); kept the plan's app bullets on the browser's new-tab input autofocus as stating no value. Matters: the a11y plan holds a row on `apps/browser`'s new-tab page, one of the user's own top-level loads the chunk must leave unchanged.

@history_item n=9 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (citations into code the take moved, re-pointed) re-pointed the a11y plan's citations into the browser's `apps/browser/src/tab.rs` among others. Matters: the a11y plan cites `apps/browser` lines; if the chunk's edits (`history.rs`, `document_loader.rs`, `favicon.rs`, a new test module) shift a cited line, the wrap re-points it, claim unchanged.

@history_item n=10 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the stand's graph re-read on pulldown-cmark 0.13.6) the latest re-read behind `absence/assistive-technology-reach`: no platform adapter (`accesskit_xplat`, `accesskit_winit`) in the stand graph, the AT witness owned by residual `res-llkgqygv`. Matters: the chunk adds no dependency, so the absence stands with no re-read owed.

@history_item n=11 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the residual owner clauses re-said to their rows' status) the owner of `absence/assistive-technology-reach`, residual `res-llkgqygv`, reads re-carried at 0.3.0 (target: next). Matters: this chunk's wrap is the Epoch 4 boundary wrap; the absence and its owner stay as written.

@history_item n=12 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — fixed the `a11y` leg at fourteen files (founder's «Плюс проверки дерева»), the tier-0 witnesses among them. Matters: the chunk's new `favicon.rs` and `history.rs` unit tests run in the workspace `test` leg only; no leg list moves.

@history_item n=13 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the a11y plan's text into rows (241 rows), the origin of every handle in the picked rows (`decision/a11y-tier-0`, `decision/accessibility-reach-tier-0`, the two `contract/keyboard-activation/*`, `decision/click-focuses-buttons`, `decision/keyboard-activation-enter-space`, `absence/assistive-technology-reach`). Matters: any amendment this chunk owes is a row edit (`+`/`~`/`−` by handle); none of the picked rows is expected to move.
