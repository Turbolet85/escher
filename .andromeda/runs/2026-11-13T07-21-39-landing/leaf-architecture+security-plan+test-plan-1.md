@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md`
source: tree «.andromeda/architecture.md is absent from the tree»
replace-with: Primary source: the architecture source (`doc:architecture`)
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=3 class=tree
leaf-text: (`.andromeda/registries/contracts/test-plan/session-lifecycle.md`)
source: tree «.andromeda/registries/contracts/test-plan/session-lifecycle.md is absent from the tree»
replace-with: (`doc:test-plan`, key `session-lifecycle`)
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=12 class=omission
leaf-text: the host reads one for its idle expiry, and the MCP server one when a request arrives with a session held.
source: dec-gfqcjdzc «clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits.»
replace-with: the host reads one for its idle expiry, the MCP server one when a request arrives with a session held, and the client one for its bounded stop and ready waits.
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=18 class=omission
leaf-text: answers `in_view` — false when no scroll could bring it into view; it is never refused for that
source: ctr-ubed335e «Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.»
replace-with: answers `in_view` — false when no scroll could bring it into view or the element was removed; it is never refused for that
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=49 class=stale
leaf-text: `passes` (on a step the load wait settled again, the passes of the settle that went quiet)
source: ctr-sk2sagao «Settle's pass count, recorded when settle answered quiet.»
replace-with: `passes` (the settle's pass count, recorded whenever the settle answered quiet)
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=60 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-07-command-and-refusal-schema chunk's evidence (no longer in the tree)
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=60 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-10-driver-cli chunk's evidence (no longer in the tree)
@leaf_edit n=8 leaf=.claude/docs/services/escher-driver.md line=60 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-12-self-description/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-12-self-description/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-12-self-description chunk's evidence (no longer in the tree)
@leaf_edit n=9 leaf=.claude/docs/services/escher-driver.md line=60 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-12-headless-screenshot/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-12-headless-screenshot/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-12-headless-screenshot chunk's evidence (no longer in the tree)
@leaf_edit n=10 leaf=.claude/docs/services/escher-driver.md line=61 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-07-act-by-id chunk's evidence (no longer in the tree)
@leaf_edit n=11 leaf=.claude/docs/services/escher-driver.md line=61 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-07-refusal-detection chunk's evidence (no longer in the tree)
@leaf_edit n=12 leaf=.claude/docs/services/escher-driver.md line=61 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-07-driver-command-spans/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-07-driver-command-spans/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-07-driver-command-spans chunk's evidence (no longer in the tree)
@leaf_edit n=13 leaf=.claude/docs/services/escher-driver.md line=63 class=tree
leaf-text: `escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/controls.md`
source: tree «escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/controls.md is absent from the tree»
replace-with: the 2026-10-10-mcp-surface chunk's evidence (no longer in the tree)
