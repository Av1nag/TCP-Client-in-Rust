use crate::{data_to_file_append, tcp_client};
use std::{fs, io::Write, num::ParseIntError};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "BTC_Stats", about = "An simple of client for Bitcoin updates.")]
struct Opt {
    // short and long flags (-m, --mode)
    #[structopt(short, long)]
    mode: String,

    /// No. of times
    // short and long flags (-t, --times)
    #[structopt(short = "t", long = "times", default_value = "0", parse(try_from_str = parse_times))]
    time_in_seconds: u32,
}

fn parse_times(s: &str) -> Result<u32, ParseIntError> {
    match s.parse() {
        Ok(value) => Ok(value),
        Err(e) => Err(e),
    }
}
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn main() {
    let opt = Opt::from_args();
    match opt.mode.as_str() {
        "cache" => {
            println!("fetching data......please wait");
            // Create the file if it does not exist
            let data_parser = data_to_file_append::main();
            let log_message = format!("{}", data_parser.0);
            // Write to a file
            writeln!(&data_parser.1, "{}", log_message).expect("write failed");

            let average_result = tcp_client::TCPClient(opt.time_in_seconds);
            let avg_prices = format!(
                "Cache complete. The average USD price of BTC is: {}",
                average_result
            );

            writeln!(&data_parser.1, "{}", avg_prices).expect("write failed");
        }
        "read" => {
            let contents =
                fs::read_to_string("data.txt")
                .expect("No file found. Consider creating a file ");
            println!("\n{contents}");

        }
        _ => {
            println!(
                "Invalid argument.\n USAGE:\n
    BTC_Project --[OPTIONS] \n
For more information try --help"
            );
        }
    }
}
