# Task — architecture · 2026-10-25-capability-statement

## The session
The session added `docs/capabilities.md`, escher's statement of what 0.2.0 can do. It has five sections: what it can do, stated limits, accessibility reach, what an MCP client carries onward, and where each claim is proven. It restates the driver's counts (twelve verbs in three levels, eight refusal causes, fifteen session errors, thirteen node keys, four exit statuses) and its existing limits.
It added `.github/scripts/test_capability_statement.py`, a pin with four cases. It reads `VERBS`, `Cause::name`'s arms, `SESSION_ERRORS`, `NODE_FIELDS` and `EXIT_STATUSES` from the driver's sources and asserts that the statement and README state them in words. It joins the `ci-scripts` leg through unittest discovery, and no leg, job, workflow or `ci-leg.sh` line changed.
It changed one line of `README.md`: the MCP sentence now says twelve driver commands as tools where it said eleven, and it links `docs/capabilities.md`.
It measured three spec claims as false. The cause set is eight, not nine. The windowed assistive-technology witness's residuals line reads `re-carried:0.2.0`, not «(target: next)». The MCP stdio carried list omits the diffs and a lint's findings.
It added no Rust symbol, verb, argument, field, cause, session error, exit, carrier, log field, span, crate, dependency or config. The workspace tally is unchanged (1105 passed). Outside the repository, a Viola pointer to the statement was written, untracked and ignored there.

## Reached files
- docs/capabilities.md
- .github/scripts/test_capability_statement.py
- README.md
- ../viola/viola-0.2.0-incubator/research/escher-capabilities.md

## Touches
- Project Intent → Front page (`.andromeda/architecture-detail/project-intent.md:21`): the page said eleven driver commands as MCP tools and now says twelve. README now links `docs/capabilities.md` as the statement of what Escher can do and its limits.
- Standard Contracts → CI contracts (`.andromeda/architecture-detail/standard-contracts.md:261`): the `ci-scripts` discovery was seven test files and is now eight. The leg runs 104 tests, up from 100, because `test_capability_statement.py` adds 4.
- Standard Contracts → Driver session (`.andromeda/architecture-detail/standard-contracts.md:149`): the sentence that opens with the cause set being eight still says the nine causes and their names are unchanged. `CAUSES: [Cause; 8]` makes that eight.
- Standard Contracts → Mutation, query and CSSOM (`.andromeda/architecture-detail/standard-contracts.md:27`): the windowed assistive-technology witness is owned «(target: next)». Its residuals line reads `re-carried:0.2.0`, and 0.2.0's requirements say this version does not reach it.
- Standard Contracts → Mutation, query and CSSOM (`.andromeda/architecture-detail/standard-contracts.md:28`): re-read, unmoved. It is a true still-owed statement with no owner disposition.
- Standard Contracts body line on the platform-tree refresh's windowed witness being still owed (`.andromeda/architecture.md:139`): re-read, unmoved.
- Cross-cutting Patterns → Invalidation and state integrity (`.andromeda/architecture-detail/cross-cutting-patterns.md:35`): the windowed witness is still owed. Re-read, unmoved.
- Infrastructure Patterns → Directory structure (body, `.andromeda/architecture.md:225-243`): the tree names no `docs/`, and `docs/` now holds escher's own `capabilities.md` beside upstream's three files.
- Infrastructure Patterns → CI/CD (body): the CI-script tests job now also runs `test_capability_statement.py`. The job count, its `needs` and its legs are unmoved, and CI#40184674449 was green, 17/17.
- Standard Contracts → Driver session counts: twelve verbs, eight causes, the session errors, node keys and exit statuses. The new pin now reads each one from its schema table, so a change to any of them would turn the statement's pin red. The values are unmoved.
- Established Decisions → Driver session (MCP crossing): ids, names, masked values, snapshot text, diffs, shots and lint findings cross. The statement's MCP section restates this, and the body line is unmoved.
- Any test inventory or count of `.github/scripts/test_*.py` files (CI-script tests) in Existing Scopes or Conventions → Tests: one more CI-script test file, `test_capability_statement.py`, with 4 cases.
- Stack and Technologies → the `ci-scripts` leg's Python requirement (3.11 or later): the new pin runs on that leg. The requirement is unmoved.
- Existing Scopes or any line holding the workspace test tally: unchanged at 206 result lines, 1105 passed, 0 failed, 12 ignored. No Rust test was added.
- Occupied Resources → Environment variables: the pin reads no env var, so the list is unmoved.

## New facts
- F1 · project-intent · `docs/capabilities.md` is escher's own capability statement for 0.2.0. Its five sections, in order, are what it can do, stated limits, accessibility reach, what an MCP client carries onward, and where each claim is proven. README links it, and Viola's planning reads it through a pointer naming the repository, the path and a commit.
- F2 · new · Directory structure: `docs/` holds escher's own `capabilities.md` beside upstream's three files.
- F3 · standard-contracts · `.github/scripts/test_capability_statement.py` pins the statement and README to the driver schema's tables. It reads the verb count from `VERBS` in `schema.rs`, the cause count from `Cause::name`'s arms in `refusal.rs`, the session-error count from `SESSION_ERRORS` in `error.rs`, and the node-key and exit-status counts from `NODE_FIELDS` and `EXIT_STATUSES` in `schema.rs`, and requires each count in words. It reads by path from the repository root, writes nothing and reads no env var.

## Untouched
- The engine contracts (Document core, the clock, events, scrolling, layout, nodes and widgets, paint and shell, the test harness, the headless stand, the Dioxus DOM bridge): the statement restates them and changes no code.
- The driver's wire `v3`, the socket, the command line's exit statuses, the MCP framing and bounds, the recording and replay contracts, and the `command` and `tool_call` spans: no carrier, verb, field, exit, log field or span changed.
- Occupied Resources (listeners, ports, outbound hosts, filesystem, the recording directory, process-wide state, names, CI infrastructure): no resource was added. The new markdown and python files are no resource.
- Established Decisions on dependency pinning, build profiles, release versioning, renderer backends, default features, fonts, CSS approximations and unsupported features: untouched.
- The select model's limits, the inert focus-fixup limit and its residuals owner, the boolean model's limits and the cross-OS shot limit: they are restated in the statement, and their standing text is unmoved.
- The native checked-state limit in the accessibility tree and its residuals owner: a different residual from the assistive-technology witness, unmoved.
- CI workflow jobs and legs: no job, leg, `needs`, cache key or workflow line changed. The thirteen linux jobs, the `a11y` leg's fourteen files and `stand-repeat`'s targets are unmoved.
- The telemetry bootstrap and the scrub's limits, restated in the statement and unmoved.
- Project Intent's upstream sync line, origin, crate purposes and apps: untouched.
- Inherited Defaults and Stack versions: no dependency or toolchain moved.
