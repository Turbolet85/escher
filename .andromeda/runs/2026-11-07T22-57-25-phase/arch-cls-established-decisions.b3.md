@answer n=1 entity=established-decisions#25.2 v=no
why: the buffered-render limit concerns a task polled during render and set_focus_to, which the graph read never calls; the read takes the screen between renders, after the mutation buffer is applied
@answer n=2 entity=established-decisions#25.3 v=yes
why: the registry's read goes through VirtualDom::in_runtime, and this entity states the PollTask path inside render_immediate where that borrow meets a render, so the new read must not run there
@answer n=3 entity=established-decisions#26.1 v=yes
why: the public vdom walk from base_scope through roots, dynamic nodes and mounted scopes, naming components by VComponent.name, is the link 1 walk the chunk reuses
@answer n=4 entity=established-decisions#26.2 v=yes
why: Readable::subscribers() inside in_runtime with ReactiveContext::origin_scope is the link 3 and link 4 mechanism of the signal registry, and its hook-local limit governs the stand's move onto the registry
@answer n=5 entity=established-decisions#27.1 v=no
why: browser app history, favicon and about pages are not reached (task item 1 not needed); the graph is read on the headless stand and the driver only
@answer n=6 entity=established-decisions#28.1 v=no
why: browser app tab lenses and webview props; the chunk changes no browser app code
@answer n=7 entity=established-decisions#29.1 v=no
why: the WPT runner's own render target, timeouts and test kinds; the chunk changes no runner and pins only the stand's own viewport
@answer n=8 entity=established-decisions#30.1 v=yes
why: the driver's socket, carriers, boundary widening, tracing span rules, MCP crossing, and the kept-check and shot carriers are what the new verb, outcome, answers and spans must honour
@answer n=9 entity=established-decisions#30.2 v=yes
why: the new closed «reacts to a click» boolean joins the snapshot diff, and this entity keeps differs as a closed set whose row-bounds reading the chunk's diff change must stay inside
@answer n=10 entity=established-decisions#31.1 v=yes
why: the recorded step, Check::replay through Session::run, the masked type stand-in and the two committed kept checks are named by task item 14
