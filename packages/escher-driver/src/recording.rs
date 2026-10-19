//! A session kept as a replayable check: the calls as they crossed in, each with whether it was
//! accepted, a digest of the answer the driver wrote for it and that answer itself.
//!
//! A check is one JSON array. Its first record names the app the session was started on; every
//! later record is one call the session answered: its verb, its arguments by their schema names,
//! `masked` on a `type` whose text was not kept, `accepted`, `digest` and `answer`, the answer's
//! line as the driver wrote it — masked as on every carrier, and a shot's without its pixels.
//! The text typed into a masked control is [`MASKED_STAND_IN`]. The form is written here by the
//! crate's own JSON writer and read back with `serde_json`, strictly and under its bounds.
//! Nothing here logs.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use crate::command::{ArgValue, Call, call_of};
use crate::error::SessionError;
use crate::execute::Outcome;
use crate::json::{Object, shot_without_image};
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
/// It tells one answer from another, a shot's pixels included: a check keeps it beside the
/// answer, which it keeps whole but for a shot's image.
pub fn answer_digest(answer: &Result<Outcome, Refusal>) -> u64 {
    fnv1a(written(answer).as_bytes())
}

/// The line the driver writes for an answer.
fn written(answer: &Result<Outcome, Refusal>) -> String {
    match answer {
        Ok(outcome) => outcome.to_json(),
        Err(refusal) => refusal.to_json(),
    }
}

/// The line a check keeps for an answer: the written line, a shot's without its image.
pub(crate) fn kept_line(answer: &Result<Outcome, Refusal>) -> String {
    match answer {
        Ok(Outcome::Shot { width, height, .. }) => shot_without_image(*width, *height),
        answer => written(answer),
    }
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
/// checks them in, so a replay meets every rule as the call did, then its answer's acceptance,
/// digest and kept line, the line inlined as the object it is.
fn record_of(
    verb: &VerbSpec,
    call: &Call,
    masked: bool,
    answer: &Result<Outcome, Refusal>,
) -> String {
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
    record.flag("accepted", answer.is_ok());
    record.text("digest", &hex(answer_digest(answer)));
    record.json("answer", &kept_line(answer));
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
        self.records.push(record_of(verb, call, masked, answer));
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
    answer: Value,
}

impl Check {
    /// Reads a check from its bytes, or refuses it as [`Cause::Malformed`], holding nothing of
    /// the bytes.
    ///
    /// The bytes are at most 1,048,576 and are one JSON array of at most 400 records. The first
    /// is `{"tool":"start","arguments":{"app":…}}`, the app a session label. Every later one has
    /// the keys `tool`, an instance verb of the table; `arguments`, an object of the verb's
    /// arguments by their schema names, each text, a whole number or a flag; `accepted`, a flag;
    /// `digest`, 16 lowercase hex digits; `answer`, an object, the answer's line as the driver
    /// wrote it, a shot's without its image; and on a `type` alone, `masked`, true. Anything else
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

    /// The kept answer, as read: an object.
    pub(crate) fn answer(&self) -> &Value {
        &self.answer
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
        &[
            "tool",
            "arguments",
            "masked",
            "accepted",
            "digest",
            "answer",
        ],
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
    let answer = record.get("answer").filter(|answer| answer.is_object())?;
    Some(Step {
        call,
        accepted,
        digest: u64::from_str_radix(digest, 16).ok()?,
        masked,
        answer: answer.clone(),
    })
}

#[cfg(all(test, not(unix)))]
#[path = "recording_off_unix_tests.rs"]
mod off_unix_tests;

#[cfg(all(test, unix))]
#[path = "recording_tests.rs"]
mod tests;
