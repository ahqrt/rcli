// rcli csv -i input.csv -o output.json --header -d ","

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about = "A simple CLI tool for CSV processing", long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    // parse opts from command line
    let opts = Opts::parse();
    print!("{:?}", opts)
}
