# unchained-reader
Tool for reading UnchainedIndex files

- [crates/utils](./crates/utils) library for reading unchained index files
- [crates/utils](./crates/utils) app for exporting appearances from the UnchainedIndex as test
files in a different format (e.g., eth_getAddressesInBlock)

The Unchained


## Obtaining the UnchainedIndex

Either use the sample in `./data/17190873/QmV...` Quick way to get a piece of the index:

1. Get manifest IPFS CID

- Visit contract https://etherscan.io/address/0x0c316b7042b419d07d343f2f4f5bd54ff731183d#readContract
- Read contract method manifestHashMap
    - address: 0xf503017d7baf7fbc0fff7492b751025c6a78179b
    - string: mainnet

This returns the manifest CID.

2. Get the manifest

`https://ipfs.unchainedindex.io/ipfs/<manifest_cid>`

3. Look for a particular chunk (based on block numbers)

4. Fetch the chunk

`https://ipfs.unchainedindex.io/ipfs/<chunk_cid>`

5. Rename the chunk to what was in the manifest range field ("range": "017190314-017193246")
with .bin suffix. This mimics how trueblocks-core handles the files.

QmVu.... -> 017190314-017193246.bin

