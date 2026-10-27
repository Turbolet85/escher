# security extract

## Relevance
partial — a docs-and-records chunk with no code, carrier or input path; security binds only the text the README and `docs/capabilities.md` state about the cold-agent run's crossing, its login and its evidence, and what committed prose may hold.

## Constraints
- The capabilities statement's restated measured instance (`docs/capabilities.md:288-291`) states the 2026-10-25 run's carriage exactly as security-plan §Threat Model Summary (detail `boundary-mcp-stdio`, the pipe's second live run, "a third carriage") records it, carried set, call count, wrong count and firing count, read from that record and `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md`. It names no carried class the record does not hold and drops none it does: the screenshot and the lint's findings were carried this time, where the 2026-10-15 text read "no `screenshot`" (per security-plan §Threat Model Summary).
- The statement's section «What an MCP client carries onward» stays a statement of the ratified crossing that adds no carrier. The boundary is the spawning client holding the child's stdio, and what the client does onward is the client's. The README's new driver and "Try it" prose must not widen that set either, and its pin reads none of it (per security-plan §Threat Model Summary).
- The run's login is described only as the security plan records it: the operator's own Claude Code login, held by the `claude` CLI outside the repository, with no API key, env read, CI secret or new secret source. No committed line this chunk writes holds a key, auth header, credential-file name, token field or e-mail address (per security-plan §Secret Management).
- The README and the statement cite the run's counts, booleans and closed words only, from the report and the verdict. They quote no transcript text, tool argument or answer, model reply, check content or server-log line. The pipe's server log, check, raw transcript and the live child's kept checks are never printed (per security-plan §Logging & Monitoring).
- Any README mention of a recorded session (`--record <dir>`) keeps the kept-check medium as recorded. That means one `0600` check in a `0700` launcher-named directory, owner-only and never committed. The two committed Counter checks under `tests/blitz-tests/checks/` are the one exception, and the cold-agent live child's checks in `target/cold-agent/record/` are never copied or committed (per security-plan §Data Protection).
- Any README mention of the session socket describes it as the plan records it: a local Unix-domain socket in an owner-only state directory, with no TCP or UDP port and no authentication beyond file mode (per security-plan §Threat Model Summary).

## Patterns to follow
- The existing «What an MCP client carries onward» section of `docs/capabilities.md`: one plain-prose statement of the ratified crossing for its readers, written from the plan's record, not from the code (per security-plan §Threat Model Summary, detail `boundary-mcp-stdio`, second entry).
- Cite each run fact by its record, as the plan does with "as measured at …/report.md". The README and the statement restate the measured instance, never a fresh reading.
- The hygiene search the plan records over the statement and its pin (`/home/|api[_-]?key|authorization:|bearer ` and two e-mail domains, case-insensitive) is the check for committed prose (per security-plan §Secret Management, detail absence `secrets`).

## Anti-patterns to avoid
- Pasting a host path, transcript excerpt, tool answer, check step or `live-check.json` content into the README or the statement. Committed copies carry host paths masked, and the raw stream is never printed (per security-plan §Logging & Monitoring).
- Restating the 2026-10-25 run with the 2026-10-15 run's carried set, or with a set wider than the record's (for example a description, a pixel count or Cells' formulas beyond "Cells' contents") (per security-plan §Threat Model Summary).

## Contract bindings
- security ↔ tests: `.github/scripts/test_capability_statement.py` holds the statement's verb and cause names and five counts to the driver schema. The security plan records that the pin reads none of the crossing text and no environment variable, so the restatement must keep the pin green without the pin vouching for the crossing (per security-plan §Threat Model Summary · §Secret Management).
- security ↔ obs: the run's evidence (verdict, events, masked transcript) is the obs plan's cold-agent record. The README and the statement read its counts and closed words only (per security-plan §Logging & Monitoring).

## Acceptance criteria contributions
- The recorded hygiene search (`/home/|api[_-]?key|authorization:|bearer ` and the two e-mail domains, case-insensitive) over `docs/capabilities.md` and `.github/scripts/test_capability_statement.py` reads 0 hits after the restatement. Over `README.md`, the lines this chunk writes add no hit, and the kept author-and-contact section's pre-existing line is outside the change (per security-plan §Secret Management).
- The restated measured instance in `docs/capabilities.md` names the same carried classes and counts as the plan's record of the pipe's second live run. A reviewer compares it line against record, with no class added or dropped (per security-plan §Threat Model Summary).
- No line the chunk adds to `README.md` or `docs/capabilities.md` quotes transcript text, a tool answer, a model reply or a kept check's content. Each run fact is a count, boolean or closed word (per security-plan §Logging & Monitoring).
