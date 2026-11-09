@answer n=1 entity=input-validation#1.101 v=no
why: the session touches no paint or layout file, so the box culling claim in blitz-paint and blitz-dom still holds
@answer n=2 entity=input-validation#1.102 v=no
why: the session reaches no select-list paint file, so the paint claim in blitz-paint still holds
@answer n=3 entity=input-validation#1.103 v=no
why: hit testing in blitz-dom node.rs is not a reached file, so the non-invertible transform claim still holds
@answer n=4 entity=input-validation#1.104 v=no
why: gradient code in blitz-paint is not a reached file
@answer n=5 entity=input-validation#1.105 v=no
why: border radius code in blitz-paint is not a reached file
@answer n=6 entity=input-validation#1.106 v=no
why: hidden input paint code in blitz-paint is not a reached file
@answer n=7 entity=input-validation#1.107 v=no
why: scroll clamping in blitz-dom scrolling.rs is not a reached file
@answer n=8 entity=input-validation#1.108 v=no
why: select list scrolling in blitz-dom and tests/blitz-tests/tests/select_popup.rs are not reached files
@answer n=9 entity=input-validation#1.109 v=no
why: form.rs method and enctype parsing is not a reached file
@answer n=10 entity=input-validation#1.110 v=no
why: form.rs submitted-control rules are not a reached file
@answer n=11 entity=input-validation#1.111 v=no
why: form.rs CRLF normalization is not a reached file
@answer n=12 entity=input-validation#1.112 v=no
why: pointer.rs accept handling is not a reached file
@answer n=13 entity=input-validation#1.113 v=no
why: events.rs NativeFormData::valid is not a reached file, and lib.rs changed only in re-exports this entity does not cite
@answer n=14 entity=input-validation#1.114 v=yes mode=stale
why: the session changed dioxus_document.rs, a reached file, so the cited data-dioxus-id parse lines 51-58 can have moved and the claim is not shown to hold on the changed file
@answer n=15 entity=input-validation#1.115 v=yes mode=add-to
why: the session writes the reached record in handle_event beside the SignalRegistry dispatch (F2), active only where record_reached is called, and dioxus_document_tests.rs is a reached file
@answer n=16 entity=input-validation#1.116 v=no
why: signal_registry.rs is not a reached file, and the registry types, scope and Debug claims still hold; the reached record is a separate root-context type
@answer n=17 entity=input-validation#1.117 v=no
why: mutation_writer.rs is not a reached file
@answer n=18 entity=input-validation#1.118 v=no
why: mutation_writer.rs is not a reached file
@answer n=19 entity=input-validation#1.119 v=no
why: mutation_writer.rs and tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs are not reached files
@answer n=20 entity=input-validation#1.120 v=no
why: fetch.js in blitz-vibey-script is not a reached file
@answer n=21 entity=input-validation#1.121 v=no
why: fetch.js in blitz-vibey-script is not a reached file
@answer n=22 entity=input-validation#1.122 v=no
why: the blitz-vibey-script document and selector code are not reached files
@answer n=23 entity=input-validation#1.123 v=no
why: blitz-vibey-script scrolling.rs is not a reached file
@answer n=24 entity=input-validation#1.124 v=no
why: blitz-vibey-script attributes.rs is not a reached file
@answer n=25 entity=input-validation#1.125 v=no
why: blitz-vibey-script timers.rs is not a reached file
