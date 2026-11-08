@answer n=1 entity=error-capture-reporting#1.1 v=no
why: the session adds no error-reporting client, so the sentry, analytics and gtag absence still holds on the changed files
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: the session adds no Sentry reference to the examples slice, so the absence still holds over the changed seven_guis files
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: the session adds no sentry or opentelemetry service, so the absence still holds on the changed files
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: the session adds no panic hook or catch_unwind, and the task states nothing in the session reaches error capture
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: the session adds no new span site and no error-reporting service, the two info_span sites staying unchanged
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: the session adds no metric, counter or timing, as the task states outright
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: no blitz-dom source moved, so the stylesheet error capture in blitz-dom net.rs still holds unchanged
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: blitz-traits and blitz-test-harness are not changed by the session, so the default no-op NetHandler error method still stands
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: blitz-net and the escher-telemetry scrub set are untouched by the session, so the failed-fetch capture and redaction still stand
