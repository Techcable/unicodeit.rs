set working-directory := "."

test: _check && format-check
    # Running rust tests
    cargo nextest run --all --all-features

check: _check && format-check


_check:
    # Checking for compilation errors
    cargo clippy --all --all-features
    # Checking documentation
    cargo doc --quiet --no-deps --workspace
    # Checking python scripts
    ruff check --quiet --exclude ./unicodeit

format-check:
    # Checking formatting
    cargo fmt --check
    ruff format --quiet --check --exclude ./unicodeit/
    ruff check --quiet --select I --exclude ./unicodeit/

format:
    # Formatting Rust code
    cargo fmt
    # Formatting Python scripts
    ruff format --exclude ./unicodeit
    ruff check --select I --fix --exclude ./unicodeit/

# Regenerate the data file
regen:
    python3 regen.py
    rustfmt --check -- src/data.rs

# Run the upstream code
upstream arg:
    PYTHONPATH="./unicodeit" python3 -m unicodeit.cli '{{arg}}'
