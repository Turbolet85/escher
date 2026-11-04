@answer n=1 entity=logging-monitoring#1.1 v=yes
why: task item 11 names the escher_telemetry target allowlist and span-line rules as the sink the probe meets, and the probe's runs print through it.
@answer n=2 entity=logging-monitoring#1.2 v=yes
why: the probe's test output must hold masked values only (item 4), and the served task's escher-session stdout contract (no typed text on a line) is what its runs meet (item 9).
@answer n=3 entity=logging-monitoring#2.1 v=no
why: the probe fetches nothing remote (not_needed item 2), so no Resource load fails in its runs and the load-failed line is not reached; the allowlist rule it instances is carried by #1.1.
@answer n=4 entity=logging-monitoring#3.1 v=no
why: the select-list host and mcp tests rule over select popups, which the Counter/CRUD and served-task probe does not drive and the chunk does not edit.
@answer n=5 entity=logging-monitoring#4.1 v=no
why: the select list's missing log sites rule over a list the probe does not drive, so no span or field of the chunk's own is at issue.
@answer n=6 entity=logging-monitoring#5.1 v=no standing=S5
why: the cold-agent pipe is not run (not_needed item 7), and its committed-copy host-path masking and kept-check rule restate the S5 committed-evidence convention.
