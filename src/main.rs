use clap::{Arg, Command};

#[derive(Debug)]
struct WgetCli {
    url: String,
    output: String,
    dest: Option<String>,
    speed_limit: Option<String>,
    background: bool,
    quiet: bool,
    mirror: bool,
}

impl WgetCli {
    fn new(matches: &clap::ArgMatches) -> Self {
        let output = "output".to_string();

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

    fn execute(&self) {
        // Placeholder for execution logic based on parsed arguments.
    }
}

fn parse_args() -> clap::ArgMatches {
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

fn main() {
    let matches = parse_args();
    let wget_cli = WgetCli::new(&matches);

    println!("{:?}", wget_cli);

    wget_cli.execute();
}
