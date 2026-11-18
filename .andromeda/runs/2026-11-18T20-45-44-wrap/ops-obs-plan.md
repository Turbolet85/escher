@row_set n=1 row=decision/cold-agent-transcript-unscrubbed cell=rule seen=1 fact=F2 detector=none
value: The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target; its kept live-run evidence stays byte-faithful, a misfiring probe over it revised and the transcript never edited.
basis: fct-doboggrw
because: The operator ruled that the kept evidence stays byte-faithful and the probe changes, never the transcript, so the row on the raw transcript must carry that rule.
@row_add n=1 row=decision/credential-probe-reads-valued-key area=pii-scrubbing-compliance fact=F1 detector=none
name: Credential probe reads a valued key name
rule: A credential probe over a kept agent transcript counts a key name carrying a value, never a bare field name; the client's init field apiKeySource with value none is no credential.
why: A field-name pattern matched the client's init field written in every run, so the probe could never read 0 on a real transcript.
basis: fct-flpwaicw
because: The credential probe over kept transcripts was revised to read a key name carrying a value, a rule for every later probe that no row of the section states.
