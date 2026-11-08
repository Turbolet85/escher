@answer n=1 entity=error-capture-reporting#1.1 v=no
why: an analytics or gtag client absent from the s02 files; the graph read adds no client and the scope names none
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: a Sentry client absent from the examples slice; the chunk adds no error capture service and the scope names none
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: Sentry and OpenTelemetry absent from the s05 files; the chunk adds no exporter and the scope names none
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: a panic hook absent from the s06 files; the hook is escher-telemetry's, which the chunk does not change (task not_needed 11), and the new code adds no set_hook or catch_unwind
@answer n=5 entity=error-capture-reporting#1.5 v=no standing=S2
why: the convention that a new module adds no span, which the new graph module and its signal registry must hold
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: metric and counter absence over the s11 files; the chunk adds no timing or counter (task not_needed 4)
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom's stylesheet error capture; the graph reads no network and the scope names no resource-failure path (task not_needed 7)
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: NetHandler's default error drop and the harness CountedHandler forwarding; the chunk implements no NetHandler and the scope names no network error path
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: blitz-net's error arm and the sink's url and error scrub on Resource load failed; the task names blitz-net fetch errors and that line as not needed (not_needed 7), and escher-telemetry is unchanged (not_needed 11)
