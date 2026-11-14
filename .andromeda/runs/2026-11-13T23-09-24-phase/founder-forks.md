# Founder forks — 2026-11-13-local-files-for-local-documents (phase, 2026-11-13)

The founder is away; the operator's invocation: «a fork that is his is listed whole in founder-forks.md with its
options and your lean, the others answered by the operator; never wait on him.» This chunk does not wait on this fork:
its scope does not need it, and the plan is complete without it. The answer decides only the owner of the carry the
P5 card's `## Beyond the plan` names.

## F1 — two ways a remote page still reaches a local file in the browser app, outside this chunk

**What stands after this chunk.** The engine's own sub-resource fetches (stylesheet, `@import`, `@font-face`, CSS
image, `<img>`, `<iframe src>`) from a non-`file:` document read no local file — the founder's 2026-11-12 word,
«Как в коде + работа», as relayed (`inputs#I1`). Two paths stay open, both in the reference browser app
(`apps/browser`), not in the engine:

1. **The favicon probe.** The browser fetches a page's `<link rel=icon href=…>` itself, with no loading document
   (`apps/browser/src/favicon.rs:49`), so a remote page naming a `file:` icon has that file read from disk; the bytes go
   to the tab strip as an image, never to the page.
2. **Navigation to a `file:` URL.** A link or form on a remote page that targets a `file:` URL goes through the
   browser's `NavigationProvider` and is loaded as a new top-level `file:` document (the user then sees it; that new
   document may load its own local files). Mainstream browsers refuse this navigation.

Neither lets the remote page's code read the file's bytes. Both change what a remote page can cause on the user's
disk, so whether to close them is a boundary call — the founder's.

**Options**
- **A — a new route entry, «Remote pages and local files in the browser» (Recommended):** placed at the end of Epoch 4,
  after «Auto-exploration»: the favicon probe stamped with the page as its loading document, and navigation from a
  non-`file:` document to a `file:` URL refused. Why it wins: it keeps «a remote page reads no local file» whole across
  the reference browser, in this version, and leaves this chunk at its relayed scope. Cost: one more Epoch 4 chunk.
- **B — recorded as a stated limit, no work:** the security plan names both paths as known gaps of the reference
  browser. Why one might choose it: the browser app is upstream's demo shell, and escher's agent surfaces never route a
  user- or agent-supplied URL through it. Cost: the gap stays documented and open.
- **C — fold into this chunk now:** both fixes ride this chunk. Cost: the P2 row read re-runs whole for the browser's
  surfaces (the pinned scope does not name `apps/browser`), and the chunk grows past its relayed item.

**Answered PROVISIONAL by the operator at the P5 review (`relay-1.md`): A, the route entry, for the founder to confirm
or replace.**

**Lean: A.** The operator's own answer is not needed for the plan to stand; the card's `## Beyond the plan` row names
option A as the proposed owner, to be confirmed or replaced on the founder's word at the wrap.
