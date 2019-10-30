#[macro_use]
extern crate clap;

mod configuration;

use clap::ArgMatches;
use configuration::{parse_input, Configuration};

fn main() {
    let config = load_yaml!("config.yml");
    let command_line_matches: ArgMatches = clap::App::from_yaml(config).get_matches();
    let configuration: Result<Configuration, String> = parse_input(&command_line_matches);
    println!("{:#?}", configuration)
}
