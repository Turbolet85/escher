//! A session kept as a replayable check: the calls as they crossed in, each with whether it was
//! accepted and a digest of the answer the driver wrote for it.
//!
//! A check is one JSON array. Its first record names the app the session was started on; every
//! later record is one call the session answered: its verb, its arguments by their schema names,
//! `masked` on a `type` whose text was not kept, `accepted` and `digest`. No screen reading is
//! kept — no snapshot text, name, value, diff or pixel — and the text typed into a masked control
//! is [`MASKED_STAND_IN`]. The form is written here by the crate's own JSON writer and read back
//! with `serde_json`, strictly and under its bounds. Nothing here logs.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use crate::command::{ArgValue, Call, call_of};
use crate::error::SessionError;
use crate::execute::Outcome;
use crate::json::Object;
use crate::refusal::{Cause, Refusal};
use crate::schema::{self, ArgKind, Level, VerbSpec};
use crate::session::valid_label;

/// What a recorded `type` into a masked control carries as its text: one fixed stand-in, so no
/// byte, length or digest of what was typed is kept.
pub const MASKED_STAND_IN: &str = "masked";

/// The most records a check holds, its opening record included.
pub(crate) const MAX_CHECK_STEPS: usize = 400;

/// The most bytes a check is.
pub(crate) const MAX_CHECK_BYTES: usize = 1_048_576;

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// The digest of an answer: FNV-1a 64 over the bytes of the line the driver writes for it,
/// [`Outcome::to_json`] or [`Refusal::to_json`].
///
/// It tells one answer from another, not a reading from a guess: a check keeps it in place of
/// the answer, so the check holds no screen reading.
pub fn answer_digest(answer: &Result<Outcome, Refusal>) -> u64 {
    let written = match answer {
        Ok(outcome) => outcome.to_json(),
        Err(refusal) => refusal.to_json(),
    };
    fnv1a(written.as_bytes())
}

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(FNV_OFFSET_BASIS, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(FNV_PRIME)
    })
}

/// The digest as the check writes it: 16 lowercase hex digits.
fn hex(digest: u64) -> String {
    format!("{digest:016x}")
}

/// The opening record: the app the session was started on.
fn opening(app: &str) -> String {
    let mut arguments = Object::new();
    arguments.text(START_APP, app);
    let mut record = Object::new();
    record.text("tool", schema::START.name);
    record.json("arguments", &arguments.end());
    record.end()
}

/// The table's name for `start`'s one argument.
const START_APP: &str = match schema::START.args {
    [app] => app.name,
    _ => "",
};

/// The verb of a call a check can keep: an instance verb, each argument one its schema names,
/// none passed twice. Any other call is refused before it runs and no record can spell it.
fn kept_verb(call: &Call) -> Option<&'static VerbSpec> {
    let verb = schema::verb(&call.verb).filter(|verb| verb.level == Level::Instance)?;
    for (index, (name, _)) in call.args.iter().enumerate() {
        let named = verb.args.iter().any(|arg| arg.name == name);
        let repeated = call.args[..index]
            .iter()
            .any(|(earlier, _)| earlier == name);
        if !named || repeated {
            return None;
        }
    }
    Some(verb)
}

/// One record of a call: its arguments in the schema's order, which is the order the schema
/// checks them in, so a replay meets every rule as the call did.
fn record_of(verb: &VerbSpec, call: &Call, masked: bool, accepted: bool, digest: u64) -> String {
    let mut arguments = Object::new();
    for arg in verb.args {
        let Some((_, value)) = call.args.iter().find(|(name, _)| name == arg.name) else {
            continue;
        };
        match value {
            ArgValue::Text(_) if masked && arg.kind == ArgKind::Text => {
                arguments.text(arg.name, MASKED_STAND_IN);
            }
            ArgValue::Text(text) => arguments.text(arg.name, text),
            ArgValue::Number(number) => arguments.integer(arg.name, *number),
            ArgValue::Flag(flag) => arguments.flag(arg.name, *flag),
        }
    }
    let mut record = Object::new();
    record.text("tool", verb.name);
    record.json("arguments", &arguments.end());
    if masked {
        record.flag("masked", true);
    }
    record.flag("accepted", accepted);
    record.text("digest", &hex(digest));
    record.end()
}

