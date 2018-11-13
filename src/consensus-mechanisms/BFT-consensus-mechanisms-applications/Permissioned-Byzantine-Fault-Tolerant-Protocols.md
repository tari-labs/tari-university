## Permissioned Byzantine Fault Tolerant Protocols 

Byzantine agreement schemes are considered well suited for permissioned block chains, where the identity of the participants is known. Examples include Hyperledger and Tendermint. Here the Federated Consensus Algorithm is implemented. [[9]] 

### Hyperledger Fabric (HLF)

HLF began as a project under the LinX Foundation in early 2016 [[13]], with the aim of creating an open-source cross-industry standard platform for distributed ledgers. HLF is an implementation of a distributed ledger platform for running smart contracts, leveraging familiar and proven technologies, with a modular architecture allowing pluggable implementations of various functions. The distributed ledger protocol of the fabric is run on the peers. The fabric distinguishes peers as validating peers (they run the consensus algorithm, thus validating the transactions) and non-validating peers (they act as a proxy that helps in connecting clients to validating peers). The validating peers run a BFT consensus protocol for executing a replicated state machine that accepts deploy, invoke and query transactions as operations. [[11]]

The block chain's hash chain is computed based on the executed transactions and resulting persistent state. The replicated execution of chaincode (the transaction which involves accepting the code of the smart contract to be deployed) is used for validating the transactions. They assume that among *n* validating peers, at most *f<n/3* (where *f* is the number of faulty nodes and *n* is the number of nodes present in the network) may behave arbitrarily, while others will execute correctly, thus adapting to concept BFT consensus. Since HLF proposes to follow PBFT, the chaincode transactions must be deterministic in nature, otherwise different peers might have different persistent state. The SIEVE protocol is used to filter out the non-deterministic transactions, thus assuring a unique persistent state among peers. [[11]]

While being redesigned for a v1.0 release, the format's goal was to achieve extensibility. This version allowed for modules such as membership and consensus mechanism to be exchanged. Being permissioned, this consensus mechanism is mainly responsible for receiving the transaction request from the clients and establishing a total execution order. So far, these pluggable consensus modules include a centralized, single order for testing purposes and a crash-tolerant ordering service based on Apache Kafka. [[9]]

### Tendermint

Tendermint Core is a BFT Proof-of-Stake (PoS) protocol which is composed of two protocols in one: a consensus algorithm and a peer-to-peer networking protocol. Jae Kwon and Ethan Buchman, inspired by the design goal behind [Raft](), specified Tendermint as an easy to understand, developer-friendly algorithm while doing algorithmically complex systems engineering. [[34]]

Tendermint is modeled as a deterministic protocol, live under partial synchrony, which achieves throughput within the bounds of the latency of the network and individual processes themselves. 

Tendermint rotates through the validator set, in a weighted round-robin fashion: where the higher the stake (i.e. voting power) that a validator possesses, the greater their weighting, the proportionally more times they will be elected as leaders. Thus, if one validator has the same amount of voting power as another validator, they will both be elected by the protocol an equal amount of times. [[34]] 

Critics have argued that Tendermint is not decentralized, and one can distinguish and target leadership, launching a DDoS attack against them, sniffling the progression of the chain. Although Sentry Architecture (containing sentry nodes, see [Sentry Nodes](#sentry-nodes)) in Tendermint has been implemented, the argument on the degree of decentralization is still questionable. 

#### Sentry Nodes 

Sentry Nodes are guardians of a validator node and provide the validator nodes with access to the rest of the network. Sentry nodes are well connected to other full nodes on the network. Sentry nodes may be dynamic, but should maintain persistent connections to some evolving random subset of each other. They should always expect to have direct incoming connections from the validator node and its backup(s). They do not report the validator node's address in the Peer Exchange Reactor (PEX) and they may be more strict about the quality of peers they keep.

Sentry nodes belonging to validators that trust each other may wish to maintain persistent connections via Virtual Private Network (VPN) with one another, but only report each other sparingly in the PEX.[[44]]

[9]: http://conferences.inf.ed.ac.uk/EuroDW2018/papers/eurodw18-Rusch.pdf
"High-Performance Consensus Mechanisms for Blockchains,
Rusch"

[13]: https://www.zurich.ibm.com/dccl/papers/cachin_dccl.pdf
"Architecture of the Hyperledger 
Blockchain Fabric, Cachin"

[11]: https://ieeexplore.ieee.org/document/8014672/
"Survey of Consensus Protocols of Blockchain Applications,
4th International Conference on Advanced Computing 
and Communication Systems, Sankar et al."

[34]: https://blog.cosmos.network/tendermint-explained-bringing-bft-based-pos-to-the-public-blockchain-domain-f22e274a0fdb
"Tendermint Explained- Bringing BFT-based
PoS to the Public Blockchain Domain"

[44]: https://github.com/tendermint/tendermint/blob/master/docs/spec/p2p/node.md
"Tendermint Peer Discovery
GitHub repository" 
