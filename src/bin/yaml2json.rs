// TODO:
// - is there a generic "value" type, so i can convert arbitrarily between
//   different encodings?
// - is there a way to retain ordering?

use std::path::{PathBuf};
use serde_json;
use serde_yaml;
use structopt::StructOpt;

type Err = Box<dyn std::error::Error>;

#[derive(Debug, StructOpt)]
struct Opt {
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let val = read_yaml(opt.input).unwrap();
    println!("{}", serde_json::to_string_pretty(&val).unwrap());
}

fn read_yaml(p: PathBuf) -> Result<serde_json::Value, Err> {
    let f = std::fs::File::open(p)?;
    // TODO: using ? gives error about DeserializeOwned not implemented.
    let v: serde_json::Value = serde_yaml::from_reader(f).unwrap();
    Ok(v)
}
