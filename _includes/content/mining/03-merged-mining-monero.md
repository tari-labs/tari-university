## Table of Contents

- [Overview](#overview)
- [Monero Details](#monero-details)
  - [Monero Codebase](#monero-codebase)
  - [Monero RPC](#monero-rpc)
- [The Process](#the-process)

## Overview

The following diagram depicts the necessary components needed for merged mining from the Monero application ecosystem.
<div class="mermaid">
stateDiagram
    MoneroD --> XMRig
    XMRig --> MoneroD
    Monero_Console_Wallet --> MoneroD
    MoneroD --> Monero_Console_Wallet
</div>

The following diagram depicts how merged mining with Monero is currently implemented for Tari.
<div class="mermaid">
stateDiagram
    MoneroD --> Tari_Merge_Mining_Proxy
    Tari_Merge_Mining_Proxy --> MoneroD
    XMRig --> Tari_Merge_Mining_Proxy
    Tari_Merge_Mining_Proxy --> XMRig
    Monero_Console_Wallet --> MoneroD
    MoneroD --> Monero_Console_Wallet
    Tari_Base_Node --> Tari_Merge_Mining_Proxy
    Tari_Merge_Mining_Proxy --> Tari_Base_Node
    Tari_Console_Wallet --> Tari_Merge_Mining_Proxy
    Tari_Merge_Mining_Proxy --> Tari_Console_Wallet
</div>

Lastly the following diagram illustrates how pool software interacts with Monero.
<div class="mermaid">
stateDiagram
    MoneroD --> Monero_Pool
    MoneroD --> Monero_Onion_Block_Explorer
    Monero_Onion_Block_Explorer --> MoneroD
    Monero_Pool --> MoneroD
    XMRig --> Monero_Pool
    Monero_Pool --> XMRig
    Monero_Wallet_RPC --> Monero_Pool
    Monero_Pool --> Monero_Wallet_RPC
</div>

## Monero Details

### Monero Codebase

Serialization and deserialization of a Monero block is needed, this could be reverse engineered from the codebase,
however libraries are available to do this. What we do need to know in this regard is that a input blob and block template/block
is a hexidecimal representation of the serialization that occurs when interacting with Monero RPC.

It is also important to note that Monero hashes are in little endian format.

The tx_extra tags are found within the coinbase transaction for the monero block [[4]], the only tag we will be inserting
into the monero block template is the merge mining tag.

The input blob for XMRig will need to be calculated after inserting this tag. This is defined by `get_block_hashing_blob` [[1]]
in the Monero codebase. This is the result of serializaing the block header, appending the merkle root and the
number of transactions in the block and then converting it to hexidecimal [[1]].

The merkle root will also need to be calculated when verifying the Monero block. This is defined in the file `tree_hash.c`, which is an
algorithm from the Monero codebase [[2]].

RandomX needs to be set up identically to Monero. This is defined by `rx_slow_hash` in the Monero codebase[[2]].

The following information hash to be stored in the Tari proof-of-work data to verify (and also calculate the achieved difficulty)
 of the Monero block:
1) The block header, which is used to calculate the input blob for RandomX.
2) The RandomX key provided with the initial template. This ensures the same result is reached as it would in Monero.
Used when calculating the achieved difficulty.
3) The number of transactions in the block (including the coinbase transaction), which is also used to calculate the input blob
for RandomX.
4) The merkle root of the block. This is used to verify the block.
5) The transaction hashes which appear in the block (including the coinbase transaction). This is used to both calculate the input blob
for RandomX as well as verify the block.
6) The coinbase transaction. This is used to verify the block.

### Monero RPC

XMRig calls the following JSON-RPC methods from MoneroD:
GET Requests:
1) /get_height - Gets the node's current height [[3]]
```
Example Request:
http://daemon_url:18081/get_height

Response:
{
  "height": 1564055,
  "status": "OK",
  "untrusted": false
}
```

