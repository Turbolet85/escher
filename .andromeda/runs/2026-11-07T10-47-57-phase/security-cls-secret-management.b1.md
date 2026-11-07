@answer n=1 entity=secret-management#1.1 v=no
why: production CI secrets (signing key, keystore, Apple certificate, WPT and GITHUB_TOKEN) live in workflow files the chunk does not edit; the chunk adds no secret, CI step or workflow
@answer n=2 entity=secret-management#1.2 v=no
why: the development-host login and the cold-agent and live-trial secret footprint sit outside the Dioxus bridge; the chunk reaches no pipe, trial or env read
@answer n=3 entity=secret-management#2.1 v=yes
why: its scan term set includes password, and the new buffer module names password and file inputs (task need 2), so the scan must still hold over whatever group the module lands in
@answer n=4 entity=secret-management#2.2 v=yes
why: its scan term set includes password, which the new buffer's password and file-input handling can introduce into the scanned group; the chunk adds no env read
@answer n=5 entity=secret-management#2.3 v=yes
why: its scan term set includes password, which the new buffer's password and file-input handling can introduce; the scope does not name this group, so the fork stays open
@answer n=6 entity=secret-management#2.4 v=no
why: its scan terms are env::var, secret, token, authorization, cookie and x-api-key, none of which the chunk introduces
@answer n=7 entity=secret-management#2.5 v=no
why: its scan terms are env::var, std::env and api_key; the chunk reads no environment variable and adds no api_key
@answer n=8 entity=secret-management#2.6 v=no
why: its scan terms are env::var, secret and token; the chunk adds none of them
@answer n=9 entity=secret-management#2.7 v=yes
why: its scan term set includes password, which the new buffer's password and file-input handling can introduce into the scanned group
@answer n=10 entity=secret-management#2.8 v=yes
why: its scan term set includes password, which the new buffer's password and file-input handling can introduce into the scanned group
@answer n=11 entity=secret-management#2.9 v=no
why: its scan terms are std::env, env::var and env! only; the chunk's new tests read no environment variable, so the environment-values convention does not apply
@answer n=12 entity=secret-management#2.10 v=yes
why: its scan term set includes password and auth, which the new buffer's password and file-input handling can introduce into the scanned group
@answer n=13 entity=secret-management#2.11 v=yes
why: its scan term set includes password, which the new buffer's password and file-input handling can introduce into the scanned group
@answer n=14 entity=secret-management#2.12 v=yes
why: its scan term set includes password, which the new buffer's password and file-input handling can introduce into the scanned group
@answer n=15 entity=secret-management#2.13 v=no
why: it scans README, docs/capabilities.md and the capability-statement script for host paths, keys and e-mail domains; the chunk edits no docs or that script