/// The check's whole text: one record per line inside one array.
fn check_text(records: &[String]) -> String {
    format!("[\n{}\n]\n", records.join(",\n"))
}

/// A session's recording: its check file and the records written to it so far.
pub(crate) struct Recording {
    file: PathBuf,
    temporary: PathBuf,
    records: Vec<String>,
}

impl Recording {
    /// Starts a recording of a session on `app` in `dir`: the directory created owner-only or
    /// checked to be, and a new check file holding the opening record.
    pub(crate) fn create(dir: &Path, app: &str) -> Result<Recording, SessionError> {
        prepare_dir(dir)?;
        let records = vec![opening(app)];
        let (file, temporary) = imp::create_check(dir, check_text(&records).as_bytes())?;
        Ok(Recording {
            file,
            temporary,
            records,
        })
    }

    /// Records one answered call, then rewrites the check whole. False when the recording has
    /// ended: a write failed or a bound would be passed, and the check file is removed.
    ///
    /// A call no record can spell — one [`kept_verb`] does not keep — is not recorded.
    pub(crate) fn record(
        &mut self,
        call: &Call,
        masked: bool,
        answer: &Result<Outcome, Refusal>,
    ) -> bool {
        let Some(verb) = kept_verb(call) else {
            return true;
        };
        let masked = masked && verb.name == schema::TYPE.name;
        self.records.push(record_of(
            verb,
            call,
            masked,
            answer.is_ok(),
            answer_digest(answer),
        ));
        let text = check_text(&self.records);
        let written = self.records.len() <= MAX_CHECK_STEPS
            && text.len() <= MAX_CHECK_BYTES
            && imp::rewrite(&self.file, &self.temporary, text.as_bytes()).is_ok();
        if !written {
            imp::remove(&self.file);
        }
        written
    }
}

/// Creates `dir` owner-only (mode `0700`) when it is absent — its parent must exist — or
/// refuses it with [`SessionError::RecordDirNotPrivate`] when it is present and open to its
/// group or to others.
pub(crate) fn prepare_dir(dir: &Path) -> Result<(), SessionError> {
    imp::prepare_dir(dir)
}

#[cfg(unix)]
mod imp {
    use std::fs::{self, DirBuilder, OpenOptions};
    use std::io::{self, Write};
    use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
    use std::path::{Path, PathBuf};

    use crate::error::SessionError;

    fn io_error(error: io::Error) -> SessionError {
        SessionError::Io(error.kind())
    }

    pub(super) fn prepare_dir(dir: &Path) -> Result<(), SessionError> {
        match DirBuilder::new().mode(0o700).create(dir) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let mode = fs::metadata(dir).map_err(io_error)?.permissions().mode();
                if mode & 0o077 == 0 {
                    Ok(())
                } else {
                    Err(SessionError::RecordDirNotPrivate)
                }
            }
            Err(error) => Err(io_error(error)),
        }
    }

    /// A new owner-only file at `path`, failing when anything is there already.
    fn create_new(path: &Path) -> io::Result<fs::File> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)
    }

    /// Creates `check-{n}.json` in `dir`, `n` the smallest from 1 that names no file there, and
    /// writes `text` to it: the file and the temporary path its rewrites go through.
    pub(super) fn create_check(
        dir: &Path,
        text: &[u8],
    ) -> Result<(PathBuf, PathBuf), SessionError> {
        let mut n: u64 = 1;
        let (path, mut file) = loop {
            let path = dir.join(format!("check-{n}.json"));
            match create_new(&path) {
                Ok(file) => break (path, file),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => n += 1,
                Err(error) => return Err(io_error(error)),
            }
        };
        let temporary = dir.join(format!(".check-{n}.json.tmp"));
        if let Err(error) = file.write_all(text) {
            let _ = fs::remove_file(&path);
            return Err(io_error(error));
        }
        Ok((path, temporary))
    }

    /// Writes `text` to a new temporary file and renames it over `file`, so the check on disk
    /// is always whole. A temporary file this wrote and could not rename is removed; one that
    /// was there before is left as it was.
    pub(super) fn rewrite(file: &Path, temporary: &Path, text: &[u8]) -> io::Result<()> {
        let mut written = create_new(temporary)?;
        let renamed = written
            .write_all(text)
            .and_then(|()| fs::rename(temporary, file));
        if renamed.is_err() {
            let _ = fs::remove_file(temporary);
        }
        renamed
    }

    pub(super) fn remove(file: &Path) {
        let _ = fs::remove_file(file);
    }
}

