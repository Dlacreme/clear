#[macro_use] extern crate pest_derive;

use clap::{App, Arg};

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[macro_use] mod log;
mod config;
mod clear;
mod parser;
mod ast;
mod gen;

fn main() {
    let mut app = App::new(PACKAGE_NAME)
        .version(PACKAGE_VERSION)
        .author(PACKAGE_AUTHORS)
        .about("Clear language compiler")
        .arg(Arg::with_name("version").short("v").long("version").help("Display version"))
        .arg(Arg::with_name("log").short("l").long("log").help("Display all the logs"))
        .arg(Arg::with_name("input_filename").help("Clear input file").index(1));

    let matches = app.clone().get_matches();

    match matches.occurrences_of("version") {
        0 => (),
        _ => {
            println!("Clear compiler v{}", PACKAGE_VERSION);
            std::process::exit(1);
        },
    }

    match matches.occurrences_of("log") {
        0 => (),
        _ => log::enable_info_log(true) ,
    };

    if let Some(input_filename) = matches.value_of("input_filename") {
        clear::build_app(config::Config::load_from_file(input_filename)
            .expect("Fail to read config file")); // TODO: provide clear explanation about the failture
    } else {
        app.print_help().unwrap();
    }

}
