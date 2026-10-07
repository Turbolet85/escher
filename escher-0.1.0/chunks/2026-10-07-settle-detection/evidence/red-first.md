# Red before green — readings on the tree before the settle loop exists

Plan step 10. Taken 2026-10-07T10:43Z on the tree at `4ed27b53` plus one file: a temporary probe at
`tests/blitz-tests/tests/stand_settle.rs` holding the fixtures of tests 3 and 7 and the Timer step of test 1, with no
`settle` and no `act` (neither existed: `git status --short -- packages tests examples` read that one untracked file
and nothing else). Run: `cargo test -p blitz-tests --locked --test stand_settle -- --nocapture`, exit 0, one probe
test printing its readings. Both layout modes read the same on every line. The probe file was then replaced by the
real check, so these readings cannot be re-taken; the conditions of readings 1 and 2 are asserted on every run by the
control assertions inside tests 3 and 7.

| # | Subject | Step, with no settle | Reading | What the check asserts after the step | Red? |
|---|---|---|---|---|---|
| 1 | test 3's fixture — a `mounted` handler writes a signal a status text shows | one `click` on the reveal button (one pump) | the revealed element is present; the status reads its first text, `waiting` | the status reads `mounted` | **red** |
| 1a | same | one further poll-and-resolve pass | the status reads `mounted` | — | (the second pass is what was missing) |
| 2 | test 7's fixture — the upper box's click removes it, the lower box moves under the pointer and has a `:hover` width | one `click` on the upper box (one pump) | `hovered()` is the lower box; its rect is 100 × 40, the un-hovered size | its rect is 200 × 40 | **red** |
| 2a | same | one further poll-and-resolve pass | its rect is 200 × 40 | — | (the second pass is what was missing) |
| 3 | test 1's Timer step, in the plan's order — a bare `click` on `#timer-reset`, then `deliver(3)`, in place of `act` | nothing after the delivery | `Elapsed: 0.0s` | `Elapsed: 0.3s` | **red** |
| 3a | the Timer step in the other order — `deliver(3)`, then a bare `click` on `#timer-reset` | the click's one pump | `Elapsed: 0.3s` | `Elapsed: 0.3s` | not red |
| 3b | the Timer's first step — `deliver(151)`, then a bare click on the title | the click's one pump | `Elapsed: 15.0s` | `Elapsed: 15.0s` | not red |

## Against the plan's prediction

The plan predicted the Timer reading NOT red (research.md §Open questions: "an input helper's single pump after
`deliver` therefore already shows the ticks"). Measured: that holds only when the delivery comes BEFORE the input
helper (3a, 3b). In the order test 1 uses — the click first, the delivery after it — nothing pumps after the
delivery, so the ticks stay unapplied and the step reads `0.0s` (3). The Timer step of test 1 is red without the
settle that `act` makes.

What the red in reading 3 is and is not: it is the absence of any pass after the delivery — a hand `pump()` after
`deliver(3)` would also turn it green, which is how every standing check reads the timer today. It is not a case one
pump fails to cover. The cases one pump does not cover are readings 1 and 2.

## Other readings the probe took (design inputs, not part of step 10)

- `has_changes()` reads **true** on a freshly booted held session of each of the four lean tasks (nothing has
  drained the set the boot's mutations marked).
- `is_animating()` reads false on each of the four lean tasks at boot, and true on an HTML fixture with an infinite
  `@keyframes` animation after its constructor's one pump.
- An HTML fixture whose `:hover` rule sets `display: none` on the box under the pointer alternates its hover node on
  every pass: hovered-is-the-box read false, true, false, true, false, true over six passes.
- A Dioxus fixture whose `mounted` handler bumps the key of the element it mounts boots (its constructor's one pump
  returns).
