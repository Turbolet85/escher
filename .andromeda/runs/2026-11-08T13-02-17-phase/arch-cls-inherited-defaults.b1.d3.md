@answer n=1 entity=inherited-defaults#1.1 v=no
why: publish = false on escher-driver and the publishing rules are not moved; the read changes no manifest, publish setting or release, and adds no crate
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: the convention that each new line passes cargo fmt, clippy -D warnings and rustdoc -D warnings through ci-leg.sh, which S4 names; the cargo-deny, coverage-floor and stand-repeat legs are every change's CI gates
@answer n=3 entity=inherited-defaults#3.1 v=no standing=S5
why: tracing ungated in escher-driver's command and tool_call spans, the convention S5 names for each new span or log line there; the engine-crate gating and Cargo features are untouched
