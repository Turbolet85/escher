//! How the driver says no: a closed set of causes, each with a fixed remedy.

use std::fmt;

/// Why the driver refuses a call.
///
/// Every text of a cause is a fixed string: none carries an id, a name, a value or a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// The call names no verb of the verb set.
    UnknownVerb,
    /// The call's arguments are not the ones the verb's schema states.
    Malformed,
    /// No element on the screen reads the id.
    NotFound,
    /// The id read an element earlier in this session and no element reads it now.
    Stale,
    /// The element carries the `disabled` attribute, which the snapshot reads as not enabled.
    Disabled,
    /// Another element is hit at the point the action would land. A hit reaches content scrolled
    /// out of a scrolling box, so an element lying where such content extends can read covered
    /// though nothing shows over it.
    Covered,
    /// The element lies outside the viewport, or outside the visible part of a scrolling box
    /// that holds it.
    OffScreen,
    /// The app this session holds gives it no way to move its time.
    TimeUnavailable,
}

/// Every [`Cause`], in the set's order.
pub const CAUSES: [Cause; 8] = [
    Cause::UnknownVerb,
    Cause::Malformed,
    Cause::NotFound,
    Cause::Stale,
    Cause::Disabled,
    Cause::Covered,
    Cause::OffScreen,
    Cause::TimeUnavailable,
];

impl Cause {
    /// The cause's name: lowercase words joined by hyphens.
    pub const fn name(self) -> &'static str {
        match self {
            Cause::UnknownVerb => "unknown-verb",
            Cause::Malformed => "malformed",
            Cause::NotFound => "not-found",
            Cause::Stale => "stale",
            Cause::Disabled => "disabled",
            Cause::Covered => "covered",
            Cause::OffScreen => "off-screen",
            Cause::TimeUnavailable => "time-unavailable",
        }
    }

    /// What the cause means, in one sentence.
    pub const fn meaning(self) -> &'static str {
        match self {
            Cause::UnknownVerb => "the call names no verb of the verb set",
            Cause::Malformed => "the call's arguments are not the ones the verb's schema states",
            Cause::NotFound => "no element on the screen reads the id",
            Cause::Stale => {
                "the id read an element earlier in this session and no element reads it now"
            }
            Cause::Disabled => {
                "the element carries the `disabled` attribute, which the snapshot reads as not \
                 enabled"
            }
            Cause::Covered => {
                "another element is hit at the point the action would land; a hit reaches content \
                 clipped by `contain: paint`, so an element lying where such content extends can \
                 read `covered` though nothing shows over it"
            }
            Cause::OffScreen => {
                "the element lies outside the viewport, or outside the visible part of a \
                 scrolling box that holds it"
            }
            Cause::TimeUnavailable => "the app this session holds gives it no way to move its time",
        }
    }

    /// What the caller does next.
    pub const fn remedy(self) -> &'static str {
        match self {
            Cause::UnknownVerb => "use a verb the verb list names",
            Cause::Malformed => {
                "pass exactly the arguments the verb's schema names, each of the kind and within \
                 the bound it states"
            }
            Cause::NotFound => "take a snapshot and use an id it lists",
            Cause::Stale => {
                "the screen changed since the id was read: take a new snapshot and act on what it \
                 lists"
            }
            Cause::Disabled => {
                "the control is disabled: change the state that disables it, then act again"
            }
            Cause::Covered => {
                "another element covers this one: act on the covering element or dismiss it first"
            }
            Cause::OffScreen => {
                "the element is out of view: bring it into view with `scroll`, then act again"
            }
            Cause::TimeUnavailable => {
                "this session cannot move time: act without `advance`, or start a session whose \
                 host supplies a time step"
            }
        }
    }
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Which rule of a verb's schema a malformed call broke.
///
/// A variant that names an argument holds the schema's own name for it, never a name or a
/// value the call supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    /// An argument the schema requires is missing.
    Missing(&'static str),
    /// An argument the schema does not name was passed.
    Unnamed,
    /// An argument was passed twice.
    Repeated(&'static str),
    /// An argument's value is not of its kind.
    WrongKind(&'static str),
    /// An argument's value is outside its bound.
    OutOfBound(&'static str),
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::Missing(name) => write!(f, "the required argument `{name}` is missing"),
            Fault::Unnamed => f.write_str("an argument the schema does not name was passed"),
            Fault::Repeated(name) => write!(f, "the argument `{name}` was passed twice"),
            Fault::WrongKind(name) => write!(f, "the value of `{name}` is not of its kind"),
            Fault::OutOfBound(name) => write!(f, "the value of `{name}` is outside its bound"),
        }
    }
}

/// A call the driver refused: its [`Cause`], and for a malformed call the [`Fault`].
///
/// It holds nothing of the call: the caller has the verb and the arguments it sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refusal {
    cause: Cause,
    fault: Option<Fault>,
}

impl Refusal {
    /// A refusal for `cause`, with no fault.
    pub const fn new(cause: Cause) -> Refusal {
        Refusal { cause, fault: None }
    }