POST Requests:
1) /get_block_template - Get a block template on which to mine a new block [[3]]
```
Example Request
{
  "jsonrpc":"2.0",
  "id":"0",
  "method":"get_block_template",
  "params":{
    "wallet_address":"44GBHzv6ZyQdJkjqZje6KLZ3xSyN1hBSFAnLP6EAqJtCRVzMzZmeXTC2AHKDS9aEDTRKmo6a6o9r9j86pYfhCWDkKjbtcns",
    "reserve_size":60
  }
}

Response:
{
  "id": "0",
  "jsonrpc": "2.0",
  "result": {
    "blockhashing_blob": "070786a498d705f8dc58791266179087907a2ff4cd883615216749b97d2f12173171c725a6f84a00000000fc751ea4a94c2f840751eaa36138eee66dda15ef554e7d6594395827994e31da10",
    "blocktemplate_blob": "070786a498d705f8dc58791266179087907a2ff4cd883615216749b97d2f12173171c725a6f84a0000000002aeab5f01fff2aa5f01e0a9d0f2f08a01028fdb3d5b5a2c363d36ea17a4add99a23a3ec7935b4c3e1e0364fcc4295c7a2ef5f01f912b15f5d17c1539d4722f79d8856d8654c5af87f54cfb3a4ff7f6b512b2a08023c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f1755090c809421d69873c161e7969b8bf33cee3b451dd4859bfc244a705f0b4900498f804b6023e13fa023a0fb759e8b7c9a39506a21442bc47077beeedc6b78d34c4ebdae91bd96097ccc9a882bc5056568b0d2f1f06559368fea4acba8e745444e883e53156d5083c1fd260edf05292934c8b40c098b81fe4e261720bdd272b209e317247a1d2c55dc4718891af0d16273c5a610f36f382a3bf50f54808aaa6a508e51d4601dd0d8fbf8b3b1685066ce121666a1409e8ac7a4d673c1cc36d10b825f764af647441f53230518e4d2efbcf8791c6060912c76e90db4982a66d51bbd96290bbb34db8080b216c2940cec407260bf5e2c3a5ee280835f15298f0801e9d98c4d414792282fbc2c28c3e20bc0fcb1829b5c3ad8f8d20847be8fdb2a949fd96f0205fbd6d271c880c5d8c83e9813606cd803a44d377fdeae45bfa67112132af601e9b3b0613ba7dff2ec3d4b935c447b47bfe39f7b950981b2f4c66c0d853e2218f1f69229a9b608c3d98be09b6d4d640a9f6ff0e920dbacf7e58b59554c0b398b1ae4b1d497104b4e4e745d850eed7eddb8aa93437427bf442ae5beb22cbf10a8fa738ea38cfa5d86dfd30675d4be11a38016e36936fd5601e52643e8b8bc433702ea7ae6149309c95b898cc854850e73fe0b95c5b8879b7325ecd4",
    "difficulty": 61043624293,
    "expected_reward": 4771949057248,
    "height": 1561970,
    "prev_hash": "f8dc58791266179087907a2ff4cd883615216749b97d2f12173171c725a6f84a",
    "reserved_offset": 129,
    "status": "OK",
    "untrusted": false
  }
}
```
2) /submit_block - Submit a mined block to the network. [[3]]
```
Example Request:
{
"jsonrpc":"2.0",
"id":"0",
"method":"submit_block",
"params":["0707e6bdfedc053771512f1bc27c62731ae9e8f2443db64ce742f4e57f5cf8d393de28551e441a0000000002fb830a01ffbf830a018cfe88bee283060274c0aae2ef5730e680308d9c00b6da59187ad0352efe3c71d36eeeb28782f29f2501bd56b952c3ddc3e350c2631d3a5086cac172c56893831228b17de296ff4669de020200000000"]
}

Rejected Response:
{
  "id": "0",
  "jsonrpc": "2.0",
  "error": {
    "code": -7,
    "message": "Block not accepted"
  }
}

Accepted Response:
{
  "id": "0",
  "jsonrpc": "2.0",
  "status": "OK"
  "untrusted": false
}
```

Something to note about the methods is that:
1) With GET methods, the method name is appended to the daemon url.
2) With POST methods, the method name is embedded in the method field of the request body and is sent to the daemon url
appended with `/json_rpc`.

## The Process

The following sequence diagram illustrates the merge mining process across the applications involved [[5]]

