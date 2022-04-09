use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};

fn get_matches() -> ArgMatches {
  Command::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(
      Arg::new("name")
        .short('n')
        .long("name")
        .takes_value(true)
        .value_name("NAME")
        .default_value("World"),
    )
    .arg(Arg::new("boop").short('b').long("boop").takes_value(false))
    .get_matches()
}

fn main() {
  let matches = get_matches();

  if matches.is_present("boop") {
    println!("{}", changemelib::boop());
  } else if let Some(i) = matches.value_of("name") {
    println!("{}", changemelib::greeting(i));
  }
}
