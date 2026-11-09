@amendment marker=2026-11-08-handler-coverage
title: the escher-session coverage role's answer line, endings and refusals
section: Self-Validation Protocol (body: the subjects list); Surface: cli › Component Patterns (surface: the new `values · cli-coverage` group)
change:
  - The subjects list gains `cli-coverage` after `cli-actions`.
  - New group `values · cli-coverage`, its first item: `escher-session coverage <app> <check>...` replays each named kept check of `<app>` on a fresh session and answers one uncoloured line of JSON `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}` on stdout, status `0` — the UI graph's handlers, read at boot and after each replayed step, that no given check exercised, `owner` left out when absent; its endings are the command line's four (`0` accepted · `1` refused `malformed` · `2` usage · `3` session error), none added; the line carries ids, owner ids, event names and two counts only.
  - Its second item: the role's one shape is `coverage <app> <check>...` — an app the binary boots, then one or more check paths, no `--session` and no `--record` word among them — and any other shape is a usage ending, status `2`; each path is read bounded at 1,048,576 + 1 bytes and parsed strictly, and a file that cannot be read, one past the bound, one that is no check and a check of another app are each refused `malformed`, status `1`, the refusal holding nothing of the path or the file.
why: the report's Symbols / APIs (the role, its reader, its answer line and endings) and its Outcome (design) MET, one uncoloured JSON line with none of the four exit statuses added, and (security) MET on the refusals. The role's crossing — kept-check files named on argv — is a boundary widening ratified by the founder's own word at P4 Q1, relayed verbatim by the overseer (inputs#I4); its records here were confirmed at this wrap's answer card Q1 by the overseer as delegate, PROVISIONAL (`delegate overseer`), to be superseded by the founder's own word in his batch at the epoch boundary.
kept: the `escher-session` item stands as written — one line of JSON per command and the four exit statuses; the coverage role adds no exit status.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the coverage answer's three stated limits and their owners
section: Self-Validation Protocol (body: the subjects list); Surface: cli › Component Patterns (surface: the new `pending · cli-coverage-limits` group)
change:
  - The subjects list gains `cli-coverage-limits` after `cli-unpaged-actions`.
  - New group `pending · cli-coverage-limits`, its item: the `coverage` answer has three stated limits — handlers inside a canvas are not counted (owner: 0.4.0, the incubator's canvas plans); a handler on no screen a replay reached is not listed, holding by construction since no reading can see it (no owner); a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread (owner: a residual row, target next).
why: the report's New text, by line, which lists the three limits in the capability statement, and the operator's own word «yes» on the P5 review card 2 (inputs#I5), approving its Beyond-the-plan rows — the canvas count deferred to 0.4.0, a new residual row (target next) for the `stopPropagation` limit, and no owner for the unreached-screen limit, which holds by construction.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
