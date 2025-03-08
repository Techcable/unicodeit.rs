# unicodeit.rs
A rust port of [unicodeit](https://www.unicodeit.net/), with a fast CLI.

Install the CLI with `cargo install unicodeit-cli` or `cargo binstall unicodeit-cli` (using [cargo-binstall] avoids compiling from source).

Available as a library through the [`unicodeit` crate](https://docs.rs/unicodeit).

[cargo-binstall]: https://github.com/cargo-bins/cargo-binstall

## Building
Building the project only requires `cargo`.

The repository comes with pre-generated data files. To regenerate these, use the Python project in the 'regen' directory using [`uv build`](https://docs.astral.sh/uv/).

## License
The project is licensed under a combination of the MIT License and the LaTeX Public License. This is exactly the same terms as the upstream [unicodeit project][unicodeit]. See `LICENSE.md` for details.

[unicodeit]: https://www.unicodeit.net