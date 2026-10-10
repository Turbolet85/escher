
## 2026-10-10-driver-cli — scope rows after the command line: eleven driver modules, five seven_guis targets, eleven driver-action checks; a shell in one test target; the front page
**Section:** §Existing Scopes → escher-driver · seven_guis · blitz-tests · §Conventions → Tests · §Stack and Technologies → Testing · §Project Intent → Front page
**Change:**
- escher-driver: was nine private modules, an in-process definition and executor, a lifecycle line protocol; now eleven — `json` (the written forms) and `cli` (`command_line`) joined — `client` gains `call`, `host` runs hosted calls and ends on a stop or its idle expiry, `wire` is the line protocol of the lifecycle and of a call and its answer, `command` gains `SessionCommand` and `validate_session`, and `execute` carries two macOS `cfg` gates. Dependencies and the one named feature unchanged.
- seven_guis: `escher-session` is the driver's command line and, in its host role, the session host; was two integration-test targets, now five — `cli_commands`, `cli_flow` and `host_timer` joined — with `tests/flows/`, two POSIX `sh` scripts that are no target.
- blitz-tests: was ten `stand_act_*` files, 29 tests, fifteen readers of `session_common`, 15 readers of `common`; now eleven files (`stand_act_filled`), 37 tests, sixteen and 17 readers; the row names what the new tests cover and the `snapshot` call builder.
- Conventions → Tests: ten driver-action checks becomes eleven.
- Testing: seven_guis' unix-gated `cli_flow` runs two `sh` scripts under plain `sh` against the built binary — a shell is that one target's requirement; no dependency, manifest line or test framework was added.
- Front page: the README now also says how the command line is run and names the MCP surface, the driver's self-description and the screenshot as still to come.
**Why:** each row restated a count or a name the chunk moved. The reader counts were measured at this wrap, since the report gives the new file but not which shared modules it declares.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/
