@fit n=1 row=decision/headless-stand-no-sink sha=f63413307554 v=fits type=2
@fit n=2 row=decision/checks-silent sha=fafeb052a19b v=fits type=2
@fit n=3 row=decision/answers-echo-nothing-typed sha=8ba750699a8c v=fits type=2
@fit n=4 row=decision/driver-library-installs-no-subscriber sha=cabe54606e45 v=fits type=1
@fit n=5 row=decision/driver-emits-no-event sha=8b56fba23dcb v=fits type=1
@fit n=6 row=decision/capture-framework-targets-warn-up sha=a5df3f46c796 v=fits type=1
@fit n=7 row=contract/telemetry-scrub-set/error sha=18d52ab10b68 v=fits type=3
@fit n=8 row=contract/telemetry-scrub-set/url sha=bf7bd6bbe7a8 v=fits type=3
@fit n=9 row=decision/escher-target-content-named-set sha=92747bb4d9ef v=fits type=1
@fit n=10 row=decision/scrub-by-field-name sha=b655275b0982 v=fits type=1
@fit n=11 row=decision/engine-sites-fixed-messages sha=98f045cec6d2 v=fits type=1
@fit n=12 row=decision/capture-chaining-panic-hook sha=247f89c195a5 v=not_master type=4
why: How the capture's panic hook is built, with no reason stated in its basis, is a mechanism the code holds, and its printing nothing already stands in the driver library's no-subscriber rule.
