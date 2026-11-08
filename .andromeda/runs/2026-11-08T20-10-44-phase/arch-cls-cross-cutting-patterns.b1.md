@answer n=1 entity=cross-cutting-patterns#1.1 v=yes
why: the coverage read goes through DioxusDocument::graph, and the accessibility feature gates the graph module, so the new code must sit behind that gate

@answer n=2 entity=cross-cutting-patterns#2.1 v=yes
why: the new verb's command span is emitted ungated by execute.rs, and no log or event may carry the coverage list's names (task item 18)

@answer n=3 entity=cross-cutting-patterns#3.1 v=yes
why: the coverage read and the bridge's event-path record reach the document through DioxusDocument, shared as Rc<RefCell<BaseDocument>> and provided as a root context

@answer n=4 entity=cross-cutting-patterns#4.1 v=no
why: the text-input IME hook in layout construction is not reached by the handler coverage read or the event-path record

@answer n=5 entity=cross-cutting-patterns#4.2 v=no
why: the stylesheet-failure redraw skip is network and render-blocking behaviour the chunk does not touch

@answer n=6 entity=cross-cutting-patterns#4.3 v=yes
why: the exercised record must hold on apps with no registry, the case where the ScopeId::ROOT SignalRegistry lookup finds none

@answer n=7 entity=cross-cutting-patterns#5.1 v=yes
why: dioxus skips event kinds with zero handlers before any handler runs, a fork the exercised record on the event path must account for

@answer n=8 entity=cross-cutting-patterns#6.1 v=no
why: the removed-element skip of queued mounted events is flush behaviour the chunk does not touch, and mounted is excluded from the list anyway

@answer n=9 entity=cross-cutting-patterns#6.2 v=yes
why: the bridge's handle_event sets the registry dispatch around runtime.handle_event, the path the scope names, and the new record must hold where no registry stands

@answer n=10 entity=cross-cutting-patterns#6.3 v=no
why: it attributes signal writes to the dispatch that started them, which the registry-free handler record never reads

@answer n=11 entity=cross-cutting-patterns#7.1 v=no
why: its damage, restyle and changed-set rules are blitz-dom invalidation that the read-only coverage list and the actions comparison never consume

@answer n=12 entity=cross-cutting-patterns#8.1 v=no
why: the restyle-by-mutator rules govern DOM mutation, which the coverage read and the replay comparison do not perform
