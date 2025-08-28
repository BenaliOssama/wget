mod executer;
mod parser;

use parser::*;
use executer::*;


fn main() {
    let matches = parse_args();
    let wget_cli = WgetCli::new(&matches);

    wget_cli.execute();
}
