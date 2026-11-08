@answer n=1 entity=inherited-defaults#1.1 v=no
why: publish flags and workspace versioning govern manifests and releases, which the chunk moves none of (release versioning is not_needed item 8)
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: the cargo fmt, clippy -D warnings and rustdoc -D warnings gate through ci-leg.sh is the convention standing S4 names, owed by every change
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: escher-driver's command and tool_call spans are built by hand and ungated, which the chunk's driver spans and the telemetry item (needs 18) must honour
