@answer n=1 entity=input-validation#1.101 v=no
why: the geometry culling sits in blitz-paint render, which the chunk neither paints through nor changes.
@answer n=2 entity=input-validation#1.102 v=no
why: select-list painting in blitz-paint is outside the chunk's reach; the chunk paints nothing.
@answer n=3 entity=input-validation#1.103 v=no
why: the non-invertible transform hit rule lives in blitz-dom node.rs, which the chunk does not edit or read for its new code.
@answer n=4 entity=input-validation#1.104 v=no
why: gradient length clamping is paint code the chunk does not reach.
@answer n=5 entity=input-validation#1.105 v=no
why: border-radius scaling is paint code the chunk does not reach.
@answer n=6 entity=input-validation#1.106 v=no
why: the hidden-input paint skip is paint code the chunk does not reach.
@answer n=7 entity=input-validation#1.107 v=no
why: scroll-target clamping in blitz-dom scrolling is not reached; the chunk runs no scroll.
@answer n=8 entity=input-validation#1.108 v=no
why: the open select-list scroll is engine UI the chunk does not drive or read.
@answer n=9 entity=input-validation#1.109 v=no
why: method and enctype parsing in blitz-dom form.rs is not reached; the chunk submits no form.
@answer n=10 entity=input-validation#1.110 v=no
why: submitted-control exclusion is form submission, which the chunk does not reach.
@answer n=11 entity=input-validation#1.111 v=no
why: CRLF normalisation of submitted names and values is form submission, which the chunk does not reach.
@answer n=12 entity=input-validation#1.112 v=no
why: the accept filter is a file-dialog rule the chunk does not touch; its file-input masking concerns the value, not accept.
@answer n=13 entity=input-validation#1.113 v=no
why: NativeFormData validity sits in dioxus-native-dom events.rs, which the chunk does not reach, and the chunk submits no form.
@answer n=14 entity=input-validation#1.114 v=no
why: the data-dioxus-id parse is mutation intake that the chunk neither changes nor reads; its record takes ids from the dispatch.
@answer n=15 entity=input-validation#1.115 v=yes
why: the reached-handler record sits beside this dispatch, which must keep acting as before, inert without a registry, and add no input class.
@answer n=16 entity=input-validation#1.116 v=yes
why: the SignalRegistry the chunk reads must keep dioxus-core-only types, no NodeId beyond the dispatch chain, and no log site or unwrap on app input.
@answer n=17 entity=input-validation#1.117 v=no
why: dangerous_inner_html is a mutation path the chunk does not reach.
@answer n=18 entity=input-validation#1.118 v=no
why: ignoring unsupported attribute value types is mutation writing the chunk does not change or read.
@answer n=19 entity=input-validation#1.119 v=no
why: falsy boolean attribute removal is mutation writing the chunk does not change or read.
@answer n=20 entity=input-validation#1.120 v=no
why: the Response status check is a JS API rule; the chunk runs no document script.
@answer n=21 entity=input-validation#1.121 v=no
why: the fetch method rule is a JS API rule; the chunk runs no document script and no fetch.
@answer n=22 entity=input-validation#1.122 v=no
why: the selector SyntaxError is a JS DOM rule; the chunk runs no document script and reads no selector.
@answer n=23 entity=input-validation#1.123 v=no
why: the scroll-option enumerations are JS API rules; the chunk runs no document script.
@answer n=24 entity=input-validation#1.124 v=no
why: width and height reflection is a JS attribute rule the chunk does not reach.
@answer n=25 entity=input-validation#1.125 v=no
why: the timer delay and clear rules are JS runtime rules the chunk does not reach.
