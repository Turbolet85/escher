# Scrolling-box bounds and hit — the two founder forks (P4 of 2026-10-10-scrolling-box-bounds-and-hit)

> **Answered at P5, 2026-10-10, by the operator — as technical forks, not the founder's** (`inputs#I4`, the relay `scrolling-box-fork-answers.md`): fork 1 = **1B**, every box that clips by `overflow`; fork 2 = **2B**, the second reader measured and pinned here, its fix owned on the route. The operator's reason for the placement: neither fork widens a boundary or changes what the project is for — each decides how far a fix the founder already ruled reaches, inside the principle he set, and such forks are the operator's under the founder's standing delegation of technical decisions. This phase had placed them with the founder; that placement was wrong and is corrected here. The text below stands as it was written, before the answers.

Written for one sitting with the founder. Nothing below is chosen. Every option is whole; a recommendation is marked and argued, never assumed. An answer in his own words that fits no option is an answer too.

Why these are his (the operator's directive at the take-up, `inputs#I1`): "A fork that changes what an agent meets beyond restating the three limit sentences, or that widens a boundary, is the founder decision: put all such forks whole in one file with their dependences, as at the Driver CLI." Both forks below decide how far an engine change reaches for every document — the class his own word has decided each time so far (the changed-set contract, `scroll_into_view` for nested boxes, this entry's two fixes).

## What is already settled, and by whom
- **The founder, 2026-10-10** (by question dialog in the overseer session, relayed by the overseer): both defects are fixed in a chunk of their own before "MCP surface". Each fix is a direct change in the engine's files; additivity is a preference, never a goal — where staying additive would bend the logic the upstream code is changed, and the next sync's conflict is the accepted price. No upstream pull request is opened until DioxusLabs/blitz#1083 gets a reaction; both fixes are flagged upstreamable in the chunk's report.
- **The founder, 2026-10-07:** an engine function is fixed itself, for every document, with no second function beside it (the nested `scroll_into_view`).
- **The operator's, asked in this session's dialog and recorded in the plan:** where the bounds fix sits in the engine (the shared reader with its callers, or the client-rect function alone — the same readings for every document either way), and whether a conformance reading is taken before and after.
- **The operator's by the same directive:** what the tool's three limit sentences say once the fixes are in.
- **As built and not in question:** nine verbs, eight refusal causes with fixed remedies, the refusal order `disabled` · `off-screen` · `covered`, no new verb, argument, result field or cause, no log field added.

## What research found that bears on both forks
Read from the code at this phase; nothing below is measured on a fixed engine yet.
- **The bounds defect is one term in one reader, compiled in two bodies.** The reader sums position less scroll offset over a box's containing blocks and includes the box's own offset — which moves its content, not the box. A workspace build and a per-package build compile different bodies, so the fix lands in both and is checked under both.
- **The hit defect has two parts.** The walk tests a box's own area in scrolled coordinates, so a point up to the scroll offset above a scrolled box matches the box itself; and it goes on into a box's children whenever the point lies in the box's overflow, with no test of whether the box clips.
- **Paint already has the rule the hit lacks.** A box clips its children to its padding box when either `overflow` axis is not `visible` — and also when it is an image, a sub-document, a text input or carries `contain: paint`. The root element is the exception: its overflow belongs to the viewport.
- **`off-screen` already reads by the overflow half of that rule.** The driver reads whether a target can be seen from the visible region, which narrows by every box whose `overflow` is not `visible`, before it reads the hit. So a control clipped out of *its own* ancestor is refused `off-screen` today under every option below; the options differ only in which *other* box's clipped-out content can still read as covering a control.
- **A second engine reader carries the same shift.** `Node::absolute_position` also subtracts a box's own scroll offset. The snapshot and the driver's verbs do not read it. What reads it: the point of a synthetic click (a control activated from the keyboard), the IME cursor area of a focused text input, the event path's position of its target, the test harness's `layout_rect` and its debug dump. One of those — the text input's pointer path — may rest on the shifted value; not measured.

## How the forks depend on each other
| Earlier answer | What follows |
|---|---|
| Fork 1 = any | Decides what the restated `covered` sentence can truthfully say: under 1A and 1B a remainder stands (content clipped by something the hit does not stop at) and the operator restates the sentence to it or records it; under 1C none is known. Independent of fork 2 and of both operator forks. |
| Fork 2 = "Fix it here" | The chunk grows by a third engine change and its own checks; fork 1 is untouched. The plan as drafted for forks 1A / 2A is revised, not discarded. |
| Fork 2 = "Measure and pin" | One more check in this chunk, no engine change; the fix gets a route owner at the wrap. |
| Neither answered before the plan is approved | The plan takes the option that stays inside the ruling already given — 1A and 2A — and says so; a later word for a wider option is a revision of that plan (`/andromeda-phase`, revise) or an entry of its own. |

---

## Fork 1 — which boxes stop a hit at their edge

