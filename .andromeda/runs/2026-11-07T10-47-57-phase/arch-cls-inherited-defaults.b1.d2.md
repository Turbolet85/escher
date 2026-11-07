@answer n=1 entity=inherited-defaults#1.1 v=no
why: the publish and versioning set of manifests; the chunk adds a crate-private module and moves no manifest, publish flag, version or pin
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S4
why: the fmt, clippy and rustdoc -D warnings legs through ci-leg.sh, a convention the task lists
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: task item 11 requires per-call-site cfg(not(feature = "tracing")) gating of traces in engine crates, and the new buffer module in dioxus-native-dom may keep traces
