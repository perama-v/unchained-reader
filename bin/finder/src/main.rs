
pub mod cli;

use clap::Parser;
use cli::AppArgs;
use unchained_utils::{UnchainedFile, BlockRange};

fn main() {
    let args = AppArgs::parse();
    let range = BlockRange::new(args.low, args.high).expect("Bad range");
    let mut file = UnchainedFile::from_file(args.name, range).expect("Couldn't read file");
    file.with_parsed("9a");
    for tx in file.parsed {
        println!("{:?} in \t\t{:?}", tx.address, tx.appearances);
    }
}

