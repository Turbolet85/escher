Question (implement, after the live run): gate entry 12, the credential probe over the committed live-* evidence, reads 1 hit — the field name `apiKeySource` in the transcript's init record (line 1), its value "none"; the client writes the field on every run and 0.2.0's committed copy carried it too. How to treat it?

Answer: "Stop here, surfaced"

Notes: The probe is at fault, not the evidence: keep live-transcript.jsonl byte-faithful, change nothing in it. I revise entry 12 through /andromeda-phase next; implement then re-enters at entry 12 alone. The live run stands as fired and is never re-fired.
