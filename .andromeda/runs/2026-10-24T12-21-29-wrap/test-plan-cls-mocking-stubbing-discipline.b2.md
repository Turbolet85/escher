# Classified — test-plan · mocking-stubbing-discipline.b2

## Answers
mocking-stubbing-discipline#3.3 · no · the session adds no mock, stub or fake, and its one new test target builds its HtmlDocument directly with no shared module
mocking-stubbing-discipline#3.4 · no · the session adds no mock or fixture, and its one new test target reads no shared module, so the fixtures reader count stays 24
mocking-stubbing-discipline#3.5 · no · the session adds no mock or fixture to slice s07, and the shared-module reader counts stay unmoved
mocking-stubbing-discipline#3.6 · no · the session adds no mock, fake or stub, so slice s08 keeps its no-match reading
mocking-stubbing-discipline#3.7 · no · the session adds no mock, stub or fake, and its changes are engine code, an allocator swap and an example
mocking-stubbing-discipline#3.8 · no · the session adds no mock, stub or fake, and its new engine state is checked by existing integration checks
