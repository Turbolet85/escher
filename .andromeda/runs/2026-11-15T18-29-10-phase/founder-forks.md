# Founder forks — 2026-11-15-taffy-from-upstream-git

Raised at P3 of the take-up of «Taffy from upstream's git», 2026-11-15. The phase stopped here on the operator's
directive (`inputs#I1`): the move cannot be taken cleanly as scoped, and what remains is not the operator's or the
agent's to settle. The measurement: `chunks/2026-11-15-taffy-from-upstream-git/evidence/measurement.md`.

## What was measured

Upstream's 13 Taffy-bump commits (`1a11761bc` … `b4952e15f`, toward git Taffy `a3997d55`) apply cleanly to HEAD
`0dba5035b`. `Cargo.lock` moves only `taffy`, and the audit reads `advisories ok, licenses ok`. **But `blitz-dom` does
not compile** — 27 errors, the same under stable and Rust 1.91. Taffy #1372 (`c2560562`) turned the scrollable
overflow rect into a new type, `ScrollableOverflow`. Upstream adapts blitz to it in `0980cce5f` (reversed scroll
origins, #1294), a commit this tree has left out since 2026-11-13 because it collides with escher's scrolled-box
bounds and hit changes. Picked here, it conflicts in exactly those files: `document.rs`, `layout/writing_mode.rs`
and `node/node.rs`. So every Taffy pin from `c2560562` on needs blitz-side scroll, bounds and hit code that this
tree does not hold.

## F1 — how, if at all, Taffy moves to git

Each option that moves Taffy changes layout for every document, remote pages included.

- **A. Take upstream's adaptation.** Pick `0980cce5f` and its panic fix `ebacd68bd` with the 13 commits, and
  re-reconcile them with escher's ratified scrolled-box bounds, hit and absolute-position changes. Reversed scroll
  origins come in as an upstream behaviour change for every document. It is the largest take, and its conflicts sit
  on ratified engine changes; its size and its gates are unmeasured.
- **B. escher adapts blitz to `ScrollableOverflow` itself.** About 27 sites in 7 `blitz-dom` files, reading the new
  accessors with no reversed-origin handling of escher's own. `0980cce5f` stays out. It is a direct engine change for
  every document, flagged upstreamable, and it widens the merge surface against `0980cce5f` for every later sync.
  Unmeasured.
- **C. Pin git Taffy at `e90751fd`, the last rev before #1372** (`ccfe15332`'s pin). That carries #1303, `1a11761bc`'s
  caller-defined float id and 8 of the 13 commits. The five later bumps (`35cdfc859` · `930448918` · `c51dd246c` ·
  `752ed2e49` · `b4952e15f`) wait for A or B. It is the smallest move, but its build and gates are unmeasured, and
  this chunk would need re-scoping.
- **D. Do not move now.** Taffy stays the registry 0.15.1. The entry is re-scoped or deferred until a registry
  release (0.16+) carries #1303 and #1372, or until a later sync takes `0980cce5f`.

## F2 — the chain's layout changes, for every document

Asked only if F1 is A, B or C. Under the playbook rule «An engine behaviour change for every document»
(`rul-47a6zcoz`) and architecture `decision/upstream-behaviour-changes-ratified-per-take`, upstream's layout changes
for every document are taken only on your own word for this take. Examples: empty flex containers skipping
processing, the stretch → flex-start fallback on overflowing lines, fitting margins and the baseline shim when
clamping to fixed max tracks, collapse-through margins no longer double-counted, percentage-width replaced
min-content contributions, and float insets and zero-height float segments. Does «Снять для Taffy» stand as that
ratification, or is the chain ratified (or refused) here, as `taffy 0.15.0 → 0.15.1` was at its own take?

## What does not need a ruling

- `Cargo.lock`: one package moves, with no second git source and no `[patch]`. That sits inside the Taffy-only
  exception (architecture `decision/no-git-dependencies`, security-plan `decision/no-git-dependency` and
  `decision/no-git-dependencies`, all three read with the exception as landed).
- Licence: git Taffy is MIT. Rust-version: 1.85, under the workspace's 1.91.
- The rows that a landed move turns stale — architecture `decision/taffy-layout` ("Taffy 0.15 from the crates.io
  registry") and `decision/deferred-upstream-dependency-bumps`, security-plan `decision/engine-dependency-pins` — are
  the wrap's amendments under your «Снять для Taffy», not new rulings.

The chunk stands promoted and `pending`, with its scope and research in the store. After your answer, `/andromeda-phase`
resumes the take-up at P4 (Setup 3 (c)), or the operator cancels it.
