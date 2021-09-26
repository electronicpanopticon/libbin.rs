use clap::{crate_authors, crate_license, crate_name, crate_version, App, Arg, ArgMatches};

const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn get_matches() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .license(crate_license!())
        .about(PKG_DESCRIPTION)
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .about("Greets someone by name")
            .takes_value(true)
            .value_name("NAME")
            .default_value("world"))
        .get_matches()
}

fn main() {

    let matches = get_matches();

    if let Some(i) = matches.value_of("name") {
        println!("{}", changemelib::greeting(i.to_string()));
    }
}
