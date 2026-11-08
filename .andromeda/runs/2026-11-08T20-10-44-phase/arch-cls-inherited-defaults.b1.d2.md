@answer n=1 entity=inherited-defaults#1.1 v=no
why: publish flags and workspace versioning; the chunk moves no manifest, version or publish setting (not-needed item 8 on release versioning)
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: fmt, clippy -D warnings and rustdoc -D warnings through ci-leg.sh is the convention S4 names; the coverage floor and stand-repeat budget it also lists are S6
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: task item 18 keeps escher-driver's command and tool_call spans ungated, and any new tracing in the engine crates must stay gated per call site
