# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · yes · the root Cargo.toml is a merge surface holding the virtual root (252-263) and the in-repo pins (44-60) this rests on, so a package the take brings keeps its publish flag; the release-versioning clause stays out under the Not needed list
inherited-defaults#2.1 · yes · the scope re-runs the fast and doc legs, the dependency audit and fork CI, and ci-leg.sh (a re-read file) runs their gates (fmt, clippy -D warnings, rustdoc -D warnings, cargo-deny over the whole graph) on the take's tree
inherited-defaults#3.1 · yes · the Cargo features rule rests on dioxus-native/Cargo.toml (lines 16-33), a merge surface whose default list the clipboard move changes where a take carries it, so the optional capability stays a feature; the per-call-site tracing gate is standing S5 and not carried
