@task_prose part=chunk
Gives a run of the driver (a held Session and the test harness it holds) one seed, and makes every source of randomness the run's app and page can read draw from it.
The app-facing source: a Dioxus app hosted by the driver reads randomness through one engine-provided source (a root context or hook in dioxus-native-dom, set through HarnessOptions) rather than from the host.
The page-facing source: a page script's Math.random in a seeded run (blitz-vibey-script's bootstrap scripts) draws from the run's seed.
The seed enters through Session::start or a builder beside with_time, and through the verb table if start takes it; the kept check (recording, replay) records it if it is recorded; help's self-description and the capability statement with its pin are updated.
Proof: an integration check, both layout modes, that two runs of one seed read the same snapshot text and diffs at every step of a scripted sequence on an app that reads randomness, and two runs of different seeds read a difference.
@task_file n=1 path=docs/capabilities.md mark=edits
what: the capability statement the driver describes itself by, updated by hand, and its pin — edits: the seeded-randomness capability entry and the pin
@task_item n=1 part=needs
The driver session start: a seed given to a run through Session::start or a builder beside with_time, held by the session and passed to the harness it holds.
@task_item n=2 part=needs
If start takes a seed, the verb table gains a new argument kind on the ratified driver carriers — the command line, the escher-session host wire, the MCP role and the kept check's arguments — a closed-value field inside a ratified carrier, its widening status a fork left to P4.
@task_item n=3 part=needs
The kept check file and the recording file: if the seed is recorded, it is held among the check's arguments and replay reads it back so a kept check replays the run it recorded; recording bounds, the private recording directory and the rule that committed checks hold no private data still bind what is written.
@task_item n=4 part=needs
escher-session commands' help and the tool_call help: the self-description names the seed argument; the help answer and the capability statement are updated by hand.
@task_item n=5 part=needs
A malformed or out-of-range seed argument is refused through the existing closed refusal causes (malformed) and the existing error object, exit codes and tool_call ending — no new cause, code or ending.
@task_item n=6 part=needs
The app-facing randomness source: dioxus-native-dom offers a hosted Dioxus app one engine-provided source (a root context or hook) seeded from HarnessOptions in the test harness.
@task_item n=7 part=needs
The page-facing randomness source: blitz-vibey-script replaces a page script's Math.random in its bootstrap scripts with a draw from the run's seed in a seeded run.
@task_item n=8 part=needs
The seed's value stays out of every log line, span field and event: the driver's command and tool_call spans keep their closed fields and carry no call content, answers echo nothing the caller typed, and the sink's scrub and content rules apply unless a ratified decision says otherwise.
@task_item n=9 part=needs
No new listener, port, socket or environment read carries a seed; the driver library still emits no event and installs no subscriber.
@task_item n=10 part=needs
The read state the proof compares: the snapshot text and the snapshot diff, read at every step of the scripted sequence, in both layout modes.
@task_item n=11 part=needs
The stand or fixture surface for the proof: an app on the seven_guis stand or a fixture that reads randomness; the headless stand still runs with no sink, and replay children stay uninstrumented.
@task_item n=12 part=needs
The windowed shell (blitz-shell) and the browser app are not seeded runs: the native main's sink installation and the windowed binary's paths stay as they are.
@task_item n=13 part=needs
The new integration check proves non-vacuity: two runs of different seeds must read a difference, its failure naming a kind and a count, not the state read.
@task_standing n=1
convention: each new check is silent and shares its helpers through the shared test modules; it prints nothing it reads
home: 3-observability-harness-contract
@task_standing n=2
convention: each new check that asserts an empty stderr unsets RUST_LOG for the child it runs
home: 3-observability-harness-contract
@task_standing n=3
convention: each new kept check or fixture committed to the tree holds no secrets and no private data
home: 9-ci-integration · ci-integration
@task_standing n=4
convention: no answer-bearing type is logged through Debug by any new code or test
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Metric coverage: metric emission, metrics exporters and WPT score publishing — the chunk adds no counter, gauge, histogram or score.
@task_item n=2 part=not_needed
Error capture and reporting: the window error event, window.onerror, script error source labels, the WPT report and the panic hook — the chunk changes no error path of a page script or a binary.
@task_item n=3 part=not_needed
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs, transcripts and cleanup — the chunk runs no agent and adds no verdict field.
@task_item n=4 part=not_needed
Form controls and their content rules: select and optgroup labels, chosen sets, multiple select form entries, open list shots and page change events — the chunk adds no form control behaviour.
@task_item n=5 part=not_needed
Stylesheet loading and file: loads from documents: the failed stylesheet pending set and the rule on file: loads from non-file: documents — the chunk loads no resource.
@task_item n=6 part=not_needed
The coverage role's coverage line and the graph and actions answers' shape — the chunk changes neither answer.
@task_item n=7 part=not_needed
The sink's resource keys (service.name, service.version) and the startup telemetry-installed event — the chunk changes no sink configuration.
