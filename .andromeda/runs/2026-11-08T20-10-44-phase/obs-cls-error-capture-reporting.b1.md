@answer n=1 entity=error-capture-reporting#1.1 v=no
why: observed absence of an error-reporting client in the s02 files; the chunk adds no error capture or exporter (not_needed: error capture)
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: observed absence of Sentry in the examples slice; the chunk adds no error capture service (not_needed: error capture)
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: observed absence of Sentry and OpenTelemetry in the s05 files; the chunk adds no exporter (not_needed: OTel SDK init)
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: observed absence of an error service or panic hook in the s06 files; the chunk adds or changes no panic hook (not_needed: panic hooks)
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: observed absence of an error reporting service in the 16 listed files; the chunk's new spans are bound by the span needs, not by this error-reporting absence
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: observed absence of metric or error exporters in the s11 files; the chunk adds no counter or timing export (not_needed: metric coverage)
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom's failed-stylesheet answer in net.rs is an engine path the chunk neither edits nor reads, and error capture is not_needed
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: blitz-traits' default NetHandler::error and CountedHandler forwarding are not named by the scope, and the chunk relies on no error-forwarding call or value
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: the blitz-net fetch error arm and the escher-telemetry scrub of url and error describe the existing Resource load failed path, which the chunk does not reach since the stand's headless boot installs no sink (needs 11)
