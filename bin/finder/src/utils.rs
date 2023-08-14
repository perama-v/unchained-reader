/// Converts a transaction id in UnchainedIndex format to one compatible with Appearances
/// specification.
///
/// See also: <https://github.com/ethereum/execution-apis/pull/456>
pub fn unchained_index_to_location(tx: u32) -> Option<String> {
    match tx {
        99999 => Some("miner".to_string()),
        99998 => Some("uncle".to_string()),
        99997 => None, // stores address 0xdeaddead..., can ignore
        99996 => None, // External (To be confirmed: used for gnosis chain somehow)
        // 99995 => Some("withdrawals".to_string()), // TBC: Future inclusion in Unchained index
        // https://github.com/TrueBlocks/trueblocks-core/issues/3122
        _ => Some(format!("{:#x}", tx)),
    }
}
