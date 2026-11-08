@answer n=1 entity=inherited-defaults#1.1 v=no
why: publish flags and the workspace version set are a re-check every change owes; the chunk adds no crate and moves no manifest.
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: fmt, clippy -D warnings and rustdoc -D warnings through ci-leg.sh are the S4 convention; the coverage floor and stand-repeat budget are S6, and the cargo-deny audit is untouched since the chunk adds no dependency.
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: the scope's telemetry item (escher-driver spans built by hand and ungated) binds the new verb's command span and the new code's tracing, which must stay gated per call site as the rule states.
