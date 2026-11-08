@answer n=1 entity=cross-cutting-patterns#1.1 v=yes
why: the read calls graph and snapshot, which dioxus-native-dom gates behind its accessibility feature, so the new code must keep that gate satisfied
@answer n=2 entity=cross-cutting-patterns#2.1 v=yes
why: the read's command span carrying verb, refused and cause prints through the sink whose formatter scrubs a closed span's fields
@answer n=3 entity=cross-cutting-patterns#3.1 v=yes
why: DioxusDocument::graph and the snapshot read borrow the shared Rc<RefCell<BaseDocument>> read-only
@answer n=4 entity=cross-cutting-patterns#4.1 v=no
why: the IME call runs when a text editor is constructed, which the read never does and the read sets no focus
@answer n=5 entity=cross-cutting-patterns#4.2 v=no
why: the stylesheet-error redraw skip governs pending resource loads, and the read waits on no load
@answer n=6 entity=cross-cutting-patterns#4.3 v=no
why: the SignalRegistry root context is consulted on event dispatch, which the read never runs
@answer n=7 entity=cross-cutting-patterns#5.1 v=yes
why: the snapshot's clickable reading of an element's own click listener depends on the bridge's zero-handler event-kind state that this rule describes
@answer n=8 entity=cross-cutting-patterns#6.1 v=no
why: the queued mounted flush runs only in poll, which the read never runs, and the read keys no NodeId
@answer n=9 entity=cross-cutting-patterns#6.2 v=no
why: the signal-registry dispatch wraps handle_event, which the read never calls
@answer n=10 entity=cross-cutting-patterns#6.3 v=no
why: signal writes from a spawned async handler task arise only inside an event dispatch, which the read never runs
@answer n=11 entity=cross-cutting-patterns#7.1 v=yes
why: the read must leave the changed set unmarked and untaken, and the no-write resolve rule in both layout modes is what a changes-nothing proof on the stand witnesses
@answer n=12 entity=cross-cutting-patterns#8.1 v=no
why: the restyle rules govern DocumentMutator attribute and child-list writes, which the read never performs
