@answer n=1 entity=established-decisions#25.2 v=no
why: the buffered-render limit concerns a task's focus call inside a render, which the graph read between driver calls does not make
@answer n=2 entity=established-decisions#25.3 v=no
why: the task-spawn poll order concerns tasks polled inside a render, and the graph read is a driver call outside any render that spawns no task
@answer n=3 entity=established-decisions#26.1 v=yes
why: link 1 reuses this public vdom walk that names each node's owning component, the graph's own owner rule
@answer n=4 entity=established-decisions#26.2 v=yes
why: links 3 and 4 read through GlobalSignal subscribers and the registry wrapper the chunk adds for hook-local state the stand moves
@answer n=5 entity=established-decisions#27.1 v=no
why: browser app history, about pages and examples are outside the chunk, which touches only dioxus-native-dom, escher-driver and seven_guis
@answer n=6 entity=established-decisions#28.1 v=no
why: tab lenses in the upstream-only browser app are outside the chunk's crates and reads
@answer n=7 entity=established-decisions#29.1 v=no
why: the WPT runner and its render target are out of scope by task item 2, and the stand's own 800x600 pin is not this runner's rule
@answer n=8 entity=established-decisions#30.1 v=yes
why: the socket, the boundary widening, the span fields, the reader-only serde_json, the accessibility feature reach and the kept-check rules govern escher-driver's carriers the chunk changes
@answer n=9 entity=established-decisions#30.2 v=yes
why: the chunk adds a closed boolean to the snapshot node fields, so the closed differs set the diff crosses under must take that name
@answer n=10 entity=established-decisions#31.1 v=yes
why: the graph read's recorded step, Check::replay through Session::run and the two committed kept checks are named in the chunk's scope
