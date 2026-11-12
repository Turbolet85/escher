@row_add n=1 row=decision/stand-repeat-zero-failure-budget area=stack-and-technologies rec=1,2
name: Stand-repeat zero-failure budget
rule: The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed.
guard: test_stand_repeat_leg_runs_three_rounds
@row_add n=2 row=contract/event-dispatch/enter-click area=standard-contracts rec=3
boundary: Event dispatch
item: pressed Enter
promise: A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
