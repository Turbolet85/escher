@row_add n=1 row=decision/ci-token-read-only area=authentication-authorization rec=1,2
name: Read-only CI tokens
rule: CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only.
guard: test_ci_workflows.py
@row_add n=2 row=decision/cold-agent-fixed-prompt area=input-validation rec=3,4
name: Cold-agent task as a fixed prompt
rule: A cold-agent task reaches the agent session only as the fixed prompt, naming no element id and no app slug.
guard: test_the_prompt_names_no_stand_id
@row_add n=3 row=decision/no-panic-on-call-input area=input-validation rec=5,6
name: No panic on call-supplied input
rule: Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
@row_add n=4 row=contract/escher-session-exit-codes/0 area=input-validation rec=7,8
boundary: escher-session exit codes
item: 0
promise: The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
@row_add n=5 row=contract/escher-session-exit-codes/1 area=input-validation rec=7,8
boundary: escher-session exit codes
item: 1
promise: The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
@row_add n=6 row=contract/escher-session-exit-codes/2 area=input-validation rec=7,8
boundary: escher-session exit codes
item: 2
promise: A usage error: the usage line goes to stderr and stdout stays empty.
@row_add n=7 row=contract/escher-session-exit-codes/3 area=input-validation rec=7,8
boundary: escher-session exit codes
item: 3
promise: A session error: the session-error object on stdout and the session error's fixed message on stderr.
@row_add n=8 row=decision/kept-checks-owner-only area=data-protection rec=9,10,11
name: Kept checks owner-only and uncommitted
rule: A kept check stays in an owner-only medium, is never logged and is never committed.
except: The two committed checks of the repository's own scripted stand-Counter sessions.
@row_add n=9 row=absence/served-network-api area=api-security rec=12,13
capability: A served network API: no binary serves a TCP or UDP port
@row_add n=10 row=absence/content-security-policy area=api-security rec=14,15
capability: CSP enforcement
