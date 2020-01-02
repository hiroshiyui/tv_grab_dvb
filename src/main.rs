extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches = App::new("EPGrab")
                        .version("0.0.1-rs")
                        .author("You, Hui-Hong <hiroshi@ghostsinthelab.org>")
                        .about("Grab DVB SI/PSI EIT information, which is required to build EPG")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Specify a configuration file")
                            .takes_value(true))
                        .subcommand(SubCommand::with_name("doctor")
                            .about("Self-test DVB hardware and required libraries")
                            .version("0.0.1-rs")
                            .author("You, Hui-Hong <hiroshi@ghostsinthelab.org>")
                            .arg(Arg::with_name("verbose")
                                .short("v")
                                .long("verbose")
                                .help("Output information verbosely")))
                        .subcommand(SubCommand::with_name("xmltv")
                            .about("Export grabbed DVB MPEG/TS PSI EIT packets to XMLTV format")
                            .version("0.0.1-rs")
                            .author("You, Hui-Hong <hiroshi@ghostsinthelab.org>"))
                        .subcommand(SubCommand::with_name("json")
                            .about("Export grabbed DVB MPEG/TS PSI EIT packets to JSON format")
                            .version("0.0.1-rs")
                            .author("You, Hui-Hong <hiroshi@ghostsinthelab.org>"))
                        .subcommand(SubCommand::with_name("webui")
                            .about("Start an EPG Web UI server at localhost")
                            .version("0.0.1-rs")
                            .author("You, Hui-Hong <hiroshi@ghostsinthelab.org>")
                            .arg(Arg::with_name("binding")
                                .short("b")
                                .long("binding")
                                .help("Specify server binding network address (default: 'localhost')"))
                            .arg(Arg::with_name("port")
                                .short("p")
                                .long("port")
                                .help("Specify server binding port (default: 3310)")))
                        .get_matches();

    let config = matches.value_of("config").unwrap_or("epgrab.conf");
    println!("Use configuration file: {}", config);


    println!("Hello, world!");
}
