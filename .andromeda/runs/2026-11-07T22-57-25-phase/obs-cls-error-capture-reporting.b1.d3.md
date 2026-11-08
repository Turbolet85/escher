@answer n=1 entity=error-capture-reporting#1.1 v=no
why: a past grep for an error-reporting client over the s02 files, not re-run by the chunk, which adds no such client
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: a past grep for sentry over the examples slice, not re-run by the chunk, which touches no such file
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: a past grep for sentry or opentelemetry over the s05 files, not re-run by the chunk
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: a past grep for a panic hook over the s06 files, and escher-telemetry's panic hook is not changed by the chunk
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: a past grep for span! over 16 listed files, not re-run by the chunk, which adds no span beyond the existing command and tool_call spans
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: a past grep for metrics over the s11 files, and the chunk adds no timing or counter
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom's stylesheet error capture is a component the chunk only uses, the scope names no stylesheet or critical-resource behaviour, and the graph reads no network
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: NetHandler's default error drop and CountedHandler's forwarding are used components the scope does not name, and the chunk adds no NetHandler
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: blitz-net's error arm and the engine-target Resource load failed line are not reached, since the chunk reads no network and its new spans are escher_ targets with no field named url or error
