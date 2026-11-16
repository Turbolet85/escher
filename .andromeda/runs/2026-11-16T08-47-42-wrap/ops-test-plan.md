@row_set n=1 row=contract/session-recording/exploration-checks cell=promise seen=1 fact=F6 detector=none
value: With a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green; a path whose accepted answer carries a panic item ends panicked with no payload, a re-reach step diverged, and its check holds the panicked step.
basis: fct-v2zu6qwi
because: A panicked exploration path's kept check now holds the panicked step where it held only the steps before it, read from the answer's panic item, so the promise must follow.
@row_set n=2 row=contract/driver-session/acting-verbs cell=promise seen=1 detector=none
value: `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
basis: fct-vzyy52tx
because: The five acting verbs gained the `caused` result field, last among their fields, so the row's list of what they return is incomplete without it.
@row_add n=1 row=contract/acting-call-answer/caused area=integration-test-strategy fact=F1 detector=none
boundary: acting call answer
item: caused
promise: Present only when the step caused something: one item per kind (panic, error, log) and level, with node id, owner id and event where known and a count, never a message, payload or error text; limited to the first handler reached, the app's root boundary alone and records of the log crate.
basis: fct-vzyy52tx
because: The acting verbs' answers gained the `caused` field with its item shape and stated limits, a promise at the driver's answer boundary no row states.
@row_add n=2 row=contract/acting-call-answer/caught-panic area=integration-test-strategy fact=F2 detector=none
boundary: acting call answer
item: caught panic
promise: After a caught panic in a step the session keeps answering and a later acting call answers with no caused; a panic in the settle runs the settle once more, and a second reads not settled, busy render.
basis: fct-xxn4lhqe
because: The acting step now runs under a panic catch with a stated settle retry, a session promise no row of the area states.
@row_add n=3 row=decision/root-boundary-err-on-own-session area=integration-test-strategy fact=F3 detector=none
name: Root-boundary Err checked on its own session
rule: A handler Err the app's root boundary takes is checked on a session of its own, its sentinel check reading caused alone.
why: The root boundary replaces the app and renders the error on screen.
guard: stand_act_errors
basis: fct-pxx4n4tk
because: The operator ruled the Err case onto its own session with a narrowed sentinel check, a test decision future checks of caused must respect.
@row_add n=4 row=decision/capture-framework-targets-at-warn area=3-test-harness-contract fact=F4 detector=none
name: Framework targets captured at warn and above
rule: The scoped log capture counts every level from targets outside the engine and escher_ prefixes, except the Dioxus framework targets (dioxus*, generational_box, warnings), counted only at warn and error.
why: The Dioxus framework targets trace on every re-render and signal write.
basis: fct-qepwv2ae
because: The operator ruled the framework target set counted at warn and above, a capture rule no row states beside the sink's allowlist.
@row_add n=5 row=decision/capture-dispatcher-pair-held area=3-test-harness-contract fact=F5 detector=none
name: Capture dispatcher pair held for the process
rule: The log capture keeps one process-wide dispatcher pair registered for the process, and a test installing its own recorder subscriber opens an empty capture first.
why: So tracing-core never falls into its one-dispatcher interest mode and caches a call site as never.
basis: fct-y36qbbsj
because: A per-thread dispatcher read red and a lone recorder cached the command span as never, so the pair and the open-a-capture-first convention bind every test that installs a subscriber.
@row_add n=6 row=decision/rustdoc-in-documentation-legs-alone area=9-ci-integration fact=F7 detector=none
name: Rustdoc checked in the documentation legs alone
rule: Rustdoc with denied warnings runs in CI's Documentation job and the local doc leg, not in the fast leg or the local gate block, so a doc-link ambiguity reads red only there.
basis: fct-vtmvqupi
because: A doc-link ambiguity passed every local gate and read red only in CI's Documentation job, so where rustdoc is checked is a CI guarantee the plan must state.
