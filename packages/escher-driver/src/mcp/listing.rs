//! The handshake answer and the tool listing: the revision spoken, the server's name and
//! version, and every verb of the table as a tool, with its description and input schema.

use serde_json::Value;

use super::framing::list;
use crate::json::{self, Object};
use crate::schema::{self, ArgKind, KEY_NAMES, MAX_MILLISECONDS, VERBS, VerbSpec};

/// The protocol revision answered when a client asks for one this server does not speak.
const LATEST_VERSION: &str = "2025-11-25";

/// The handshake revisions this server speaks.
const PROTOCOL_VERSIONS: [&str; 4] = [LATEST_VERSION, "2025-06-18", "2025-03-26", "2024-11-05"];

/// The name the server gives itself in its `initialize` answer.
const SERVER_NAME: &str = "escher";

/// A list of strings.
fn strings<'a>(items: impl IntoIterator<Item = &'a str>) -> String {
    list(items.into_iter().map(|item| {
        let mut out = String::new();
        json::string(&mut out, item);
        out
    }))
}

/// What `initialize` answers: the revision spoken, the tools capability and the server's name
/// and version.
pub(super) fn initialized(params: Option<&Value>) -> String {
    let asked = params
        .and_then(|params| params.get("protocolVersion"))
        .and_then(Value::as_str);
    let version = PROTOCOL_VERSIONS
        .into_iter()
        .find(|spoken| Some(*spoken) == asked)
        .unwrap_or(LATEST_VERSION);
    let mut capabilities = Object::new();
    capabilities.json("tools", &Object::new().end());
    let mut info = Object::new();
    info.text("name", SERVER_NAME);
    info.text("version", env!("CARGO_PKG_VERSION"));
    let mut result = Object::new();
    result.text("protocolVersion", version);
    result.json("capabilities", &capabilities.end());
    result.json("serverInfo", &info.end());
    result.end()
}

/// A tool's description: the verb's help, then each of its result fields with its kind and its
/// help, all read from the table.
pub(super) fn description(verb: &VerbSpec) -> String {
    let fields: Vec<String> = verb
        .fields
        .iter()
        .map(|field| format!("{} ({}): {}", field.name, field.kind.name(), field.help))
        .collect();
    format!("{}. Returns: {}", verb.help, fields.join("; "))
}

/// One tool: a verb of the table, described by its help and its result fields, with an input
/// schema built from its arguments.
pub(super) fn tool(verb: &VerbSpec, apps: &[&str]) -> String {
    let mut properties = Object::new();
    for arg in verb.args {
        let mut property = Object::new();
        match arg.kind {
            ArgKind::Id | ArgKind::Text => property.text("type", "string"),
            ArgKind::Name => {
                property.text("type", "string");
                if verb.name == schema::START.name {
                    property.json("enum", &strings(apps.iter().copied()));
                }
            }
            ArgKind::Key => {
                property.text("type", "string");
                property.json("enum", &strings(KEY_NAMES));
            }
            ArgKind::Flag => property.text("type", "boolean"),
            ArgKind::Milliseconds => {
                property.text("type", "integer");
                property.count("minimum", 1);
                property.count("maximum", u64::from(MAX_MILLISECONDS));
            }
        }
        property.text(
            "description",
            &format!("{} ({})", arg.help, arg.kind.bound()),
        );
        properties.json(arg.name, &property.end());
    }
    let mut input_schema = Object::new();
    input_schema.text("type", "object");
    input_schema.json("properties", &properties.end());
    input_schema.json(
        "required",
        &strings(
            verb.args
                .iter()
                .filter(|arg| arg.required)
                .map(|arg| arg.name),
        ),
    );
    input_schema.flag("additionalProperties", false);
    let mut object = Object::new();
    object.text("name", verb.name);
    object.text("description", &description(verb));
    object.json("inputSchema", &input_schema.end());
    object.end()
}

/// What `tools/list` answers: every verb of the table as a tool, in the table's order.
pub(super) fn tool_list(apps: &[&str]) -> String {
    let mut result = Object::new();
    result.json("tools", &list(VERBS.iter().map(|verb| tool(verb, apps))));
    result.end()
}
