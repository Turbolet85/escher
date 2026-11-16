# Fan-out results — 2026-11-15-session-binary-smoke

architecture: add 1 · set 0 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: add 0 · set 1 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 2 · set 3 · retire 0 · none 0 · ops-test-plan.md (re-spawn r1 after [SET_IN_READ] on line 19, a set of overview/session-lifecycle; the re-spawned file omits that set — raised by main in ops-main-test-plan.md)
obs-plan: add 0 · set 1 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: no proposer

ops-architecture.md
contract/agent-run-run-selections/session · reject — check 2: the fact's home is test-plan contract/agent-run-selection/session

ops-security-plan.md
decision/agent-run-selection-allowlist · apply — check 5: the plan's expected amendment names `session` among the allowlist's selections; a closed word, no new input class (playbook: accurate this-chunk addition)
contract/agent-run-cli/2 · apply — check 5: the plan's expected amendment names `session` among contract/agent-run-cli/2's selections (playbook: accurate this-chunk addition)
decision/agent-run-session-reaches-seven-guis · reject — check 2: the fact's home is test-plan contract/agent-run-selection/session
decision/agent-run-empty-run-never-passes · reject — check 2: the fact's home is test-plan decision/empty-run-is-never-a-pass, standing since before this chunk (`agent-run.sh:132`'s rule pre-dates it)

ops-layout-templates.md
contract/agent-run-cli/usage · apply — check 5: the plan's expected amendment, the usage line names `session` (playbook: accurate this-chunk addition)

ops-test-plan.md
contract/agent-run-selection/session · apply — check 5: the plan's expected amendment, a selection row `session` joins the selection rows (playbook: accurate this-chunk addition)
decision/session-smoke-through-agent-run · reject — check 7: not_master type 8 — a history of how the smoke used to be reached; its live part is contract/agent-run-selection/session and the plan's role entry, which the route holds
contract/agent-run-verbs/run · apply — check 5: the plan's expected amendment, the run verb's `-p seven_guis --test cli_flow` under `session` (playbook: accurate this-chunk addition)
decision/agent-run-holds-no-process · apply — check 5: the plan's expected amendment, the `except` gains the session flows' hosts (playbook: accurate this-chunk addition)
decision/test-data-is-code · apply — check 1: the report's Symbols / APIs, `boot` builds blitz-tests alone and `run session`'s cargo call builds seven_guis (playbook: accurate this-chunk addition)

ops-obs-plan.md
contract/agent-run-events/run-end · reject — check 2: the fact's home is test-plan decision/empty-run-is-never-a-pass, standing before this chunk; the empty-run outcome is no change of this chunk

ops-main-test-plan.md
overview/session-lifecycle · apply — check 5: the plan's expected amendment restates (test-plan · overview/session-lifecycle)'s «The session is not part of the five-command agent-run contract»; raised by main, the re-spawned proposer having dropped it
