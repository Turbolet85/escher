@answer n=1 entity=inherited-defaults#1.1 v=no
why: the publish flags and the in-repo `=` version pins are not moved by the chunk, which adds no crate, no publish flag and no version change
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S3
why: fmt, clippy and rustdoc `-D warnings` through ci-leg.sh are the convention S3 names, a standing convention every change owes
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: the accessibility Cargo feature gates the graph reader and the snapshot field, and features are forwarded down the crates (task item 16)
