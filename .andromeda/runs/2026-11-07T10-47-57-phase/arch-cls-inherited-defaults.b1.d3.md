@answer n=1 entity=inherited-defaults#1.1 v=no
why: publishability flags and workspace versioning of the published crates; the chunk adds a module and moves no manifest, version or publish flag (not_needed n=9)
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: the fmt, clippy -D warnings and rustdoc -D warnings gates run as ci-leg.sh legs, a convention every change owes (S4)
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: tracing gated per call site with a cfg(not(feature = "tracing")) fallback in the engine crates; task item n=11 names it and the buffer module's traces must follow it
