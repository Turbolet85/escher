@row_add n=1 row=decision/bounded-package-install area=standard-contracts rec=1
name: Bounded package install
rule: Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains.
guard: InstallScriptTest; test_o_package_installs_are_bounded
@row_add n=2 row=decision/windows-entries-pass-through area=standard-contracts rec=1
name: Windows script entries are pass-throughs
rule: The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic.
@row_add n=3 row=contract/cold-agent-verdict/wrong-call-bar-pct area=standard-contracts rec=2,1
boundary: cold-agent verdict.json
item: wrong_call_bar_pct
promise: The verdict carries `wrong_call_bar_pct`, 5.
