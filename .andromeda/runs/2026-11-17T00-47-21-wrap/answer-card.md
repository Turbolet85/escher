# Answer card — 2026-11-16-error-field-in-the-answer (wrap 2026-11-17T00-47-21)

## The answer it cites
- inputs#I1, item 2 — the founder's own chosen label, relayed verbatim by the overseer (2026-11-16):
  «Открыть error в caused (Recommended)». As read to him: the answer to an agent gets its own scrub set without `error`
  (the error's text shown), the other names stay hidden, the log sink's stderr lines do not change; its stated price,
  that the error's text can hold what its author put into it, such as a failed fetch's URL.

## The operations lying wholly inside it (9)
- ops-architecture.md · decision/no-echo-of-typed-input · standard-contracts · fit: fits type 2 — the exception names error as kept, scrubbed by the answer's own names
- ops-main-architecture.md · contract/acting-verb-results/caused · standard-contracts · fit: fits type 3 — «a log's scrubbed fields» → «a log's fields, error kept»
- ops-security-plan.md · decision/caused-text-crossing · threat-model-summary · fit: fits type 1 — error crosses as written within the text bound, every other content-named field redacted
- ops-security-plan.md · decision/mask-holds-on-every-carrier · input-validation · fit: fits type 2 — the exception names the error field beside the message
- ops-security-plan.md · decision/socket-carries-screen-reading · input-validation · fit: fits type 1 — caused text scrubbed by the answer's own field names, error kept
- ops-security-plan.md · decision/escher-telemetry-allowlist-sink · input-validation · fit: fits type 2 — the sink's scrubbing reaches the sink only, the capture holding its own set
- ops-obs-plan.md · contract/telemetry-scrub-set/error · 8-pii-scrubbing-compliance · fit: fits type 3 — the sink prints error [redacted], the capture keeps it as written
- ops-obs-plan.md · decision/scrub-by-field-name · 8-pii-scrubbing-compliance · fit: fits type 1 — the sink by the content-named set, the capture by the answer's own hidden set
- ops-obs-plan.md · decision/answers-echo-nothing-typed · 8-pii-scrubbing-compliance · fit: fits type 2 — the exception: a field of the answer's hidden set redacted, error shown

## Reaching past it — asked separately (Q2)
- ops-security-plan.md · contract/driver-session-reply/no-attribute-text · input-validation · fit: fits type 3 — the promise «a reply never carries an attribute's text» gains an exception for a caused item's message AND its error field. The error half lies inside inputs#I1; the message half rests on the earlier card «Применить все 14» (inputs#I1 item 1), whose 14 operations did not touch this row. Not in the plan's expected amendments.

## Rejected before the question (check 2, one fact twice — not asked)
- ops-architecture.md · contract/acting-verb-results/caused-fields — home obs-plan decision/scrub-by-field-name + security-plan decision/caused-text-crossing
- ops-layout-templates.md · contract/escher-session-caused/error — same home
- ops-obs-plan.md · decision/answer-hidden-field-set — same home

## Applied without a halt (no widening)
- ops-test-plan.md · decision/stand-boots-offline, decision/stand-load-page-fixed-answer — the test-only FailingNet stand-in
- ops-test-plan.md · decision/doc-leg-gates-public-doc-edits — the operator's directive at this wrap

## The answers (operator, 2026-11-17, by question dialog in this wrap)
- Q1: «Confirm all 9 (Recommended)» — the confirm that the nine lie inside his answer; the ratification stays the founder's word inputs#I1, relayed verbatim by the overseer.
- Q2: «Hold for the founder» — contract/driver-session-reply/no-attribute-text does not land in this wrap; held PROVISIONAL for the founder's own word.
