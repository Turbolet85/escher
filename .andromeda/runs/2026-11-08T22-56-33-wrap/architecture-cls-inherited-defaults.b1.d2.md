@answer n=1 entity=inherited-defaults#1.1 v=no
why: the session moved no manifest, so the publish flags the entity cites in Cargo.toml still hold, and no crate was added
@answer n=2 entity=inherited-defaults#2.1 v=no
why: the CI workflows, ci-leg.sh and the coverage floors are unchanged, and the gates it states still hold over the new modules
@answer n=3 entity=inherited-defaults#3.1 v=no
why: the session adds no tracing span, event, log field or Cargo feature, and the manifests it cites are not reached
