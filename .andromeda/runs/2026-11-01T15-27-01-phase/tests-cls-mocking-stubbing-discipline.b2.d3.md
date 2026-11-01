# Classified — tests · mocking-stubbing-discipline.b2

## Answers
mocking-stubbing-discipline#2.1 · no · the hover_state_tests fixtures live in packages/blitz-dom/src/document.rs, which is not in Reached files, so the chunk neither edits nor re-reads it
mocking-stubbing-discipline#3.1 · yes · Cargo.toml (coupled pins), .github/workflows/ci.yml and ci-leg.sh are edited and the manifests and lock are re-read for new packages, so a mocking crate arriving in build config would break this absence
mocking-stubbing-discipline#3.2 · yes · examples/seven_guis/src/tasks/cells.rs is a conflict surface in the apps (seven_guis) this absence covers, so a resolution there can move the rule
mocking-stubbing-discipline#3.3 · no · slice s03 is not named in Reached files and no reached file is shown to sit in it
mocking-stubbing-discipline#3.4 · no · slice s06 is not named in Reached files and no reached file is shown to sit in it
mocking-stubbing-discipline#3.5 · no · slice s07 is not named in Reached files and no reached file is shown to sit in it
mocking-stubbing-discipline#3.6 · no · slice s08 has no mock, fake or stub match, and no reached file is shown to sit in it
mocking-stubbing-discipline#3.7 · no · slice s09 is not named in Reached files and no reached file is shown to sit in it
mocking-stubbing-discipline#3.8 · no · slice s11 is not named in Reached files and no reached file is shown to sit in it
