@row_add n=1 row=decision/credential-probe-reads-valued-keys area=secret-management fact=F1 detector=none
name: Credential probe reads a key carrying a value
rule: A credential probe over an agent client's transcript counts a key name carrying a value, never a field name alone, so the client's init field apiKeySource with the value none is no credential.
why: A field-name pattern matched the claude client's init field in every run's transcript and could never read clean.
guard: the probe's control, planting apiKeySource none as clean and an api_key line with a value as a hit
basis: fct-flpwaicw, fct-p7guf2ud
because: The chunk revised the credential probe to read a key name carrying a value after a field-name pattern hit apiKeySource, and no row of Secret Management states how a credential probe reads the evidence.
@row_add n=2 row=decision/cold-agent-live-run-fired-once area=threat-model-summary fact=F2 detector=none
name: Founder's cold-agent live run fired once
rule: The founder's live cold-agent run to the model provider is fired once and never re-fired, even across a surfaced stop or a plan revision; a second live run to the provider needs the founder's new word.
basis: fct-but3etew, fct-omkbgbqa
because: The chunk held the one live run unre-fired across its surfaced stop and the plan's revision, and no row of the provider crossing states that bound on live runs.
