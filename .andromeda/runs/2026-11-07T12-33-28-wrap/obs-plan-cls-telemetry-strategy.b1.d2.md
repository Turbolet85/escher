@answer n=1 entity=telemetry-strategy#1.1 v=no
why: the session adds no tracing site or feature wiring, and the crates this entity cites (blitz-dom, escher-telemetry, escher-driver) are not among the reached files.
@answer n=2 entity=telemetry-strategy#2.1 v=no
why: no manifest moved and blitz-dom is not a reached file, while the replay re-fires the same dioxus-native-dom trace! line set, so the blitz_dom event reading holds.
@answer n=3 entity=telemetry-strategy#3.1 v=no
why: the searched files are workflows, manifests and scripts, none reached, and no dependency or manifest moved.
@answer n=4 entity=telemetry-strategy#3.2 v=no
why: the examples slice holds no reached file, and the new buffer module carries no tracing, log or metrics token.
@answer n=5 entity=telemetry-strategy#3.3 v=no
why: the reached probe file is in this 61-file slice, but its renamed pins keep the silence census at 0 and the file count is unchanged.
@answer n=6 entity=telemetry-strategy#3.4 v=no
why: the wpt runner's 12 listed files are not reached by the session.
