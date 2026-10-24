# Classified — test-plan · mocking-stubbing-discipline.b2

## Answers
mocking-stubbing-discipline#3.3 · no · the session adds no mock, stub or fake (the task's Untouched line: no stand-in was added), so this slice's absence holds
mocking-stubbing-discipline#3.4 · yes · stale · stand_snapshot_parts now reads many_items from the shared fixtures module it moved into, and stand_act_spans reads it too, so this slice's fixture reading can have moved on a reached file
mocking-stubbing-discipline#3.5 · no · host_log.rs's new process-client check adds no fixture, mock or stub, and the fixture move sits in blitz-tests' fixtures module, so this reading holds
mocking-stubbing-discipline#3.6 · no · the session adds no mock, fake or stub (the task's Untouched line: no stand-in was added), so the no-match reading over this slice holds
mocking-stubbing-discipline#3.7 · no · the session adds no mock, stub or fake (the task's Untouched line: no stand-in was added), so this slice's absence holds
mocking-stubbing-discipline#3.8 · no · the session adds no mock, stub or fake (the task's Untouched line: no stand-in was added), and test_ci_workflows.py only pins the ten words, so this slice's absence holds
