@row_add n=1 row=decision/script-seed-never-logged area=pii-scrubbing-compliance fact=F1 detector=D-obs-pii
name: Script document seed never logged
rule: A script document's Math.random seed comes only from the embedder's building code, never from page input, an env var or a carrier, and the seed and its draws are never logged.
basis: fct-xvmt5uxa
because: The chunk adds a seeded page Math.random set by the embedder's builder with no log site or env read, and the PII section must state that the seed and its draws stay off every log.
@row_add n=2 row=decision/boundary-registry-memory-only area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Error-boundary registry stays in memory
rule: The dioxus bridge's error-boundary registry holds each boundary's scope and error context in process memory only, pruned of dropped boundaries before each render, and nothing logs, fields or prints it.
basis: fct-xvmt5uxa
because: The chunk makes the registry hold each boundary's scope beside its error context and prunes it before each render, and the PII section must state that this held error context never reaches a log.
