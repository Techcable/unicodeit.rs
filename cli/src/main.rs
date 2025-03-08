use clap::Arg;

pub fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("latex")
                .required(true)
                .help("The latex code to convert to unicode"),
        )
        .get_matches();
    let latex = matches
        .get_one::<String>("latex")
        .expect("Must have `latex` arg");
    println!("{}", unicodeit::replace(latex))
}
