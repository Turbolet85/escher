//! A kept check run again: each step's call run through [`Session::run`], which checks it
//! first, and its answer compared with the one the check keeps.
//!
//! A step holds when it is accepted or refused as it was and its answer's line, read as JSON, is
//! the kept one — a shot's line without its image, so a shot holds on its acceptance and its
//! size alone. What a replay returns names the first step that did not hold and the keys of its
//! answer that differ, each as the schema's own word; it keeps nothing of a reading. Nothing here
//! prints or logs: each call's own `command` span is the only record a replay leaves.

use serde_json::Value;

use crate::execute::Outcome;
use crate::recording::{Check, Step, kept_line};
use crate::refusal::Refusal;
use crate::schema;
use crate::session::Session;

/// The key a refusal's answer is written under.
const REFUSED: &str = "refused";

/// What replaying a [`Check`] read: whether it holds, how many steps ran and, where it does
/// not hold, the first step that changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    another_app: bool,
    ran: usize,
    change: Option<Change>,
}

/// The first step of a replay that did not hold: its index among the check's steps, whether it
/// was accepted then and now, and the keys of its answer that differ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    step: usize,
    accepted_then: bool,
    accepted_now: bool,
    keys: Vec<&'static str>,
}

impl Check {
    /// Runs the check's steps on `session`, in order, each call through [`Session::run`], and
    /// stops at the first step that does not hold.
    ///
    /// A session started with another label than the app the check was recorded on runs
    /// nothing, and the replay does not hold.
    pub fn replay(&self, session: &mut Session) -> Replay {
        if session.label() != self.app() {
            return Replay {
                another_app: true,
                ran: 0,
                change: None,
            };
        }
        let mut ran = 0;
        let mut change = None;
        for (index, step) in self.steps().iter().enumerate() {
            let answer = session.run(step.call());
            ran += 1;
            change = change_of(index, step, &answer);
            if change.is_some() {
                break;
            }
        }
        Replay {
            another_app: false,
            ran,
            change,
        }
    }
}

/// How `answer` differs from what `step` keeps; `None` when the step holds.
fn change_of(index: usize, step: &Step, answer: &Result<Outcome, Refusal>) -> Option<Change> {
    let now: Option<Value> = serde_json::from_str(&kept_line(answer)).ok();
    let then = step.answer();
    let accepted_now = answer.is_ok();
    if step.accepted() == accepted_now && now.as_ref() == Some(then) {
        return None;
    }
    let fields = schema::verb(&step.call().verb).map_or(&[][..], |verb| verb.fields);
    let keys = fields
        .iter()
        .map(|field| field.name)
        .chain([REFUSED])
        .filter(|key| then.get(key) != now.as_ref().and_then(|now| now.get(key)))
        .collect();
    Some(Change {
        step: index,
        accepted_then: step.accepted(),
        accepted_now,
        keys,
    })
}

impl Replay {
    /// Whether every step held, on a session of the check's own app.
    pub fn holds(&self) -> bool {
        !self.another_app && self.change.is_none()
    }

    /// Whether the session was started on another app than the check's, so nothing ran.
    pub fn another_app(&self) -> bool {
        self.another_app
    }

    /// How many steps ran, the one that changed included.
    pub fn ran(&self) -> usize {
        self.ran
    }

    /// The first step that did not hold, if one did not.
    pub fn change(&self) -> Option<&Change> {
        self.change.as_ref()
    }
}

impl Change {
    /// The step's index among the check's steps, from 0.
    pub fn step(&self) -> usize {
        self.step
    }

    /// Whether the call was accepted when the check was recorded.
    pub fn accepted_then(&self) -> bool {
        self.accepted_then
    }

    /// Whether the call was accepted on the replay.
    pub fn accepted_now(&self) -> bool {
        self.accepted_now
    }

    /// The keys of the answer whose reading differs, as the schema writes them: the result
    /// fields of the step's verb in the table's order, then `refused`. A key no table holds is
    /// not named.
    pub fn keys(&self) -> &[&'static str] {
        &self.keys
    }
}

#[cfg(test)]
mod tests {
    use blitz_test_harness::Harness;

    use super::*;
    use crate::command::Call;

    /// A session over an empty app: no virtual DOM of a stand task is needed.
    fn empty(app: &str) -> Session {
        Session::start(app, || Harness::from_component(|| Ok(Default::default())))
            .expect("the session starts")
    }

    fn call(verb: &str, args: &[(&str, crate::ArgValue)]) -> Call {
        Call {
            verb: verb.to_string(),
            args: args
                .iter()
                .map(|(name, value)| (name.to_string(), value.clone()))
                .collect(),
        }
    }

    /// One step's record: the call's verb and arguments as written, then its answer.
    fn record(tool: &str, arguments: &str, answer: &Result<Outcome, Refusal>) -> String {
        format!(
            r#"{{"tool":"{tool}","arguments":{arguments},"accepted":{},"digest":"{:016x}","answer":{}}}"#,
            answer.is_ok(),
            crate::answer_digest(answer),
            kept_line(answer)
        )
    }

