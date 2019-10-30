use clap;

#[derive(Debug)]
pub struct Configuration {
    amazing: bool,
    epic: usize,
}

pub fn parse_input(matches: &clap::ArgMatches) -> Result<Configuration, String> {
    let amazing: bool = match matches.is_present("amazing") {
        true => true,
        false => false,
    };

    let epic: usize = match matches.value_of("epic") {
        Some(s) => s.parse().map_err(|_| "usize is invalid".to_owned())?,
        None => 11,
    };

    Ok(Configuration {
        amazing: amazing,
        epic: epic,
    })
}