#[cfg(not(unix))]
mod imp {
    use std::io;
    use std::path::{Path, PathBuf};

    use crate::error::SessionError;

    pub(super) fn prepare_dir(_dir: &Path) -> Result<(), SessionError> {
        Err(SessionError::Unsupported)
    }

    pub(super) fn create_check(
        _dir: &Path,
        _text: &[u8],
    ) -> Result<(PathBuf, PathBuf), SessionError> {
        Err(SessionError::Unsupported)
    }

    pub(super) fn rewrite(_file: &Path, _temporary: &Path, _text: &[u8]) -> io::Result<()> {
        Err(io::ErrorKind::Unsupported.into())
    }

    pub(super) fn remove(_file: &Path) {}
}

/// A kept check, read back: the app it was recorded on and its steps, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    app: String,
    steps: Vec<Step>,
}

/// One recorded call of a [`Check`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    call: Call,
    accepted: bool,
    digest: u64,
    masked: bool,
}

impl Check {
    /// Reads a check from its bytes, or refuses it as [`Cause::Malformed`], holding nothing of
    /// the bytes.
    ///
    /// The bytes are at most 1,048,576 and are one JSON array of at most 400 records. The first
    /// is `{"tool":"start","arguments":{"app":…}}`, the app a session label. Every later one has
    /// the keys `tool`, an instance verb of the table; `arguments`, an object of the verb's
    /// arguments by their schema names, each text, a whole number or a flag; `accepted`, a flag;
    /// `digest`, 16 lowercase hex digits; and on a `type` alone, `masked`, true. Anything else
    /// is refused. Each step's call is the call its arguments spell, as an MCP tool call's are
    /// read; it is checked against the schema only when it is run.
    pub fn from_bytes(bytes: &[u8]) -> Result<Check, Refusal> {
        let malformed = Refusal::new(Cause::Malformed);
        if bytes.len() > MAX_CHECK_BYTES {
            return Err(malformed);
        }
        let Ok(Value::Array(records)) = serde_json::from_slice::<Value>(bytes) else {
            return Err(malformed);
        };
        if records.len() > MAX_CHECK_STEPS {
            return Err(malformed);
        }
        let (first, rest) = records.split_first().ok_or(malformed)?;
        let app = opening_app(first).ok_or(malformed)?;
        let steps = rest
            .iter()
            .map(step_of)
            .collect::<Option<Vec<Step>>>()
            .ok_or(malformed)?;
        Ok(Check { app, steps })
    }

    /// The app the check was recorded on.
    pub fn app(&self) -> &str {
        &self.app
    }

    /// The recorded calls, in order, the opening record not among them.
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }
}

impl Step {
    /// The call as it crossed in, a masked control's text as [`MASKED_STAND_IN`].
    pub fn call(&self) -> &Call {
        &self.call
    }

    /// Whether the call was accepted: false when it was refused.
    pub fn accepted(&self) -> bool {
        self.accepted
    }

    /// The digest of the answer the driver wrote for the call ([`answer_digest`]).
    pub fn digest(&self) -> u64 {
        self.digest
    }

