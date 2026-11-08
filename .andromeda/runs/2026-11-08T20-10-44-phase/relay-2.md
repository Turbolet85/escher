# The P4 dialog of 2026-11-08-handler-coverage and its answers (2026-11-08)

## Q1 (Carrier) — the founder's fork, playbook `rul-izni57qv` «Boundary widening»
Handler coverage (v030-07): how should a set of kept checks reach the one command? Options, in order:
1. A command-line role (Recommended) — `escher-session coverage <task> <check.json>…` boots a fresh in-process session for each kept check named, replays it with `Check::replay`, and prints one JSON line listing the handlers no replay reached; the widening: check files named on argv become a new input, read by the strict `Check::from_bytes`; no new verb, `VERBS` stays 14, socket, MCP and the cold-agent pipe unchanged; an in-process escher_driver function under it.
2. B: fifteenth instance verb — a no-argument `coverage` verb on every carrier over one held session's calls; `VERBS` 14 → 15.
3. C: in-process only for now — `escher_driver::handler_coverage(checks, boot)` and its stand test, no carrier change.

Answer: «A command-line role (Recommended)».
Its note, verbatim: Q1 is the founder's own word, by dialog in the overseer session, relayed by the overseer: «A. Роль командной строки (Рекомендую)».

## Q2 (Screens) — the operator's call
Which screens' handlers form the set the unexercised list is drawn from? Options, in order:
1. Every screen read (Recommended) — the graph read at boot and after every replayed step, handlers (id, event) combined; a handler on a screen no replay reached is a stated limit beside the canvas limit.
2. Boot screen only.

Answer: «Every screen read (Recommended)».
Its note, verbatim: Q2 is the overseer's technical call.