The entry's words are "a hit never reaches content scrolled out of its box". A walk cannot tell scrolled-out content from content that merely overflows: the measured case of an unscrolled box whose rows extend below it is the same case. So the fix is a stop at the edge of a box that clips, and the question is which boxes count. In every option the stop is at the padding box, the root element never stops a hit, and a point inside a box's own area is hit exactly as today, in the same stacking order.

**Question:** at which boxes does a hit stop — the boxes the entry names, or every box that hides what lies outside it?

### 1A — Scroll containers (the ruling as worded; what the plan takes if this fork is not answered)
A hit stops at a box whose `overflow` is `auto`, `scroll` or `hidden` — CSS's own "scroll container": a box whose content can be scrolled, by a user or by a program. This is exactly the entry: a scrolling box, and nothing wider.
What stays as it reads today: content clipped by `overflow: clip`, by `contain: paint`, or lying outside an image, a sub-document or a text input's own box is still reached by a hit, so a control lying where such content extends can still read `covered`. The stand has no such box.
Cost: the hit and the `off-screen` reading then use two different sets of boxes (the visible region already counts `overflow: clip`).
```
control BEFORE the box, a clipped-out row extending across it

box's overflow      today      1A         1B         1C
auto / scroll       covered    clicked    clicked    clicked
hidden              covered    clicked    clicked    clicked
clip                covered    covered    clicked    clicked
contain: paint      covered    covered    covered    clicked
```

### 1B — Every box that clips by `overflow` (recommended)
As 1A, and `overflow: clip` as well: a hit stops wherever either axis is not `visible`.
Why recommended: it is the one set the driver already reads `off-screen` by, so the two refusals agree about what hides content, and the hit follows paint's overflow rule exactly. The step past his words is one keyword. The engine change is the same lines either way.
What stays as it reads today: content clipped by `contain: paint`, or lying outside an image, a sub-document or a text input's own box.
Cost: one keyword wider than "a scrolling box" — an `overflow: clip` box never scrolls.

### 1C — Everything paint clips
As 1B, and the four classes paint clips besides: an image, a sub-document, a text input (at its content box) and `contain: paint`. What is not painted is not hit, with no remainder known.
Cost: the widest change, into paths this chunk has not measured — a pointer in a text input's padding, the forwarding of a hit into a sub-document. Each would need its own check before it could be called true, and a regression there is in typing and in framed content, the two places an agent's `type` and an app's embedded page live.

---

## Fork 2 — the second reader with the same shift

`Node::absolute_position` is not on the entry and is not what a snapshot's `bounds` or any driver verb reads. It was found while reading the callers of the reader that is on the entry. For a box that is itself scrolled it is hypothesised to read shifted by the box's own scroll offset, exactly as the bounds reader does — read from its code, not measured.

What that would mean where it is read: a control activated from the keyboard reports its synthetic click at a point off the control's centre by the control's own scroll offset; a focused, horizontally scrolled text input reports its IME cursor area shifted; the test harness's `layout_rect` and `center_of` say a scrolled box stands where it does not. `scroll_into_view` already hands the offset in to cancel it.

**Question:** is the second reader left, measured, or fixed in this chunk?
*Independent of fork 1.*

### 2A — Leave it; record it and give it a route owner (recommended; what the plan takes if this fork is not answered)
This chunk fixes the two defects he ruled on and nothing else. The finding goes into the chunk's report as read from the code and not measured, and the wrap places it on a route entry as a hypothesis, where taking it up is decided.
Why recommended: the chunk stays the one he ruled; nothing an agent meets through the driver today reads this function; and one of its callers may rest on the shifted value, so fixing it unmeasured could break typing into a scrolled text input.
Cost: a known, unmeasured wrong reading stays in the engine, and the harness helper built on it stays untrustworthy for a scrolled box — a check written with it could pass for the wrong reason.

### 2B — Measure it here and pin it as it reads; fix owned on the route
As at the Driver CLI (his answer then: measure and state). This chunk adds one check that reads the function for a scrolled box in both layout modes and pins the reading, red by design when it is fixed; the wrap gives the fix a route owner carrying the measured figures.
Cost: one more check in this chunk, and a pinned-wrong reading in the suite until its fix lands. No engine behaviour changes.

### 2C — Fix it here
The function stops subtracting a box's own scroll offset; each caller that needs the content origin takes the offset itself, the text input's pointer path measured first.
Cost: a third every-document change in upstream-owned code in one chunk — the synthetic click's coordinates and the IME area change for every app — and the largest check surface: keyboard activation, IME placement and typing into a scrolled text input each need a check of their own, in both layout modes and on the macOS and Windows CI legs.

---

## Where this run stands
Chunk `2026-10-10-scrolling-box-bounds-and-hit` is promoted (`pending`), with `scope.md` and `research.md` written and closed. The operator's technical forks are asked in this session's dialog. Whether the plan waits for these two answers or is written on 1A and 2A — the options inside the ruling already given — is the operator's to say; either way these answers are his, given as text. The run dir is `.andromeda/runs/2026-10-10T13-07-20-phase/`.
