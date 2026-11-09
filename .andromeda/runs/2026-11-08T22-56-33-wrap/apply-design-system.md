@apply_row n=1 ref_file=fanout-design-system--cascade.md ref_n=4 v=applied edit=E1
@apply_row n=2 ref_file=fanout-design-system--cascade.md ref_n=1 v=applied edit=E2
@apply_row n=3 ref_file=fanout-design-system--cascade.md ref_n=2 v=applied edit=E3
@apply_row n=4 ref_file=fanout-design-system--cascade.md ref_n=3 v=applied edit=E4
@fact id=fct-h64spsrj expect_rev=3
old: cli-actions · cli-unpaged-graph
new: cli-actions · cli-coverage · cli-unpaged-graph
old: cli-unpaged-actions · web-spa-tokens
new: cli-unpaged-actions · cli-coverage-limits · web-spa-tokens

@fact after=fct-cu455asz file=surface gid=new:cli-coverage kind=values subject=cli-coverage as=n1
- `escher-session coverage <app> <check>...` replays each named kept check of `<app>` on a fresh session and answers one uncoloured line of JSON `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}` on stdout, status `0`: the UI graph's handlers, read at boot and after each replayed step, that no given check exercised, `owner` left out when absent; its endings are the command line's four — `0` accepted · `1` refused `malformed` · `2` usage · `3` session error — none added; the line carries ids, owner ids, event names and two counts only (packages/escher-driver/src/json.rs:419-442; packages/escher-driver/src/coverage.rs:64-102; packages/escher-driver/src/cli.rs:183-199; as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md)

@fact after=@n1 as=n2
- The `coverage` role's one shape is `coverage <app> <check>...` — an app the binary boots, then one or more check paths, no `--session` and no `--record` word among them — and any other shape is a usage ending, status `2`; each path is read bounded at 1,048,576 + 1 bytes and parsed strictly, and a file that cannot be read, one past the bound, one that is no check and a check of another app are each refused `malformed`, status `1`, the refusal holding nothing of the path or the file (packages/escher-driver/src/cli.rs:495-505; packages/escher-driver/src/cli.rs:183-199; as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md)

@fact after=fct-wvdzkcqu file=surface gid=new:cli-coverage-limits kind=pending subject=cli-coverage-limits
- The `coverage` answer has three stated limits (docs/capabilities.md:273-281): handlers inside a canvas are not counted · owner: 0.4.0, the incubator's canvas plans; a handler on no screen a replay reached is not listed, holding by construction since no reading can see it · no owner; a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread · owner: a residual row, target next (as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md)
