//! Generates test case JSON-RPC responses for appearances of a single address

use std::{fs::File, path::PathBuf};

use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{cli::RangeParam, utils::unchained_index_to_location};

/// Response to address_getAppearances
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearancesResponse {
    pub id: u32,
    pub jsonrpc: String,
    pub result: Vec<RelevantTransaction>,
}

impl AppearancesResponse {
    pub fn create(
        value: AppearanceSource,
        address: String,
        range: RangeParam,
        start_block: Option<u32>,
        end_block: Option<u32>,
    ) -> anyhow::Result<Self> {
        match (start_block, end_block, &range) {
            (None, _, RangeParam::Single | RangeParam::Custom) => {
                bail!("Must provide start block for specific blocks")
            }
            (_, None, RangeParam::Custom) => {
                bail!("Must provide end block for custom block range")
            }
            (Some(start), Some(end), RangeParam::Custom) => {
                if end < start {
                    bail!("Custom range start must be earlier than end")
                }
            }
            _ => {}
        }

        let result: Vec<RelevantTransaction> = value
            .data
            .into_iter()
            .filter(|x| x.address == address)
            .filter(|x| match &range {
                RangeParam::All => true,
                RangeParam::Single => match start_block {
                    Some(block) => x.block_number == block,
                    None => false,
                },
                RangeParam::Custom => match (start_block, end_block) {
                    (Some(start), Some(end)) => x.block_number >= start && x.block_number <= end,
                    _ => false,
                },
            })
            .filter_map(|x| match unchained_index_to_location(x.transaction_index) {
                Some(location) => Some(RelevantTransaction {
                    block_number: format!("{:#x}", x.block_number),
                    location,
                }),
                None => None,
            })
            .collect();

        Ok(AppearancesResponse {
            id: 1,
            jsonrpc: "2.0".to_string(),
            result,
        })
    }
}

/// A transaction identifier that is relevant for a particular address.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelevantTransaction {
    pub block_number: String,
    /// The index of the transaction index in which the address appeared.
    pub location: String,
}

/// Data containing information useful for test vector generation.
///
/// Data comes from trueblocks-core via
/// ```
/// chifra list <transaction> --fmt json | jq
/// ```
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AppearanceSource {
    data: Vec<AppearanceData>,
}

impl AppearanceSource {
    pub fn from_file(path: &PathBuf) -> anyhow::Result<AppearanceSource> {
        let file = File::open(path).or_else(|e| bail!("Could not open {:?} {}", path, e))?;

        let data: AppearanceSource = serde_json::from_reader(file)?;
        Ok(data)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceData {
    address: String,
    block_number: u32,
    transaction_index: u32,
}
