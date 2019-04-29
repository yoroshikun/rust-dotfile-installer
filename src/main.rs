extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;

mod brew;
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
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Sets a more verbose output"),
        )
        .arg(
            Arg::with_name("yes")
                .short("y")
                .long("yes")
                .help("Sets yes to all installs that require user input, this is useful for a completely autonomous install"),
        )
        .get_matches();

    let verbose = if matches.is_present("verbose") {
        true
    } else {
        false
    };
    let yes = if matches.is_present("yes") {
        true
    } else {
        false
    };

    println!("This program will install your dotfiles now,\nthis may takes some time, type 'yes' to continue");
    if yes {
        if verbose {
            println!("skipping confirmation becaues yes variable is present")
        }
    } else {
        confirmation_loop()
    }

    package_manager_installer::install_package_manager(verbose, yes);

    let config = brew::config_readfile(verbose);

    if config.is_ok() {
        println!("Config file read");
        let config = config.unwrap();
        if !config.brew_formulas.is_empty() {
            brew::brew_install(verbose, config.brew_formulas);
        }
    } else {
        println!("{:?}", config.err());
        println!("Error: No Config File found");
        println!("Help: If you have not made one please do so");
        println!("Information on how to do so is located here");
        process::exit(0x0100)
    }

    println!("Completed");
}

fn confirmation_loop() {
    loop {
        let mut confirmation = String::new();
        stdin()
            .read_line(&mut confirmation)
            .ok()
            .expect("Failed to read line");
        let confirmation: &str = confirmation.trim();

        match confirmation {
            "yes" => break,
            "no" => {
                println!("Exiting because you have not agreed!");
                process::exit(0x0100)
            }
            _ => {
                println!("Please enter yes or no");
                continue;
            }
        }
    }
}
