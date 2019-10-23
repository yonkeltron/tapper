use clap::{App, Arg, SubCommand};
use testanything::tap_writer::TapWriter;

fn main() {
    let matches = App::new("tapper")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("plan")
                .about("emit plan line")
                .author(env!("CARGO_PKG_AUTHORS"))
                .version(env!("CARGO_PKG_VERSION"))
                .arg(
                    Arg::with_name("to")
                        .short("t")
                        .long("to")
                        .help("last test number")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .required(true),
                )
                .arg(
                    Arg::with_name("from")
                        .short("f")
                        .long("from")
                        .help("first test number")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("emit test line")
                .author(env!("CARGO_PKG_AUTHORS"))
                .version(env!("CARGO_PKG_VERSION"))
                .arg(
                    Arg::with_name("STATUS")
                        .help("status of test")
                        .required(true)
                        .index(1)
                        .value_name("STATUS")
                        .possible_values(&["pass", "fail"]),
                )
                .arg(
                    Arg::with_name("message")
                        .help("test message")
                        .short("m")
                        .long("message")
                        .takes_value(true)
                        .value_name("MESSAGE")
                        .required(true),
                )
                .arg(
                    Arg::with_name("diagnostic")
                        .help("diagnostic message")
                        .short("d")
                        .long("diagnostic")
                        .takes_value(true)
                        .value_name("DIAGNOSTIC")
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("plan", Some(plan_matches)) => {
            let raw_from = plan_matches.value_of("from").unwrap_or("0");
            let raw_to = plan_matches.value_of("to").expect("unable to read to");
        }
        ("test", Some(test_matches)) => println!("{:#?}", test_matches),
        _ => println!("No subcommand given. Try running with --help"),
    }
}
