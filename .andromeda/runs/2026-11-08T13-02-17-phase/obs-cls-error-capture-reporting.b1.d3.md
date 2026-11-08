@answer n=1 entity=error-capture-reporting#1.1 v=no
why: an absence baseline for an error-reporting client over s02 slice files; the chunk adds no such client and nothing it builds needs to honour the search
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: an absence baseline for Sentry over the examples slice; the chunk adds no error-capture service there or in escher-driver
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: an absence baseline for Sentry or opentelemetry over s05 files; the chunk adds no reporting service
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: an absence baseline for a reporting service or panic hook over s06 files; the chunk installs no panic hook and adds no catch_unwind
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: an absence of span! and otel over 16 listed files; escher-driver already holds its two info_span! sites so it is not among them, and the chunk adds no span
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: an absence of metrics and reporting services over s11 files; the chunk adds no metric or counter
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom's typed stylesheet error answer is the engine's resource-load error capture, listed not needed; the read's minimal fixtures load no failing stylesheet
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: NetHandler's default error drop and CountedHandler's forwarding are the engine's resource-load error capture, listed not needed; the scope names no handler and the fixtures fetch nothing that fails
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: the blitz-net fetch error arm and its Resource load failed line are the engine's resource-load logging, listed not needed; the chunk's span fields are closed verb and cause words the scrub never touches
