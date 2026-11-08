@answer n=1 entity=secret-management#1.1 v=no
why: the CI signing, keystore and token writes of the publish and wpt workflows are not reached by the UI graph, which is the not-needed CI secrets item
@answer n=2 entity=secret-management#1.2 v=no
why: the operator's claude.ai login is held outside the repository, the chunk adds no secret source and the cold-agent pipe is not changed
@answer n=3 entity=secret-management#2.1 v=no
why: a credential-absence re-check over the 21 s02 files that the graph adds no credential to, so a re-check every change owes
@answer n=4 entity=secret-management#2.2 v=no standing=S1
why: the no-environment-read convention for the library and the escher-session binary, which S1 names
@answer n=5 entity=secret-management#2.3 v=no
why: a credential-absence re-check over the 86 s04 files, whose only hit is the formula Token enum and not a credential, so a re-check every change owes
@answer n=6 entity=secret-management#2.4 v=yes
why: the only password handling in the scanned files is the input type mapping to Role::PasswordInput, which the graph's password value masking must match
@answer n=7 entity=secret-management#2.5 v=no standing=S1
why: the no-environment-read convention for the library, which S1 names
@answer n=8 entity=secret-management#2.6 v=no standing=S1
why: the no-environment-read convention for the library, which S1 names
@answer n=9 entity=secret-management#2.7 v=no
why: a credential-absence re-check over the 16 listed s08 files, which the chunk leaves alone under the not-needed form item, so a re-check every change owes
@answer n=10 entity=secret-management#2.8 v=no standing=S1
why: the no-environment-read convention for the library, which S1 names
@answer n=11 entity=secret-management#2.9 v=no standing=S1
why: the no-environment-read convention for the library, which S1 names, and its one test read is the test-only read S1 already records
@answer n=12 entity=secret-management#2.10 v=no standing=S1
why: the no-environment-read convention for the library, which S1 names
@answer n=13 entity=secret-management#2.11 v=no
why: a credential-absence re-check over the 61 s12 files that the graph adds no credential to, so a re-check every change owes
@answer n=14 entity=secret-management#2.12 v=no
why: a credential-absence re-check over the 12 listed s13 files that the graph adds no credential to, so a re-check every change owes
@answer n=15 entity=secret-management#2.13 v=no
why: a host-path, key and e-mail re-check over the README, the capabilities doc and the pin, which the chunk's checks do not change, so a re-check every change owes
