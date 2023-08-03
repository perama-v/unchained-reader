mod appearances;
mod block;
pub mod cli;

use appearances::{AppearanceSource, AppearancesResponse};
use block::AddressesInBlockResponse;
use clap::Parser;
use cli::{AddressEndpoint, AppArgs};
use unchained_utils::{BlockRange, UnchainedFile};

fn main() {
    let args = AppArgs::parse();
    match args.endpoint {
        AddressEndpoint::GetAddressesInBlock { .. } => generate_addresses_in_block(args),
        AddressEndpoint::GetAppearances { .. } => generate_appearances(args),
    }
}
/// For address_getAddressesInBlock
fn generate_appearances(args: AppArgs) {
    let (address, range, start_block, end_block, file) = match args.endpoint {
        AddressEndpoint::GetAppearances {
            address,
            range,
            start_block,
            end_block,
            file,
        } => (address, range, start_block, end_block, file),
        _ => return,
    };
    // Read file, parse, return formatted.
    let source = AppearanceSource::from_file(&file).expect("Couldn't read file");
    let response_test_vector =
        AppearancesResponse::create(source, address, range, start_block, end_block)
            .expect("Could not generate test from data");
    println!(
        "{}",
        serde_json::to_string(&response_test_vector).expect("Could not create JSON response")
    );
}

/// For address_getAddressesInBlock
fn generate_addresses_in_block(args: AppArgs) {
    let (block, chunk_file) = match args.endpoint {
        AddressEndpoint::GetAddressesInBlock { block, chunk_file } => (block, chunk_file),
        _ => return,
    };

    let range = BlockRange::new(block, block).expect("Bad range");
    let mut file = UnchainedFile::from_file(chunk_file, range).expect("Couldn't read file");
    file.with_parsed(None)
        .expect("Could not add appearance data");
    let response = AddressesInBlockResponse::create(file.parsed, block);
    println!(
        "{}",
        serde_json::to_string(&response).expect("Could not create JSON response")
    );
}
