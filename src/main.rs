mod options;
mod password_generation;

use std::env;
use std::io::{Write};
use clap::{arg, command, ArgMatches, value_parser};
use crate::options::Options;

fn main() {
    // Create the command-line parser to parse the arguments given
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
        eprintln!("Error: you specified that the number of random characters is 0.");
        eprintln!("You must specify an integer password with length >= 1.");
        std::process::exit(2);
    }

    // The user provided a valid password length in the CLI.
    // Now, we start the RNG.
    let mut password_rng = rand::rng();

    let mut generated_password = String::new();

    // Initalize a loop counter to the password length, and
    // decrement it until it becomes 0.
    let mut i = password_generation_options.get_password_length();
    while i > 0 {
        // For every iteration of this loop, choose a random Unicode
        // code point, convert it into a UTF-8 character, and push
        // this character into the `password_rng` variable as defined
        // above.
        let random_codepoint = password_generation::generate_random_code_point(
            &mut password_rng
        );

        let random_character = std::char::from_u32(random_codepoint).unwrap();

        generated_password.push(random_character);

        i -= 1;
    }

    // After the successful generation of the password, print
    // it to the standard output, and exit with code 0.
    println!("Generated password: {generated_password}");
    std::process::exit(0);
    
}


fn parse_arguments(arg_matches: &ArgMatches) -> options::Options {
    // Get the number of characters as entered from the command line.
    let length_arg = arg_matches.get_one::<u16>("password_length").cloned();
    // The user did not specify password length in the command line, or
    // they wrote an argument with the wrong type (e.g. float, integer array, etc.).
    if length_arg == None {
        eprintln!("Error: You must enter an integer number of characters in your password.");
        std::process::exit(1);
    } else {
        let passwd_len: u16 = length_arg.unwrap();
        return Options::new(passwd_len);
    }

}
