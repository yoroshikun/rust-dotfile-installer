extern crate clap;

mod package_manager_installer;

use clap::{App, Arg};

fn main() {
    let matches = App::new("New Super Program")
        .version("1.0")
        .author("Drew Hutton")
        .about("Installs dotfiles, modularly and performantly")
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets a more verbose output"),
        )
        .get_matches();

    // Set verbose here so we can use it later
    let verbose = if matches.is_present("v") { true } else { false };

    package_manager_installer::install_package_manager();

    println!("Hello, world!");
}
