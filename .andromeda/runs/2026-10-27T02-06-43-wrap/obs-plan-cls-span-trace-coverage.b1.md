# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · no examples file is among the reached files (the reached code is escher-driver and blitz-tests only)
span-trace-coverage#1.2 · no · no apps file is among the reached files
span-trace-coverage#1.3 · no · the reached code files add no span or instrument site (no span added, lint.rs and mcp_tests.rs read census 0) and no file is added to the 15 s05 files
span-trace-coverage#1.4 · no · the reached code files add no span or instrument site (no span added, lint.rs and mcp_tests.rs read census 0) and no file is added to the 17 listed s06 files
span-trace-coverage#1.5 · no · no layout file is among the reached files, and nothing under packages/blitz-dom is edited
span-trace-coverage#1.6 · no · the reached code files add no span or #[instrument] site (no span added, lint.rs and mcp_tests.rs read census 0, the new helpers open none) and no file is added to the 16 listed files
span-trace-coverage#1.7 · no · the reached code files add no span or instrument site (no span added, lint.rs and mcp_tests.rs read census 0) and no file is added to the 32 listed s09 files
span-trace-coverage#1.8 · no · no blitz-vibey-script or blitz file is among the reached files
span-trace-coverage#1.9 · no · the reached code files add no span or instrument site (no span added, lint.rs and mcp_tests.rs read census 0) and no file is added to the 21 listed s11 files
span-trace-coverage#1.10 · no · stand_requirement_sweep.rs still installs no subscriber, reads no env var and prints nothing with its new helpers, lint.rs and mcp_tests.rs read census 0, and no file is added to the 61 slice files
span-trace-coverage#1.11 · no · no WPT runner file is among the reached files, and the runner is untouched
span-trace-coverage#2.1 · no · lint.rs stays silent after the split (census 0, its new readings and placement helpers open none), so Session::lint still opens none of its own, and the command span, its eight fields and the 2 info_span! sites are unchanged
span-trace-coverage#2.2 · no · neither host_log.rs nor the served host is among the reached files, and the part read still runs through Session::run with its command span unchanged
span-trace-coverage#3.1 · no · the tool_call span in mcp.rs is unchanged (mcp.rs is not reached), and the reached mcp_tests.rs holds no span or tracing site (census 0), so the two fields and the twelve-verb table still hold
span-trace-coverage#3.2 · no · scripts/cold-agent.sh is not among the reached files, and tool_call keeps its two fields, tool and ending, so the verdict's span counts read the same
