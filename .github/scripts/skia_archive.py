#!/usr/bin/env python3
"""Fetch skia-bindings' prebuilt archive for one target once, verified against its recorded sha256.

Usage: skia_archive.py TARGET DIR

Reads the target's archive key and sha256 from `.github/skia-archives.toml`. A copy already at
DIR/skia-binaries-{key}.tar.gz whose sum matches is reused; otherwise the archive is fetched from the
recorded URL into a temporary file in DIR (120 s timeout, up to 3 attempts, 10 s apart) and renamed into
place only when its sum matches. On success prints one line, the SKIA_BINARIES_URL template that names the
verified copy, `file://DIR/skia-binaries-{key}.tar.gz` with `{key}` left for skia-bindings to fill, so a
build asking for another key finds no file. Exits 0 on success, 1 when no verified archive was obtained,
2 on a bad call or a target the record does not hold. Diagnostics are fixed lines on stderr.
"""

import hashlib
import os
import sys
import tempfile
import time
import tomllib
import urllib.request
from pathlib import Path

RECORD = Path(__file__).resolve().parents[1] / "skia-archives.toml"
ATTEMPTS = 3
PAUSE_SECONDS = 10
TIMEOUT_SECONDS = 120
ARCHIVE_NAME = "skia-binaries-{key}.tar.gz"


def load_record(path):
    with open(path, "rb") as file:
        return tomllib.load(file)


def sha256_of(path):
    digest = hashlib.sha256()
    with open(path, "rb") as file:
        for block in iter(lambda: file.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def archive_path(directory, key):
    return Path(directory) / ARCHIVE_NAME.format(key=key)


def url_template(directory):
    """The SKIA_BINARIES_URL naming the verified copy, `{key}` kept for skia-bindings to fill."""
    return "file://" + (Path(directory).resolve() / ARCHIVE_NAME).as_posix()


def fetch_to(url, destination, opener):
    with opener(url, timeout=TIMEOUT_SECONDS) as response, open(destination, "wb") as file:
        while block := response.read(1 << 20):
            file.write(block)


def verified_archive(record, target, directory, opener=urllib.request.urlopen, pause=PAUSE_SECONDS):
    """Return the path of the target's verified archive in `directory`, or None when no attempt verified one."""
    entry = record["targets"][target]
    key, expected = entry["key"], entry["sha256"]
    final = archive_path(directory, key)
    if final.is_file() and sha256_of(final) == expected:
        return final
    Path(directory).mkdir(parents=True, exist_ok=True)
    url = record["url"].format(tag=record["tag"], key=key)
    for attempt in range(1, ATTEMPTS + 1):
        if attempt > 1:
            time.sleep(pause)
        handle, partial = tempfile.mkstemp(dir=directory, prefix=".skia-archive-", suffix=".part")
        os.close(handle)
        try:
            fetch_to(url, partial, opener)
        except Exception:
            os.remove(partial)
            print(f"skia-archive: attempt {attempt} of {ATTEMPTS} failed in fetch", file=sys.stderr)
            continue
        if sha256_of(partial) != expected:
            os.remove(partial)
            print(f"skia-archive: attempt {attempt} of {ATTEMPTS} failed in verify", file=sys.stderr)
            continue
        os.replace(partial, final)
        return final
    return None


def main(argv, record_path=RECORD, opener=urllib.request.urlopen, pause=PAUSE_SECONDS):
    if len(argv) != 2:
        print("skia-archive: usage", file=sys.stderr)
        return 2
    target, directory = argv
    record = load_record(record_path)
    if target not in record.get("targets", {}):
        print("skia-archive: no recorded archive for this target", file=sys.stderr)
        return 2
    if verified_archive(record, target, directory, opener, pause) is None:
        print("skia-archive: no verified archive", file=sys.stderr)
        return 1
    print(url_template(directory))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
