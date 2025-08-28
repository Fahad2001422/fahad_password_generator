mod options;
mod password_generation;

use std::char::from_u32;
use std::env;
use std::io::{stderr, Write};
use clap::{arg, command, ArgMatches, value_parser};
use crate::options::Options;

fn main() {
    // Create the command-line parser, and parse the arguments given
    let matches = command!().arg(
            arg!(password_length: <PASSWD_LEN> "Specifies how many characters must be generated")
                .value_parser(value_parser!(u16).range(0..))
    ).get_matches();

    // Now, parse the command-line arguments given by the user
    // to generate the options::Options object
    let password_generation_options = parse_arguments(&matches);

    // The user specified a length of 0 in the command line, which
    // is different from if the user did not provide anything
    if password_generation_options.get_password_length() == 0 {
        eprintln!("Error: you specified that the number of random characters is 0");
        eprintln!("You must specify a password length greater than one.");
        std::process::exit(1);
    }
}


fn parse_arguments(arg_matches: &ArgMatches) -> options::Options {
    // Get the number of characters as entered from the command line
    let length_arg = arg_matches.get_one::<u16>("password_length").cloned();
    // The user did not specify password length in the command line
    if length_arg == None {
        eprintln!("Error: You must enter an integer number \
        the number of characters in your password.");
        std::process::exit(2);
    } else {
        let passwd_len: u16 = length_arg.unwrap();
        return Options::new(passwd_len);
    }

}