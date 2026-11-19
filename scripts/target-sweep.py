#!/usr/bin/env python3
"""The stale-hash sweep of target/debug.

Cargo mints a new unit hash when a unit's features, its dependencies' versions, its profile or flags, or the rustc
version change, and never collects the superseded ones. This keeps every unit the project's own selections build,
read from cargo's JSON artifact lists, and removes the rest of target/debug's deps/, .fingerprint/, build/ and
incremental/ units. It touches nothing else.

Usage: python3 scripts/target-sweep.py {plan|run}, from the repository root. One JSON line on stdout of closed words
and counts; exit 0 swept or planned, 1 failed, 2 usage, 3 refused.
"""

import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

USAGE = "usage: python3 scripts/target-sweep.py {plan|run}"
DEBUG = Path("target") / "debug"
UNIT_DIRS = ("deps", ".fingerprint", "build", "incremental")
HASH = re.compile(r"-([0-9a-f]{16})(?:\.|$)")
INCREMENTAL = re.compile(r"^(.+)-[0-9a-z]+$")

# Each line is a selection a leg or a harness of this repository builds; the comment names who builds it.
KEEP_RUNS = (
    ["test", "--workspace", "--no-run"],  # legs test, a11y
    ["test", "-p", "blitz-tests", "--no-run"],  # stand-repeat, agent-run boot and run
    ["test", "-p", "blitz-tests", "--features", "blitz-paint/font-embolden", "--test", "select_popup_painted",
     "--no-run"],  # embolden
    ["test", "-p", "seven_guis", "--no-run"],  # stand-repeat, agent-run run session
    ["build", "-p", "seven_guis", "--bin", "escher-session"],  # the cold-agent pipe's precondition
    # The clippy lines carry the leg's own `-D warnings`: clippy tracks its arguments, so a keep run without them
    # would rebuild these units in place, and the next clippy leg rebuild them again.
    ["clippy", "--workspace", "--", "-D", "warnings"],  # clippy line 1
    ["clippy", "-p", "blitz-dom", "--no-default-features", "--features",
     "svg,woff,accessibility,system-fonts,file-input,custom-widget", "--all-targets", "--", "-D",
     "warnings"],  # clippy line 2
)


def remove_path(path):
    if path.is_dir() and not path.is_symlink():
        shutil.rmtree(path)
    else:
        path.unlink()


def cargo_args(run):
    split = run.index("--") if "--" in run else len(run)
    return ["cargo", *run[:split], "--locked", "--message-format=json", *run[split:]]


