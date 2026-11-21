//! A script document built `with_seed` draws `Math.random` from one splitmix64 stream per seed:
//! one seed reads one sequence, another seed another, and an unseeded document draws from the
//! host. In both layout modes.

use blitz_dom::{Document, DocumentConfig};
use blitz_vibey_script::ScriptDocument;

/// Three `Math.random` draws as the page writes them into `#result`, comma-joined.
fn draws(seed: Option<u64>, incremental: bool) -> String {
    let html = "<html><body><div id='result'></div><script>\
        document.getElementById('result').textContent = \
        [Math.random(), Math.random(), Math.random()].join(',');\
        </script></body></html>";
    let config = DocumentConfig {
        incremental: Some(incremental),
        ..DocumentConfig::default()
    };
    let doc = ScriptDocument::from_html(html, config);
    let mut doc = match seed {
        Some(seed) => doc.with_seed(seed),
        None => doc,
    };
    doc.execute_scripts();
    let inner = doc.inner();
    let id = inner.query_selector("#result").unwrap().unwrap();
    inner.get_node(id).unwrap().text_content()
}

#[test]
fn seed_zero_twice_reads_one_sequence() {
    for incremental in [false, true] {
        let first = draws(Some(0), incremental);
        assert_eq!(
            first,
            draws(Some(0), incremental),
            "incremental={incremental}"
        );
        assert_eq!(
            first, "0.8833108082136426,0.43152799704850997,0.026433771592597743",
            "incremental={incremental}: the measured sequence"
        );
    }
}

#[test]
fn two_seeds_read_two_sequences() {
    for incremental in [false, true] {
        assert_ne!(
            draws(Some(0), incremental),
            draws(Some(1), incremental),
            "incremental={incremental}"
        );
    }
}

#[test]
fn an_unseeded_document_draws_from_the_host() {
    for incremental in [false, true] {
        let host = draws(None, incremental);
        assert_ne!(
            host,
            draws(Some(0), incremental),
            "incremental={incremental}"
        );
        let values: Vec<f64> = host.split(',').map(|v| v.parse().unwrap()).collect();
        assert_eq!(values.len(), 3, "incremental={incremental}");
        for (index, value) in values.into_iter().enumerate() {
            assert!(
                (0.0..1.0).contains(&value),
                "incremental={incremental}: value {index}"
            );
        }
    }
}
