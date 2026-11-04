@answer n=1 entity=inherited-defaults#1.1 v=no
why: the chunk adds no workspace crate or package (task item 16) and moves no publish flag or version; its code sits in crates already set `publish = false`
@answer n=2 entity=inherited-defaults#2.1 v=yes
why: task item 18 names the CI gates probe test and example code must pass (test leg, stand-repeat zero-failure budget, clippy, fmt, rustdoc, per-crate coverage floors), which this entity states
@answer n=3 entity=inherited-defaults#3.1 v=yes
why: task item 17 names the `accessibility` Cargo feature that gates the dioxus-native-dom snapshot modules a probe check reads, so the feature gating binds
