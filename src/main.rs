#[macro_use]
extern crate clap;
use clap::{App, Arg};
use lib::api;

fn main() {
    let matches = App::new("Dumb password generator")
        .version("0.0.1")
        .about("Generates random password")
        .author(crate_authors!())
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .help("password size")
            .default_value("8")
        )
        .arg(Arg::with_name("skip-digits")
            .long("skip-digits")
            .help("Ignore digits (0-9)")
            .multiple(true)
        )
        .arg(Arg::with_name("skip-special")
            .long("skip-special")
            .help("Ignore special chars")
            .multiple(true)
        )
        .get_matches();
    
    let mut opts = api::Options::default();
    opts.size = value_t!(matches, "size", usize).unwrap_or(8);
    if matches.is_present("skip-digits") {
        opts.digits = false;
    }
    if matches.is_present("skip-special") {
        opts.special = false;
    }

    let password = api::gen(opts);
    println!("{}", password);
}