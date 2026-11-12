@row_add n=1 row=contract/cli-exit-codes/3 area=error-handling rec=1,2
boundary: CLI exit codes
item: 3
promise: Session error: the error object with kind, message and remedy on stdout and its fixed message on stderr; a boot that fails ends with its session error.
@row_add n=2 row=decision/screenshot-in-band area=input-validation rec=3,4,5
name: Screenshot carried in band
rule: A screenshot's PNG crosses in band, as base64 inside its JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked, under the unchanged answer bound.
why: The in-band image was the ratified choice, a boundary widening.
@row_add n=3 row=contract/actions-answer/nodes area=input-validation rec=3,6
boundary: The actions answer
item: nodes
promise: `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
@row_add n=4 row=contract/actions-answer/keyed area=input-validation rec=7,6
boundary: The actions answer
item: keyed
promise: A boolean on each listed node; the field is provisional.
@row_add n=5 row=decision/actions-keyed-field area=input-validation rec=8,9
name: Keyed flag in the actions answer
rule: The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId.
why: A ratified boundary widening.
state: provisional
