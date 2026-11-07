@answer n=1 entity=data-protection#1.1 v=no
why: the recording medium of per-session check-{n}.json files in escher-driver's recording.rs, which the chunk does not touch and whose kept checks it neither writes nor reads (task not_needed n=4)
@answer n=2 entity=data-protection#2.1 v=yes
why: the chunk re-runs the stand_* witnesses, and this entity names stand_replay as the strict reader of the committed counter-cli.json and counter-mcp.json, whose snapshot values the chunk must keep identical (task item 7)
