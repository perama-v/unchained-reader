//! Generates test case JSON-RPC responses

use serde::{Deserialize, Serialize};
use unchained_utils::structure::AddressData;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressesInBlockResponse {
    pub id: u32,
    pub jsonrpc: String,
    pub result: Vec<AddressAppearance>,
}


impl AddressesInBlockResponse {
    pub fn create(data: Vec<AddressData>) -> Self {
        AddressesInBlockResponse {
            id: 1,
            jsonrpc: "2.0".to_string(),
            result: data.into_iter().map(|x| {
                let address = hex::encode(x.address);
                let transaction_indices = x.appearances.into_iter().map(|y| {
                    y.index
                }).collect();

                AddressAppearance { address, transaction_indices }
            }).collect(),
        }
    }
}

/// Holds selected transactions for a given address.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressAppearance {
    /// The address that appeared in a transaction.
    pub address: String,
    /// The transaction index where the address appeared.
    pub transaction_indices: Vec<u32>,
}
