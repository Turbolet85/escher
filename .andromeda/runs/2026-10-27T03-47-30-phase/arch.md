# arch extract

## Relevance
partial. The README and the capabilities statement are governed by arch §Project Intent (Front page, Capability statement) and §Standard Contracts → CI contracts (the capability-statement pin). The design-system stand rows land in architecture as a wrap amendment. The two ledger refs and the incubator ignore are outside the plan's sections: no arch section registers the incubator path.

## Constraints
- arch §Project Intent → Front page requires the root `README.md` to be escher's own page, not upstream Blitz's. It says what escher is and its state under `## Status`, how the driver's command line and MCP surface are run, how to ask for `help`, its Blitz lineage, the licence and an Author / Contact section, and it links `docs/capabilities.md`. The fork's `ci.yml` badge is its one image. The name is «Escher» in the prose and the title, while crate, binary and path names, URLs and the e-mail stay unchanged. The plan's row still records 0.2.0 as "planned in five stages", so the row is owed a wrap amendment once the page reads 0.2.0 done and 0.3.0 planned.
- arch §Project Intent → Front page requires that no build or doc step reads `README.md`. One test reads it: the capability-statement pin, which reads its count of the driver commands as tools. At an Upstream sync, an upstream change to `README.md` resolves to escher's version.
- arch §Project Intent → Capability statement requires `docs/capabilities.md` («What Escher can do») to keep its five sections in this order: `## What it can do` · `## Stated limits` · `## Accessibility reach` · `## What an MCP client carries onward` · `## Where each claim is proven`. README.md links to it, no check reads its claims beyond the pin's names and counts, and no build or doc step reads it.
- arch §Standard Contracts → CI contracts requires the pin `.github/scripts/test_capability_statement.py` to find two things. In the statement: every verb of `VERBS` and every cause name as a code span, and each count in words (`{word} verbs` · `{word} refusal causes` · `{word} session errors` · `{word} node keys` · `{word} exit statuses`; whitespace is folded, so a phrase may wrap a line). In README.md: «the {word} driver commands as tools». Any rewording of these lines has to keep those phrases.
- arch §Standard Contracts → Driver session requires the schema to be stated once as `'static` data (twelve verbs, eight closed causes). The README's and the statement's counts follow that table and are not set by prose.
- arch §Occupied Resources → Outbound hosts requires the model provider to be reached only on the operator-driven live cold-agent run, never in CI, with no escher code making the request. Restating the statement's 2026-10-25 run must keep that framing. The run's committed copies are registered under `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-*` (arch §Occupied Resources → Filesystem).
- arch §Standard Contracts → Headless stand already states the stand's accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) on the load page's `load-fetch` and the bundled `DEJAVU_SANS` font context. Research has to answer which arch section each moved design-system row lands in, so the move neither duplicates nor contradicts that row. Candidates are §Standard Contracts → Headless stand, §Existing Scopes (the `seven_guis` unit) and §Infrastructure Patterns → Deployment model (where `DEJAVU_SANS` is named).

## Patterns to follow
- The 0.1.0 `## Status` form recorded in arch §Project Intent → Front page: a done version lists its stages with its recorded facts (for 0.1.0: capabilities verified, cold-agent tool calls with 0 wrong, surviving mutants), and a planned version gets one line per epoch of its working route. 0.2.0 done and 0.3.0's six epochs follow the same shape.
- arch §Infrastructure Patterns → Directory structure places `docs/capabilities.md` beside upstream's three files under `docs/`, so an upstream change under `docs/` never conflicts with it. Edit that file in place and add no new file under `docs/`.
- arch §Project Intent → Upstream sync records Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 as waiting for escher's next version. The README's 0.3.0 upstream-sync stage is written as a plan, not as done.

## Anti-patterns to avoid
- Capitalising code identifiers: `escher-session`, `escher-driver`, `escher-0.3.0/`, URLs and the e-mail stay as spelled (arch §Project Intent → Front page).
- Moving a count, verb or cause through prose, or adding a build or doc step that reads the README or the statement (arch §Standard Contracts → CI contracts; arch §Project Intent → Capability statement).

## Contract bindings
- arch §Project Intent → Capability statement ↔ tests: `test_capability_statement.py`, run by the CI-scripts leg (arch §Standard Contracts → CI contracts), holds the README's tool count and the statement's names and counts to the driver schema.
- arch ↔ security-plan: the statement's `## What an MCP client carries onward` and its 2026-10-25 run text must match the security plan's registered cold-agent crossings (ids, names and values in snapshot texts, paged parts and diffs, a lint's findings and one screenshot). Outbound reach is per arch §Occupied Resources → Outbound hosts.
- arch ↔ design-system ↔ layout-templates: moving the four stand rows (accent, success, cards, DejaVu Sans) out of design-system's product surface is a wrap amendment across three masters. Phase edits no spec source.

## Acceptance criteria contributions
- `.github/scripts/test_capability_statement.py` passes after the edits, and the README still reads «the twelve driver commands as tools» (per arch §Standard Contracts → CI contracts).
- Every prose mention of the project in `README.md` reads «Escher», and every crate, binary, path, URL and e-mail token keeps its code spelling (per arch §Project Intent → Front page).
- `docs/capabilities.md` keeps its five `##` sections in the recorded order (per arch §Project Intent → Capability statement).
- The wrap restates arch §Project Intent → Front page to the new `## Status` (0.1.0 and 0.2.0 done and verified, 0.3.0 planned) and records where each moved stand row lands in architecture (per arch §Project Intent → Front page; arch §Standard Contracts → Headless stand).
