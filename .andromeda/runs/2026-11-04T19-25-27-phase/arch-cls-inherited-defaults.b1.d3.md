@answer n=1 entity=inherited-defaults#1.1 v=no
why: the probe adds no workspace crate or manifest, and the test crate it lands in (tests/blitz-tests) stays publish = false, so the publishability and versioning rule rules over nothing the chunk changes
@answer n=2 entity=inherited-defaults#2.1 v=yes
why: task item 18 names the gates a probe test or example must pass (the test leg, stand-repeat's zero-failure budget, clippy, fmt, rustdoc and per-crate coverage floors), which this entity states
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: the probe reads the dioxus-native-dom snapshot read-only behind the accessibility feature (task item 17), so its call must sit behind the per-call-site Cargo feature gate this entity states
