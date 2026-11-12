@row_add n=1 row=contract/test-harness/settle area=standard-contracts rec=1,2
boundary: Test harness
item: settle
promise: settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
@row_add n=2 row=decision/actionable-elements-keyed area=standard-contracts rec=3,4
name: Every actionable element reads an author key
rule: Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /.
why: A ratified public contract addition.
guard: stand_actionable_keys
@row_add n=3 row=contract/escher-session-exit-codes/0 area=standard-contracts rec=5,6
boundary: escher-session exit codes
item: 0
promise: The command was accepted.
