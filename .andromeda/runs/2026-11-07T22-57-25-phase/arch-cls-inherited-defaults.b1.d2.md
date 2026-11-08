@answer n=1 entity=inherited-defaults#1.1 v=no
why: release metadata (publish flags, workspace version, in-repo pins) that the graph code and tests do not change, and the chunk adds no in-repo crate edge or version move
@answer n=2 entity=inherited-defaults#2.1 v=no standing=S3
why: the fmt, clippy and rustdoc gates every change passes through ci-leg, which standing S3 names
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: Cargo features gate optional capabilities, and the graph reader and snapshot field follow the accessibility gate with features forwarded down the crates (task n=16)
