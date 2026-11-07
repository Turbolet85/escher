@answer n=1 entity=inherited-defaults#1.1 v=no
why: the session moves no manifest, and the publish flags it lists in Cargo.toml files still hold, the new private module adds no published crate or version change
@answer n=2 entity=inherited-defaults#2.1 v=no
why: the session changes no CI job, step, leg, or coverage floor, and the fmt, clippy and rustdoc gates still apply unchanged to the new module and the reached files
@answer n=3 entity=inherited-defaults#3.1 v=no
why: the session adds no Cargo feature and no tracing or log site, the replay re-fires the writer's existing trace calls only
