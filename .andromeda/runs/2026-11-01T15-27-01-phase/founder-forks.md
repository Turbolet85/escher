# Upstream sync ahead of the UI graph — the founder's forks

Chunk `2026-11-01-upstream-sync-ui-graph`. Written at P4 in the away form on the operator's word (inputs#I2): every fork of his, the `node.rs:1775` fix first, in one file, **no provisional answer given**. The plan is not written and nothing is built until he answers.

Both forks change behaviour for every document, remote pages included. By the playbook's engine-change rule his own answer is the ratification, recorded with his word as he gives it; a delegate's answer would stand PROVISIONAL. Every reading below was measured on trial worktrees (`evidence/trial-takes.md`, `evidence/panic-reading.md`). The real tree is untouched.

**To answer:** one line per fork, its number and the option's letter, e.g. `F1 A`, or his own words. `all recommended` is fine.

| # | The question | Recommended | Size |
|---|---|---|---|
| F1 | A key press can crash the engine after a control's container stops showing | A — stop the crash, focus stays | very small |
| F2 | Which upstream take this sync lands | A — twelve commits, pins unchanged | medium, measured green |

The two are independent: any F1 goes with any F2.

---

## F1 — The key-press panic at `node.rs:1775`

**What happens.** A control sits beside a block, so the engine wraps it in an anonymous box. When an ancestor stops showing, that box is dropped, but the control keeps pointing at it. The next key press crashes the engine.

**What we measured, wider than recorded:**
- It panics in both layout modes for `hidden`, for `style="display:none"`, and **for CSS alone with no script** (`#root:focus-within { display: none; }`, one click, one key). So any remote page can do it.
- A control in no anonymous box does not panic.
- **Upstream has the same bug**, at its `node/node.rs:1740`, so no take of upstream's fixes it. A fix here is escher's, and upstreamable; nothing is sent.

### Options
- **A — Stop the crash; focus stays where it is (Recommended).** About seven lines: when the engine drops an anonymous box, the controls it held stop pointing at it. Every other reader of that stale pointer is cured with it.
  - Measured on the take: no panic in any case, both modes; `select_popup`'s skipped key press restored, 25 passed; every gate green.
  - What it leaves: a focused control that no longer shows keeps focus and still takes keys. Measured: Enter on a hidden `summary` opened its `details`. That is already what happens today where no anonymous box is involved, so this adds no new behaviour beyond the crash ending.
  - His word of 2026-10-31 stands as he gave it: «Снимать фокус без blur (Рекомендую)» covered `inert`, with `disabled` and `hidden` left out.
- **B — A, and a control that stops showing also gives up focus.** Closer to HTML's focus fixup: after the next layout, a focused node with no box loses focus, firing no `blur`, as his `inert` answer did, and keys go to the document.
  - It ends the hidden control taking keys.
  - It reaches past his 2026-10-31 word, adds keyboard and focus-order rows (SC 2.1.1, 2.4.3), and moves `focused` in the snapshot where a page hides what holds focus.
  - **Not measured:** no trial was built. Small to medium, in the resolve path.
- **C — Leave it.** No engine change. A remote page keeps a crash it can reach with CSS alone, `select_popup` keeps its skip, and the owner moves to a later entry. Not recommended: the fix is seven lines and measured.

---

## F2 — Which upstream take

`upstream/main` reads `e42ee1f25`. 57 commits lie past the merge base; 24 are already in; **33 remain**: the 16 left out at the last take and 17 new. His last two words were cherry-picks with the pins unchanged: «Десять коммитов (Рекомендую)» and «Одиннадцать коммитов (Рекомендую)». The route's entry says "merged", and this fork settles what that means here.

