@amendment marker=2026-11-08-what-can-i-do-here
title: the `actions` answer line on the cli surface
section: §Self-Validation Protocol (body: the subjects list); §Surface: cli → Component Patterns (surface.md: the new cli-actions entity, after the graph entity)
change:
  - The subjects list gains `cli-actions` after `cli-graph`.
  - New: `escher-session actions` takes no argument, and an argument is refused `malformed`; it answers one uncoloured line of JSON `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}` on stdout, status `0`, the same answer on the session socket's `call`, over the `mcp` role and in a kept check.
  - It lists every actionable node in snapshot pre-order (focusable, of an interactive role or holding a current listener for an event other than `mounted`, or owned by another node), each with the verb table's id-taking instance verbs and the cause each would be refused with, `owner` and `cause` left out when absent.
  - The line carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text or `NodeId`.
  - New: the answer reports whether each call would be accepted, not what it would do; a node's verbs are decided by the same `validate` and per-verb target check (`check_target`) every id-taking verb meets before it acts, so a listed cause is the cause the call returns, and a node that resolves to no element reads `stale`.
why: the report's new `actions` verb, its written form and `Session::actions` deciding each verb through `validate` then `check_target`, with every listed cause equal to what the call returns (matrix#v030-06). The answer's crossing onto every ratified carrier is a boundary widening on pattern `rul-izni57qv`, ratified by the founder's word «Новый глагол actions (Рекомендую)», given by dialog in the overseer session and relayed verbatim by the overseer (inputs#I4, relayed again as inputs#I6), and confirmed by his own confirm at this wrap halt, «Подтвердить все 24 (Рекомендую)», by dialog in the overseer session, relayed by the operator (answer-card.md).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the unpaged `actions` answer, a stated limit
section: §Self-Validation Protocol (body: the subjects list); §Surface: cli → Component Patterns (surface.md: the new pending cli-unpaged-actions item, after the unpaged graph)
change:
  - The subjects list gains `cli-unpaged-actions` after `cli-unpaged-graph`.
  - New pending item: the `actions` answer is unpaged, taking no argument and answering the whole screen in one line; Cells at boot answers 106,088 bytes for 678 nodes, within the 1,048,576-byte answer bound — the stated limit «An unpaged actions answer» beside «An unpaged graph»; owner residual `res-7cc5bi4k`, target next.
why: the report's measured Cells answer (106,088 bytes for 678 nodes, through the binary) and `res-7cc5bi4k` widened to own the `actions` answer beside the unpaged graph, on the operator's own yes on the P5 review card (inputs#I5).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
