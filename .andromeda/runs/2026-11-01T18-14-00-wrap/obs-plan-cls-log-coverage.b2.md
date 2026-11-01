# Classified — obs-plan · log-coverage.b2

## Answers
log-coverage#9.5 · no · examples/seven_guis is untouched and mcp_log was re-run, still reading 0 option needles at trace
log-coverage#9.6 · no · examples/seven_guis/tests/common/mod.rs is not reached and the session left examples/seven_guis whole unedited
log-coverage#9.7 · yes · stale · the "packages/ unchanged" census gate no longer holds, since blitz-net's cfg(test) module gained two TcpListener binds (lib.rs:526, :534) that the census pattern matches, though the ten test files still read 0
log-coverage#9.8 · no · escher-driver has no diff and still holds 2 info_span! sites, and lint.rs, mcp_tests.rs and stand_requirement_sweep.rs are not reached
log-coverage#9.9 · yes · add-to · F1 (the e42ee1f25 take reads silent with the same 2 census hits) and F3 (the new url_search_params tests add no env read, print or tracing site) join this census
log-coverage#9.10 · no · the five checks' files are not reached and their census still reads 0, with the new blitz-net listeners outside these files
log-coverage#9.11 · no · select_popup.rs is reached but its row-3 skip removal adds no print, env read or sink, so the census still reads 0, and the other two files are untouched
