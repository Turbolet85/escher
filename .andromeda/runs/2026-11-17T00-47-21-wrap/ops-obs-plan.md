@row_set n=1 row=contract/telemetry-scrub-set/error cell=promise seen=2 detector=D-obs-pii
value: An `error` field is in the sink's scrub set, so the sink prints its value as `[redacted]`; the scoped capture keeps it in a caused item as written, cut at the text bound, so a failed fetch's error text crosses the answer carriers as a field value.
basis: fct-tylyvsk5
because: The capture now judges a caused item's fields by the answer's own hidden set without `error`, so the row's claim that the capture keeps `error` as `[redacted]` and that the error text reaches no carrier is false.
@row_set n=2 row=decision/scrub-by-field-name cell=rule seen=1 detector=D-obs-pii
value: Scrubbing is by field name: in escher's own sink by the content-named set, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source by the answer's own hidden set; a scrubbed field reads [redacted], the message and other fields kept as written.
basis: fct-vk5mjci2
because: The capture no longer reads the sink's content-named set but its own hidden set without `error`, so a content-named field no longer always reads [redacted] in a caused item.
@row_set n=3 row=decision/answers-echo-nothing-typed cell=except seen=2 detector=D-obs-pii
value: A caused item's message, chain and kept fields hold what their author wrote, a field of the answer's own hidden set reading [redacted] and an error field shown as written, cut at the text bound.
basis: fct-tylyvsk5
because: A caused item's `error` field, content-named for the sink, now crosses as written, so the exception's «a content-named field's value reading [redacted]» no longer holds.
@row_add n=1 row=decision/answer-hidden-field-set area=8-pii-scrubbing-compliance fact=F1 detector=D-obs-pii
name: The answer's own hidden field set
rule: A caused item of an answer is judged by the answer's own hidden set of field names, apart from the sink's content-named set, a hidden field reading [redacted] on every answer carrier.
why: A boundary widening, ratified.
except: A field named error, which the sink redacts, is shown in a caused item as written, cut at the text bound.
basis: fct-tylyvsk5
because: The chunk splits the answer's hidden set from the sink's content set and lets `error` cross as written by a ratified widening, a decision no row of the PII section yet states.
