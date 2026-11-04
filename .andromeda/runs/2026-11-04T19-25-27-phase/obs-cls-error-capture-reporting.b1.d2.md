@answer n=1 entity=error-capture-reporting#1.1 v=no
why: an analytics or gtag absence over the s02 files; the chunk re-reads or re-measures none of them and adds no reporting client
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: a Sentry absence over the examples slice; the probe adds no reporter, and the examples-code rules that bind it are not this record
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: a Sentry and opentelemetry absence over the s05 files, which the chunk does not reach
@answer n=4 entity=error-capture-reporting#1.4 v=yes
why: the MountedData panic reading (task item 7) turns on the chaining panic hook, and this record places no panic::set_hook in the listed s06 files, so the probe must add no hook and the reading must reconcile with that absence
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: a span! and opentelemetry absence over 16 listed files; the chunk adds no span, and the absence re-measures no span count
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: a metric, counter and histogram absence over the s11 files; metric coverage is not needed for this chunk
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom's failed-stylesheet capture is a behaviour of a component the chunk only uses, and the scope names no stylesheet path or call that carries it
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: the NetHandler::error default and CountedHandler forwarding are a net-handler behaviour the probe neither reads nor must act on; the scope names poll(None) but not the error path
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: the blitz-net error arm and the escher-telemetry scrub set describe a resource-load log line the headless probe never emits (no sink installed), and the probe reads no error text
