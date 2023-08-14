## appearance-finder

Application for generating test cases for addresses that appear in a block

- Input: UnchainedIndex chunk file
- Output: Test vector for a response to `eth_getAddressesInBlock`

Note that the UnchainedIndex represents extra-block appearances as sentinel numbers:
- https://github.com/TrueBlocks/trueblocks-core/blob/master/src/libs/etherlib/node.cpp#L300
    - 99999 miner
    - 99998 uncle reward
    - 99997 null recipient (miner forgot to set self as recipient)
    - 99996 external rewards (gnosis specific)
    - <pending> withdrawals
- When parsing the unchainedIndex to generate test cases these should be mapped to the appropriate
fields