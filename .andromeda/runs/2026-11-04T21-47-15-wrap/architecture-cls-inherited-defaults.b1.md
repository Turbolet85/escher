@answer n=1 entity=inherited-defaults#1.1 v=no
why: the publish flags sit in manifests the session did not move, and the new target lives in tests/blitz-tests, already publish = false, while the Cargo.lock bump touches no in-repo version or publish setting
@answer n=2 entity=inherited-defaults#2.1 v=no
why: the cargo-deny audit of advisories and licences still runs through ci-leg.sh and ci.yml, which the session did not change, and the deny.toml ignore addition does not alter what the audit covers
@answer n=3 entity=inherited-defaults#3.1 v=no
why: the session changes no packages/ crate or tracing gate, and the new test target installs no subscriber and prints nothing
