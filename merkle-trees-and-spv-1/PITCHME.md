# Introduction to Simple Payment Verification (SPV), Merkle Trees and Bloom Filters

---
# Types of Validating Nodes : Full Node
  * Stores the all blocks in the longest chain  
  * Size is a problem 

  ![](https://raw.githubusercontent.com/tari-labs/tari-university/mikethetike-merkle-trees/merkle-trees-and-spv-1/sources/eth-blockchain-size.png "Well that escalated quickly")

---

# Types of Validating Nodes: Simple Payment Verification (SPV) client
* Examples: miner, wallets
* Only have block headers and asks a full node for blocks as they are needed
* If the block is in the longest chain and 6 blocks deep, it's accepted as valid

---

# Inefficient SPV implementation
* Only store block headers
* When a transaction is needed, download the entire block
* Hash to compare it to header to determine if valid
> Do we really need the entire block to verify a single transaction?

---
# Merkle Trees (and more importantly branches)

* A binary tree structure where:
  * Data is stored in the leaf nodes
  * A non-leaf node is the hash of it's 2 descendants
* Most importantly, descendant nodes can be pruned, leaving the rest of the tree's hash integrity in tact.
* This means we only need the Merkle branch with the transaction we want to verify

> Non-binary Merkle trees also exist

--- 
# Merkle Tree Example

![Merkle1](https://raw.githubusercontent.com/tari-labs/tari-university/mikethetike-merkle-trees/merkle-trees-and-spv-1/sources/merkle-1.svg.png)

--- 
# Merkle Tree Example

Adding a node

![Merkle2](https://raw.githubusercontent.com/tari-labs/tari-university/mikethetike-merkle-trees/merkle-trees-and-spv-1/sources/merkle-2.svg.png)

---

# Merkle Tree Example

Pruned Merkle Branch containing only C

![Merkle3](https://raw.githubusercontent.com/tari-labs/tari-university/mikethetike-merkle-trees/merkle-trees-and-spv-1/sources/merkle-3.svg.png)

---
# Merkle Tree Example 2

![Merkle4](https://raw.githubusercontent.com/tari-labs/tari-university/mikethetike-merkle-trees/merkle-trees-and-spv-1/sources/hashchain.png)



> By guardtime.com [CC BY-SA 3.0  (https://creativecommons.org/licenses/by-sa/3.0)], from Wikimedia Commons
---
# SPV and Merkle Branches

* SPV client request Merkle Branch from full node for transactions. 
* SPV client checks the Merkle Root against block header
* SPV client checks that block is at correct depth

---
# Privacy Problems
* When wallets only request transactions to the addresses it owns, full nodes will become aware of the owner
* To mitigate this, Bloom Filters can be used

--- 
# Bloom Filters



---
# Useful resources
* [https://bitcoin.org/en/developer-guide#simplified-payment-verification-spv Bitcoin Developer Guide]
