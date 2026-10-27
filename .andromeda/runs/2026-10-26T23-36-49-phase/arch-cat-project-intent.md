# arch · Project Intent — category read

## Carried
- (.andromeda/architecture-detail/project-intent.md:21) The refactored `the_listing_is_the_twelve_verbs_of_the_table` must keep asserting the twelve tools `VERBS` holds, the same count the front page states; the chunk changes no verb, so that count must stay twelve.
  > since 2026-10-10-mcp-surface how an MCP client drives the same apps (it spawns `target/debug/escher-session mcp` as its own child and gets the driver commands as tools, the page saying twelve, the count `VERBS` holds)
- (.andromeda/architecture-detail/project-intent.md:21) The capability-statement pin reads the README's tool count, so it must still pass under `ci-leg.sh fast` (its `ci-scripts` leg) once the chunk lands.
  > No build or doc step reads it, and one test does — the capability-statement pin reads its count of the driver commands as tools (§Standard Contracts → CI contracts)
- (.andromeda/architecture-detail/project-intent.md:27) The chunk must leave the driver schema's tables unchanged: verb and cause names, verbs, refusal causes, session errors, node keys and exit statuses. That keeps `docs/capabilities.md` and its CI-script pin true without an edit, and the pin must pass in the fast gate.
  > the CI-script pin `.github/scripts/test_capability_statement.py` holds its verb and cause names and its five counts — verbs, refusal causes, session errors, node keys, exit statuses — to the driver schema's tables (§Standard Contracts → CI contracts), and no check reads its other claims
- (.andromeda/architecture.md:293) The body's broad line for the same binding: the front page's tool count is read by one test, so it must hold to `VERBS` unchanged.
  > no build or doc step reads it, one test does — the capability-statement pin reads its count of the driver commands an MCP client gets as tools

## Bindings
- **Front page and capability-statement counts ↔ the body's §Standard Contracts (CI contracts and Driver session)**: the README's "twelve" and `docs/capabilities.md`'s five counts are pinned to the schema's tables by `test_capability_statement.py`. The chunk's refactor of the MCP listing test must assert the same twelve-verb `VERBS` listing and change no schema table. Cited at (.andromeda/architecture-detail/project-intent.md:21), (.andromeda/architecture-detail/project-intent.md:27) and (.andromeda/architecture.md:293).
- **Capability statement ↔ the body's Code quality line (the fast gate)**: the pin runs in the `ci-scripts` leg of `bash .github/scripts/ci-leg.sh fast`, which the chunk's "every check green" acceptance must pass. Cited at (.andromeda/architecture-detail/project-intent.md:27).
