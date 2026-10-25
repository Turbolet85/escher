# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · yes · the stderr-only sink limit reads against this record of no error-reporting client in the 21 s02 files
error-capture-reporting#1.2 · yes · the two sink-installing binaries' stderr-only output reads against no Sentry client in the examples slice
error-capture-reporting#1.3 · yes · the stderr-only destination and the NOT YET MEASURED trace-context limit read against no Sentry or OpenTelemetry in the 15 s05 files
error-capture-reporting#1.4 · yes · the chained std panic hook limit (raw panic message on stderr) reads against this panic-hook record in the 17 listed s06 files
error-capture-reporting#1.5 · yes · the NOT YET MEASURED trace-context limit and the driver spans it states read against no OpenTelemetry or span! in the 16 listed files
error-capture-reporting#1.6 · yes · the stderr-only destination reads against no error-reporting service in the 21 s11 files; its metric terms are §5 and not claimed
