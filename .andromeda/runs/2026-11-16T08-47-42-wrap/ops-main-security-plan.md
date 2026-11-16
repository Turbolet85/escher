@row_set n=1 row=decision/driver-installs-no-subscriber cell=except seen=1 detector=none
value: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
basis: fct-xxn4lhqe (Symbols / APIs: settled_step runs inside one escher_telemetry::capture)
because: An acting step now sets escher-telemetry's scoped capture as its default, so the no-subscriber rule must name that exception.
@row_set n=2 row=decision/driver-no-subscriber-no-env cell=except seen=1 detector=none
value: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
basis: fct-xxn4lhqe (Symbols / APIs: settled_step runs inside one escher_telemetry::capture)
because: An acting step now sets escher-telemetry's scoped capture as its default, so the no-subscriber rule must name that exception.
