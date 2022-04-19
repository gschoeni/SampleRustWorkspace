
use clap::{arg, Command};
use libsample::common;
use libsample::util;

fn main() {
    let matches = Command::new("sample-tool")
        .version("0.0.1")
        .about("Showing how to create CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initializes something")
        )
        .subcommand(
            Command::new("add")
                .about("Add two numbers")
                .arg(arg!(<NUM_1> "The first number"))
                .arg(arg!(<NUM_2> "The second number"))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            common();
            println!("running sample-tool init...");
        }
        Some(("add", sub_matches)) => {
            let num_1 = sub_matches.value_of("NUM_1").expect("required").parse::<usize>().expect("could not convert first param to usize");
            let num_2 = sub_matches.value_of("NUM_2").expect("required").parse::<usize>().expect("could not convert second param to usize");
            let result = util::math::add(num_1, num_2);
            println!("{}", result);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
