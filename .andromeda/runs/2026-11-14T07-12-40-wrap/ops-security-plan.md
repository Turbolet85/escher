@row_add n=1 row=decision/no-run-seed-on-driver-carriers area=input-validation fact=F1 detector=D-security-input
name: No run seed on the driver's carriers
rule: No run seed crosses any driver carrier: no verb, argument, command-line flag, MCP tool field or kept-check key takes or carries one, and every driver session's run reads the default seed.
state: provisional
basis: fct-gn7pklgp
because: The chunk answered fork F1 as A, provisionally, so no seed widens the validated driver surface, and no standing row says so.
@row_add n=2 row=decision/seeded-source-in-process-seed-only area=input-validation fact=F2 detector=D-security-input
name: Seeded source takes its seed from in-process code only
rule: The seeded source of randomness takes its seed only from in-process code, a root context the virtual DOM's builder gives else the default seed 0, and outside any run seeds from the host's per-process random keys, reading no environment variable, file or carrier.
basis: fct-uaxpbl54
because: The chunk adds the seeded source with no external input, and the input-validation rows hold no rule that keeps its seed off every outside source.
@row_add n=3 row=decision/seeded-source-holds-no-state-out area=data-protection fact=F3 detector=none
name: Seeded source exposes no state
rule: The seeded source of randomness holds a number and nothing else; its debug form writes the type name alone, never its state, and it logs nothing.
guard: debug_writes_no_state
basis: fct-ltjnlyo6
because: The chunk adds the seeded source whose state never leaves by its debug form or a log, and no data-protection row says so.