    /// Whether the call is a `type` whose text was not kept.
    pub fn masked(&self) -> bool {
        self.masked
    }
}

/// The object `value` is, when every key it has is one of `keys`.
fn object_of<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a Map<String, Value>> {
    let Value::Object(object) = value else {
        return None;
    };
    object
        .keys()
        .all(|key| keys.contains(&key.as_str()))
        .then_some(object)
}

fn opening_app(record: &Value) -> Option<String> {
    let record = object_of(record, &["tool", "arguments"])?;
    if record.get("tool")?.as_str()? != schema::START.name {
        return None;
    }
    let arguments = object_of(record.get("arguments")?, &[START_APP])?;
    let app = arguments.get(START_APP)?.as_str()?;
    valid_label(app).then(|| app.to_string())
}

fn step_of(record: &Value) -> Option<Step> {
    let record = object_of(
        record,
        &["tool", "arguments", "masked", "accepted", "digest"],
    )?;
    let verb =
        schema::verb(record.get("tool")?.as_str()?).filter(|verb| verb.level == Level::Instance)?;
    let names: Vec<&str> = verb.args.iter().map(|arg| arg.name).collect();
    let arguments = object_of(record.get("arguments")?, &names)?;
    let call = call_of(verb, arguments).ok()?;
    let masked = match record.get("masked") {
        None => false,
        Some(Value::Bool(true)) if verb.name == schema::TYPE.name => true,
        Some(_) => return None,
    };
    let accepted = record.get("accepted")?.as_bool()?;
    let digest = record.get("digest")?.as_str()?;
    let is_hex = digest.len() == 16
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !is_hex {
        return None;
    }
    Some(Step {
        call,
        accepted,
        digest: u64::from_str_radix(digest, 16).ok()?,
        masked,
    })
}

#[cfg(all(test, not(unix)))]
mod off_unix_tests {
    use super::*;

