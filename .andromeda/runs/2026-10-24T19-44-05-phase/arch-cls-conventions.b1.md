# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the fmt and clippy legs at ci-leg.sh:20-25 gate the reached ci-leg.sh merge surface and the changed test files
conventions#2.1 · no · no Cargo manifest is reached and the chunk adds no crate or dependency, so only the every-change re-check applies
conventions#3.1 · yes · the reached test files and common/mod.rs keep their `//!` docs accurate for the widened rows, and a new table constant states its rationale
conventions#4.1 · no · the TODO/FIXME and ordering markers sit in engine and runtime sources the chunk does not reach, and its known limits are owned by residuals lines
conventions#5.1 · yes · the select row reads the choice only through public `BaseDocument::select_value` or the snapshot's `value`, since `node::select` is crate-private
conventions#6.1 · no · the chunk adds no feature, cfg gate or tracing site, and its a11y checks build under the default `accessibility` feature
conventions#7.1 · no · the chunk writes no unsafe block and reaches no engine borrowing code
conventions#8.1 · yes · the reached test files keep one behaviour per file under a `//!` doc in both layout modes with shared tables in common/mod.rs, and the CI Python pin stays unittest with one TestCase per function
conventions#9.1 · no · the chunk edits nothing in escher-driver, so its unit-test modules are not reached
conventions#9.2 · no · the new select row reads the stand's flight booker and adds no fixture page, so tests/fixtures/mod.rs is not reached
conventions#10.1 · no · the chunk edits no app or example source, and the stand styles its rows read are not reached
conventions#11.1 · no · the chunk touches no examples/assets HTML fixture