def keep_set():
    """The unit hashes and the per-crate counts of incremental units the keep runs name, or None if one fails."""
    hashes = set()
    crates = {}
    for run in KEEP_RUNS:
        proc = subprocess.run(cargo_args(run), stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
        if proc.returncode != 0:
            return None
        for line in proc.stdout.splitlines():
            try:
                message = json.loads(line)
            except ValueError:
                continue
            if not isinstance(message, dict):
                continue
            names = []
            if message.get("reason") == "compiler-artifact":
                # A build script's compile unit is named by its directory: build/{package}-{hash}/build-script-build.
                names = [n for f in message.get("filenames") or [] for n in (Path(f).name, Path(f).parent.name)]
                if names and str(message.get("package_id", "")).startswith("path+"):
                    target = message.get("target") or {}
                    crate = str(target.get("name", "")).replace("-", "_")
                    found = {m.group(1) for n in names if (m := HASH.search(n))}
                    crates.setdefault(crate, set()).update(found)
                if message.get("executable"):
                    names.append(Path(message["executable"]).name)
            elif message.get("reason") == "build-script-executed" and message.get("out_dir"):
                names = [Path(message["out_dir"]).parent.name]
            for name in names:
                match = HASH.search(name)
                if match:
                    hashes.add(match.group(1))
    return hashes, {crate: len(found) for crate, found in crates.items()}


def newest_mtime(path):
    newest = path.lstat().st_mtime
    if path.is_dir() and not path.is_symlink():
        for child in path.rglob("*"):
            newest = max(newest, child.lstat().st_mtime)
    return newest


def tree_bytes(path):
    seen = set()
    total = 0
    for child in path.rglob("*"):
        st = child.lstat()
        if (st.st_dev, st.st_ino) in seen:
            continue
        seen.add((st.st_dev, st.st_ino))
        total += st.st_size
    return total


def units(kept, crate_counts, started):
    """The entries to remove, as {unit key: [paths]}, and the count of units kept."""
    by_hash = {}
    for area in ("deps", ".fingerprint", "build"):
        root = DEBUG / area
        if not root.is_dir():
            continue
        for entry in root.iterdir():
            match = HASH.search(entry.name)
            if match:
                by_hash.setdefault(match.group(1), []).append(entry)
    remove = {}
    kept_count = 0
    for unit_hash, entries in by_hash.items():
        fingerprints = [e for e in entries if e.parent.name == ".fingerprint"]
        newest = max(newest_mtime(e) for e in (fingerprints or entries))
        # A unit with no deps/ or build/ entry keeps its artifacts elsewhere (an example under examples/, a doc
        # unit under target/doc), where cargo's JSON names no hash; its fingerprint is left with them.
        outside = len(fingerprints) == len(entries)
        if unit_hash in kept or newest >= started or outside:
            kept_count += 1
        else:
            remove[unit_hash] = entries
    # rustc names an incremental directory by its own crate id, not cargo's hash: per crate, the units the keep runs
    # name keep that many of its directories, the most recently written first.
    incremental = DEBUG / "incremental"
    by_crate = {}
    if incremental.is_dir():
        for entry in incremental.iterdir():
            match = INCREMENTAL.match(entry.name)
            if match and entry.is_dir():
                by_crate.setdefault(match.group(1), []).append((newest_mtime(entry), entry))
    for crate, entries in by_crate.items():
        entries.sort(key=lambda pair: pair[0], reverse=True)
        allowance = crate_counts.get(crate, 0)
        for mtime, entry in entries:
            if allowance > 0 or mtime >= started:
                allowance -= 1
                kept_count += 1
            else:
                remove[f"incremental:{entry.name}"] = [entry]
    return remove, kept_count


def emit(out, line):
    print(json.dumps(line), file=out)


def main(argv, out=sys.stdout):
    if len(argv) != 1 or argv[0] not in ("plan", "run"):
        print(USAGE, file=sys.stderr)
        return 2
    verb = argv[0]
    if os.name != "posix":
        emit(out, {"verb": verb, "outcome": "refused", "cause": "unsupported-host"})
        return 3
    import fcntl

    if not DEBUG.is_dir():
        emit(out, {"verb": verb, "outcome": "refused", "cause": "no-build-directory"})
        return 3
    lock_path = DEBUG / ".cargo-lock"

    def try_lock():
        fd = os.open(lock_path, os.O_RDWR | os.O_CREAT, 0o644)
        try:
            fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError:
            os.close(fd)
            return None
        return fd

    def release(fd):
        fcntl.flock(fd, fcntl.LOCK_UN)
        os.close(fd)

    # Probed before the keep runs too: their own cargo would wait out a build in flight and then read the lock free.
    probe = try_lock()
    if probe is None:
        emit(out, {"verb": verb, "outcome": "refused", "cause": "build-in-flight"})
        return 3
    release(probe)

    clock = time.monotonic()
    started = time.time()
    keep = keep_set()
    if keep is None:
        emit(out, {"verb": verb, "outcome": "failed", "cause": "keep-run"})
        return 1
    kept, crate_counts = keep

    fd = try_lock()
    if fd is None:
        emit(out, {"verb": verb, "outcome": "refused", "cause": "build-in-flight"})
        return 3
    try:
        remove, kept_count = units(kept, crate_counts, started)
        bytes_before = tree_bytes(DEBUG)
        if verb == "run":
            for entries in remove.values():
                for entry in entries:
                    remove_path(entry)
        bytes_after = tree_bytes(DEBUG)
    finally:
        release(fd)
    line = {"verb": verb, "outcome": "swept" if verb == "run" else "planned", "units_kept": kept_count}
    line["units_removed" if verb == "run" else "units_to_remove"] = len(remove)
    line.update(bytes_before=bytes_before, bytes_after=bytes_after, seconds=round(time.monotonic() - clock, 2))
    emit(out, line)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
