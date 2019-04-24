extern crate clap;

mod package_manager_installer;

use clap::{App, Arg};
use std::io::stdin;
use std::process;

fn main() {
    let matches = App::new("New Super Program")
        .version("0.1-alpha")
        .author("Drew Hutton")
        .about("Installs dotfiles, modularly and performantly")
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets a more verbose output"),
        )
        .arg(
            Arg::with_name("yes")
                .short("y")
                .help("Sets yes to all installs that require user input, this is useful for a completely autonomous install"),
        )
        .get_matches();

    // Set verbose here so we can use it later
    let verbose = if matches.is_present("v") { true } else { false };
    let yes = if matches.is_present("y") { true } else { false };

    println!("This program will install your dotfiles now,\nthis may takes some time, type 'yes' to continue");
    let mut yes_no_input = String::new();
    stdin().read_line(&mut yes_no_input).unwrap();
    let yes_no_question = yes_no_input.trim();

    if yes_no_question == "no" {
        println!("Exiting because you have not agreed!");
        process::exit(0x0100);
    }

    package_manager_installer::install_package_manager(verbose, yes);

    println!("Hello, world!");
}
