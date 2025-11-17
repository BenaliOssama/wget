use clap::{Arg, Command};

#[derive(Debug, Clone)]
pub struct WgetCli {
    pub url: String,
    pub urls_file: Option<String>,
    pub output: String,
    pub dest: Option<String>,
    pub speed_limit: Option<f64>,
    pub background: bool,
    pub quiet: bool,
    pub mirror: bool,
}

impl WgetCli {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let output = "index.html".to_string();
        let speed_limit = matches.get_one::<String>("speed_limit").map(|c| c.clone());
        Self {
            url: matches.get_one::<String>("url").unwrap().clone(),
            urls_file: matches
                .get_one::<String>("urls_file")
                .map(|c| c.clone())
                .clone(),
            output: matches
                .get_one::<String>("output")
                .unwrap_or(&output)
                .clone(),
            dest: matches.get_one::<String>("dest").map(|c| c.clone()),
            quiet: matches.contains_id("quiet"),
            mirror: matches.contains_id("mirror"),
            background: matches.contains_id("background"),
            speed_limit: calcule_speed_limit(speed_limit),
        }
    }

    pub fn handle_destination(&mut self) {
        if let Some(destination) = &self.dest {
            self.output = destination.to_string() + &self.output;
        }
    }
}

fn calcule_speed_limit(limit_option: Option<String>) -> Option<f64> {
    if let Some(mut s) = limit_option {
        let mut multiper = 1.0;
        if s.ends_with(|v| v == 'k' || v == 'm' || v == 'g') {
            multiper = mulipy(s.pop().unwrap());
        }
        return Some(s.parse::<f64>().expect("unvalid speed limit") * multiper);
    }
    None
}

fn mulipy(c: char) -> f64 {
    match c {
        'k' => 1024.0,
        'm' => 1024.0 * 1024.0,
        'g' => 1024.0 * 1024.0 * 1024.0,
        _ => 1.0,
    }
}

pub fn parse_args() -> clap::ArgMatches {
    Command::new("wget")
        .version("1.0")
        .about("A basic wget clone in Rust")
        .arg(Arg::new("url").help("The URL to download").required(true))
        .arg(
            Arg::new("urls_file")
                .short('i')
                .long("input-file")
                .help("urls file name")
                .value_name("urls"),
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
                // .action(clap::ArgAction::SetTrue)
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
                // .action(clap::ArgAction::SetTrue)
                .help("Download in the background"),
        )
        .get_matches()
}
