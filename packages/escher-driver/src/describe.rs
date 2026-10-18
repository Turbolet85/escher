//! The tool's description of itself: the verb table, the node keys, the refusal causes, the
//! exit statuses and the session errors, written as one document from the table alone.

use crate::error::SESSION_ERRORS;
use crate::json::{self, Object};
use crate::refusal::CAUSES;
use crate::schema::{
    ArgSpec, BUSY_CLASSES, EXIT_STATUSES, FieldSpec, HELP, KEY_NAMES, NODE_FIELDS, VERBS, VerbSpec,
};

/// The tool's description of itself, as one line of JSON: the answer to `help`.
///
/// Its keys are `help`'s result fields, in their order: `verbs`, each verb of [`VERBS`] with its
/// level, help, arguments (kind, bound, required, help) and result fields (kind, always, help);
/// `node_keys`, each key of [`NODE_FIELDS`] with its help; `key_names` and `busy_classes`;
/// `causes`, each [`Cause`](crate::Cause) with its meaning and remedy; `exit_statuses`, each row
/// of [`EXIT_STATUSES`]; and `session_errors`, each [`SessionError`](crate::SessionError)'s kind
/// with its meaning and remedy. Every text in it is a fixed string of the table, so the document
/// is the same on every call and holds nothing a caller supplied.
pub fn describe() -> String {
    let values = [
        json::list(VERBS.iter().map(verb)),
        json::list(NODE_FIELDS.iter().map(|key| {
            let mut object = Object::new();
            object.text("name", key.name);
            object.text("help", key.help);
            object.end()
        })),
        words(&KEY_NAMES),
        words(&BUSY_CLASSES),
        json::list(CAUSES.iter().map(|cause| {
            let mut object = Object::new();
            object.text("name", cause.name());
            object.text("meaning", cause.meaning());
            object.text("remedy", cause.remedy());
            object.end()
        })),
        json::list(EXIT_STATUSES.iter().map(|ending| {
            let mut object = Object::new();
            object.count("status", u64::from(ending.status));
            object.text("name", ending.name);
            object.text("meaning", ending.meaning);
            object.end()
        })),
        json::list(SESSION_ERRORS.iter().map(|error| {
            let mut object = Object::new();
            object.text("kind", error.kind);
            object.text("meaning", error.meaning);
            object.text("remedy", error.remedy);
            object.end()
        })),
    ];
    let mut document = Object::new();
    for (field, value) in HELP.fields.iter().zip(values) {
        document.json(field.name, &value);
    }
    document.end()
}

fn words(items: &[&str]) -> String {
    json::list(items.iter().map(|item| {
        let mut text = String::new();
        json::string(&mut text, item);
        text
    }))
}

fn verb(verb: &VerbSpec) -> String {
    let mut object = Object::new();
    object.text("name", verb.name);
    object.text("level", verb.level.name());
    object.text("help", verb.help);
    object.json("args", &json::list(verb.args.iter().map(arg)));
    object.json("fields", &json::list(verb.fields.iter().map(field)));
    object.end()
}

fn arg(arg: &ArgSpec) -> String {
    let mut object = Object::new();
    object.text("name", arg.name);
    object.text("kind", arg.kind.name());
    object.text("bound", arg.kind.bound());
    object.flag("required", arg.required);
    object.text("help", arg.help);
    object.end()
}

