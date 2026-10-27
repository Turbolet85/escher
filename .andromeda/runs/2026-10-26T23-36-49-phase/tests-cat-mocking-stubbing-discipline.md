# tests · Mocking & Stubbing Discipline — category read

## Nothing in scope
Nearest: (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:16) the `LoadNet` row governs the load page's network stand-in, through which `stand_load_page` reads the lint; this chunk only factors `lint.rs::findings` and two table-asserting tests into table walks or helpers, adds, removes and changes no stand-in, fake or module-level dependency, and must leave every stand-in row (`HeldNet`, `LateNet`, `LoadNet`, `TimerTicks`) as it stands, so the category holds nothing this chunk must match beyond the scope's own "every check green".
