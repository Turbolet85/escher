@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=15 class=stale
leaf-text: a panicked path's check holding the steps before the panic.
source: ctr-wrq2o6ky «With a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green; a path whose accepted answer carries a panic item ends»
replace-with: a path whose accepted answer carries a panic item ending panicked with no payload, a re-reach step diverged, and its check holding the panicked step.
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=18 class=omission
leaf-text: returning after settle with the diff of the screen before and after,
source: ctr-bujbd4ha «`click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something;»
replace-with: returning after settle with the diff of the screen before and after, with `caused` where the step caused something,
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=37 class=omission
leaf-text: `the_counter_is_explored_and_each_path_replays_green`).
source: dec-xd4p3k72 «A handler Err the app's root boundary takes is checked on a session of its own, its sentinel check reading caused alone. · why: The root boundary replaces the app and renders the error on screen. · guard: stand_act_errors»
replace-with: `the_counter_is_explored_and_each_path_replays_green`); a handler Err the app's root boundary takes is checked on a session of its own, its sentinel check reading `caused` alone, the root boundary replacing the app and rendering the error on screen (`stand_act_errors`).