<div class="mermaid">
sequenceDiagram
loop Mining
XMRig -->>+ Tari_Merge_Mining_Proxy: get_height
Tari_Merge_Mining_Proxy -->>+ MoneroD: get_height
MoneroD -->>+ Tari_Merge_Mining_Proxy: response
Tari_Merge_Mining_Proxy -->>+ Tari_Base_Node: get_tip_info
Tari_Base_Node -->>+ Tari_Merge_Mining_Proxy : TipInfoResponse
Tari_Merge_Mining_Proxy -->>+ XMRig: response
XMRig -->>+ Tari_Merge_Mining_Proxy: get_new_block_template
Tari_Merge_Mining_Proxy -->>+ MoneroD: get_new_block_template
MoneroD -->>+ Tari_Merge_Mining_Proxy: response           
Tari_Merge_Mining_Proxy -->>+ Tari_Base_Node: get_new_block_template
Tari_Base_Node -->>+ Tari_Merge_Mining_Proxy: NewBlockTemplateResponse
Tari_Merge_Mining_Proxy -->>+ Tari_Console_Wallet: add_coinbase
Tari_Console_Wallet -->>+ Tari_Merge_Mining_Proxy: CoinbaseResponse
Tari_Merge_Mining_Proxy -->>+ Tari_Base_Node: get_new_block
Tari_Base_Node -->>+ Tari_Merge_Mining_Proxy: NewBlockResponse
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: append_merge_mining_tag
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: create_blockhashing_blob
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: serialize_monero_block_to_hex
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: min(monero_difficulty,tari_difficulty)
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: blocktemplate.save()
Tari_Merge_Mining_Proxy -->>+ XMRig: response
loop Solution not found
XMRig -->>+ Tari_Merge_Mining_Proxy: get_height
Tari_Merge_Mining_Proxy -->>+ MoneroD: get_height
MoneroD -->>+ Tari_Merge_Mining_Proxy: response
Tari_Merge_Mining_Proxy -->>+ Tari_Base_Node: get_tip_info
Tari_Base_Node -->>+ Tari_Merge_Mining_Proxy : TipInfoResponse
alt Height was changed
XMRig -->>+ Tari_Merge_Mining_Proxy: get_new_block_template
Tari_Merge_Mining_Proxy -->>+ MoneroD: get_new_block_template
MoneroD -->>+ Tari_Merge_Mining_Proxy: response           
Tari_Merge_Mining_Proxy -->>+ Tari_Base_Node: get_new_block_template
Tari_Base_Node -->>+ Tari_Merge_Mining_Proxy: NewBlockTemplateResponse
Tari_Merge_Mining_Proxy -->>+ Tari_Console_Wallet: add_coinbase
Tari_Console_Wallet -->>+ Tari_Merge_Mining_Proxy: CoinbaseResponse
Tari_Merge_Mining_Proxy -->>+ Tari_Base_Node: get_new_block
Tari_Base_Node -->>+ Tari_Merge_Mining_Proxy: NewBlockResponse
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: append_merge_mining_tag
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: create_blockhashing_blob
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: serialize_monero_block_to_hex
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: min(monero_difficulty,tari_difficulty)
Tari_Merge_Mining_Proxy -->>+ Tari_Merge_Mining_Proxy: blocktemplate.save()
Tari_Merge_Mining_Proxy -->>+ XMRig: response
else
XMRig -->>+ XMRig: Solve
end
end
XMRig -->>+ Tari_Merge_Mining_Proxy: submit_block
alt Difficulty > Monero Difficulty
Tari_Merge_Mining_Proxy -->>+ MoneroD: submit_block
MoneroD -->>+ Tari_Merge_Mining_Proxy : response
else
end
alt Difficulty > Tari Difficulty
Tari_Merge_Mining_Proxy -->>+ Tari_Base_Node: submit_block
Tari_Base_Node -->>+ Tari_Base_Node: verify
Tari_Base_Node -->>+ Tari_Merge_Mining_Proxy : response
else
end
Tari_Merge_Mining_Proxy -->>+ XMRig: response
end
</div>

Since the diagram above is quite lengthy and complex the verification criteria of a Monero block in Tari is detailed below:
1) The stored miner transaction contains the merge mining hash of the Tari block.
2) The merkle root in the proof of work is the same as the merkle root calculated from the stored transaction hashes.


## References

[[1]] StackExchange "Format of a block in the Monero Blockchain" [online].
Available: <https://monero.stackexchange.com/questions/3958/what-is-the-format-of-a-block-in-the-monero-blockchain>. Date accessed: 2020&#8209;10&#8209;21.

[1]: https://monero.stackexchange.com/questions/3958/what-is-the-format-of-a-block-in-the-monero-blockchain
"Format of a block in the Monero Blockchain"

[[2]] Github "Monero Project" [online]. Available <https://github.com/monero-project/monero>. Date accessed: 2020&#8209;10&#8209;21.

[2]: https://github.com/monero-project/monero
"Monero Project"

[[3]] GetMonero "Daemon RPC" [online].
Available: <https://web.getmonero.org/resources/developer-guides/daemon-rpc.html>. Date accessed: 2020&#8209;10&#8209;21.

[3]: https://web.getmonero.org/resources/developer-guides/daemon-rpc.html
"Daemon RPC"

[[4]] StackExchange "How are the various tx-extra types/formats used?" [online]. <https://monero.stackexchange.com/questions/1381/how-are-the-various-tx-extra-types-formats-used>. Date accessed: 2020&#8209;10&#8209;21.

[4]: https://monero.stackexchange.com/questions/1381/how-are-the-various-tx-extra-types-formats-used
"How are the various tx-extra types/formats used?"

[[5]] Github "Monero Project" [online]. Available <https://github.com/tari-project>. Date accessed: 2020&#8209;10&#8209;21.

[5]: https://github.com/tari-project
"Tari Project"
