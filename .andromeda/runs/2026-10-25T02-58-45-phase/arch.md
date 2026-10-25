# arch extract

## Relevance
partial — a document-only chunk: no crate, contract, resource or boundary moves, but the statement must agree with the plan's stated contracts and limits, and CARRY 1 edits the front page the plan governs.

## Constraints
- The root `README.md` is escher's own front page, its content set (what escher is, status, how the driver's command line and MCP surface are run, lineage, licence, contact, the name spelled «Escher» in prose) and no build, doc or test step reads it; an upstream `README.md` change resolves to escher's version at a sync (per architecture §Project Intent → Front page). Any README edit (CARRY 1's verb count, a 0.2.0 stage line naming the statement) stays inside that content set.
- The driver's schema is stated once as `'static` data: twelve verbs, eight closed causes, no refusal holding what a call supplied (per architecture §Standard Contracts → Driver session). The statement's counts and verb names must match that table, never a separately maintained number; whether README.md and the help texts already agree with twelve is research's question.
- What crosses a driver carrier is closed: ids, names, masked values, snapshot text, diffs, shots and lint findings, never a `NodeId`, over the `std`-only socket and the `mcp` role's stdio; a new carrier or exit widens the boundary (per architecture §Established Decisions → Driver session). The «what an MCP client carries onward» section describes exactly that set and adds no carrier, verb, field or exit.
- The model provider is an outbound host reached only by the spawned `claude` client on the operator-driven live cold-agent run, never in CI, and no escher code makes the request (per architecture §Occupied Resources → Outbound hosts). The statement places onward forwarding beyond escher's boundary in those terms.
- No new listener, port, env var, workspace crate or filesystem resource lands without an §Occupied Resources registration (per architecture §Occupied Resources → Network ports and listeners); a statement placed in another repository or a new tracked directory is a placement question for P4, and a new top-level directory would move the tree in §Infrastructure Patterns → Directory structure (per architecture §Infrastructure Patterns → Directory structure).
- The plan states limits the statement must carry or reconcile: the select model paints no popup, clicks no option and models no `multiple` select or `optgroup` options (per architecture §Established Decisions → Scrolling and input); HTML's focus fixup is not built (per architecture §Established Decisions → DOM semantics); a scrolling box under a transform is read by `visible_region` where it stands untransformed (per architecture §Standard Contracts → Scrolling, selection, tree); a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (per architecture §Existing Scopes).
- Two further plan-stated limits not in the scope's list: the accessibility tree states no native checkbox's or radio's checked state, and the platform-tree refresh's windowed witness is still owed (per architecture §Standard Contracts → Mutation, query and CSSOM); and the engine's unsupported features — Shadow DOM, container queries, 3D transforms, anchor positioning, subgrid and masonry, `:focus-visible`/`:valid`/`:invalid`/`:required`/`:target` always false (per architecture §Established Decisions → Unsupported features). Whether a consumer-facing statement must name the engine-level set is P3's completeness question.

## Patterns to follow
- The tool's own self-description (`help`, `describe()`) is written from the schema's tables alone — verbs, node keys with help, causes and session errors with remedies, exit statuses (per architecture §Standard Contracts → Driver session); the statement reads its capability and limit lists from those tables and the schema's help texts rather than re-deriving them.
- The front page's `## Status` lists stages one line per epoch of the version's working route, built vs planned (per architecture §Project Intent → Front page); a 0.2.0 line naming the statement follows that shape.
- Capabilities are stated per crate as each crate's own description states it (per architecture §Project Intent → Crate purposes); the statement keeps crate, binary and path names verbatim.

## Anti-patterns to avoid
- Changing code, schema, verbs, carriers, log fields or spans to make the statement true — the boundary moves only by a ratified widening (per architecture §Established Decisions → Driver session).
- Hand-copying a count (verbs, causes, node keys) into prose that then drifts from the `'static` table, as README.md's «eleven» did (per architecture §Standard Contracts → Driver session).

## Contract bindings
- arch §Established Decisions → Driver session ↔ security-plan §Surfaces: the carried-content set the «MCP client carries onward» section states is the security plan's ratified crossing.
- arch §Occupied Resources → Outbound hosts ↔ security-plan cold-agent crossing and test-plan §3: the cold-agent pipe as the measured instance of onward forwarding.
- arch §Standard Contracts → Mutation, query and CSSOM ↔ a11y-plan §1 / §3: the a11y-reach section's measured-off limits (native checkbox state in the tree, the windowed refresh witness) against the a11y plan and `.andromeda/residuals.md`.
- arch §Existing Scopes ↔ test-plan: the cross-OS PNG byte difference as a stated limit on any byte-compared check.

## Acceptance criteria contributions
- README.md names the MCP role's tools as twelve driver commands, matching the verb table (per architecture §Standard Contracts → Driver session).
- The statement's carried-content list equals the closed set ids, names, masked values, snapshot text, diffs, shots, lint findings, with no `NodeId` (per architecture §Established Decisions → Driver session).
- The statement's limits include the select model's unmodelled parts, the focus-fixup gap, the untransformed `visible_region` reading and the cross-OS PNG difference, each present or its omission reasoned (per architecture §Established Decisions → Scrolling and input; §Established Decisions → DOM semantics; §Standard Contracts → Scrolling, selection, tree; §Existing Scopes).
- The chunk's diff adds no listener, port, env var, crate or code change (per architecture §Occupied Resources → Network ports and listeners).
