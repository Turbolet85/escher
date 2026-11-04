@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the probe is new Rust code, so it must compile under edition 2024 and rust-version 1.91.0
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the probe reads dioxus-native-dom and dioxus-native at their workspace pins, which the chunk must not move
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: Stylo styling is not read by any of the four links or the stand's checks
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: the stand builds its Dioxus tree in memory and the probe parses no HTML or XML
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: no link reads geometry (not_needed item 14), so taffy's layout engine does not reach the probe's reading
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: the entity states the text and font stack, not a rule the probe acts on; the one-font pin is a stand fixture
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: painting and renderer backends are not_needed (item 5), and no verdict reads a paint
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: geometry, colour, GPU and SVG types belong to painting, which no link reads
@answer n=9 entity=stack-and-technologies#1.9 v=yes
why: the probe pumps input through the Harness, and a module glob-importing dioxus::prelude::* must name ::keyboard_types, which two versions in the graph make ambiguous
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: no link reads the accesskit tree, and the platform adapter is not_needed (item 12)
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: the stand boots offline with no network, so blitz-net and its clients are not reached
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: Boa page scripts are not_needed (item 2)
@answer n=13 entity=stack-and-technologies#1.13 v=yes
why: the Dioxus 0.7.3 crates are what the probe reads through the bridge, and the chunk must keep them pinned
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: the probe takes no shot, adds no dependency and writes no JSON, so the image and serde pins are not its rule
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: the probe runs natively on the headless stand, and the Trunk browser-target build is not reached
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: style threading is not_needed (item 11), and the parallelism and test-that pins are not a rule the probe's code acts on
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: browser history persistence is not_needed (item 4)
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: browser app extras are not_needed (item 4)
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the rdme markdown app is not_needed (item 4)
@answer n=20 entity=stack-and-technologies#1.20 v=yes
why: the probe is an integration test in blitz-tests, which takes the headless stand and escher-driver as dev-dependencies
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the Nix flake and dev-host toolchain are environment, and the chunk adds no toolchain pin
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the test leg and the per-crate coverage floor that item 18 names are the CI gates any probe test must pass
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: item 18 names the fmt, clippy, rustdoc, coverage-floor and stand-repeat gates the probe's tests must pass
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the cognitive-complexity reading runs on the dev host alone, under no CI leg, and is not one of the gates item 18 names
