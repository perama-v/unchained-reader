# unchained-reader
Tool for reading UnchainedIndex files

- [crates/utils](./crates/utils) library for reading unchained index files
- [bin/finder](./bin/finder) app for exporting transactionIndices from the UnchainedIndex as test
files in a different format (e.g., eth_getAddressesInBlock)

The app has a CLI interface:
```command
cargo run -p appearance-finder -- --help
```

## address_getAddressesInBlock

### Obtaining the UnchainedIndex

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

### Test vector generation

Use the ./bin/finder application to generate test cases for a single block as follows:
```command
$ cargo run -p appearance-finder get-addresses-in-block --help
```
E.g.,
```command
$ cargo run -p appearance-finder get-addresses-in-block --block 17190873 --chunk-file data/17190873/017190314-017193246.bin
```

A test vector using this output can be seen in [./data/17190873/get-addresses-in-block.io](./data/17190873/get-addresses-in-block.io),
which matches the format in [https://github.com/ethereum/execution-apis/tree/main/tests](https://github.com/ethereum/execution-apis/tree/main/tests).

Example truncated test vector:
```console
>> {"jsonrpc":"2.0","id":1,"method":"eth_getAddressesInBlock","params":["17190873"]}
<< {"id":1,"jsonrpc":"2.0","result":{"blockNumber":"0x1064fd9","addresses":[{"address":"0x00000000000000adc04c56bf30ac9d3c0aaf14dc","indices":["0xbc","0xc6"]},{"address":"0x000000000000012f9f5834e18ae9de5bb945fcbc","indices":["0xbb"]},{"address":"0x0000000000000130ad696f883928513d6c60e898","indices":["0xbb"]},{"address":"0x000000000000017fe957866391fcbff1e7cd8771","indices":["0x77"]},{"address":"0x000000000000018078abcfe65140564ba897c5c7","indices":["0x77"]},
...
{"address":"0xfc720f8d776e87e9dfb0f59732de8b259875fa32","indices":["0x0","0x1","0x2","0x3","0x4","0xe","0xf","0x11","0x12","0x13","0x14","0x15","0x16","0x17","0x18","0x19","0x1a","0x1c","0x25","0x57","0x7b","0x82","0x83","0x84","0x85","0x86","0x87","0x88","0x89","0x8a","0x8b","0x8c","0x8e","0x8f","0x91","0x92","0x93","0x94","0x95","0x96","0x97","0x98","0x99","0x9a","0x9b","0x9c"]},{"address":"0xfd50b5a6a7c13d92aeafe33bc6337fe5355d6c0d","indices":["0x50"]},{"address":"0xfe6d1cd1076aa6c0a68125ec2c89ab42114c953c","indices":["0x58"]},{"address":"0xfe8058b2cf7c5f4542acdab09879500baf2ef020","indices":["0x26"]},{"address":"0xfffd8963efd1fc6a506488495d951d5263988d25","indices":["0x6","0x7","0x8","0x3e","0x4a","0x4f","0x50","0x78","0xa2"]},{"address":"0xffffffffffffffffffffffffffffffffffffffff","indices":["0x6","0x8","0xc","0x47","0x7e","0x80","0x81","0xa5","0xa8","0xa9","0xaf","0xb7","0xc2"]}]}}
```
This allows another implementation of `eth_getAddressesInBlock` to compare their response
to the implementation in trueblocks-core, which was used to generate the data in this test file.

## address_getAppearances

Data for a randomly selected address that has an interesting transaction in block 17190873
is present. Test cases can be generated for that address
using the data in data in [./data/17190873/address_0x30a4639850b3ddeaaca4f06280aa751682f11382.json](./data/17190873/address_0x30a4639850b3ddeaaca4f06280aa751682f11382.json)

If test vectors for other data are needed proceed below, otherwise skip to the
generate test vectors section.
### Test vectors for arbitrary data

Test cases are generated using trueblocks-core.

Install trueblocks then for `some_address` run:
```
chifra list 0x30a4639850b3ddeaaca4f06280aa751682f11382 --fmt json | jq
```
That file can then be ingested by `appearance-finder` app as shown below.

### Generate test cases

Test cases for the `address_getAppearances` method can be generated as follows:

Use the ./bin/finder application to generate test cases for a single block as follows:
```command
$ cargo run -p appearance-finder get-appearances --help
```
E.g., all blocks
```command
$ cargo run -p appearance-finder get-appearances --address 0x30a4639850b3ddeaaca4f06280aa751682f11382 --range all --file ./data/17190873/address_0x30a46.json
```
E.g., one block
```command
$ cargo run -p appearance-finder get-appearances --address 0x30a4639850b3ddeaaca4f06280aa751682f11382 --range single --start-block 17190873 --file ./data/17190873/address_0x30a46.json
```
E.g., range of blocks
```command
$ cargo run -p appearance-finder get-appearances --address 0x30a4639850b3ddeaaca4f06280aa751682f11382 --range custom --start-block 17190873 --end-block 17190889 --file ./data/17190873/address_0x30a46.json
```
