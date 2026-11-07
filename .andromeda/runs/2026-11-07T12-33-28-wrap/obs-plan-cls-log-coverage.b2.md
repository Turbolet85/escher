@answer n=1 entity=log-coverage#9.5 v=no
why: the host_log option-needle census sits in an unreached seven_guis test file, and the replay re-fires the same MutationWriter trace! calls so the log stream is unchanged
@answer n=2 entity=log-coverage#9.6 v=no
why: the mcp_log option-needle census sits in an unreached seven_guis test file, and the replay re-fires the same trace! calls so the log stream is unchanged
@answer n=3 entity=log-coverage#9.7 v=no
why: seven_guis's shared option_needles module is not among the reached files and no session change touches its lines
@answer n=4 entity=log-coverage#9.8 v=yes mode=stale
why: the row's "packages/ unchanged" reading no longer holds because the session changes packages/dioxus-native-dom's mutation_buffer, dioxus_document and lib.rs, though the census over the ten a11y files still reads 0
@answer n=5 entity=log-coverage#9.9 v=no
why: lint.rs, mcp_tests.rs and stand_requirement_sweep.rs sit in the unchanged driver and stand files, and the session adds no print, env read or span there
@answer n=6 entity=log-coverage#9.10 v=no
why: its census is pinned to the fixed sha ebacd68bd and blitz-vibey-script's tests are not among the reached files
@answer n=7 entity=log-coverage#9.11 v=no
why: the session adds no env read, print, tracing site or wall-clock read, so the upstream take's two-hit census stands
@answer n=8 entity=log-coverage#9.12 v=no
why: the five checks' files are not reached, and the session adds no print, env read, subscriber or listener to them
@answer n=9 entity=log-coverage#9.13 v=yes mode=add-to
why: the probe file is reached, and F3 (the two renamed pins install no subscriber, read no env var, print nothing and bind no listener, the silence census reading 0) belongs beside this silence row
@answer n=10 entity=log-coverage#9.14 v=no
why: the popup checks' files are not reached, and the session adds no print, env read, subscriber or listener to them
