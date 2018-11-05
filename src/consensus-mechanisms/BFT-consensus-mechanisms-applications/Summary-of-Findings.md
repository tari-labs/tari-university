## Summary of Findings

Here is a table highlighting characteristics of the above mentioned BFT Protocols. Asymptotic Security, Permissionless Blockchain, Timing Assumptions, Decentralized Control, Low Latency and Flexible Trust form part of the value system. 

- Asymptotic Security: This depends only on digital signatures (and hash functions) for security
- Permissionless Protocol: This allows anybody to create an address and begin interacting with the protocol.
- Timing Assumptions: Please see [Many Forms of Timing Assumptions (Degrees of Synchrony)](#many-forms-of-timing-assumptions-(degrees-of-synchrony))
- Decentralized Control:Consensus is achieved and defended by protecting the identity of that node until their job is done, through a leaderless nodes. 
- Low Latency: This describes a computer network that is optimized to process a very high volume of data messages with minimal delay.
Flexible Trust: Where users have the freedom to trust any combinations of parties they see fit.

Important characteristics of each protocol are summarized in the table below. 

| Protocol                   | Permissionless Protocol |  Timing Assumptions   | Decentralized Control | Low Latency | Flexible Trust | Asymptotic Security |
| -------------------------- | :-----------------------: | :-------------------: | :-------------------: | :---------: | :------------: | :-----------------: |
| Hyperledger Fabric (HLF)   |                           | Partially synchronous |           ✓           |             |       ✓        |                     |
| Tendermint                 |                           | Partially synchronous |                       |      ✓      |       ✓        |          ✓          |
| Paxos                      |             ✓             | Partially synchronous |           ✓           |      ✓      |       ✓        |                     |
| Chandra-Toureg             |             ✓             | Partially synchronous |           ✓           |             |       ✓        |                     |
| Raft                       |             ✓             |  Weakly synchronous   |           ✓           |      ✓      |       ✓        |                     |
| HashGraph                  |             ✓             |     Asynchronous      |           ✓           |      ✓      |       ✓        |                     |
| SINTRA                     |             ✓             |     Asynchronous      |           ✓           |             |       ✓        |                     |
| HoneyBadgerBFT             |             ✓             |     Asynchronous      |           ✓           |      ✓      |       ✓        |          ✓          |
| Stellar Consensus Protocol |             ✓             |     Asynchronous      |           ✓           |      ✓      |       ✓        |          ✓          |
| LinBFT                     |             ✓             | Partially synchronous |           ✓           |             |       ✓        |                     |
| Algorand                   |             ✓             |      Synchronous      |           ✓           |      ✓      |       ✓        |                     |
| Thunderella                |             ✓             |      Synchronous      |           ✓           |      ✓      |       ✓        |                     |
| Avalanche                  |             ✓             |      Synchronous      |           ✓           |      ✓      |       ✓        |                     |
| PARSEC                     |             ✓             |  Weakly synchronous   |           ✓           |             |       ✓        |                     |
| Democratic BFT             |             ✓             | Partially synchronous |           ✓           |      ✓      |       ✓        |                     |

BFT consensus protocols have been considered as a means to diseminate and validate information, can schnorr multisignatures perform the same function in validating information through the action of signing. This will form part of the next review. 
