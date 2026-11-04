@answer n=1 entity=e2e-test-strategy#1.1 v=no
why: the session adds no escher-session, driver or seven_guis test, and the Cargo.lock bump (pulldown-cmark, reached only through rdme) does not reach the binary's measured behaviour
@answer n=2 entity=e2e-test-strategy#1.2 v=no
why: the select-list hosted shot is a cli_commands test the session does not touch
@answer n=3 entity=e2e-test-strategy#1.3 v=no
why: the MCP role and its stand_mcp_tools and mcp_log tests are unchanged, the session touching only blitz-tests' dioxus_runtime_probe target
@answer n=4 entity=e2e-test-strategy#2.1 v=yes mode=add-to
why: the session re-measures the MountedData hazard at HEAD and pins it, so F2 (the two should_panic pins, their hazard_ready precondition beside the synchronous mounted_data_asked_at_once test) joins this record
@answer n=5 entity=e2e-test-strategy#3.1 v=no
why: the select list's headless paint path is untouched by the session, which changes no driver, harness or select code
@answer n=6 entity=e2e-test-strategy#4.1 v=no
why: the session adds no assertion to the example fixtures searched by slice s02, its new target sitting under tests/blitz-tests
@answer n=7 entity=e2e-test-strategy#4.2 v=no
why: the search covers the apps' tests, and the session's new target lives under tests/blitz-tests outside that slice, so the claim holds
