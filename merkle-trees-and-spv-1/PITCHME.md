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
* Most importantly, descendant nodes can be pruned, leaving the rest of the tree's hash integrity intact.
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

* Provide filtering when receiving blocks
* Provides privacy

--- 
# Bloom Filter Implementation
* Choose _k_ hash functions
* Speed of hash function is most important when choosing hash functions
* Start with empty array of _m_ bits
* Hash _transaction hashes_, _public keys_ or _other criteria_ with each of the hash functions, and set bits in filter array

![Bloom1](https://upload.wikimedia.org/wikipedia/commons/f/f1/Gbf.gif)

> By The original uploader was Rlaufer at English Wikipedia. (Transferred from en.wikipedia to Commons.) [GFDL (http://www.gnu.org/copyleft/fdl.html) or CC-BY-SA-3.0 (http://creativecommons.org/licenses/by-sa/3.0/)], via Wikimedia Commons

---
# Bloom Filter Implementation

* Client sends filter to full nodes
* Full nodes hash transactions using the same hash functions, and only forward transactions when no 0's in the filter match
* Transactions are sent as pruned Merkle Trees
---
# False Positives
* Client will receive more transactions than it is looking for because of the hashing function, but will not miss any transactions
> Unless the full node is intentionally omitting them
* For more privacy, client can set extra bits in the filter


---
# Useful resources
* [Bitcoin Developer Guide](https://bitcoin.org/en/developer-guide#simplified-payment-verification-spv)
* [Bloom Filters tutorial](http://llimllib.github.io/bloomfilter-tutorial/)