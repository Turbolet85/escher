# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · no · the only edit is the a11y cargo line at ci-leg.sh:39, in place with the file's line count unchanged, so the fmt, clippy and doc legs cited at :20-25 and :37 still hold on the changed file
conventions#2.1 · no · no manifest, Cargo.lock or dependency changed
conventions#3.1 · no · no Rust source or doc comment changed
conventions#4.1 · no · no TODO or FIXME marker in the cited source was added or removed
conventions#5.1 · no · no Rust source changed, so no visibility or naming moved
conventions#6.1 · no · no feature, cfg gate or tracing call site added or moved (silence census 0)
conventions#7.1 · no · no Rust source changed, so no unsafe block or borrow workaround moved
conventions#8.1 · no · no test file added or reshaped and the one edit is an assertion list inside an existing pin, so its unittest, one-target-per-file and shared-module claims still hold on the changed files
conventions#9.1 · no · no escher-driver unit-test module file added, moved or renamed
conventions#9.2 · no · no test file added or reshaped, so the fixtures module and its readers hold
conventions#10.1 · no · no app or example source changed and the stand's tasks are unedited
conventions#11.1 · no · no file under examples/assets is among the reached files
