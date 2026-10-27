#!/usr/bin/env python3
"""Prepares a release by building and signing the appropriate binaries

Uses `zigbuild` for cross compilation."""

import argparse
import shutil
import subprocess
import sys
from pathlib import Path


def target_extension(target: str):
    return ".exe" if "windows" in target else ""


def binary_name(*, binary: str, target: str, version: str) -> str:
    return f"{binary}-{version}-{target}" + target_extension(target)


SUPPORTED_PLATFORMS = [
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "x86_64-pc-windows-gnu",
]
BINARIES = ["unicodeit", "unicodeit-repl"]


def determine_build_command(target_platform: str):
    if "apple-darwin" in target_platform and sys.platform == "darwin":
        # using zigbuild when compiling for apple targets causes duplicate libraries to be linked
        return "build"
    else:
        return "zigbuild"


def main():
    parser = argparse.ArgumentParser(
        description=__doc__,
    )
    parser.add_argument(
        "--out", help="Where to place the built files", default="staging"
    )
    parser.add_argument(
        "--version", help="The version that is being built", required=True
    )
    parser.add_argument(
        "--no-clean",
        help="Do not run `cargo clean`",
        dest="clean",
        action="store_false",
    )
    args = parser.parse_args()

    out_dir = Path(args.out)
    if args.clean:
        print("Running `cargo clean`")
        subprocess.run(["cargo", "+stable", "clean"], check=True)

    if out_dir.is_dir():
        shutil.rmtree(out_dir)
    out_dir.mkdir(exist_ok=False)

    for target in SUPPORTED_PLATFORMS:
        build_command = determine_build_command(target)
        print(f"Building for {target!r}")
        subprocess.run(
            [
                "cargo",
                "+stable",
                build_command,
                "--target",
                target,
                "--workspace",
                "--release",
            ],
            check=True,
        )
        target_build_dir = Path("target", target, "release")
        assert target_build_dir.is_dir()
        for binary in BINARIES:
            desired_binary = target_build_dir / (binary + target_extension(target))
            assert desired_binary.is_file()
            result_binary_path = out_dir / binary_name(
                target=target, version=args.version, binary=binary
            )
            shutil.copy2(desired_binary, result_binary_path)
            with open(f"{result_binary_path}.sha256sum", "wt") as hashfile:
                subprocess.run(
                    ["sha256sum", result_binary_path],
                    check=True,
                    stdout=hashfile,
                )
            subprocess.run(
                ["gpg", "--armor", "--detach-sign", result_binary_path.name],
                cwd=out_dir,
                check=True,
            )


if __name__ == "__main__":
    main()
