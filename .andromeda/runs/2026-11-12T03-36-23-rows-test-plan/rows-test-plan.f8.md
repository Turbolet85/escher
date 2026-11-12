@row_add n=1 row=contract/escher-session-exit-status/0 area=session-lifecycle rec=2,1
boundary: escher-session exit status
item: 0
promise: Accepted: the answer as one uncoloured line of JSON on stdout.
@row_add n=2 row=contract/escher-session-exit-status/1 area=session-lifecycle rec=2,1
boundary: escher-session exit status
item: 1
promise: Refused: the refusal on stdout.
@row_add n=3 row=contract/escher-session-exit-status/2 area=session-lifecycle rec=2,1
boundary: escher-session exit status
item: 2
promise: Usage: one fixed line on stderr with stdout empty, nothing booted and no state created.
@row_add n=4 row=contract/escher-session-exit-status/3 area=session-lifecycle rec=2,1
boundary: escher-session exit status
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr; `RecordDirNotPrivate` and `RecordingFailed` end as session errors.
