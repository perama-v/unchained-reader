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
<< {"id":1,"jsonrpc":"2.0","result":[{"address":"00000000000000adc04c56bf30ac9d3c0aaf14dc","transactionIndices":[188,198]},{"address":"000000000000012f9f5834e18ae9de5bb945fcbc","transactionIndices":[187]},{"address":"0000000000000130ad696f883928513d6c60e898","transactionIndices":[187]},{"address":"000000000000017fe957866391fcbff1e7cd8771","transactionIndices":[119]},{"address":"000000000000018078abcfe65140564ba897c5c7","transactionIndices":[119]},{"address":"000000000000027fd9e732802372528dd0182613","transactionIndices":[178]},{"address":"000000000000028099f6a81fd29448d84671c902","transactionIndices":[178]},{"address":"00000000000004209cb07257da66821be694bb8d","transactionIndices":[174]},
...
{"address":"fc720f8d776e87e9dfb0f59732de8b259875fa32","transactionIndices":[0,1,2,3,4,14,15,17,18,19,20,21,22,23,24,25,26,28,37,87,123,130,131,132,133,134,135,136,137,138,139,140,142,143,145,146,147,148,149,150,151,152,153,154,155,156]},{"address":"fd50b5a6a7c13d92aeafe33bc6337fe5355d6c0d","transactionIndices":[80]},{"address":"fe6d1cd1076aa6c0a68125ec2c89ab42114c953c","transactionIndices":[88]},{"address":"fe8058b2cf7c5f4542acdab09879500baf2ef020","transactionIndices":[38]},{"address":"fffd8963efd1fc6a506488495d951d5263988d25","transactionIndices":[6,7,8,62,74,79,80,120,162]},{"address":"ffffffffffffffffffffffffffffffffffffffff","transactionIndices":[6,8,12,71,126,128,129,165,168,169,175,183,194]}]}
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
