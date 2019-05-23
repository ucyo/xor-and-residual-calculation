
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("compress") {
        compress(&matches.subcommand_matches("compress").unwrap());
    } else {
        App::from_yaml(yaml).print_help().unwrap();
    }
}

fn compress(matches: &clap::ArgMatches) {
    println!("{:?}", matches);
}
