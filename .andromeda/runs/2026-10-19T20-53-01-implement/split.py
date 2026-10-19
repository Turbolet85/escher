"""Moves the three inline test modules of escher-driver's cli.rs and recording.rs to sibling files."""

import sys
from pathlib import Path

SRC = Path("packages/escher-driver/src")

CLI_DOC = """//! Drives the command line's argv reader against the verb table: each line read by its rules in
//! order, the usage errors, each ending's streams and status, the usage line, and the `mcp` role
//! and `--record` readers. A child module of `cli`, so it reads the reader's private items and its
//! tests keep their `cli::tests` paths.
"""

REC_DOC = """//! Drives a session recording and its kept check: the answer's digest, the records kept and read
//! back, a masked `type`'s stand-in, the owner-only check and directory, the bound and the failed
//! write that end a recording, and the strict reader's grammar and bounds. A child module of
//! `recording`, so it reads the module's private items and its tests keep their
//! `recording::tests` paths.
"""

OFF_UNIX_DOC = """//! Off unix a recording is refused as `Unsupported`, its directory never created. A child module
//! of `recording`, so its test keeps its `recording::off_unix_tests` path.
"""


def dedent(lines):
    out = []
    for line in lines:
        if line == "":
            out.append(line)
        elif line.startswith("    "):
            out.append(line[4:])
        else:
            sys.exit(f"not indented: {line!r}")
    return out


def body(lines, first, last):
    # 1-based, inclusive
    return dedent(lines[first - 1 : last])


def write(path, doc, lines):
    path.write_text(doc + "\n" + "\n".join(lines) + "\n", encoding="utf-8", newline="\n")


cli_path = SRC / "cli.rs"
cli = cli_path.read_text(encoding="utf-8").split("\n")
assert cli[475] == "#[cfg(test)]" and cli[476] == "mod tests {" and cli[1031] == "}", "cli spans moved"
write(SRC / "cli_tests.rs", CLI_DOC, body(cli, 478, 1031))
cli_head = cli[:475]
cli_path.write_text(
    "\n".join(cli_head + ["#[cfg(test)]", '#[path = "cli_tests.rs"]', "mod tests;"]) + "\n",
    encoding="utf-8",
    newline="\n",
)

rec_path = SRC / "recording.rs"
rec = rec_path.read_text(encoding="utf-8").split("\n")
assert rec[452] == "#[cfg(all(test, not(unix)))]" and rec[453] == "mod off_unix_tests {" and rec[466] == "}"
assert rec[468] == "#[cfg(all(test, unix))]" and rec[469] == "mod tests {" and rec[980] == "}"
write(SRC / "recording_off_unix_tests.rs", OFF_UNIX_DOC, body(rec, 455, 466))
write(SRC / "recording_tests.rs", REC_DOC, body(rec, 471, 980))
rec_head = rec[:452]
rec_path.write_text(
    "\n".join(
        rec_head
        + [
            "#[cfg(all(test, not(unix)))]",
            '#[path = "recording_off_unix_tests.rs"]',
            "mod off_unix_tests;",
            "",
            "#[cfg(all(test, unix))]",
            '#[path = "recording_tests.rs"]',
            "mod tests;",
        ]
    )
    + "\n",
    encoding="utf-8",
    newline="\n",
)
print("split done")
