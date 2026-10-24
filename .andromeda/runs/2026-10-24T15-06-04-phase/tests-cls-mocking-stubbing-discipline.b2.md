# Classified — tests · mocking-stubbing-discipline.b2

## Answers
mocking-stubbing-discipline#3.3 · yes · the stand-in rule binds these stub and mock hits, since a new fixture check uses HeldNet and TimerTicks rather than a new mock
mocking-stubbing-discipline#3.4 · yes · the fixture rule binds these hits, since fixtures are rsx! components and inline literals booted through fixtures::boot, nothing is seeded, and no new mock is added
mocking-stubbing-discipline#3.5 · yes · the same fixture rule binds these hits, since nothing is seeded and every fixture check boots through fixtures::boot
mocking-stubbing-discipline#3.6 · yes · no mock, fake or stub exists in this slice, and the chunk's new fixture checks add none, the stand-in rule holding wherever they land
mocking-stubbing-discipline#3.7 · yes · the mock and stub types here fall under the stand-in rule that a new fixture check reuses HeldNet and TimerTicks rather than a new type
mocking-stubbing-discipline#3.8 · yes · the mocks, stubs and fakes here are the doubles the stand-in rule governs, so the chunk's new checks reuse HeldNet and TimerTicks and add none
