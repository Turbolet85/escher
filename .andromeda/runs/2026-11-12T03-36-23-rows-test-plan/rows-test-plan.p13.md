@row_add n=1 row=contract/escher-session-command-line/command-form area=session-lifecycle rec=1
boundary: escher-session command line
item: command form
promise: `escher-session <verb> … --session <dir>` is the whole command line, fourteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
@row_add n=2 row=contract/escher-session-command-line/snapshot area=session-lifecycle rec=1
boundary: escher-session command line
item: snapshot
promise: `--part` is a whole number 1 to 1000; a paged answer is one line `{"text":…,"part":n,"parts":m}`, status `0`; a whole screen still answers `{"text":…}` alone.
@row_add n=3 row=contract/escher-session-command-line/screenshot area=session-lifecycle rec=1
boundary: escher-session command line
item: screenshot
promise: `--id` is optional; the accepted answer is one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`.
@row_add n=4 row=contract/escher-session-command-line/lint area=session-lifecycle rec=1
boundary: escher-session command line
item: lint
promise: `lint` takes no argument; its accepted answer is one line `{"findings":[…]}`, status `0`.
@row_add n=5 row=contract/escher-session-command-line/graph area=session-lifecycle rec=1
boundary: escher-session command line
item: graph
promise: `graph` takes no argument (an argument is refused `malformed`); its accepted answer is one line `{"nodes":[…],"states":[…]}`, status `0`.
@row_add n=6 row=contract/escher-session-command-line/start area=session-lifecycle rec=1
boundary: escher-session command line
item: start
promise: `start <task> [--record <dir>]`, the task one of `counter` · `flight-booker` · `timer` · `crud`; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
@row_add n=7 row=contract/escher-session-command-line/serve area=session-lifecycle rec=1
boundary: escher-session command line
item: serve
promise: The host role is `serve <task> --session <dir> [--record <dir>]`; it ends with nothing written and status `0` after a stop or an expiry, and `3` on a session error.
@row_add n=8 row=contract/escher-session-command-line/mcp area=session-lifecycle rec=1
boundary: escher-session command line
item: mcp
promise: The `mcp` role is `mcp` alone or `mcp --record <dir>` (anything else is usage): an MCP server on the binary's standard streams holding its session in process, ending with nothing on stdout, `0` on `Ok`, `3` with the error's fixed message on stderr on `Err`.
@row_add n=9 row=contract/escher-session-command-line/mcp-record-check area=session-lifecycle rec=1
boundary: escher-session command line
item: mcp --record
promise: `mcp --record` checks the recording directory before it serves; a refusal is status `3`, its fixed message on stderr and stdout empty.
@row_add n=10 row=contract/escher-session-command-line/record-flag area=session-lifecycle rec=1
boundary: escher-session command line
item: --record
promise: `--record <dir>` is taken at most once, on `start`, the host role and the `mcp` role; anywhere else, twice, or with no value it is usage, status `2`.
@row_add n=11 row=contract/escher-session-command-line/help area=session-lifecycle rec=1
boundary: escher-session command line
item: help
promise: `help` takes no argument and at most one `--session <dir>`, not read, and answers the self-description line on stdout, status `0`, reaching no session; an argument is refused `malformed`, status `1`; `--session` with no directory or given twice is usage, status `2`.
@row_add n=12 row=contract/escher-session-command-line/coverage area=session-lifecycle rec=1
boundary: escher-session command line
item: coverage
promise: The coverage role is `coverage <app> <check>...`, no `--session` or `--record` among its paths; it reads each named kept check bounded and answers the coverage line on stdout, status `0`; any other shape is usage, `2`; a failing boot is its session error, `3`.
@row_add n=13 row=contract/escher-session-command-line/coverage-refusal area=session-lifecycle rec=1
boundary: escher-session command line
item: coverage refusal
promise: A check file the coverage role cannot read, one past the bound, one that is no check or a check of another app is refused `malformed`, status `1`, holding nothing of the path or the file.
@row_add n=14 row=contract/escher-session-command-line/two-argument-form area=session-lifecycle rec=1
boundary: escher-session command line
item: two-argument form
promise: The two-argument form `<task> <state-dir>` reads as an unknown verb, refused, status `1`.
@row_add n=15 row=contract/escher-session-command-line/usage-line area=session-lifecycle rec=1
boundary: escher-session command line
item: usage line
promise: The usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, listing `graph` after `lint` and `help` after `stop`.
@row_add n=16 row=contract/escher-session-command-line/no-echo area=session-lifecycle rec=1
boundary: escher-session command line
item: caller input
promise: No ending writes back anything the caller typed.
@row_add n=17 row=contract/escher-session-exit-status/0 area=session-lifecycle rec=1
boundary: escher-session exit status
item: 0
promise: Accepted: the answer as one uncoloured line of JSON on stdout.
@row_add n=18 row=contract/escher-session-exit-status/1 area=session-lifecycle rec=1
boundary: escher-session exit status
item: 1
promise: Refused: the refusal on stdout.
@row_add n=19 row=contract/escher-session-exit-status/2 area=session-lifecycle rec=1
boundary: escher-session exit status
item: 2
promise: Usage: one fixed line on stderr with stdout empty, nothing booted and no state created.
@row_add n=20 row=contract/escher-session-exit-status/3 area=session-lifecycle rec=1
boundary: escher-session exit status
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr; `RecordDirNotPrivate` and `RecordingFailed` end as session errors.
@row_add n=21 row=contract/escher-session-error-object/fields area=session-lifecycle rec=1
boundary: escher-session error object
item: fields
promise: A session error object carries `kind`, `message` and `remedy`.
@row_add n=22 row=contract/session-host-stderr/service-name area=session-lifecycle rec=1
boundary: session host stderr lines
item: service.name
promise: The host's stderr lines carry `service.name=seven_guis`.
@row_add n=23 row=decision/session-checks-own-their-host area=session-lifecycle rec=2
name: Session checks own their host
rule: A check that needs a session process starts and stops its own host, bounds every wait, holds the host behind a guard that kills and reaps it, and sends the child's streams to null unless it reads them.
@row_add n=24 row=decision/socket-session-checks-unix-gated area=session-lifecycle rec=2
name: Socket session checks unix-gated
rule: A check holding a session over the socket is `cfg(unix)`-gated; a check driving the `mcp` stdio child bounds every read, kills and reaps the child in its guard's drop and is not unix-gated, so the windows job runs it.
except: The `mcp_server` command-line comparison is unix-gated.
@row_add n=25 row=decision/recording-checks-unix-gated area=session-lifecycle rec=2
name: Recording checks unix-gated
rule: A check that records is `cfg(unix)`-gated item by item, in-process and `mcp` stdio ones included, beside an off-unix check that a recording is refused `Unsupported`, which the windows job alone runs.
@row_add n=26 row=decision/log-silence-read-on-a-sink-host area=session-lifecycle rec=2
name: Log silence read on a host with a sink
rule: A zero read from a host that installs no log sink says nothing about a host that does.
@row_add n=27 row=decision/no-screenshot-in-cross-os-checks area=session-lifecycle rec=3
name: No screenshot step in cross-OS byte-compared checks
rule: A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS.
why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
guard: stand_session_recording
@row_add n=28 row=decision/two-clicks-without-advance-double-click area=session-lifecycle rec=3
name: Two clicks with no advance are a double click
rule: Under the virtual clock, two clicks with no `advance` between them are a double click.
why: Ratified.
guard: stand_virtual_clock
@row_none n=1 rec=4 type=S
