@answer n=1 entity=secret-management#1.1 v=no
why: the CI secrets sit in the publish-browser and wpt workflows, which the chunk does not touch
@answer n=2 entity=secret-management#1.2 v=no
why: the dev-host login and live runs are outside the chunk, and its no-new-secret-source posture is a re-check every change owes
@answer n=3 entity=secret-management#2.1 v=no
why: a credential and header-word scan over the 21 s02 files, and the chunk adds no fetch or request (task not_needed n=1, n=5)
@answer n=4 entity=secret-management#2.2 v=yes
why: the env-read half binds because the scope names the driver's and the new tests' env reads (task item 16), and the new tests' compile-time env! use matches the pattern
@answer n=5 entity=secret-management#2.3 v=no
why: a credential-word scan over the 86 s04 files, the only match a formula Token, and the chunk adds no credential handling
@answer n=6 entity=secret-management#2.4 v=no
why: the one password match is the blitz-dom input-type role in a file the chunk does not reach
@answer n=7 entity=secret-management#2.5 v=yes
why: the env-read half binds because the scope names the driver's env reads and the new tests' compile-time route (task item 16)
@answer n=8 entity=secret-management#2.6 v=yes
why: the env-read half binds because the scope names the new tests' compile-time directory route (task item 16)
@answer n=9 entity=secret-management#2.7 v=no
why: a credential-word scan with no env pattern over the 16 listed s08 files, a re-check every change owes
@answer n=10 entity=secret-management#2.8 v=yes
why: the env-read half (std::env, env!, getenv) binds because the scope names the driver's and the new tests' env reads (task item 16)
@answer n=11 entity=secret-management#2.9 v=yes
why: the env-read half binds because the scope names the new tests' compile-time env! route and the driver's no-env rule (task item 16)
@answer n=12 entity=secret-management#2.10 v=yes
why: the env-read half binds because the scope names the driver's and the new tests' env reads (task item 16)
@answer n=13 entity=secret-management#2.11 v=no
why: a credential-word scan with no env pattern over the 61 s12 files, a re-check every change owes
@answer n=14 entity=secret-management#2.12 v=no
why: a credential-word scan over the 12 listed s13 files, a re-check every change owes, since the reached docs and script carry their own scan at 2.13
@answer n=15 entity=secret-management#2.13 v=yes
why: it names docs/capabilities.md and .github/scripts/test_capability_statement.py, both reached by the chunk (task_file n=13, n=14), so no /home/ path, API key, authorization header or bearer text may enter them
