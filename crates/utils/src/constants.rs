/// Constants particular to Unchained Index files.

/*
The unchained index files (block-a_block_b.bin) have structure:
- header (44 bytes)
    - 0xdeadbeef (4 bytes)
    - version_hash (32 bytes)
    - naddresses (4 bytes)
    - nappearances (4 bytes)
- addresses (naddresses * 28 bytes)
    - address (20 bytes)
    - offset (4 bytes)
    - count (4 bytes)
- appearances (nappearances * 8 bytes)
    - blocknumber (4 bytes)
    - transaction_index (4 bytes)
*/

/// Byte size of an address.
pub const ADDR: usize = 20;

/// Byte size of address entry (28).
pub const AD_ENTRY: usize = ADDR + VAL + VAL;

/// Byte size of appearance entry (8).
pub const AP_ENTRY: usize = VAL + VAL;

/// Byte size of file header (44).
pub const HEAD: usize = 4 + VER + VAL + VAL;

/// Magic bytes (0xdeadbeef little endian).
pub const MAGIC: [u8; 4] = [0xef, 0xbe, 0xad, 0xde];

/// Byte size of a standard value.
pub const VAL: usize = 4;

/// Byte size of file version.
pub const VER: usize = 32;
