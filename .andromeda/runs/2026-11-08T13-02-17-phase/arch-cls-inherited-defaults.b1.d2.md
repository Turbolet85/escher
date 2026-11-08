@answer n=1 entity=inherited-defaults#1.1 v=no
why: the chunk adds no crate and edits no Cargo.toml, so escher-driver stays publish = false and the versioning rule is untouched
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: the fmt, clippy -D warnings and rustdoc -D warnings legs through ci-leg.sh are the convention S4 names for every new line; the deny and coverage legs are CI every change owes equally
@answer n=3 entity=inherited-defaults#3.1 v=no standing=S5
why: escher-driver's command and tool_call spans being ungated is the convention S5 names for every new span or log line in escher-driver
