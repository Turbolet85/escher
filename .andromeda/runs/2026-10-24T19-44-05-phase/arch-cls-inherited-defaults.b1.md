# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · no · the chunk edits no manifest and adds no crate, so blitz-tests keeps its `publish = false` untouched and no reached file carries publish metadata
inherited-defaults#2.1 · yes · the reached Rust tests must pass `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings`, legs that `ci-leg.sh` (a merge surface the widened `a11y` leg edits) runs, so the new leg stays one of its legs
inherited-defaults#3.1 · yes · the widened `a11y` targets build under blitz-dom's default `accessibility` Cargo feature, so the leg must keep default features on and add none; no tracing site is added or touched
