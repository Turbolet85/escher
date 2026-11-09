@answer n=1 entity=error-capture-reporting#1.1 v=no
why: section 7 stays untouched and the session adds no error-reporting client, dependency or print, so the absence holds
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: section 7 stays untouched and the session adds no error-capture service or dependency
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: section 7 stays untouched and the session adds no error-reporting service, span or dependency
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: section 7 stays untouched, the panic hook is unchanged and the session adds no error-reporting service
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: section 7 stays untouched and the session adds no span, metric or dependency, so the search still finds no match
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: section 7 stays untouched and the session adds no metric, counter or dependency
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom is an untouched engine crate and the session's reached files do not include packages/blitz-dom/src/net.rs
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: blitz-traits and blitz-test-harness are untouched engine crates and the session's reached files do not include them
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: blitz-net and escher-telemetry's scrub set are untouched, so the fetch error path and its redaction stand
