use clap::{App, Arg, ArgMatches};

use changemelib::*;

const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_LICENSE: &str = env!("CARGO_PKG_LICENSE");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_matches() -> ArgMatches {
    App::new("My Super Program")
        .version(PKG_VERSION)
        .author(PKG_AUTHORS)
        .license(PKG_LICENSE)
        .about(PKG_DESCRIPTION)
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .about("Greets someone")
            .takes_value(true)
            .value_name("NAME")
            .default_value("world"))
        .get_matches()
}

fn main() {

    let matches = get_matches();

    if let Some(i) = matches.value_of("name") {
        println!("Hello, {}!", i);
    }

}
