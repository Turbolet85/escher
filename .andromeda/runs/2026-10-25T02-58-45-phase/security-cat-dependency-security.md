# security · Dependency Security — category read

## Nothing in scope
Nearest: (.andromeda/security-plan.md:253) ci.yml's `audit` job ("Dependency audit") runs cargo-deny over the whole workspace's graph on every push, so it runs on this chunk's push too. The chunk is a capability statement, a document with no code, manifest, `Cargo.lock` or `deny.toml` change and no new dependency, so it moves nothing that job reads and takes on no dependency rule, pin, ignore or licence check of its own.
