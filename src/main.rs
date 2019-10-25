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
                )
                .arg(
                    Arg::with_name("name")
                        .help("name of test suite")
                        .takes_value(true)
                        .index(1)
                        .value_name("NAME")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("emit test line")
                .author(env!("CARGO_PKG_AUTHORS"))
                .version(env!("CARGO_PKG_VERSION"))
                .arg(
                    Arg::with_name("status")
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
                )
                .arg(
                    Arg::with_name("number")
                        .help("test sequence number")
                        .short("n")
                        .long("number")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("plan", Some(plan_matches)) => {
            let raw_from = plan_matches.value_of("from").unwrap_or("0");
            let raw_to = plan_matches.value_of("to").expect("unable to read to");
            let from = raw_from.parse::<i32>().unwrap_or(-1);
            let to = raw_to.parse::<i32>().unwrap_or(-1);

            let name = plan_matches.value_of("name").unwrap_or("Untitled");

            TapWriter::new(&name).plan(from, to)
        }
        ("test", Some(test_matches)) => {
            let message = test_matches
                .value_of("message")
                .expect("unable to read message");
            let raw_number = test_matches
                .value_of("number")
                .expect("unable to read number");

            let number = raw_number.parse::<i32>().unwrap_or(-1);

            let tap_writer = TapWriter::new("tapper");

            match test_matches
                .value_of("status")
                .expect("unable to read status")
            {
                "pass" => tap_writer.ok(number, message),
                "fail" => tap_writer.not_ok(number, message),
                &_ => tap_writer.not_ok(number, message),
            };
        }
        _ => println!("No subcommand given. Try running with --help"),
    }
}
