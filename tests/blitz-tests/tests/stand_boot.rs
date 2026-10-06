//! The headless stand boots each lean 7GUIs task inside its `TaskShell` chrome,
//! offline, at the pinned viewport with the bundled font, and lays it out the
//! same way on every fresh boot and in both layout modes.

use std::sync::{Arc, Mutex};

use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::net::{NetHandler, NetProvider, Request};
use seven_guis::stand::{self, LeanTask};

/// A `NetProvider` which records the urls it is asked for.
#[derive(Default)]
struct RecordingNetProvider {
    requests: Mutex<Vec<String>>,
}

impl NetProvider for RecordingNetProvider {
    fn fetch(&self, _doc_id: usize, request: Request, _handler: Box<dyn NetHandler>) {
        self.requests.lock().unwrap().push(request.url.to_string());
    }
}

fn title(task: LeanTask) -> &'static str {
    match task {
        LeanTask::Counter => "Counter",
        LeanTask::FlightBooker => "Flight Booker",
        LeanTask::Timer => "Timer",
        LeanTask::Crud => "CRUD",
    }
}

fn recording_options(net: &Arc<RecordingNetProvider>) -> HarnessOptions {
    HarnessOptions {
        net_provider: Some(Arc::clone(net) as _),
        ..stand::options(true)
    }
}

#[track_caller]
fn assert_present<D: blitz_dom::Document>(harness: &Harness<D>, selector: &str, task: LeanTask) {
    assert!(
        harness.query(selector).is_some(),
        "{task:?}: expected `{selector}` in the stand document"
    );
}

#[test]
fn each_lean_task_mounts_inside_task_shell() {
    for task in LeanTask::ALL {
        let harness = stand::boot(task, stand::options(true));

        assert_present(&harness, "main#main > #task-shell", task);
        assert_present(&harness, "#task-shell > #task-header > #back-btn", task);
        assert_present(&harness, "#task-header > #back-btn + #task-title", task);
        assert_present(&harness, "#task-shell > #task-header + #task-body", task);
        assert_eq!(harness.text_content("#task-title"), title(task), "{task:?}");
    }
}

#[test]
fn task_title_measures_non_zero_under_the_bundled_font() {
    for task in LeanTask::ALL {
        let harness = stand::boot(task, stand::options(true));

        let rect = harness.layout_rect("#task-title");
        assert!(
            rect.width > 0.0 && rect.height > 0.0,
            "{task:?}: #task-title laid out at {}x{}",
            rect.width,
            rect.height
        );
    }
}

#[test]
fn two_fresh_boots_lay_out_identically() {
    for task in LeanTask::ALL {
        let first = stand::boot(task, stand::options(true)).dom_string();
        let second = stand::boot(task, stand::options(true)).dom_string();

        assert!(first.contains("#task-shell"), "{task:?}: {first}");
        assert_eq!(first, second, "{task:?}");
    }
}

#[test]
fn incremental_and_full_layout_boots_lay_out_identically() {
    for task in LeanTask::ALL {
        let full = stand::boot(task, stand::options(false)).dom_string();
        let incremental = stand::boot(task, stand::options(true)).dom_string();

        assert!(full.contains("#task-shell"), "{task:?}: {full}");
        assert_eq!(full, incremental, "{task:?}");
    }
}

#[test]
fn stand_options_reach_the_injected_net_provider() {
    let net = Arc::new(RecordingNetProvider::default());

    Harness::from_html_with(
        r#"<html><head><link rel="stylesheet" href="http://example.test/a.css"></head></html>"#,
        recording_options(&net),
    );

    assert_eq!(*net.requests.lock().unwrap(), ["http://example.test/a.css"]);
}

#[test]
fn boot_and_click_make_no_net_requests() {
    for task in LeanTask::ALL {
        let net = Arc::new(RecordingNetProvider::default());
        let mut harness = stand::boot(task, recording_options(&net));

        harness.click("#back-btn");

        assert_eq!(
            *net.requests.lock().unwrap(),
            Vec::<String>::new(),
            "{task:?}"
        );
    }
}