### Options
- **A — Twelve commits, pins unchanged (Recommended).** The same shape as his last two.
  - Measured with the thirteenth, `AGENTS.md`, in the tree and F1 A on top: **218 result lines · 1294 passed · 0 failed · 12 ignored**; fmt, clippy, the audit, doc and the CI scripts green; no manifest, lock or `deny.toml` line moves; every stand number, contrast row and Tab order holds; `font-embolden`'s witness green.
  - Against the last CI tally of 217 · 1236: +1 line and +58 passed, each named.
  - What it brings, for every document:
    - `fe33b778b` restyles less on a DOM change. Its risk is a missed restyle; the incremental oracle and 39 tests of its own read green.
    - `f6edb9701` a failed stylesheet no longer blocks rendering.
    - `6fca6308d` paints `mix-blend-mode` and `isolation`.
    - `dee854c76` a `<picture>` gets its own box.
    - `f76777766` an inline box holding only blocks stops painting its decorations over them.
    - `ae774d7f4` `getComputedStyle` width and height read unrounded, a value scripts see.
    - `91ef65e21` gives scripts `URLSearchParams`. Read as a realization on the registered script crossing, as the CSS accessors were, not a widening: no fetch, log or print added.
    - `8b1a0a3ce` layout damage for SVG attributes and `<col span>`; `7a3dd8918` preferences set before the default sheet parses, ported to our Stylo's names; `a26c6ddeb` `CustomWidget` attributes replayed; `6ef4b532d` a guard on a removed element's mounted event; `e42ee1f25` off-screen culling.
  - What stays out, 21 commits: git Stylo and what needs it (`06ecec886`, `3954ecb85`, `1c5265a8b`, `0674e8efa`, `784a567e4`, and `ac6f9f582` `background-clip: text`, `e5929f9e2` `-webkit-text-fill-color`), the Taffy-main commits and `0980cce5f` with `ebacd68bd`, Dioxus 0.8 (`286874cc1`), winit beta.4 (`a09f46792`, `527394724`), and `AGENTS.md`. Each moves to the next sync with its measured reason.
- **B — A plus winit beta.4 (fourteen).** A registry pin moved inside upstream's own set, no git dependency, but "pins unchanged" no longer holds.
  - Measured: **219 · 1297 · 0 · 12**, the audit and doc green.
  - The lock moves: `arboard`, `clipboard-win`, `error-code` leave, `x11rb` and `x11rb-protocol` enter, each reviewed by hand at implement. The clipboard goes through winit.
  - Passed over by him at the last sync. Nothing in this version needs it yet.
- **C — Merge `upstream/main` whole.** Git Stylo, git Taffy and a Dioxus alpha in the root manifest, reversing his rule of 2026-10-13 that no git dependency stands there.
  - Fifteen files, thirty conflict hunks. **Not built past the conflicts.**
  - Known work: porting escher's stable ids to Dioxus 0.8's vdom (10 + 15 errors measured 2026-10-24), and `0980cce5f` against the scrolled-box bounds and the hit, two of his ratified changes. It is a chunk or more of its own.
  - The one argument for it now: the UI graph's entries read components, handlers and state through the Dioxus bridge, so building them on 0.7 means porting that reading too when 0.8 is taken. **That cost is unmeasured.** Dioxus 0.8 is still an alpha.
- **D — Take nothing.** The whole 33 go to the next sync. The freight below still lands.

---

## What he is not being asked

These are the leans the plan will state. They are the operator's to change at the review, and listed so he sees them:
- **The folded work lands under any answer:** `matrix_test`'s dead free-disk step goes, skipped on all four rows again at CI#41248966449; a `font-embolden` run joins a local gate, on his «В ближайший Upstream sync (Рекомендую)»; the orphan «Re-export taffy» comment goes, upstream's own file holding the same orphan; the three merge-surface records are re-read row by row, nothing sent upstream.
- **`AGENTS.md` stays out.** Upstream's file tells an agent WPT is the main signal, not to add tests for what WPT covers, and to run cargo unlocked. All three contradict this fork, as a `README.md` conflict already resolves to escher's.
- **`Harness::text_contrast` reads `None` past a `mix-blend-mode` other than `normal`**, beside its `opacity`, filter and background-image conditions: a blended text's painted colour is not the computed pair. A test-harness reader, no engine change, and no stand row moves. The lint's `contrast` reads the same way.
- **Upstream's `6ef4b532d` test stays out.** It cannot read on Dioxus 0.7, where the element's mounted event fires first. Its guard is taken.
- **One new test-only loopback bind** comes with `f6edb9701`, in `blitz-net`'s unit tests (`127.0.0.1:0`): served by no binary, registered as test-only like the last one, no crossing.
- **The merge base stays `23b9b26a9`** under A, B or D, as at his last two takes.

## The answer

By dialog in the escher overseer session, 2026-11-01, the dialogs putting the forks to him in Russian; his labels relayed verbatim by the operator (`~/dev/projects/additional/escher-overseer/relays/upstream-sync-founder-2026-11-01.md`):

- F1 the key-press panic at `node.rs:1775`: «Убрать падение, фокус остаётся (Рекомендую)» — option A.
- F2 which upstream take: «Двенадцать коммитов (Рекомендую)» — option A.

The leans under «What he is not being asked» stand as the operator's, on the operator's word: the folded work lands; `AGENTS.md` stays out; `text_contrast` reads `None` past a `mix-blend-mode` other than `normal`; upstream's `6ef4b532d` test stays out; one test-only loopback bind; the merge base stays `23b9b26a9`.
