# Classified — test-plan · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · no · examples/seven_guis and the escher-session binary have no diff, and the session names none of the snapshot sizes or the commands they measure
e2e-test-strategy#1.2 · no · the hosted-versus-in-process select-list shot is an examples/seven_guis test with no diff; the session moves only select_popup's test in blitz-tests
e2e-test-strategy#1.3 · no · the MCP tests (examples/seven_guis mcp_server and mcp_log, blitz-tests stand_mcp_tools) are unreached, and the stand counts and the new check's missing stand_ prefix leave them unmoved
e2e-test-strategy#2.1 · no · the RefCell panic sits at events.rs, which is not reached; the session's key-press fix is the layout_parent clear in deallocate_anonymous_block, a different trigger
e2e-test-strategy#3.1 · yes · stale · paint_scene sits in blitz-paint, whose lib.rs and render.rs are merge surfaces reached whole, and the report does not show the list's paint route still holds there
e2e-test-strategy#4.1 · no · the example fixtures are unreached, no examples/ file has a diff, and the session adds no assert or expect to any example file
e2e-test-strategy#4.2 · no · no file under apps/ was reached, and the session's new tests sit in packages/ and tests/blitz-tests, outside the apps search
