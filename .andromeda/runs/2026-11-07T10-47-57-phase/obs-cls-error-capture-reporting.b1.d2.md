@answer n=1 entity=error-capture-reporting#1.1 v=no
why: an absence search over s02 files the chunk does not reach, and the chunk adds no error-reporting client
@answer n=2 entity=error-capture-reporting#1.2 v=no
why: an absence search over the examples slice, and the chunk touches no example app or capture service
@answer n=3 entity=error-capture-reporting#1.3 v=no
why: an absence search over s05 files, and the chunk adds no service or span (scope needs item 8)
@answer n=4 entity=error-capture-reporting#1.4 v=no
why: an absence search for panic hooks over s06 files, and the render sites add no panic path (scope needs item 11)
@answer n=5 entity=error-capture-reporting#1.5 v=no
why: an absence search for span and otel matches over files the chunk does not reach, and the chunk adds no span (scope needs item 8)
@answer n=6 entity=error-capture-reporting#1.6 v=no
why: an absence search for metrics over s11 files, and metric coverage is excluded as not needed with the chunk adding no counter (scope not_needed item 4)
@answer n=7 entity=error-capture-reporting#2.1 v=no
why: blitz-dom's failed-stylesheet capture is a style path the chunk does not change, and blitz-dom error capture is excluded as not needed (scope not_needed item 8)
@answer n=8 entity=error-capture-reporting#2.2 v=no
why: NetHandler::error's default and the harness forwarding are behaviour of components the chunk neither calls nor overrides, and the scope names no resource-error path for its witnesses
@answer n=9 entity=error-capture-reporting#2.3 v=no
why: the blitz-net error arm and its scrubbed log line are a network path the chunk does not touch, and blitz-net error capture is excluded as not needed (scope not_needed item 8)
