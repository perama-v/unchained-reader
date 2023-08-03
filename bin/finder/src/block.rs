//! Generates test case JSON-RPC responses for addresses in a single block

use serde::{Deserialize, Serialize};
use unchained_utils::structure::AddressData;

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

impl AddressesInBlockResponse {
    pub fn create(data: Vec<AddressData>, block_number: u32) -> Self {
        let addresses = data
            .into_iter()
            .map(|x| {
                let address = format!("0x{}", hex::encode(x.address));
                let indices = x
                    .appearances
                    .into_iter()
                    .map(|y| format!("{:#x}", y.index))
                    .collect();

                BlockAppearance { address, indices }
            })
            .collect();

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
    pub indices: Vec<String>,
}
