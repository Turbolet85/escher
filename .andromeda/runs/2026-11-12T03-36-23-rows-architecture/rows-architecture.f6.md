@row_add n=1 row=contract/escher-session-exit-codes/1 area=standard-contracts rec=1,2
boundary: escher-session exit codes
item: 1
promise: The command was refused; one line of JSON answers it on stdout.
@row_add n=2 row=contract/escher-session-exit-codes/2 area=standard-contracts rec=1,2
boundary: escher-session exit codes
item: 2
promise: Usage error: a fixed line on stderr, stdout empty, nothing booted and no state created.
@row_add n=3 row=contract/escher-session-exit-codes/3 area=standard-contracts rec=1,2
boundary: escher-session exit codes
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr.
@row_add n=4 row=decision/one-ci-leg-runner area=standard-contracts rec=3,4,5
name: One CI leg runner
rule: Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs.
@row_add n=5 row=absence/served-network-port area=occupied-resources rec=6,7
capability: A served network port: no TCP or UDP port any binary serves
@row_add n=6 row=name/crate/escher-telemetry area=occupied-resources rec=8,9
class: crate
name: escher-telemetry
@row_add n=7 row=name/crate/escher-driver area=occupied-resources rec=8,9
class: crate
name: escher-driver
@row_add n=8 row=name/binary/blitz area=occupied-resources rec=8,9
class: binary
name: blitz
@row_add n=9 row=name/binary/seven-guis-native area=occupied-resources rec=8,9
class: binary
name: seven_guis_native
@row_add n=10 row=name/binary/escher-session area=occupied-resources rec=8,9
class: binary
name: escher-session
