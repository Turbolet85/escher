//! A run's randomness comes from one seed: two runs of an app reading the seeded source, given
//! one seed, read the same state at every step. In both layout modes. The fixture, stated here
//! (its one reader), reads `use_seeded_random`, draws three die values at its first render and
//! one more on each click of its `roll` button. Each run boots it through `Harness::from_vdom`
//! at the stand's options, holds it in a driver session and runs a `snapshot`, then five
//! `click`s on `roll` each followed by a `snapshot`, through `Session::run` alone; a run reads
//! as its eleven answer lines. *One seed* reads the same lines at every step, in this process
//! and in a second one (this test binary re-run on one ignored child, compared by digest).
//! *Two seeds* read different boot screens. *No seed* reads as `DEFAULT_SEED`. *A kept check*
//! recorded on a seeded run replays every step on a fresh session of that seed and stops at its
//! first step on a session of another seed. A failure names the layout mode and a step index,
//! never an answer.

use std::process::Command;

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{DEFAULT_SEED, DioxusDocument, SeededRandom, use_seeded_random};
use escher_driver::{Call, Session, answer_digest};
use seven_guis::stand;

mod session_common;
use session_common::{click, snapshot};

/// The label every run's session is started with.
const LABEL: &str = "dice";

/// The child's line prefix, before a layout mode and a digest.
const CHILD_LINE: &str = "seeded-digest";

/// The clicks on `roll` a run takes, each followed by a `snapshot`.
const ROLLS: usize = 5;

/// A run's answer lines: the boot `snapshot`, then a `click` and a `snapshot` per roll.
const STEPS: usize = 1 + 2 * ROLLS;

/// Three die values at the first render, one more on each click of `roll`.
fn dice() -> Element {
    let random = use_seeded_random();
    let mut rolls = use_signal({
        let random = random.clone();
        move || (0..3).map(|_| random.below(6) + 1).collect::<Vec<u64>>()
    });
    rsx! {
        div { id: "dice",
            button {
                key: "roll",
                id: "roll",
                onclick: move |_| rolls.write().push(random.below(6) + 1),
                "Roll"
            }
            ul { id: "rolls",
                for (i , value) in rolls.read().iter().enumerate() {
                    li { key: "{i}", "{value}" }
                }
            }
        }
    }
}

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

/// The fixture booted at the stand's options, its root given `seed` where there is one.
fn boot(seed: Option<u64>, incremental: bool) -> Harness<DioxusDocument> {
    let vdom = VirtualDom::new(dice);
    let vdom = match seed {
        Some(seed) => vdom.with_root_context(SeededRandom::from_seed(seed)),
        None => vdom,
    };
    Harness::from_vdom(vdom, stand::options(incremental))
}

fn hold(seed: Option<u64>, incremental: bool) -> Session {
    Session::start(LABEL, || boot(seed, incremental)).expect("the label is a label")
}

/// The calls a run makes, in order.
fn script() -> Vec<Call> {
    let mut calls = vec![snapshot()];
    for _ in 0..ROLLS {
        calls.extend([click("roll"), snapshot()]);
    }
    assert!(calls.len() == STEPS, "the script holds {STEPS} calls");
    calls
}

/// Runs the script on `session`: each answer's line and digest, every call accepted.
fn drive(session: &mut Session, mode: &str) -> Vec<(String, u64)> {
    script()
        .iter()
        .enumerate()
        .map(|(step, call)| {
            let answer = session.run(call);
            assert!(answer.is_ok(), "{mode} step {step}: the call is accepted");
            let digest = answer_digest(&answer);
            let line = match answer {
                Ok(outcome) => outcome.to_json(),
                Err(refusal) => refusal.to_json(),
            };
            (line, digest)
        })
        .collect()
}

fn run(seed: Option<u64>, incremental: bool) -> Vec<(String, u64)> {
    drive(&mut hold(seed, incremental), mode_name(incremental))
}

#[track_caller]
fn assert_same_at_every_step(a: &[(String, u64)], b: &[(String, u64)], what: &str) {
    assert!(
        a.len() == STEPS && b.len() == STEPS,
        "{what}: {STEPS} steps"
    );
    for (step, (left, right)) in a.iter().zip(b).enumerate() {
        assert!(left == right, "{what}: step {step} differs");
    }
}

