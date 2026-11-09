@apply_row n=1 ref_file=fanout-layout-templates.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-layout-templates.md ref_n=2 v=applied edit=E2
@fact id=fct-exqct2ck expect_rev=3
old: mcp/listing.rs:61-115); `start` checks its recording
new: mcp/listing.rs:61-115); a fourth role, `escher-session coverage <app> <check>...` — one app the binary boots, then one or more kept-check paths, no `--session` and no `--record` word among them, any other shape usage (stderr, exit 2), the usage line naming it (`; coverage role: coverage <app> <check>...;` before `; apps:`) — reads each path bounded at `MAX_CHECK_BYTES + 1` (1,048,576 + 1 bytes) and parses it by `Check::from_bytes`, a file that cannot be read, one past the bound, one that is no check and a check whose app is not `<app>` each refused `malformed`, exit 1, holding nothing of the path or the file; accepted, it replays each check on a fresh boot through the same boot `main` hands to the command line, so a kept check is read by this role as by `Check::replay`, and answers one uncoloured line `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}` on stdout, exit 0, `owner` left out when absent, carrying ids, owner ids, event names and two counts only; a boot that fails ends with the session error, exit 3 (packages/escher-driver/src/cli.rs:45-47; packages/escher-driver/src/cli.rs:79-88; packages/escher-driver/src/cli.rs:183-199; packages/escher-driver/src/cli.rs:462-473; packages/escher-driver/src/cli.rs:495-505; packages/escher-driver/src/json.rs:419-442; examples/seven_guis/src/session_host.rs:5-7); `start` checks its recording

@fact id=fct-nxmh4fah expect_rev=1
old: and holding it while tool calls arrive — adding no markup
new: and holding it while tool calls arrive — and its `coverage` role a further consumer, booting the kept checks' app once per check through the same boot and replaying each check on that fresh mount — adding no markup
old: (examples/seven_guis/src/session_host.rs:14-30; examples/seven_guis/src/session_host.rs:44-53;
new: (examples/seven_guis/src/session_host.rs:5-7; examples/seven_guis/src/session_host.rs:14-30; examples/seven_guis/src/session_host.rs:44-53;
