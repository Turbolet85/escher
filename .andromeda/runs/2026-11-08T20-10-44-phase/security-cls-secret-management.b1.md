@answer n=1 entity=secret-management#1.1 v=no
why: the chunk edits no publish or WPT workflow and adds no CI secret, so the production secret sources stay where they are (items 6 and 7 are not reached)
@answer n=2 entity=secret-management#1.2 v=no
why: the chunk runs no live trial, changes no cold-agent pipe and adds no secret source or auth library; its env discipline is item 16 and the absence records below
@answer n=3 entity=secret-management#2.1 v=yes
why: the probe covers password and token words, and the password masking the chunk's scope names (item 8) must add no match to a reached file of this surface
@answer n=4 entity=secret-management#2.2 v=yes
why: the probe covers runtime env reads, which item 16 names for the escher-driver library and the new tests; compile-time env! stays allowed
@answer n=5 entity=secret-management#2.3 v=yes
why: the probe covers password and token words, and the masking of password values (item 8) must keep a reached file of this surface free of a match
@answer n=6 entity=secret-management#2.4 v=yes
why: the probe covers runtime env reads and token words, and item 16 names the driver's env reads, so a reached file of this surface must add no runtime read
@answer n=7 entity=secret-management#2.5 v=yes
why: the probe covers runtime env reads and api_key, and item 16 names the driver's env reads and the new tests' directories
@answer n=8 entity=secret-management#2.6 v=yes
why: the probe covers env reads, secret and token words, and item 16 and the masking scope (item 8) name both the env reads and the password values
@answer n=9 entity=secret-management#2.7 v=yes
why: the probe covers password, token and secret words, and the no-control-value answers the masking scope names (item 8) may not add a match in a reached file
@answer n=10 entity=secret-management#2.8 v=yes
why: the probe covers env reads and credential words, and item 16 names the driver's env reads while item 8 names password values
@answer n=11 entity=secret-management#2.9 v=yes
why: the probe covers env reads and its one match is a test, and item 16 binds the new tests to compile-time CARGO_TARGET_TMPDIR or CARGO_MANIFEST_DIR only
@answer n=12 entity=secret-management#2.10 v=yes
why: the probe covers env reads and auth words, and item 16 names the driver's and escher-session's env reads
@answer n=13 entity=secret-management#2.11 v=yes
why: the probe covers credential words over a 61-file surface, and the password masking (item 8) must add no match to a reached file of it
@answer n=14 entity=secret-management#2.12 v=yes
why: the probe covers password and credential words over a set of docs and scripts, and the masking scope names password values the capability statement must not carry
@answer n=15 entity=secret-management#2.13 v=yes
why: the chunk edits docs/capabilities.md and .github/scripts/test_capability_statement.py, which this probe reads, so the new verb's doc must add no home path, bearer, authorization, api key or e-mail domain
