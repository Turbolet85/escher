# security-plan · picked accounting

## Picked
authentication-authorization#2.5 · not touched · the report adds lines to test_ci_workflows.py only at 359-362, after its cited 201-205, and fact 2 rules the other rows citing that file no-change, so its read-only-token claim and citation still hold
input-validation#1.7 · proposed · its `InstallScriptTest` citations test_ci_workflows.py:422-456 and :476-492 lie past the pin's block added at 359-362 and have moved; the rest of the row (apt-install.sh, ci.yml's ten calls) is unreached
input-validation#1.58 · not touched · its host_log, stand_act_spans and mcp_log readings are dated and cite their chunks, mcp_log.rs and the accessibility sources are unreached, and the report reads host_log 3 under both builds with 0 needles, so the host-stderr claim still holds
logging-monitoring#1.1 · proposed · the report moves line 14 (the 2026-10-14 one-off client reading, now joined by a standing check, F1), the paged-part limit F2 belongs beside it, and line 13's host_log.rs:251-264 citation lies past the blocks added at 11-15 and 40-131
