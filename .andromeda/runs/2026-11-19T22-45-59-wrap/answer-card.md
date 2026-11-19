# Answer card — Q1 · 2026-11-19-upstream-sync-ahead-of-the-debts

The founder's answers to the nine boundary widenings of the upstream merge (`547d7514b`), the forks of the chunk's
`evidence/founder-forks.md` (W1–W9), given by three question dialogs in the overseer session and relayed verbatim by
the overseer, 2026-11-19 — inputs#I4 (`~/dev/projects/additional/escher-overseer/relays/upstream-sync-0.4.0-founder-2026-11-19.md`):

- W1 — the CDP server: «Взять, выключено везде (Recommended)» (option A: the crate in, devtools off in every default; registered as an opt-in listener with no authentication)
- W2 — `parley_code_editor`: «Оставить крейт вне сборки (Recommended)» (option A) — no master operation
- W3 — TLS through rustls: «Взять (Recommended)» (option A)
- W4 — skia by default: «Взять как в upstream» (option A, against the marked recommendation; the build-time fetch measured)
- W5 — focus events: «Очередь взять, молчания оставить (Recommended)» (option A: the queue, the two silences kept)
- W6 — `isTrusted = false` for script events: «Взять (Recommended)» (option A)
- W7 — a cancelled pointerup still clicks: «Взять (Recommended)» (option A)
- W8 — SVG presentation and `hr` hints: «Взять, путь mask измерить (Recommended)» (option A; the mask measured through the stamping constructor — no stop owed)
- W9 — `visibility: collapse` out of the tree: «Взять (Recommended)» (option A)

The playbook rule «Boundary widening by upstream merge» (rul-72xhfjoz) escalates each; his relayed-verbatim answers
are his own word. Escalated operations read as lying wholly inside those answers, each with its fit verdict:

1. ops-architecture.md · decision/blitz-net-reqwest-fetching · stack-and-technologies · fit: fits type 1 — W3
2. ops-architecture.md · decision/one-local-unix-listener · occupied-resources · fit: fits type 2 — W1
3. ops-architecture.md · decision/session-socket-one-listener · occupied-resources · fit: fits type 2 — W1
4. ops-architecture.md · decision/driver-session-unix-socket · occupied-resources · fit: fits type 2 — W1
5. ops-architecture.md · contract/event-dispatch/focus-change · standard-contracts · fit: fits type 3 — W5
6. ops-architecture.md · decision/default-action-event-order · fit: fits type 2 — W7
7. ops-architecture.md · decision/handlers-before-default-action · fit: fits type 2 — W7
8. ops-architecture.md · name/other/blitz-cdp-port · occupied-resources · fit: fits type 5 — W1
9. ops-architecture.md · name/other/blitz-devtools-frontend-dir · occupied-resources · fit: fits type 5 — W1
10. ops-architecture.md · name/crate/blitz-vibey-cdp-server · occupied-resources · fit: fits type 5 — W1
11. ops-security-plan.md · decision/session-socket-sole-listener · fit: fits type 1 — W1
12. ops-security-plan.md · decision/tls-via-native-tls (retire) · fit: — — W3
13. ops-security-plan.md · decision/tls-via-rustls · data-protection · fit: fits type 1 — W3
14. ops-security-plan.md · decision/cdp-server-opt-in-loopback · api-security · fit: fits type 1 — W1
15. ops-security-plan.md · contract/environment-variables/blitz-cdp-port · secret-management · fit: fits type 3 — W1
16. ops-security-plan.md · contract/environment-variables/blitz-devtools-frontend-dir · secret-management · fit: fits type 3 — W1
17. ops-security-plan.md · decision/skia-default-build-fetch · dependency-security · fit: fits type 1 — W4
18. ops-security-plan.md · decision/programmatic-focus-events · input-validation · fit: fits type 2 — W5
19. ops-security-plan.md · decision/event-is-trusted-by-source · input-validation · fit: fits type 1 — W6
20. ops-security-plan.md · decision/cancelled-pointerup-still-clicks · input-validation · fit: fits type 4 — W7
21. ops-security-plan.md · decision/svg-mask-url-fetch · input-validation · fit: fits type 1 — W8
22. ops-a11y-plan.md · decision/hidden-content-excluded · fit: fits type 2 — W9
23. ops-a11y-plan.md · contract/accessibility-tree/hidden-and-inert-nodes · fit: fits type 3 — W9
24. ops-a11y-plan.md · decision/hidden-content-excluded-from-a11y-tree · fit: fits type 2 — W9

Not asked — fit read not_master: ops-security-plan.md · decision/presentational-hints-typed-parse · not_master type 5
(W8; an attribute inventory the code holds — rejected before the card).

## The answer
Q1, answered by the operator, 2026-11-20, in this wrap's question dialog: «Confirm all 24 (Recommended)» — every listed operation lies inside the founder's answers; the ratification stays his answers (inputs#I4).
