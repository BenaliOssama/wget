use clap::{Arg, Command};

#[derive(Debug)]
struct WgetCli {
    url: String,
    output: String,
    dest: Option<String>,
    speed_limit: Option<String>,
    backgroung: bool,
    quite: bool,
    mirror: bool,
}

impl WgetCli {
    fn new() -> Self {
        Self {
            url: String::new(),
            output: String::new(),
            dest: None,
            speed_limit: None,
            backgroung: false,
            quite: false,
            mirror: false,
        }
    }
    fn execute(&self) {
        todo!()
    }
}

fn main() {
    let output = "output".to_string();
    let matches = Command::new("wget")
        .version("1.0")
        .about("A basic wget clone in Rust")
        .arg(
            Arg::new("url")
                .help("The URL to download")
                .required(true)
                // .index(1),
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
        .get_matches();
    let mut wget_cli = WgetCli::new();
    wget_cli.url = matches.get_one::<String>("url").unwrap().clone();
    wget_cli.output = matches
        .get_one::<String>("output")
        .unwrap_or(&output)
        .clone();
    wget_cli.dest = matches.get_one::<String>("dest").map(|c| c.clone());
    wget_cli.quite = matches.contains_id("quiet");
    wget_cli.mirror = matches.contains_id("mirror");
    wget_cli.backgroung = matches.contains_id("background");
    wget_cli.speed_limit = matches.get_one::<String>("speed_limit").map(|c| c.clone());
    println!("{:?}", wget_cli);
    wget_cli.execute();
}
