@history_item n=1 marker=2026-10-06-project-readme
**2026-10-06-project-readme** (README is escher's front page) — added the Front page bullet: the root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version; no build, doc or test step read it then. Origin of decision/readme-is-escher-own and decision/readme-is-escher-page: this chunk's heading edit is escher-owned text, and a later sync must keep escher's side.

@history_item n=2 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (the front page states the version state; «Escher» in its prose) — the README gained `## Status` with 0.1.0 «done and verified» and its recorded facts, and the name «Escher» capitalised in prose and title, crate, binary and path names, URLs and the e-mail unchanged. The precedent for the «done and verified» heading form this chunk gives 0.3.0, and the source of decision/escher-capitalised-in-prose and decision/escher-name-spelling, which the edited heading line must keep.

@history_item n=3 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the front page: 0.2.0 done and verified, 0.3.0 planned) — restated the Front page clause: 0.1.0 and 0.2.0 each «done and verified» with their recorded facts, 0.3.0 planned in six stages; the 0.2.0 figures block and the clause landed together because the block contradicted the clause until both did. Direct precedent for closing a version's heading in the same form; a heading change may leave the Front page row's wording of 0.3.0's state stale, which the scope leaves to the wrap.

@history_item n=4 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the default branch GitHub renders the front page from) — recorded that GitHub renders the root `README.md` of the default branch, then `build/escher-0.3.0`, read by `gh repo view … defaultBranchRef` and `gh api …/readme`; until the switch a build branch's README edit did not show. Trap for v040-03 «The version on the GitHub page»: an edit on `build/escher-0.4.0` reaches the GitHub page only if that branch is (or becomes) the default, so the proof should read the default branch.

@history_item n=5 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the README's pinned line and the paint-order citation re-pointed) — README citations moved `README.md:48` → `:49` when new README text shifted the pinned phrase «the … driver commands as tools» and the `docs/capabilities.md` link. Precedent: a README edit that moves lines owes a citation re-point; a one-line in-place heading replacement moves none, which the plan should confirm.

@history_item n=6 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (the capability statement, README's twelve tools, the AT owner and two counts) — README links `docs/capabilities.md`; the capability-statement pin `test_capability_statement.py` reads README's «the {word} driver commands as tools» by path (was «no build, doc or test step reads it»). Origin of decision/capability-statement and contract/root-readme/mcp-tool-count: the heading edit must leave the pinned count phrase and the link untouched, so the pin stays green.

@history_item n=7 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (Project Intent: 1 row) — ~ decision/capability-statement: the pin also holds the statement's a11y leg file count and stand-repeat named-target count to the CI leg runner, no check reading its other claims. Confirms what the pin reads (counts, not a version heading), backing the scope's inferred fact that the heading change moves no pin.

@history_item n=8 marker=2026-11-18-cold-agent-live-run
**2026-11-18-cold-agent-live-run** (Project Intent: 1 row) — + decision/cold-agent-result-stated-measured: the capability statement states a cold-agent result as measured, with its reach. The interim 0.3.0 heading «built, its cold-agent run passed» rested on that run; the replacement «done and verified» drops the run from the heading, which must not contradict the statement's measured instance.

@history_item n=9 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (fourteen verbs, actions the tenth instance verb) — the front page's count of driver commands as tools moved thirteen → fourteen, as 2026-11-07-ui-graph moved it twelve → thirteen and 2026-10-13-upstream-sync-polish-and-ship to eleven. Precedent for contract/root-readme/mcp-tool-count: the README count follows `VERBS` (fifteen since 2026-11-17-layout-on-the-carriers, whose entries name no README change) — a count this chunk must not touch, but a stale one would turn the pin red.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (`type` replaces; … two engine limits measured and said by the tool) — the operator's review set that the tool itself must say its measured limits, since an agent learns the tool from the tool. Origin of decision/measured-limits-stated-in-the-tool, a picked row; it binds the driver's fixed texts, not the README, so this chunk's heading edit leaves it untouched.
