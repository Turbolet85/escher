@answer n=1 entity=inherited-defaults#1.1 v=no
why: the session adds no crate and changes no manifest, so the publish flags it cites (including escher-driver and blitz-tests) stand unchanged
@answer n=2 entity=inherited-defaults#2.1 v=no
why: the session leaves CI workflows, the ci-leg script, the coverage floors and the stand-repeat budget unchanged, and the new code meets the same gates the entity states without counts
@answer n=3 entity=inherited-defaults#3.1 v=no
why: the session adds no Cargo feature and no tracing span, and the cited manifests and the ungated escher-driver spans stay as they are
