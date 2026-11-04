@answer n=1 entity=secret-management#1.1 v=no
why: production CI signing and token secrets in the publish, wpt and wpt-post-results workflows; the chunk touches no workflow and that signing material is the scope's not_needed item
@answer n=2 entity=secret-management#1.2 v=no
why: the dev-host cold-agent pipe, operator login and live trial; the probe runs no model session (scope not_needed item)
@answer n=3 entity=secret-management#2.1 v=no
why: the credential-word absence over s02 names no reached path, and the probe adds no credential handling (its masked password value is carried by the masking task item)
@answer n=4 entity=secret-management#2.2 v=no standing=S1
why: its env-read absence over s03 states the convention that a new test reads no env var beyond the registered build-time ones
@answer n=5 entity=secret-management#2.3 v=yes
why: the s04 search covers examples/seven_guis/cells.rs, a reached path, and the probe's seven_guis example must keep the credential-word baseline where only the formula Token enum matches
@answer n=6 entity=secret-management#2.4 v=yes
why: the one password occurrence maps input type password to Role::PasswordInput, which the probe's masked password value (task item n=4) reads through the bridge
@answer n=7 entity=secret-management#2.5 v=no standing=S1
why: its env-read absence over s06 states the no-env-read convention for new tests, and its api_key pattern names no reached path
@answer n=8 entity=secret-management#2.6 v=no standing=S1
why: its env::var absence over s07 states the no-env-read convention for new tests, and its secret and token pattern names no reached path
@answer n=9 entity=secret-management#2.7 v=no
why: the no-credential-match search over s08 names no reached path, and the probe adds no credential handling
@answer n=10 entity=secret-management#2.8 v=no standing=S1
why: its std::env, env! and getenv absence over s09 states the no-env-read convention for new tests, and its credential pattern names no reached path
@answer n=11 entity=secret-management#2.9 v=no standing=S1
why: its env-only absence over s10, whose one match is a test, states the convention that a new test reads no env var beyond the registered build-time ones
@answer n=12 entity=secret-management#2.10 v=no standing=S1
why: its std::env, env::var and getenv absence over s11 states the no-env-read convention for new tests, and its credential pattern names no reached path
@answer n=13 entity=secret-management#2.11 v=no
why: the credential-word absence over s12 names no reached path, and the probe adds no credential handling
@answer n=14 entity=secret-management#2.12 v=no
why: the credential-word absence over the listed s13 files names no reached path, and the probe adds no credential handling
@answer n=15 entity=secret-management#2.13 v=no standing=S5
why: its host-path absence over the docs and README states the no-host-path rule for committed evidence
