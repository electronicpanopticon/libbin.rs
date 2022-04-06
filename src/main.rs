use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};

const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn get_matches() -> ArgMatches {
  Command::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(
      Arg::new("name")
        .short('n')
        .long("name")
        .about("Greets someone by name")
        .takes_value(true)
        .value_name("NAME")
        .default_value("World"),
    )
    .arg("-b, --boop 'Go boop.'")
    .get_matches()
}

fn main() {
  let matches = get_matches();

  if matches.is_present("boop") {
    println!("{}", changemelib::boop());
  } else if let Some(i) = matches.value_of("name") {
    println!("{}", changemelib::greeting(i.to_string()));
  }
}
