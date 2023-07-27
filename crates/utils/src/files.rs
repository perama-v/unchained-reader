use regex::Regex;
use std::{
    fs,
    num::ParseIntError,
    path::{Path, PathBuf},
};

use super::{
    constants::{AD_ENTRY, AP_ENTRY, HEAD},
    parse::{BlockRange, UnchainedFile},
    structure::{Body, Header, Section, TransactionId},
};

use thiserror::{self, Error};
#[derive(Debug, Error)]
pub enum FilesError {
    #[error("File {path} has appearance (block {block} index {index}) out of expected range ({newest}-{oldest})")]
    AppearanceOufOfRange {
        path: PathBuf,
        oldest: u32,
        newest: u32,
        block: u32,
        index: u32,
    },
    #[error("File {filename} could not be opened {source}")]
    FileOpener {
        source: std::io::Error,
        filename: PathBuf,
    },
    #[error("DirEntry error {0}")]
    DirEntry(#[from] std::io::Error),
    #[error("Unable to read path {path} as string.")]
    InvalidPathName { path: PathBuf },
    #[error("Regex error {0}")]
    RegexError(#[from] regex::Error),
    #[error("File {filename} title lacks 9-digit block range")]
    InvalidFilenameRange { filename: String },
    #[error("File {filename} contains an invalid upper/lower bound {source}")]
    InvalidFileBound {
        filename: PathBuf,
        source: ParseIntError,
    },
}

/// Details for files in the Unchained Index chunk directory.
pub struct ChunksDir {
    pub dir: PathBuf,
    pub paths: Vec<ChunkFile>,
}

impl ChunksDir {
    /// Obtains information about all the available chunk files.
    ///
    /// # Example
    /// If the chunk files are in "xyz/trueblocks/unchained/mainnet/finalized",
    /// then this is the path passed in.
    pub fn new(dir_path: &Path) -> Result<Self, FilesError> {
        let files = fs::read_dir(dir_path).map_err(|e| FilesError::FileOpener {
            filename: dir_path.to_path_buf(),
            source: e,
        })?;
        let mut paths: Vec<ChunkFile> = vec![];
        for file in files {
            let path = file.map_err(FilesError::DirEntry)?.path();
            let range = get_range(&path)?;
            let chunk = ChunkFile { path, range };
            paths.push(chunk);
        }

        paths.sort_by_key(|k| k.range.old);
        Ok(ChunksDir {
            dir: dir_path.to_path_buf(),
            paths,
        })
    }
    /// Obtains the details of chunk files relevant for a given block range.
    ///
    /// Chunks are relevant if they intersect the desired range.
    pub fn for_range(&self, desired_range: &BlockRange) -> Option<Vec<&ChunkFile>> {
        let mut relevant: Vec<&ChunkFile> = vec![];
        for chunk in &self.paths {
            if chunk.range.intersection_exists(desired_range) {
                relevant.push(chunk);
            }
        }
        if relevant.is_empty() {
            return None;
        }
        Some(relevant)
    }
}

#[derive(Clone, Debug)]
pub struct ChunkFile {
    pub path: PathBuf,
    pub range: BlockRange,
}

/// Determines the byte indices for a given chunk file.
pub fn file_structure(h: &Header) -> Body {
    let app_start = HEAD + h.n_addresses as usize * AD_ENTRY;
    let total_bytes = app_start + h.n_appearances as usize * AP_ENTRY;
    Body {
        addresses: Section {
            start: HEAD,
            current: HEAD,
            end: app_start - 1,
        },
        appearances: Section {
            start: app_start,
            current: app_start,
            end: total_bytes - 1,
        },
    }
}

/// Get first and last block that an index chunk covers.
pub fn get_range(path: &Path) -> Result<BlockRange, FilesError> {
    // Two 9 digit values .../123456789-123456789.bin
    let path_string = path.to_str().ok_or(FilesError::InvalidPathName {
        path: path.to_path_buf(),
    })?;
    let bounds = Regex::new(
        r"(?x)
    (?P<low>\d{9})  # the earliest block.
    -
    (?P<high>\d{9}) # the the latest block.
    ",
    )
    .map_err(FilesError::RegexError)?
    .captures(path_string)
    .ok_or(FilesError::InvalidFilenameRange {
        filename: path_string.to_owned(),
    })?;

    Ok(BlockRange {
        old: bounds["low"]
            .parse::<u32>()
            .map_err(|e| FilesError::InvalidFileBound {
                filename: path.to_path_buf(),
                source: e,
            })?,
        new: bounds["high"]
            .parse::<u32>()
            .map_err(|e| FilesError::InvalidFileBound {
                filename: path.to_path_buf(),
                source: e,
            })?,
    })
}

/// Checks that given appearance is within chunk file bounds.
pub fn no_unexpected_appearances(
    appearance: &TransactionId,
    uf: &UnchainedFile,
) -> Result<(), FilesError> {
    if appearance.block < uf.present.old || appearance.block > uf.present.new {
        return Err(FilesError::AppearanceOufOfRange {
            path: uf.path.to_path_buf(),
            oldest: uf.present.old,
            newest: uf.present.new,
            block: appearance.block,
            index: appearance.index,
        });
    }
    Ok(())
}
