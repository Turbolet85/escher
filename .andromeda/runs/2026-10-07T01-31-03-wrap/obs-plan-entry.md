
## 2026-10-07-change-tracking-and-diff — blitz-dom log-site citations re-pointed
**Section:** citations into blitz-dom `document.rs` and `mutator.rs`
**Change:** no claim changes. 14 of 16 citations re-pointed by the chunk's measured line map — 8 into `document.rs` (`+7` for old lines 1011–1548, `+11` from 1549) and 6 into `mutator.rs` (`+45` from old line 675); the 2 into blitz-shell `window.rs` keep their numbers.
**Why:** the chunk inserted lines above the cited log sites — the changed-set marks in the mutator, the drain and the mark in the document — and added no log, print, env read or file write: the census over its two new files and every line it added reads 0, and no agent-run event carries a content-named key.
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/
