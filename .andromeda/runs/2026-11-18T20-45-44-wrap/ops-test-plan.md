@row_add n=1 row=decision/cold-agent-live-run-once-per-version area=3-test-harness-contract fact=F1 detector=none
name: Live cold-agent run fired once per version
rule: The live cold-agent run is fired once per version under a 3 USD cap and is never re-fired, not across a surfaced stop nor a plan revision; a second run needs a new ruling.
why: ratified
basis: fct-but3etew
because: The report rules the live run a one-time firing never repeated across a surfaced stop and a plan revision, a decision no standing row of the harness contract states.
@row_add n=2 row=decision/credential-probe-reads-key-with-value area=3-test-harness-contract fact=F2 detector=none
name: Credential probe reads a key carrying a value
rule: A credential probe over an agent client's transcript reads a key name carrying a value, never a field name alone.
why: The client writes apiKeySource with value none in every run's init record, so a probe of the field name alone reads every transcript as holding a credential.
basis: fct-flpwaicw
because: The report revises the credential probe to read a key name carrying a value after the name-only pattern matched the client's init field, a convention binding every later probe over a transcript.
@row_add n=3 row=decision/live-run-evidence-byte-faithful area=3-test-harness-contract fact=F3 detector=none
name: Live-run evidence transcript kept byte-faithful
rule: The live cold-agent run's evidence transcript is kept byte-faithful: a probe that misreads it is changed, never the transcript.
why: ratified
basis: fct-doboggrw
because: The report rules that the evidence stays byte-faithful and the probe changes, a decision no standing row of the harness contract states.
@row_add n=4 row=decision/capability-statement-states-measured-live-run area=test-strategy fact=F4 detector=none
name: Capability statement states the measured live run
rule: The capability statement states the live cold-agent run as measured with its reach: the run's date, app and call counts, wrong calls and client version, and each capability out of its reach named with its stand witness.
why: ratified
basis: fct-qqkt3ncp
because: The report has the capability statement carry the measured run's counts and name the out-of-reach capabilities with their stand witnesses, a rule for the statement no standing row of the test strategy states.
