@answer n=1 entity=error-capture-reporting#1.1 v=no
why: an absence of a Sentry, analytics or gtag client in the s02 files, which the chunk neither adds nor reads
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: an absence of an error-capture service in the examples slice, which the probe does not add
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: an absence of an error-reporting service in the s05 files, which the chunk does not touch
@answer n=4 entity=error-capture-reporting#1.4 v=yes
why: the absence of any panic hook in the listed s06 files bears on the MountedData panic reading, which must state escher's chaining hook at escher_telemetry::panic, and the probe must add no hook
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: an absence of an error service or span macro in the 16 listed files, and the chunk adds no span or service
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: an absence of metric and error-service calls in the s11 files, and the chunk adds no timing or counter
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: the blitz-dom stylesheet failure path is not one of the four links and the scope does not name it
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: the NetHandler default and CountedHandler forwarding are not named by the scope, and the probe calls no handler error path
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: the blitz-net fetch error path and the sink's scrub of url and error are not reached, since the probe reads no network and logs nothing
