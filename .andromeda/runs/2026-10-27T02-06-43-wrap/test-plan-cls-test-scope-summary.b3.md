# Classified — test-plan · test-scope-summary.b3

## Answers
test-scope-summary#12.1 · no · the two served-page checks are not reached, and their lint-at-boot reading still holds since lint.rs keeps its findings and their order and the sweep test keeps its rows and witnesses, with CI#40434241601 green 17/17
test-scope-summary#12.2 · no · stand_snapshot.rs and host_log.rs are not reached, and no span, field or event was added, so the flight booker's select reading and the host's no-id, no-name log reading hold
test-scope-summary#12.3 · no · examples/seven_guis is unchanged and no escher-driver public symbol moved, so the stand module's three named unit tests and its 5-passed count hold
test-scope-summary#12.4 · no · lint.rs is reached, but its findings and their order are unchanged, stand_lint stays green at 7 tests, and its test module only shifted from :526-604 to :542-620 with no test added, so no unit test pins the select-option overlap and overlaps still reads it
test-scope-summary#13.1 · no · the WPT runner under wpt/runner is not reached and nothing under it changed, so its unit-test counts hold
