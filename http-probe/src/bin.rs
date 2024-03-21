use std::{
    fs::File,
    io::{BufRead, BufReader},
};


use http_probe_lib::{error::HttpProbeError, test_domains};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

#[tokio::main]
async fn main() -> Result<(), HttpProbeError> {
    let args = Args::parse();

    let file = File::open(args.input).map_err(HttpProbeError::FileError)?;

    // Assume domains are line-separated
    let buf = BufReader::new(file);
    let domains: Vec<Box<str>> = buf.lines()
        .map(|x| x.unwrap().into_boxed_str())
        .collect();

    let listening_hosts = test_domains(domains, 10).await?;
    listening_hosts.into_iter().for_each(|url| println!("{}", url));

    Ok(())
}

