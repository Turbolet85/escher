# Session Handoff

**Last Updated:** 2026-11-23T16:56:00Z
**Branch:** build/escher-0.4.0 · 0 ahead of origin/build/escher-0.4.0 as read at this wrap's Setup (HEAD `0ebf3c5ef`, the pre-CI commit)
**Status:** clean
**Last Commit:** 2026-11-23-canvas-shot-across-hosts — the wrap (see `git log -1`)

## Position
- Done: `2026-11-23-canvas-shot-across-hosts`. A canvas-only shot's bytes are now held equal on linux x86_64, windows x86_64 and macOS aarch64. The holder is `stand_canvas_cross_os`: three canvas-only fixtures, whole and element shots, both layout modes, twelve answer digests in one committed table, no pixels committed. CI#45209188300 read green 17/17 on `0ebf3c5ef`, and the test read `5 passed` on all three hosts. Nothing claimed: `v040-09` stays pooled, and its ledger note records this cross-host part. Wrap run `2026-11-23T16-32-42-wrap`.
- Next: «Circle Drawer on a canvas» (v040-09), Epoch 3's last entry — `/andromeda-phase` to promote and plan it. Its wrap closes the epoch, so it runs the whole-mode leaf pass.

## Work done
- Rows, ratified on the founder's F1 «A: требовать побайтовое равенство (Recommended)» (inputs#I3, relayed verbatim). Each sidecar entry carries his label and that source:
  - test-plan (TX 3407, 3409, 3410): the three no-shot cross-OS rows gain the canvas-only `except`, and each `why` is restated to the measured cause; `+ decision/canvas-shots-cross-os-by-digest`. The `why` of `no-screenshot-step-in-committed-checks`, `checks-hold-no-screenshot-step` and `no-screenshot-in-recorded-check-scripts` is scoped to a shot holding document text;
  - architecture (TX 3406, 3408): `decision/no-screenshot-in-cross-os-byte-check` gains the same `except` and `why`, and `decision/committed-stand-checks`'s `why` is scoped.
- No row rests on a delegate's word alone, so none is PROVISIONAL.
- The two items not put to the founder (narrowing the rule to macOS for page-text shots; `apple-font-embolden` off in test builds) land as no rule. Each is a line in `escher-0.5.0-incubator/deferred-from-0.4.0.md`, with `v040-09`'s pooled state.
- Route: a `WATCH:` on «Circle Drawer on a canvas» (`cry-vnddnt4v`), from the P5 card: its canvas element shot joins `stand_canvas_cross_os`'s table.
- Leaf pass: 10 findings applied, 0 declined. Every swept line was answered by a finding, and no comparer was re-spawned. No leaf line was written by hand.

## Drift resolved
- 15 proposer operations, plus 6 of main's own:
  - 11 landed (fit `fits`);
  - 2 rejected at check 2 (a design-system contract and an obs-plan decision restating the new test-plan decision);
  - main's 4 `why` sets restate the measured cause, and its 2 cascade `why` sets were caught by its own sweep.
- 0 escalations, 0 open; rule `rul-izni57qv` (boundary widening) read not met, so there was no answer card.

## Notes
- Upstreamable, not yet sent: the paint placement (#1578), the widget pointer frame, `Widget::fonts`, `BaseDocument::font_ctx()`. Carried: the single list box; the post-layout focus fixup and the Tab walk's skip of unshown nodes; the inline-box placeholder defect (parley 0.13); the W8 fragment mask as a stamped `data:` URL; upstream's `docs/licensing.md` on `webpki-root-certs`.
- Carried:
  - security-plan `dec-kxrtstbq`'s `verified_was` still quotes `Cargo.lock:8040-8041`;
  - `host-linux.md` is 9 template lines behind (U04 `noted`);
  - the stale «24 `stand_*` readers» in `verification-harness.md` (CARRY `cry-p6543j6z`, Epoch 3's whole-mode leaf pass) — re-read standing at this wrap.
- Left in place on the operator's word: the local branch `build/probe-canvas-shot` and its worktree `target/tmp/probe-canvas-shot`; `target/tmp/trial-canvas`, `target/tmp/trial-dioxus8`, `target/tmp/trial-editor` (git worktrees); `target/trial-dioxus8-build`; the scratchpads (`auditclone` among them).
- Left for the operator to remove: `target/mutants-peak`, `target/mutants-tmp`, `target/mutants-raw-2026-11-20/`, `target/tmp/upstream-547d`, `target/p5-baseline-debug`, `target/cold-agent/` (`bash scripts/cold-agent.sh cleanup`), `target/tmp/smoke-order-1`, `target/tmp/trial-pas`, `target/tmp/escher-session.head`, and the scratch clones `…/scratchpad/guardclone` and `…/guardclone-impl`.
- Last failed command: none.
