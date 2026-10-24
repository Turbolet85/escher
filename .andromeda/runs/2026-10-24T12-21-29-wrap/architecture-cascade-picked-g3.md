# architecture · picked accounting · group 3 of 4

## Picked
standard-contracts#1.1 · not touched · its `DocumentConfig` field list is unchanged; the `style_threading` doc-comment edit is the threading decision's and the config.rs line moves are the citation sweep's
standard-contracts#2.1 · not touched · the `clock` field is untouched and no wall-clock read stands in blitz-dom outside clock.rs (gate entry 12)
standard-contracts#3.1 · not touched · the mutator's methods and signatures are unchanged; the `class` arm keeping `ElementData::classes` in sync is carried by the nodes-and-widgets proposal
standard-contracts#5.1 · proposed · the hit walk now skips nodes dropped since the last resolve (`f102663b8`)
standard-contracts#7.1 · proposed · `compute_replaced_layout` now coerces a replaced element's natural height through its `aspect-ratio` (`50bf2f235`)
standard-contracts#8.1 · proposed · two new public fields land, `Node::out_of_flow` and `ElementData::classes`
standard-contracts#9.1 · not touched · `ElementData::is_disabled` is unchanged; its element.rs line moves with the citation sweep
standard-contracts#20.1 · proposed · `launch_cfg_with_props` now builds its document with `StyleThreading::Parallel`
standard-contracts#23.1 · proposed · the root examples gain `multi_doc`, a command line reading its arguments and three env variables
standard-contracts#24.1 · proposed · the browser's document config and View Source's document now set `StyleThreading::Parallel`
standard-contracts#25.1 · not touched · the report states no runner output moved; attr_test.rs's new reads move only citations (the sweep's), and mimalloc is carried at process-wide state
occupied-resources#2.1 · not touched · no outbound host enters with the take
occupied-resources#3.1 · not touched · no change writes a new path
occupied-resources#5.1 · proposed · the process-wide `PARALLEL_TRAVERSAL_LOCK` lands and mimalloc becomes the global allocator of the browser and the WPT runner
occupied-resources#7.1 · proposed · `examples/multi_doc.rs` reads three new env variables, `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT`
