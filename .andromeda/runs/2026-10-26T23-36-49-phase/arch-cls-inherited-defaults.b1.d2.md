# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · no · the chunk edits no manifest and adds no crate, so escher-driver's and blitz-tests' `publish = false` and the workspace version and in-repo pins stay as they are
inherited-defaults#2.1 · yes · the fmt, clippy `--locked -D warnings` and rustdoc `-D warnings` gates it states are the task's own Gates (`ci-leg.sh fast` and `doc`), which the refactored functions and helpers must pass
inherited-defaults#3.1 · yes · escher-driver's `command` span, which the Needs hold to recording `verb` and `cause` alone, stays ungated and the refactor adds no tracing call site