    fn check(app: &str, records: &[String]) -> Check {
        let mut all = vec![format!(
            r#"{{"tool":"start","arguments":{{"app":"{app}"}}}}"#
        )];
        all.extend_from_slice(records);
        Check::from_bytes(format!("[{}]", all.join(",")).as_bytes()).expect("the check reads")
    }

    /// The three calls the session tests record, as a check spells them.
    const SCRIPT: [(&str, &str); 3] = [
        ("snapshot", "{}"),
        ("click", r#"{"id":"replay-names-no-element"}"#),
        ("advance", r#"{"ms":100}"#),
    ];

    /// The script's calls run on a fresh session, kept as its check.
    fn recorded() -> Check {
        let mut session = empty("counter");
        let calls = [
            call("snapshot", &[]),
            call(
                "click",
                &[(
                    "id",
                    crate::ArgValue::Text("replay-names-no-element".into()),
                )],
            ),
            call("advance", &[("ms", crate::ArgValue::Number(100))]),
        ];
        assert_eq!(calls.len(), SCRIPT.len());
        let records: Vec<String> = calls
            .iter()
            .zip(SCRIPT)
            .map(|(call, (tool, arguments))| record(tool, arguments, &session.run(call)))
            .collect();
        check("counter", &records)
    }

    #[test]
    fn a_check_replayed_on_a_fresh_session_holds() {
        let check = recorded();
        assert_eq!(check.steps().len(), 3);
        assert!(!check.steps()[1].accepted());
        let replay = check.replay(&mut empty("counter"));
        assert!(replay.holds() && !replay.another_app());
        assert_eq!(replay.ran(), 3);
        assert_eq!(replay.change(), None);
    }

    #[test]
    fn an_acceptance_that_flips_is_a_change_naming_the_keys_that_differ() {
        let acted = Ok(Outcome::Acted {
            settled: true,
            busy: None,
            diff: dioxus_native_dom::SnapshotDiff {
                added: Vec::new(),
                removed: Vec::new(),
                changed: Vec::new(),
            },
        });
        let check = check(
            "counter",
            &[record(
                "click",
                r#"{"id":"replay-names-no-element"}"#,
                &acted,
            )],
        );
        let replay = check.replay(&mut empty("counter"));
        assert!(!replay.holds());
        assert_eq!(replay.ran(), 1);
        let change = replay.change().expect("the step changed");
        assert_eq!(change.step(), 0);
        assert!(change.accepted_then() && !change.accepted_now());
        assert_eq!(
            change.keys(),
            ["settled", "added", "removed", "changed", "refused"]
        );
    }

    #[test]
    fn a_changed_answer_names_its_keys_and_stops_the_replay() {
        let screen = empty("counter").run(&call("snapshot", &[]));
        // An answer kept as the driver writes it, its digest not compared on replay.
        let kept = |tool: &str, arguments: &str, answer: &str| {
            format!(
                r#"{{"tool":"{tool}","arguments":{arguments},"accepted":true,"digest":"0123456789abcdef","answer":{answer}}}"#
            )
        };
        let records = [
            record("snapshot", "{}", &screen),
            kept(
                "advance",
                r#"{"ms":100}"#,
                r#"{"settled":true,"added":[],"removed":[],"changed":[],"advanced_ms":99}"#,
            ),
            kept("snapshot", "{}", r#"{"text":"other"}"#),
        ];
        let replay = check("counter", &records).replay(&mut empty("counter"));
        assert!(!replay.holds());
        // The third step is never run.
        assert_eq!(replay.ran(), 2);
        let change = replay.change().expect("the step changed");
        assert_eq!(change.step(), 1);
        assert!(change.accepted_then() && change.accepted_now());
        assert_eq!(change.keys(), ["advanced_ms"]);

        let replay = check("counter", &records[2..]).replay(&mut empty("counter"));
        let change = replay.change().expect("the step changed");
        assert_eq!((change.step(), change.keys()), (0, &["text"][..]));
    }

    #[test]
    fn a_shot_holds_at_its_size_whatever_its_pixels() {
        let shot = |width, png: &[u8]| -> Result<Outcome, Refusal> {
            Ok(Outcome::Shot {
                width,
                height: 600,
                png: png.to_vec(),
            })
        };
        let check = check("counter", &[record("screenshot", "{}", &shot(800, b"one"))]);
        let [step] = check.steps() else {
            panic!("one step");
        };
        assert_eq!(change_of(0, step, &shot(800, b"one")), None);
        assert_eq!(change_of(0, step, &shot(800, b"another")), None);
        let wider = change_of(0, step, &shot(801, b"one")).expect("the size changed");
        assert_eq!(wider.keys(), ["width"]);
        let refused = change_of(0, step, &Err(Refusal::new(crate::Cause::FontsUnavailable)))
            .expect("the shot was refused");
        assert_eq!(refused.keys(), ["width", "height", "refused"]);
    }

    #[test]
    fn a_check_of_another_app_runs_nothing() {
        let check = recorded();
        let mut session = empty("crud");
        let replay = check.replay(&mut session);
        assert!(!replay.holds() && replay.another_app());
        assert_eq!((replay.ran(), replay.change()), (0, None));
        // Its `advance` did not run: the session's clock never moved.
        assert!(session.harness().time() == 0.0);
    }
}