#[test]
fn the_fixture_reads_the_seeded_source() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let seven = hold(Some(7), incremental)
            .harness()
            .doc
            .snapshot()
            .to_text();
        let eight = hold(Some(8), incremental)
            .harness()
            .doc
            .snapshot()
            .to_text();
        assert!(
            seven != eight,
            "{mode}: seeds 7 and 8 boot different screens"
        );
    }
}

#[test]
fn one_seed_two_runs_read_the_same_state_at_every_step() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        assert_same_at_every_step(&run(Some(7), incremental), &run(Some(7), incremental), mode);
    }
}

#[test]
fn two_seeds_read_different_states() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let seven = run(Some(7), incremental);
        let eight = run(Some(8), incremental);
        assert!(seven[0] != eight[0], "{mode} step 0: seeds 7 and 8 differ");
    }
}

#[test]
fn a_run_given_no_seed_reads_the_default_seed() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        assert_same_at_every_step(
            &run(None, incremental),
            &run(Some(DEFAULT_SEED), incremental),
            mode,
        );
    }
}

/// Each layout mode's seed-7 digests, in step order.
fn seven_digests() -> Vec<(&'static str, Vec<u64>)> {
    [false, true]
        .into_iter()
        .map(|incremental| {
            let digests = run(Some(7), incremental)
                .into_iter()
                .map(|(_, digest)| digest)
                .collect();
            (mode_name(incremental), digests)
        })
        .collect()
}

#[test]
fn a_second_process_reads_the_same_state() {
    let parent = seven_digests();
    let output = Command::new(std::env::current_exe().expect("the test binary has a path"))
        .args([
            "--ignored",
            "--exact",
            "seeded_digests_in_a_child_process",
            "--nocapture",
        ])
        .output()
        .expect("the child runs");
    assert!(output.status.success(), "the child succeeds");
    let stdout = String::from_utf8_lossy(&output.stdout);
    for (mode, digests) in parent {
        let child: Vec<u64> = stdout
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(' ');
                match (parts.next(), parts.next(), parts.next(), parts.next()) {
                    (Some(CHILD_LINE), Some(child_mode), Some(digest), None)
                        if child_mode == mode =>
                    {
                        digest.parse().ok()
                    }
                    _ => None,
                }
            })
            .collect();
        assert!(child.len() == STEPS, "{mode}: the child read {STEPS} steps");
        for (step, (left, right)) in digests.iter().zip(&child).enumerate() {
            assert!(left == right, "{mode}: step {step} differs in the child");
        }
    }
}

#[test]
#[ignore = "spawned as a child process by a_second_process_reads_the_same_state"]
fn seeded_digests_in_a_child_process() {
    for (mode, digests) in seven_digests() {
        for digest in digests {
            println!("{CHILD_LINE} {mode} {digest}");
        }
    }
}

#[cfg(unix)]
#[test]
fn a_kept_check_replays_its_seeded_run() {
    use escher_driver::Check;
    use std::fs;

    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let dir = session_common::state_dir(&format!("seeded-rec-{mode}"));
        let _ = fs::remove_dir_all(&dir);
        let mut session = hold(Some(7), incremental)
            .recording_into(&dir)
            .expect("the recording starts");
        drive(&mut session, mode);
        drop(session);
        let bytes = fs::read(dir.join("check-1.json")).expect("the check is kept");
        let check = Check::from_bytes(&bytes).expect("the check reads");
        assert!(check.steps().len() == STEPS, "{mode}: one step per call");

        let same = check.replay(&mut hold(Some(7), incremental));
        assert!(same.holds(), "{mode}: the check holds on seed 7");
        assert!(same.ran() == STEPS, "{mode}: every step ran on seed 7");

        let other = check.replay(&mut hold(Some(8), incremental));
        assert!(!other.holds(), "{mode}: the check does not hold on seed 8");
        let first = other.change().map(|change| change.step());
        assert!(first == Some(0), "{mode}: seed 8 stops at step 0");

        fs::remove_dir_all(&dir).expect("the recording directory is removed");
    }
}