    /// A refusal of a malformed call, naming the rule it broke.
    pub const fn malformed(fault: Fault) -> Refusal {
        Refusal {
            cause: Cause::Malformed,
            fault: Some(fault),
        }
    }

    /// Why the call was refused.
    pub const fn cause(&self) -> Cause {
        self.cause
    }

    /// The rule a malformed call broke; `None` for every other cause.
    pub const fn fault(&self) -> Option<Fault> {
        self.fault
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.cause.name();
        let remedy = self.cause.remedy();
        match self.fault {
            Some(fault) => write!(f, "{name} ({fault}): {remedy}"),
            None => write!(f, "{name}: {remedy}"),
        }
    }
}

impl std::error::Error for Refusal {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_eight_causes_are_named_in_order() {
        let names = [
            "unknown-verb",
            "malformed",
            "not-found",
            "stale",
            "disabled",
            "covered",
            "off-screen",
            "time-unavailable",
        ];
        assert_eq!(CAUSES.len(), 8);
        assert_eq!(CAUSES.map(Cause::name), names);
        for (cause, name) in CAUSES.into_iter().zip(names) {
            assert_eq!(cause.to_string(), name);
        }
    }

    #[test]
    fn each_cause_states_its_meaning_and_its_remedy() {
        let rows = [
            (
                "unknown-verb",
                "the call names no verb of the verb set",
                "use a verb the verb list names",
            ),
            (
                "malformed",
                "the call's arguments are not the ones the verb's schema states",
                "pass exactly the arguments the verb's schema names, each of the kind and within \
                 the bound it states",
            ),
            (
                "not-found",
                "no element on the screen reads the id",
                "take a snapshot and use an id it lists",
            ),
            (
                "stale",
                "the id read an element earlier in this session and no element reads it now",
                "the screen changed since the id was read: take a new snapshot and act on what it \
                 lists",
            ),
            (
                "disabled",
                "the element carries the `disabled` attribute, which the snapshot reads as not \
                 enabled",
                "the control is disabled: change the state that disables it, then act again",
            ),
            (
                "covered",
                "another element is hit at the point the action would land; a hit reaches content \
                 clipped by `contain: paint`, so an element lying where such content extends can \
                 read `covered` though nothing shows over it",
                "another element covers this one: act on the covering element or dismiss it first",
            ),
            (
                "off-screen",
                "the element lies outside the viewport, or outside the visible part of a \
                 scrolling box that holds it",
                "the element is out of view: bring it into view with `scroll`, then act again",
            ),
            (
                "time-unavailable",
                "the app this session holds gives it no way to move its time",
                "this session cannot move time: act without `advance`, or start a session whose \
                 host supplies a time step",
            ),
        ];
        assert_eq!(rows.len(), 8);
        assert_eq!(CAUSES.len(), rows.len());
        for (cause, (name, meaning, remedy)) in CAUSES.into_iter().zip(rows) {
            assert_eq!(cause.name(), name);
            assert_eq!(cause.meaning(), meaning, "{name}");
            assert_eq!(cause.remedy(), remedy, "{name}");
        }
        for (index, cause) in CAUSES.into_iter().enumerate() {
            let name = cause.name();
            assert!(!cause.meaning().is_empty(), "{name}");
            assert!(!cause.remedy().is_empty(), "{name}");
            for text in [name, cause.meaning(), cause.remedy()] {
                assert!(!text.contains('/'), "{name}");
            }
            for other in CAUSES.into_iter().skip(index + 1) {
                assert_ne!(name, other.name());
                assert_ne!(cause.remedy(), other.remedy(), "{name}");
            }
        }
    }

    #[test]
    fn a_refusal_reads_as_its_cause_then_its_remedy() {
        let faults = [
            Fault::Missing("id"),
            Fault::Unnamed,
            Fault::Repeated("id"),
            Fault::WrongKind("id"),
            Fault::OutOfBound("id"),
        ];
        let refusals: Vec<Refusal> = CAUSES
            .into_iter()
            .map(Refusal::new)
            .chain(faults.into_iter().map(Refusal::malformed))
            .collect();
        assert_eq!(refusals.len(), 13);
        for refusal in &refusals {
            let text = refusal.to_string();
            let name = refusal.cause().name();
            assert!(!text.is_empty(), "{name}");
            assert!(text.starts_with(name), "{name}");
            assert!(text.ends_with(refusal.cause().remedy()), "{name}");
            assert!(!text.contains('/'), "{name}");
            let _: &dyn std::error::Error = refusal;
        }
        for cause in CAUSES {
            assert_eq!(Refusal::new(cause).cause(), cause);
            assert_eq!(Refusal::new(cause).fault(), None);
        }
        for fault in faults {
            let refusal = Refusal::malformed(fault);
            assert_eq!(refusal.cause(), Cause::Malformed);
            assert_eq!(refusal.fault(), Some(fault));
            assert!(refusal.to_string().contains(&fault.to_string()));
        }
    }
}
