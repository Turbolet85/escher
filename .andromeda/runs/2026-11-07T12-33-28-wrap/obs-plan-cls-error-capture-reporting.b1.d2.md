@answer n=1 entity=error-capture-reporting#1.1 v=no
why: the change adds no error-reporting client or sentry, analytics or gtag token to the files it reaches, so the absence still holds on them
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: the reached dioxus-native-dom files bring no sentry or error-capture service, so the examples-slice absence still holds
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: the change brings no sentry or opentelemetry service into the reached files, so the s05 absence still holds
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: the change adds no panic hook or catch_unwind and the panic-token silence gates read 0, so the absence still holds
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: the change adds no span!, otel or opentelemetry token to the reached files, so the no-match still stands
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: the change adds no metric, counter!, histogram or exporter call and metric coverage is not reached, so the s11 absence still holds
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: the change never reaches blitz-dom's net.rs, so its stylesheet error capture is unmoved
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: the change never reaches blitz-traits or blitz-test-harness, so the default NetHandler::error and the CountedHandler forward are unmoved
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: the change never reaches blitz-net or escher-telemetry's scrub set, so the error arm and the redaction stand
