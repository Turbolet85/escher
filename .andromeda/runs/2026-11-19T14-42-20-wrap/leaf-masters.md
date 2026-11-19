@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=15 class=tree
leaf-text: the cold-agent run pipe, the code-graph tooling
source: nam-isvpzpue «name/other/target-sweep-py — target-sweep.py»
replace-with: the cold-agent run pipe, the dev-host build-directory sweep (`target-sweep.py`, its Windows pass-through `target-sweep.ps1`), the code-graph tooling
@leaf_edit n=2 leaf=CLAUDE.md line=58 class=omission
leaf-text: Every build, test and CI cargo leg passes `--locked` against the committed `Cargo.lock`
source: dec-kuxsgya2 «Every build, the flake, the bundle, every CI cargo leg and every cargo run of the build-directory sweep passes --locked so the lockfile is verified.»
replace-with: Every build, test and CI cargo leg, and every cargo run of the build-directory sweep, passes `--locked` against the committed `Cargo.lock`
@leaf_edit n=3 leaf=CLAUDE.md line=108 class=omission
leaf-text: as CI runs them — it runs no rustdoc
source: ctr-nwhn7bv7 «After a green verdict, with `CI` unset or empty, it runs the target sweep, its line printed and its exit discarded; a red leg calls no sweep.»
replace-with: as CI runs them, then, on a dev host with `CI` unset or empty and after a green verdict, the build-directory sweep `scripts/target-sweep.py`, its line printed and its refusal or failure never reddening the leg (a red leg calls none) — it runs no rustdoc