fn field(field: &FieldSpec) -> String {
    let mut object = Object::new();
    object.text("name", field.name);
    object.text("kind", field.kind.name());
    object.flag("always", field.always);
    object.text("help", field.help);
    object.end()
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use crate::wire::MAX_ANSWER_BYTES;

    fn document() -> serde_json::Map<String, Value> {
        let Ok(Value::Object(document)) = serde_json::from_str::<Value>(&describe()) else {
            panic!("the document is an object");
        };
        document
    }

    fn entries<'d>(document: &'d serde_json::Map<String, Value>, key: &str) -> &'d Vec<Value> {
        let Some(Value::Array(entries)) = document.get(key) else {
            panic!("the document holds a list under each of its keys");
        };
        entries
    }

    #[test]
    fn the_document_holds_helps_seven_fields_in_order() {
        let text = describe();
        let keys: Vec<&str> = HELP.fields.iter().map(|field| field.name).collect();
        assert_eq!(
            keys,
            [
                "verbs",
                "node_keys",
                "key_names",
                "busy_classes",
                "causes",
                "exit_statuses",
                "session_errors",
            ]
        );
        let document = document();
        assert_eq!(document.len(), 7);
        let at: Vec<usize> = keys
            .iter()
            .map(|key| {
                text.find(&format!("\"{key}\":["))
                    .expect("each key is written")
            })
            .collect();
        assert!(at.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(text.starts_with("{\"verbs\":["));
        let counts = [11, 13, 12, 3, 8, 4, 13];
        for (row, (key, count)) in keys.iter().zip(counts).enumerate() {
            assert!(entries(&document, key).len() == count, "row {row}");
        }
        // The image's field kind and the eighth cause, with its remedy.
        assert!(text.contains(r#"{"name":"png","kind":"png","always":true,"#));
        assert!(text.contains(
            r#"{"name":"fonts-unavailable","meaning":"the app this session holds has no font it can paint text with","remedy":"boot the app with a font it can decode"}"#
        ));
        assert!(!text.contains('\n') && !text.contains('\u{1b}'));
        assert!(text.len() <= MAX_ANSWER_BYTES);
        assert_eq!(describe(), text);
    }

    #[test]
    fn every_text_in_it_is_its_tables() {
        let document = document();
        let verbs = entries(&document, "verbs");
        assert_eq!(verbs.len(), VERBS.len());
        for (row, (listed, verb)) in verbs.iter().zip(VERBS).enumerate() {
            assert!(listed["name"] == verb.name, "row {row}");
            assert!(listed["level"] == verb.level.name(), "row {row}");
            assert!(listed["help"] == verb.help, "row {row}");
            let Value::Array(args) = &listed["args"] else {
                panic!("row {row}: a verb lists its arguments");
            };
            assert!(args.len() == verb.args.len(), "row {row}");
            for (listed, arg) in args.iter().zip(verb.args) {
                assert!(listed["name"] == arg.name, "row {row}");
                assert!(listed["kind"] == arg.kind.name(), "row {row}");
                assert!(listed["bound"] == arg.kind.bound(), "row {row}");
                assert!(listed["required"] == arg.required, "row {row}");
                assert!(listed["help"] == arg.help, "row {row}");
            }
            let Value::Array(fields) = &listed["fields"] else {
                panic!("row {row}: a verb lists its result fields");
            };
            assert!(fields.len() == verb.fields.len(), "row {row}");
            for (listed, field) in fields.iter().zip(verb.fields) {
                assert!(listed["name"] == field.name, "row {row}");
                assert!(listed["kind"] == field.kind.name(), "row {row}");
                assert!(listed["always"] == field.always, "row {row}");
                assert!(listed["help"] == field.help, "row {row}");
            }
        }
        for (row, (listed, key)) in entries(&document, "node_keys")
            .iter()
            .zip(&NODE_FIELDS)
            .enumerate()
        {
            assert!(
                listed["name"] == key.name && listed["help"] == key.help,
                "row {row}"
            );
        }
        assert!(document["key_names"] == Value::from(KEY_NAMES.to_vec()));
        assert!(document["busy_classes"] == Value::from(BUSY_CLASSES.to_vec()));
        for (row, (listed, cause)) in entries(&document, "causes").iter().zip(CAUSES).enumerate() {
            assert!(listed["name"] == cause.name(), "row {row}");
            assert!(listed["meaning"] == cause.meaning(), "row {row}");
            assert!(listed["remedy"] == cause.remedy(), "row {row}");
        }
        for (row, (listed, ending)) in entries(&document, "exit_statuses")
            .iter()
            .zip(&EXIT_STATUSES)
            .enumerate()
        {
            assert!(listed["status"] == ending.status, "row {row}");
            assert!(listed["name"] == ending.name, "row {row}");
            assert!(listed["meaning"] == ending.meaning, "row {row}");
        }
        for (row, (listed, error)) in entries(&document, "session_errors")
            .iter()
            .zip(&SESSION_ERRORS)
            .enumerate()
        {
            assert!(listed["kind"] == error.kind, "row {row}");
            assert!(listed["meaning"] == error.meaning, "row {row}");
            assert!(listed["remedy"] == error.remedy, "row {row}");
            assert!(listed.get("repeats").is_none(), "row {row}");
        }
    }
}
