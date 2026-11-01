# Classified — arch · standard-contracts.b4

## Answers
standard-contracts#21.1 · yes · the kept checks stand_replay replays under tests/blitz-tests/checks/ hold snapshot answers whole, so a re-measured moved number re-records them under the 400-record and 1,048,576-byte bounds, read strictly by Check::from_bytes
standard-contracts#21.2 · yes · Check::replay is the path stand_replay runs over those kept checks, so a moved number shows there as the first step whose kept answer differs
standard-contracts#21.3 · yes · stand_lint pins the lint's readings (at_boot [0,0,0,0], the planted row pair, truncated and clipped) in both layout modes, re-asserted by the proof gates, and reads snapshot bounds the take can move
standard-contracts#21.4 · yes · the scope names settle's report of a load in flight (f6edb9701), which settle_waiting polls on NotSettled busy loads; stand_replay's acting steps wait through it
standard-contracts#22.1 · yes · stand_lint's at_boot zeros, the open list's row bounds, and the multiple select's row client rects are the values the re-asserted stand checks read, against the engine-debts merge surface
standard-contracts#22.2 · yes · stand_load_page is a stand check re-asserted in both layout modes, and f6edb9701 moves settle's load-in-flight report that its click on load-fetch waits through
standard-contracts#23.1 · yes · bootstrap.rs (BOOTSTRAP_JS concat of nine scripts) and events.rs (JS preventDefault fed back into EventState) are predicted conflict surfaces, and the scope names the bootstrap's globals
standard-contracts#24.1 · yes · the scope names the document_may_load gate on fetch(), module import and script src, which conflicts in bootstrap.rs and events.rs must keep
standard-contracts#25.1 · yes · the CSSOM accessors are installed by the bootstrap's style.js inside the BOOTSTRAP_JS concat that the scope names through bootstrap.rs, and the stylo pin move shifts their name set
standard-contracts#26.1 · no · the dioxus-native launch, Config, hooks and renderer surface in packages/dioxus-native is not reached or named; a winit pin move is a build gate every change owes, with no rule here for the chunk to act on
