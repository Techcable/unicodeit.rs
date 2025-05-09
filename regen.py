# SPDX-License-Identifier: MIT
#
# Regenerates the datafile for unicodeit.rs
#
# Relies on the upstream `unicodeit` project to be
# kept as a submodule

import contextlib
import operator
import sys
from pathlib import Path


def regenerate_data(base_dir=Path(__file__).parent):
    data_module = base_dir / "unicodeit/unicodeit/data.py"
    output_file = base_dir / "src/data.rs"
    if not data_module.is_file():
        raise AssertionError(f"Missing data module: {data_module}")
    sys.path.append(str(data_module.parent.parent))

    import unicodeit
    import unicodeit.data

    with (
        open(output_file, "wt") as outf,
        contextlib.redirect_stdout(outf),
    ):
        print("// WARNING: DO NOT MANUALLY EDIT")
        print("// This file has been automatically generated by regen.py")
        print(f"// unicodeit.__version__: {unicodeit.__version__}")
        print("//")
        print("// SPDX-License-Identifier: MIT AND LPPL-1.3c")
        print("#![allow(dead_code)]")
        print()
        print("/// The version of the upstream [unicodeit project]")
        print("/// where the library's data has been generated from.")
        print("///")
        print("/// [unicodeit project]: https://github.com/svenkreiss/unicodeit")
        print(f'pub const UNICODEIT_VERSION: &str = "{unicodeit.__version__}";')

        def escape_chars(c: str):
            if len(c) > 1:
                return "".join(map(escape_chars, c))
            elif c in ("\\", '"', "'"):
                return f"\\{c}"
            elif c.isprintable() and c.isascii():
                return c
            else:
                return f"\\u{{{hex(ord(c)).lstrip('0x')}}}"

        def print_table(
            name, data, *, element_type="(&str, &str)", handle_element=None
        ):
            print()
            print("#[rustfmt::skip]")
            print(f"pub const {name}: &[{element_type}] = &[")

            for element in data:
                if handle_element is not None:
                    print(" " * 4 + handle_element(element) + ",")
                else:
                    key, val = element
                    print(" " * 4 + f'(r##"{key}"##, "{escape_chars(val)}"),')
            print("];")

        target_tables = {
            "REPLACEMENTS": unicodeit.data.REPLACEMENTS,
            "COMBINING_MARKS": unicodeit.data.COMBININGMARKS,
            "SUB_SUPER_SCRIPTS": unicodeit.data.SUBSUPERSCRIPTS,
        }
        for name, data in target_tables.items():
            print_table(name, data)

        # used in replace.rs
        print_table(
            "REPLACEMENTS_WITH_BRACKET_SUFFIX",
            [
                (key, rep)
                for key, rep in unicodeit.data.REPLACEMENTS
                if key.endswith("{}")
            ],
        )
        print_table(
            "COMBINING_MARKS_ESCAPED_LATEX",
            map(operator.itemgetter(0), unicodeit.data.COMBININGMARKS),
            element_type="&str",
            handle_element=lambda element: f'r##"\\ {element[1:]}{{"##',
        )


if __name__ == "__main__":
    regenerate_data()
