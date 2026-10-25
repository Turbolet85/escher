# Classified — architecture · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · no · the session changed no manifest (its Untouched list names no manifest, lock, deny.toml or toolchain change) and none of the Cargo.toml files holding the publish flags it cites is among the reached files, so the publishability claim still holds
inherited-defaults#2.1 · no · ci-leg.sh changed only the a11y leg's test list at line 39, inside the cited 37-64 block with the file's line count unchanged, the fmt, clippy, rustdoc, audit, coverage and stand-repeat legs it names are untouched and ci.yml is byte-identical, so its claim that these gates run through the script still holds on the changed file
inherited-defaults#3.1 · no · the session changed no Rust source, Cargo feature, tracing call site or log site (silence census 0), and none of the Cargo.toml and net.rs lines it cites is among the reached files, so its feature-gating and ungated-crate claims still hold
