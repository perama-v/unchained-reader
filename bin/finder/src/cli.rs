use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Test case generator for address_* endpoints
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    /// eth_getAddressesInBlock
    #[clap(subcommand)]
    pub endpoint: AddressEndpoint,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AddressEndpoint {
    /// address_getAddressesInBlock
    ///
    /// Gets all appearances for all addresses in a single block.
    GetAddressesInBlock {
        /// Block that appearances are to be found for.
        #[clap(short, long)]
        block: u32,
        /// UnchaineIndex file to search
        #[clap(short, long)]
        chunk_file: PathBuf,
    },
    /// address_getAppearances
    ///
    /// Gets all appearances for one addresses across multiple blocks.
    GetAppearances {
        /// Address to get appearances for
        #[clap(short, long)]
        address: String,
        /// Block range kind to use for the test vector.
        #[clap(short, long)]
        range: RangeParam,
        /// Optional start block. If no range is given, all blocks are used.
        #[clap(short, long)]
        start_block: Option<u32>,
        /// Optional end block (inclusive, may be equal to start block for narrow range)
        #[clap(short, long)]
        end_block: Option<u32>,
        /// File containing a JSON formatted response from 'chifra <address> list --fmt json'
        #[clap(short, long)]
        file: PathBuf,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum RangeParam {
    All,
    Single,
    Custom,
}
