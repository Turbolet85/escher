@answer n=1 entity=secret-management#1.1 v=no
why: CI secrets of the publish-browser and wpt workflows, which the task drops as not_needed (publish workflow and WPT runner); the chunk touches neither workflow
@answer n=2 entity=secret-management#1.2 v=no standing=S5
why: committed-evidence hygiene (no key, credential-file name or token field) that every committed evidence owes under S5; the cold-agent login itself is not_needed
@answer n=3 entity=secret-management#2.1 v=yes
why: the masked password-input fixture (task item 4) carries the searched word `password`, so this census over the 21 s02 files binds if that group holds the chunk's new tests
@answer n=4 entity=secret-management#2.2 v=yes
why: the probe's registered build-time env! reads (CARGO_MANIFEST_DIR, CARGO_TARGET_TMPDIR, CARGO_BIN_EXE) and the masked password fixture both trip this census, if the s03 group holds the chunk's tests
@answer n=5 entity=secret-management#2.3 v=yes
why: the masked password-input fixture (task item 4) carries `password`, which this census over the 86 s04 files searches, if that group holds the chunk's tests
@answer n=6 entity=secret-management#2.4 v=no
why: its pattern (env::var, secret, token, authorization, cookie, x-api-key) is one the probe's compile-time CARGO_* reads and masked fixture do not write
@answer n=7 entity=secret-management#2.5 v=no
why: a census over 17 listed s06 files that the probe neither edits nor adds to; its CARGO_* reads are compile-time env!, not env::var or std::env
@answer n=8 entity=secret-management#2.6 v=no
why: a census of env::var, secret and token over 8 s07 files, which the probe's new text does not match
@answer n=9 entity=secret-management#2.7 v=no
why: a census over 16 listed s08 files that the probe edits none of
@answer n=10 entity=secret-management#2.8 v=no
why: a census over 32 listed s09 files that the probe edits none of
@answer n=11 entity=secret-management#2.9 v=yes
why: the probe's registered env! reads of CARGO_MANIFEST_DIR, CARGO_TARGET_TMPDIR and CARGO_BIN_EXE add instances that the one-match env census over the 32 s10 files must record, if that group holds the chunk's tests
@answer n=12 entity=secret-management#2.10 v=no
why: a census over 21 listed s11 files that the probe neither edits nor adds to
@answer n=13 entity=secret-management#2.11 v=yes
why: the masked password-input fixture (task item 4) carries `password`, which this census over the 61 s12 files searches, if that group holds the chunk's tests
@answer n=14 entity=secret-management#2.12 v=no
why: a census over 12 listed s13 files that the probe edits none of
@answer n=15 entity=secret-management#2.13 v=no standing=S5
why: a host-path and credential census over README, docs and the pin, which the chunk does not reach; the no-host-path rule for its own committed evidence is the S5 convention
