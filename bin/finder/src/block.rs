//! Generates test case JSON-RPC responses for addresses in a single block

use serde::{Deserialize, Serialize};
use unchained_utils::structure::AddressData;

use crate::utils::unchained_index_to_location;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressesInBlockResponse {
    pub id: u32,
    pub jsonrpc: String,
    pub result: BlockAddresses,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockAddresses {
    pub block_number: String,
    pub addresses: Vec<BlockAppearance>,
}

/// Block of the merge (Ethereum consensus/execution)
const THE_MERGE: u32 = 15537393;

impl AddressesInBlockResponse {
    pub fn create(data: Vec<AddressData>, block_number: u32) -> Self {
        if block_number == 0 {
            // See also: <https://github.com/ethereum/execution-apis/pull/456>
            todo!("Every appearance will be 'alloc'")
        }
        let mut addresses: Vec<BlockAppearance> = data
            .into_iter()
            .map(|x| {
                let address = format!("0x{}", hex::encode(x.address));
                let locations = x
                    .appearances
                    .into_iter()
                    .filter_map(|y| unchained_index_to_location(y.index))
                    .collect();

                BlockAppearance { address, locations }
            })
            .collect();

        if block_number > THE_MERGE {
            // UnchainedIndex did not store withdrawals
            // We can provide them manually until the index is integrated
            // See also: <https://github.com/TrueBlocks/trueblocks-core/issues/3122>
            if block_number == 17190873 {
                // Used as a test case.
                let withdrawal = BlockAppearance {
                    address: "0x1cedc0f3af8f9841b0a1f5c1a4ddc6e1a1629074".to_string(),
                    locations: vec!["withdrawals".to_string()],
                };
                addresses.push(withdrawal);
            } else {
                todo!("Post merge block warning. Check if UnchainedIndex includes withdrawals, or provide withdrawal addresses manually in codebase")
            }
        }
        addresses.sort_by(|a1, a2| a1.address.cmp(&a2.address));
        AddressesInBlockResponse {
            id: 1,
            jsonrpc: "2.0".to_string(),
            result: BlockAddresses {
                block_number: format!("{:#x}", block_number),
                addresses,
            },
        }
    }
}

/// Holds selected transactions for a given address in a single block.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockAppearance {
    /// The address that appeared in a transaction.
    pub address: String,
    /// The transaction index where the address appeared.
    pub locations: Vec<String>,
}
