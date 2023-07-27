//! Contains the structure of the Unchained Index as defined in
//! the Unchained Index specification.

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Deserialize, Serializer};
use std::{io::Read, path::PathBuf, fmt::Display};

use super::constants::{ADDR, MAGIC, VAL, VER};

use thiserror::{self, Error};
#[derive(Debug, Error)]
pub enum StructureError {
    #[error("file {path} has magic bytes {found}, but expected {expected}")]
    InvalidMagicBytes {
        path: PathBuf,
        found: String,
        expected: String,
    },
    #[error("Unable to read address {source}")]
    InvalidAddress { source: std::io::Error },
    #[error("Unable to read count {source}")]
    InvalidCount { source: std::io::Error },
    #[error("Unable to read offset {source}")]
    InvalidOffset { source: std::io::Error },
    #[error("Unable to read n_addresses {source}")]
    InvalidNumAddresses { source: std::io::Error },
    #[error("Unable to read n_appearances {source}")]
    InvalidNumAppearances { source: std::io::Error },
    #[error("Unable to read transaction block {source}")]
    InvalidTransactionBlock { source: std::io::Error },
    #[error("Unable to read transaction index {source}")]
    InvalidTransactionIndex { source: std::io::Error },
    #[error("unable to read magic bytes from file {path} {source}")]
    NoMagicBytes {
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("unable to read magic bytes from file {path} {source}")]
    NoVersion {
        source: std::io::Error,
        path: PathBuf,
    },
}

#[derive(Default)]
/// Stores values extracted from file header.
pub struct Header {
    pub n_addresses: u32,
    pub n_appearances: u32,
}

impl Header {
    /// Obtains values from file header and validates magic number.
    pub fn from_reader(mut rdr: impl Read, path: &PathBuf) -> Result<Header, StructureError> {
        let mut magic: [u8; VAL] = [0; VAL];
        rdr.read_exact(&mut magic)
            .map_err(|e| StructureError::NoMagicBytes {
                path: path.to_owned(),
                source: e,
            })?;
        if magic != MAGIC {
            return Err(StructureError::InvalidMagicBytes {
                path: path.to_path_buf(),
                found: hex::encode(magic),
                expected: hex::encode(MAGIC),
            });
        }
        let mut version: [u8; VER] = [0; VER];
        rdr.read_exact(&mut version)
            .map_err(|e| StructureError::NoVersion {
                path: path.to_owned(),
                source: e,
            })?;
        let n_addresses = rdr
            .read_u32::<LittleEndian>()
            .map_err(|e| StructureError::InvalidNumAddresses { source: e })?;
        let n_appearances = rdr
            .read_u32::<LittleEndian>()
            .map_err(|e| StructureError::InvalidNumAppearances { source: e })?;
        Ok(Header {
            n_addresses,
            n_appearances,
        })
    }
}

/// Records information about important byte indices in the chunk file.
pub struct Body {
    /// Table in binary file containing addresses.
    pub addresses: Section,
    /// Table in binary file containing appearances (transaction IDs).
    pub appearances: Section,
}

/// Byte indices and length of entry for particular section.
pub struct Section {
    /// Byte index of start of section.
    pub start: usize,
    /// Which byte is currently of interest for this section.
    pub current: usize,
    /// Byte index of end of section.
    pub end: usize,
}

/// Content of an entry in the Addresses table.
#[derive(Clone)]
pub struct AddressEntry {
    /// Address bytes. Length 20 bytes.
    pub address: Vec<u8>,
    pub offset: u32,
    pub count: u32,
}

impl AddressEntry {
    /// Reads an address entry from the current reader position.
    pub fn from_reader(mut rdr: impl Read) -> Result<Self, StructureError> {
        let mut addr_buf: [u8; ADDR] = [0; ADDR];
        rdr.read_exact(&mut addr_buf)
            .map_err(|e| StructureError::InvalidAddress { source: e })?;
        let address = addr_buf.to_vec();
        let offset = rdr
            .read_u32::<LittleEndian>()
            .map_err(|e| StructureError::InvalidOffset { source: e })?;
        let count = rdr
            .read_u32::<LittleEndian>()
            .map_err(|e| StructureError::InvalidCount { source: e })?;

        Ok(AddressEntry {
            address,
            offset,
            count,
        })
    }
}

/// Holds selected transactions for a given address.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AddressData {
    /// The address that appeared in a transaction.
    pub address: Vec<u8>,
    /// The transactions where the address appeared.
    pub appearances: Vec<TransactionId>,
}

/// Content of an entry in the Appearances (transactions) table.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TransactionId {
    /// The Ethereum execution block number.
    pub block: u32,
    /// The index of the transaction in a block.
    pub index: u32,
}

impl TransactionId {
    /// Reads an appearance (Tx) entry from the current reader position.
    pub fn from_reader(mut rdr: impl Read) -> Result<Self, StructureError> {
        let block = rdr
            .read_u32::<LittleEndian>()
            .map_err(|e| StructureError::InvalidTransactionBlock { source: e })?;
        let index = rdr
            .read_u32::<LittleEndian>()
            .map_err(|e| StructureError::InvalidTransactionIndex { source: e })?;
        Ok(TransactionId { block, index })
    }
}
