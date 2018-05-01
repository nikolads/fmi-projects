extern crate clap;

use clap::{App, Arg, AppSettings, ArgMatches};

pub fn parse() -> ArgMatches<'static> {
    let app = App::new("Paralel Depth First Search")
        .version(crate_version!())
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .value_name("N")
            .help("Number of threads to use"))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .value_name("FILE")
            .help("File to read graph input data from"))
        .arg(Arg::with_name("vertices")
            .short("n")
            .long("vertices")
            .takes_value(true)
            .value_name("N")
            .help("Generate a graph with N vertices"))
        .arg(Arg::with_name("edges")
            .short("m")
            .long("edges")
            .takes_value(true)
            .value_name("N")
            .help("Generate a graph with N edges"));

    app.get_matches()
}
