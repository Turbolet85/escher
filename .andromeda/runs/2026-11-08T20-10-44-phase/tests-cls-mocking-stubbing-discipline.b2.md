@answer n=1 entity=mocking-stubbing-discipline#1.26 v=no
why: the rule covers the WPT diff script's GitHub API dry-run, which the chunk neither reaches nor edits
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: the rule covers blitz-dom hover_state_tests font-metric skips, outside the reached dioxus-native-dom files and not a test the chunk adds
@answer n=3 entity=mocking-stubbing-discipline#3.1 v=no
why: the absence covers mocking libraries in CI and build configuration, which the chunk leaves alone since it adds no dependency or workflow
@answer n=4 entity=mocking-stubbing-discipline#3.2 v=no
why: the rule covers mocking libraries in the apps, which the chunk does not touch (apps/browser is not needed)
@answer n=5 entity=mocking-stubbing-discipline#3.3 v=yes
why: the chunk's unit tests over the bridge dispatch record, the signal registry and the graph reader, with the cfg(test) shim dispatch_to_vdom, add code in the reached dioxus-native-dom files, so they must add no mock, stub or fake in the searched slice
@answer n=6 entity=mocking-stubbing-discipline#3.4 v=no
why: slice s06 is not named by the scope or a reached file, and the chunk's recorded checks fall under the recorded-check convention rather than this search
@answer n=7 entity=mocking-stubbing-discipline#3.5 v=no
why: slice s07 is not named by the scope or a reached file
@answer n=8 entity=mocking-stubbing-discipline#3.6 v=no
why: slice s08 has no match and is not named by the scope or a reached file
@answer n=9 entity=mocking-stubbing-discipline#3.7 v=no
why: slice s09 is not named by the scope or a reached file
@answer n=10 entity=mocking-stubbing-discipline#3.8 v=no
why: the search over slice s11 covers the network stand-ins HeldNet, LateNet and LoadNet, which the chunk neither adds nor changes (not needed item 11)
