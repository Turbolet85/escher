@answer n=1 entity=error-capture-reporting#1.1 v=no
why: a measured absence of an error-reporting client in the s02 files; it states no rule the chunk's code or tests must honour
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: a measured absence of a Sentry capture service in the examples slice; it states no rule over the chunk's units
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: a measured absence of error-reporting services in the s05 files; it states no rule over the chunk's units
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: a measured absence of a reporting service or panic hook in the listed s06 files; the chunk adds no hook and states no rule over one
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: a measured absence of span macros and reporters in 16 listed files; it states no rule the chunk must honour
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: a measured absence of metric and reporter macros in the s11 files; the chunk adds no timing or counter and the entity states no rule
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom's failed-stylesheet handling in net.rs; the chunk does not touch resource loading and the scope names no such behaviour
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: blitz-traits' default NetHandler error drop and the test harness forward; a component the chunk only uses, and the scope names no error forwarding
@answer n=9 entity=error-capture-reporting#2.3 v=yes
why: the scope's sink scrub rule (an engine target prints only the safe fields) applied to blitz-net's Resource load failed line, which the host_log and mcp_log trace assertions read through the sink
