use clap::{Arg, Command};

#[derive(Debug)]
pub struct WgetCli {
    pub url: String,
    pub output: String,
    pub dest: Option<String>,
    pub speed_limit: Option<String>,
    pub background: bool,
    pub quiet: bool,
    pub mirror: bool,
}

impl WgetCli {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let output = "index.html".to_string();

        Self {
            url: matches.get_one::<String>("url").unwrap().clone(),
            output: matches
                .get_one::<String>("output")
                .unwrap_or(&output)
                .clone(),
            dest: matches.get_one::<String>("dest").map(|c| c.clone()),
            quiet: matches.contains_id("quiet"),
            mirror: matches.contains_id("mirror"),
            background: matches.contains_id("background"),
            speed_limit: matches.get_one::<String>("speed_limit").map(|c| c.clone()),
        }
    }

    pub fn handle_destination(&mut self) {
        if let Some(destination) = &self.dest {
            self.output = destination.to_string() + &self.output;
        }
    }

}

pub fn parse_args() -> clap::ArgMatches {
    Command::new("wget")
        .version("1.0")
        .about("A basic wget clone in Rust")
        .arg(
            Arg::new("url")
                .help("The URL to download")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('O')
                .long("output")
                .help("Output file name")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("dest")
                .short('d')
                .long("dest")
                .help("The destination of the file")
                .value_name("DEST"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(clap::ArgAction::SetTrue)
                .help("Suppress output (quiet mode)"),
        )
        .arg(
            Arg::new("mirror")
                .short('m')
                .long("mirror")
                .action(clap::ArgAction::SetTrue)
                .help("Enable mirroring (recursive downloading)"),
        )
        .arg(
            Arg::new("speed_limit")
                .short('l')
                .long("limit-rate")
                .help("Set the maximum download speed")
                .value_name("SPEED"),
        )
        .arg(
            Arg::new("background")
                .short('b')
                .long("background")
                .action(clap::ArgAction::SetTrue)
                .help("Download in the background"),
        )
        .get_matches()
}