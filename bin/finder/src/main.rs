pub mod cli;
mod generator;

use clap::Parser;
use cli::AppArgs;
use generator::AddressesInBlockResponse;
use unchained_utils::{BlockRange, UnchainedFile};


fn main() {
    let args = AppArgs::parse();
    let range = BlockRange::new(args.low, args.high).expect("Bad range");
    let mut file = UnchainedFile::from_file(args.name, range).expect("Couldn't read file");
    file.with_parsed(None).expect("Could not add appearance data");
    let response = AddressesInBlockResponse::create(file.parsed);
    println!("{}",serde_json::to_string(&response).expect("Could not create JSON response"));

}