    #[test]
    fn off_unix_a_recording_is_refused_as_unsupported() {
        let dir = Path::new("kept-off-unix");
        assert!(matches!(prepare_dir(dir), Err(SessionError::Unsupported)));
        assert!(matches!(
            Recording::create(dir, "counter"),
            Err(SessionError::Unsupported)
        ));
        assert!(!dir.exists());
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use dioxus_native_dom::SnapshotDiff;

    use super::*;

    /// A fresh path under the workspace's `target/tmp`, named for the test: nothing is there.
    fn scratch(name: &str) -> PathBuf {
        let dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp")).join(
            format!("escher-driver-recording-{}-{name}", std::process::id()),
        );
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.parent().expect("a parent")).expect("target/tmp is created");
        dir
    }

    fn text(value: &str) -> ArgValue {
        ArgValue::Text(value.to_string())
    }

    fn call(verb: &str, args: &[(&str, ArgValue)]) -> Call {
        Call {
            verb: verb.to_string(),
            args: args
                .iter()
                .map(|(name, value)| (name.to_string(), value.clone()))
                .collect(),
        }
    }

    fn acted() -> Result<Outcome, Refusal> {
        Ok(Outcome::Acted {
            settled: true,
            busy: None,
            diff: SnapshotDiff {
                added: Vec::new(),
                removed: Vec::new(),
                changed: Vec::new(),
            },
        })
    }

    fn mode(path: &Path) -> u32 {
        fs::metadata(path)
            .expect("the path is there")
            .permissions()
            .mode()
            & 0o777
    }

    #[test]
    fn the_digest_is_fnv_1a_64_of_the_written_answer() {
        let rows: [(&[u8], u64); 3] = [
            (b"", 0xcbf2_9ce4_8422_2325),
            (b"a", 0xaf63_dc4c_8601_ec8c),
            (b"foobar", 0x8594_4171_f739_67e8),
        ];
        assert_eq!(rows.len(), 3);
        for (row, (bytes, digest)) in rows.into_iter().enumerate() {
            assert!(fnv1a(bytes) == digest, "row {row}");
        }
        assert_eq!(hex(fnv1a(b"")), "cbf29ce484222325");
        assert_eq!(hex(0), "0000000000000000");
        let refused = Err(Refusal::new(Cause::NotFound));
        assert_eq!(
            answer_digest(&refused),
            fnv1a(Refusal::new(Cause::NotFound).to_json().as_bytes())
        );
        let shown = Ok(Outcome::Screen {
            text: "Button \"Go\" id=\"go\"\n".to_string(),
        });
        let written = Outcome::Screen {
            text: "Button \"Go\" id=\"go\"\n".to_string(),
        }
        .to_json();
        assert_eq!(answer_digest(&shown), fnv1a(written.as_bytes()));
        assert!(answer_digest(&shown) != answer_digest(&refused));
    }

    #[test]
    fn a_recording_reads_back_as_the_calls_it_kept() {
        let dir = scratch("round-trip");
        let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
        let not_found = Err(Refusal::new(Cause::NotFound));
        let calls = [
            (call("snapshot", &[]), false, acted()),
            (call("click", &[("id", text("increment"))]), false, acted()),
            (
                call(
                    "type",
                    &[("text", text("Ada \"L\"\n")), ("id", text("who"))],
                ),
                false,
                acted(),
            ),
            (
                call(
                    "press",
                    &[("shift", ArgValue::Flag(true)), ("key", text("tab"))],
                ),
                false,
                acted(),
            ),
            (
                call("advance", &[("ms", ArgValue::Number(250))]),
                false,
                acted(),
            ),
            (
                call("advance", &[("ms", ArgValue::Number(-1))]),
                false,
                acted(),
            ),
            (call("scroll", &[("id", text("row-14"))]), false, acted()),
            (call("screenshot", &[]), false, acted()),
            (
                call("click", &[("id", ArgValue::Number(7))]),
                false,
                not_found,
            ),
        ];
        assert_eq!(calls.len(), 9);
        for (row, (call, masked, answer)) in calls.iter().enumerate() {
            assert!(recording.record(call, *masked, answer), "row {row}");
        }
        let file = dir.join("check-1.json");
        let bytes = fs::read(&file).expect("the check is written");
        let check = Check::from_bytes(&bytes).expect("the check reads back");
        assert_eq!(check.app(), "counter");
        assert_eq!(check.steps().len(), calls.len());
        for (row, (step, (call, _, answer))) in check.steps().iter().zip(&calls).enumerate() {
            // A call's arguments are kept in the schema's order.
            let verb = schema::verb(&call.verb).expect("a verb of the table");
            let mut ordered = call.args.clone();
            ordered.sort_by_key(|(name, _)| verb.args.iter().position(|arg| arg.name == name));
            assert!(step.call().verb == call.verb, "row {row}");
            assert!(step.call().args == ordered, "row {row}");
            assert!(step.accepted() == answer.is_ok(), "row {row}");
            assert!(step.digest() == answer_digest(answer), "row {row}");
            assert!(!step.masked(), "row {row}");
        }
        let text = String::from_utf8(bytes).expect("the check is text");
        assert!(text.starts_with("[\n{\"tool\":\"start\",\"arguments\":{\"app\":\"counter\"}},\n"));
        assert!(text.contains(
            r#"{"tool":"press","arguments":{"key":"tab","shift":true},"accepted":true,"digest":""#
        ));
        assert!(text.ends_with("}\n]\n"));
        assert_eq!(text.lines().count(), calls.len() + 3);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_masked_type_keeps_the_stand_in_and_nothing_of_the_text() {
        let dir = scratch("masked");
        let secret = "synthetic-pw-9Kx";
        let mut recording = Recording::create(&dir, "login").expect("the recording starts");
        let typed = call("type", &[("id", text("password")), ("text", text(secret))]);
        assert!(recording.record(&typed, true, &acted()));
        // Only a `type` is masked.
        assert!(recording.record(&call("click", &[("id", text("go"))]), true, &acted()));
        let bytes = fs::read(dir.join("check-1.json")).expect("the check is written");
        let written = String::from_utf8(bytes.clone()).expect("the check is text");
        assert!(!written.contains(secret));
        assert!(written.contains(&format!(
            r#"{{"tool":"type","arguments":{{"id":"password","text":"{MASKED_STAND_IN}"}},"masked":true,"accepted":true,"digest":""#
        )));
        // A secret of another length leaves the same bytes: no length is kept.
        let other = scratch("masked-other");
        let mut recording = Recording::create(&other, "login").expect("the recording starts");
        let longer = call(
            "type",
            &[("id", text("password")), ("text", text(&secret.repeat(3)))],
        );
        assert!(recording.record(&longer, true, &acted()));
        assert!(recording.record(&call("click", &[("id", text("go"))]), true, &acted()));
        assert_eq!(
            fs::read(other.join("check-1.json")).ok(),
            Some(bytes.clone())
        );
        let _ = fs::remove_dir_all(&other);
        let check = Check::from_bytes(&bytes).expect("the check reads back");
        let [masked, clicked] = check.steps() else {
            panic!("two steps are kept");
        };
        assert!(masked.masked() && !clicked.masked());
        assert_eq!(
            masked.call().args,
            vec![
                ("id".to_string(), text("password")),
                ("text".to_string(), text(MASKED_STAND_IN))
            ]
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_call_no_record_can_spell_is_not_kept() {
        let dir = scratch("unkept");
        let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
        let rows = [
            call("capture", &[]),
            call("start", &[("app", text("counter"))]),
            call("stop", &[]),
            call("help", &[]),
            call("click", &[("target", text("save"))]),
            call("click", &[("id", text("a")), ("id", text("b"))]),
        ];
        assert_eq!(rows.len(), 6);
        let refused = Err(Refusal::new(Cause::UnknownVerb));
        for (row, unkept) in rows.iter().enumerate() {
            assert!(recording.record(unkept, false, &refused), "row {row}");
        }
        let bytes = fs::read(dir.join("check-1.json")).expect("the check is written");
        let check = Check::from_bytes(&bytes).expect("the check reads back");
        assert!(check.steps().is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_check_and_its_directory_are_owner_only_and_no_file_is_replaced() {
        let dir = scratch("modes");
        fs::create_dir(&dir).expect("the directory is created");
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).expect("made private");
        fs::write(dir.join("check-1.json"), "kept").expect("a file of another session");
        fs::write(dir.join("check-3.json"), "kept").expect("a file of another session");
        let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
        let file = dir.join("check-2.json");
        assert_eq!(mode(&file), 0o600);
        assert!(recording.record(&call("snapshot", &[]), false, &acted()));
        // The rewrite went through its temporary file, which is gone, and kept the mode.
        assert_eq!(mode(&file), 0o600);
        assert!(!dir.join(".check-2.json.tmp").exists());
        assert_eq!(
            fs::read_to_string(dir.join("check-1.json")).ok().as_deref(),
            Some("kept")
        );
        assert_eq!(
            fs::read_to_string(dir.join("check-3.json")).ok().as_deref(),
            Some("kept")
        );
        let next = Recording::create(&dir, "counter").expect("a second recording starts");
        assert!(dir.join("check-4.json").exists());
        drop((recording, next));

        let created = scratch("created");
        let _recording = Recording::create(&created, "counter").expect("the recording starts");
        assert_eq!(mode(&created), 0o700);
        assert_eq!(mode(&created.join("check-1.json")), 0o600);

        let open = scratch("open");
        fs::create_dir(&open).expect("the directory is created");
        for open_mode in [0o750, 0o705, 0o777] {
            fs::set_permissions(&open, fs::Permissions::from_mode(open_mode)).expect("opened");
            assert!(matches!(
                Recording::create(&open, "counter"),
                Err(SessionError::RecordDirNotPrivate)
            ));
            assert_eq!(fs::read_dir(&open).expect("listed").count(), 0);
        }
        for dir in [dir, created, open] {
            let _ = fs::remove_dir_all(&dir);
        }
    }

    #[test]
    fn a_recording_that_would_pass_its_bound_or_cannot_write_ends_and_removes_its_check() {
        let dir = scratch("bound");
        let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
        let snapshot = call("snapshot", &[]);
        for row in 1..MAX_CHECK_STEPS {
            assert!(recording.record(&snapshot, false, &acted()), "row {row}");
        }
        let file = dir.join("check-1.json");
        let bytes = fs::read(&file).expect("the check is written");
        assert_eq!(
            Check::from_bytes(&bytes).map(|check| check.steps().len()),
            Ok(MAX_CHECK_STEPS - 1)
        );
        assert!(!recording.record(&snapshot, false, &acted()));
        assert!(!file.exists());
        assert_eq!(fs::read_dir(&dir).expect("listed").count(), 0);

        let blocked = scratch("blocked");
        let mut recording = Recording::create(&blocked, "counter").expect("the recording starts");
        fs::write(blocked.join(".check-1.json.tmp"), "kept").expect("the temporary path is taken");
        assert!(!recording.record(&snapshot, false, &acted()));
        // The check is gone, and the file the recording did not create is left as it was.
        assert!(!blocked.join("check-1.json").exists());
        assert_eq!(
            fs::read_to_string(blocked.join(".check-1.json.tmp"))
                .ok()
                .as_deref(),
            Some("kept")
        );
        assert_eq!(fs::read_dir(&blocked).expect("listed").count(), 1);

        let wide = scratch("wide");
        let mut recording = Recording::create(&wide, "counter").expect("the recording starts");
        let widest = call(
            "type",
            &[
                ("id", text(&"i".repeat(1024))),
                ("text", text(&"\n".repeat(4096))),
            ],
        );
        let mut kept = 0;
        while recording.record(&widest, false, &acted()) {
            kept += 1;
        }
        assert!(kept > 0 && kept < MAX_CHECK_STEPS);
        assert_eq!(fs::read_dir(&wide).expect("listed").count(), 0);
        for dir in [dir, blocked, wide] {
            let _ = fs::remove_dir_all(&dir);
        }
    }

    const OPENING: &str = r#"{"tool":"start","arguments":{"app":"counter"}}"#;
    const CLICK: &str =
        r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef"}"#;

    fn check(records: &[&str]) -> Vec<u8> {
        format!("[{}]", records.join(",")).into_bytes()
    }

    #[test]
    fn a_check_in_the_grammar_is_read() {
        let rows = [
            check(&[OPENING]),
            check(&[OPENING, CLICK]),
            check(&[
                OPENING,
                r#"{"digest":"ffffffffffffffff","accepted":false,"arguments":{"text":"masked","id":"p"},"masked":true,"tool":"type"}"#,
            ]),
            check(&[
                OPENING,
                r#"{"tool":"advance","arguments":{"ms":0},"accepted":false,"digest":"0000000000000000"}"#,
            ]),
            check(&[
                OPENING,
                r#"{"tool":"screenshot","arguments":{},"accepted":true,"digest":"0123456789abcdef"}"#,
            ]),
            format!(" [ {OPENING} ,\n {CLICK} ] \n").into_bytes(),
        ];
        assert_eq!(rows.len(), 6);
        for (row, bytes) in rows.iter().enumerate() {
            assert!(Check::from_bytes(bytes).is_ok(), "row {row}");
        }
        let read = Check::from_bytes(&rows[2]).expect("read");
        let [step] = read.steps() else {
            panic!("one step");
        };
        assert!(step.masked() && !step.accepted() && step.digest() == u64::MAX);
    }

    #[test]
    fn a_check_outside_the_grammar_is_refused_as_malformed() {
        let step = |record: &str| check(&[OPENING, record]);
        let rows = [
            b"".to_vec(),
            b"nope".to_vec(),
            b"{}".to_vec(),
            b"[]".to_vec(),
            format!("[{OPENING}").into_bytes(),
            check(&[CLICK]),
            check(&[r#"{"tool":"start","arguments":{"app":7}}"#]),
            check(&[r#"{"tool":"start","arguments":{"app":"Counter"}}"#]),
            check(&[r#"{"tool":"start","arguments":{"app":"counter","id":"x"}}"#]),
            check(&[r#"{"tool":"start","arguments":{"app":"counter"},"accepted":true}"#]),
            check(&[r#"{"tool":"start"}"#]),
            check(&[r#""start""#]),
            step(r#"{"tool":"stop","arguments":{},"accepted":true,"digest":"0123456789abcdef"}"#),
            step(
                r#"{"tool":"capture","arguments":{},"accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(r#"{"tool":7,"arguments":{},"accepted":true,"digest":"0123456789abcdef"}"#),
            step(r#"{"arguments":{},"accepted":true,"digest":"0123456789abcdef"}"#),
            step(
                r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef","answer":"x"}"#,
            ),
            step(
                r#"{"tool":"click","arguments":{"target":"save"},"accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(
                r#"{"tool":"click","arguments":{"id":null},"accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(
                r#"{"tool":"click","arguments":{"id":["save"]},"accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(
                r#"{"tool":"advance","arguments":{"ms":1.5},"accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(r#"{"tool":"click","arguments":[],"accepted":true,"digest":"0123456789abcdef"}"#),
            step(r#"{"tool":"click","arguments":{"id":"save"},"digest":"0123456789abcdef"}"#),
            step(
                r#"{"tool":"click","arguments":{"id":"save"},"accepted":"true","digest":"0123456789abcdef"}"#,
            ),
            step(r#"{"tool":"click","arguments":{"id":"save"},"accepted":true}"#),
            step(
                r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcde"}"#,
            ),
            step(
                r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef0"}"#,
            ),
            step(
                r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789ABCDEF"}"#,
            ),
            step(
                r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdeg"}"#,
            ),
            step(r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":7}"#),
            step(
                r#"{"tool":"click","arguments":{"id":"save"},"masked":true,"accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(
                r#"{"tool":"type","arguments":{"id":"p","text":"masked"},"masked":false,"accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(
                r#"{"tool":"type","arguments":{"id":"p","text":"masked"},"masked":"true","accepted":true,"digest":"0123456789abcdef"}"#,
            ),
            step(r#""click""#),
            check(&[OPENING, CLICK, "null"]),
        ];
        assert_eq!(rows.len(), 35);
        for (row, bytes) in rows.iter().enumerate() {
            assert!(
                Check::from_bytes(bytes) == Err(Refusal::new(Cause::Malformed)),
                "row {row}"
            );
        }
    }

    #[test]
    fn a_check_at_its_bounds_is_read_and_one_past_them_is_refused() {
        let mut records = vec![OPENING; 1];
        records.extend(std::iter::repeat_n(CLICK, MAX_CHECK_STEPS - 1));
        assert_eq!(records.len(), 400);
        let at_steps = check(&records);
        assert!(at_steps.len() <= MAX_CHECK_BYTES);
        assert_eq!(
            Check::from_bytes(&at_steps).map(|read| read.steps().len()),
            Ok(399)
        );
        records.push(CLICK);
        assert_eq!(
            Check::from_bytes(&check(&records)),
            Err(Refusal::new(Cause::Malformed))
        );

        let mut at_bytes = check(&[OPENING, CLICK]);
        at_bytes.resize(MAX_CHECK_BYTES, b' ');
        assert_eq!(at_bytes.len(), 1_048_576);
        assert!(Check::from_bytes(&at_bytes).is_ok());
        at_bytes.push(b' ');
        assert_eq!(
            Check::from_bytes(&at_bytes),
            Err(Refusal::new(Cause::Malformed))
        );
        assert_eq!((MAX_CHECK_STEPS, MAX_CHECK_BYTES), (400, 1_048_576));
    }
}
