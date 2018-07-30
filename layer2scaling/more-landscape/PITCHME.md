## Layer 2 Scaling Survey

- What is Layer 2 scaling, how will it be applicable o Tari? *[See [layer2scaling-landscape](https://gitpitch.com/tari-labs/tari-university/master?p=/layer2scaling/layer2scaling-landscape#/)]*

- Layer 2 scaling current initiatives (update)
  - TumbleBit
  - Counterparty Eco System
  - 2-Way Pegged Secondary Block Chains
  - Lumino
  - Scriptless scripts
  - DAG Derivative Protocols:

- Observations

---

## Layer 2 scaling current initiatives (update)

#### #1 TumbleBit

- TumbleBit protocol was invented at the Boston University
- Unidirectional, unlinkable payment hub for Bitcoin
- Combines off-chain cryptographics with on-chain Bitcoin scripting to realize smart contracts not dependent on Segwit

![TumbleBitOverview](https://github.com/tari-labs/tari-university/raw/L2ScalingUpdate/layer2scaling/more-landscape/sources/TumbleBitOverview.PNG)



+++

#### #1 TumbleBit (cont'd)

- Bitcoin scripting used: Hashing conditions, signing conditions, conditional execution, 2-of-2 multi signatures and timelocking
- Fast, anonymous, off-chain payments through an **untrusted** intermediary called the Tumbler
- Classic mixing/tumbling/washing mode or as a fully fledged payment hub
- Two interleaved fair-exchange protocols - *RSA-Puzzle-Solver Protocol* and *Puzzle-Promise Protocol* , relies on the RSA cryptosystem's blinding properties
- Anonymizing through Tor to ensure that the Tumbler server can operate as a hidden service

+++

#### #1 TumbleBit (cont'd)

- Boston University provided a proof-of-concept and reference implementation
- NTumbleBit - C# production implementation of the TumbleBit protocol by Stratis with their Breeze implementation (Nearly production ready)

<u>Strengths</u>

- Anonymity properties
- DoS & Sybil protection
- Balance 
- 2x modes of operation: classic tumbler, payment hub
- Scale ability
- Batch processing 
- Masternode compatibility

+++

#### #1 TumbleBit (cont'd)

<u>Weaknesses</u>

- Payees have better privacy than the payers
- Tumbler service not distributed
- Equal denominations required 

<u>Opportunities for Tari</u>

Has alignment with Tari's base requirements as a trustless Masternode matching/batch processing engine with strong privacy features

---

#### #2 Counterparty Eco System

- Counterparty is NOT a block chain -> protocol & network of nodes for smart contract apps using the EVM linked to Bitcoin block chain
- Uses embedded consensus (identical ledgers without P2P network) and federated nodes
- Transaction meta data are written into Bitcoin transactions on the Bitcoin block chain
- Read and validated by the federated nodes, executed by them

![CounterpartyStack](https://github.com/tari-labs/tari-university/raw/L2ScalingUpdate/layer2scaling/more-landscape/sources/CounterpartyStack.png)



+++

#### #2 Counterparty Eco System (cont'd)

- Counterparty smart contracts “lives” at Bitcoin addresses that starts with a `C`
- Broadcast an `execute` transaction to call a specific function in smart contract code
- Transaction confirmed by Bitcoin miner -> Counterparty federated nodes execute the function 
- The contract state is modified as the smart contract code executes and stored in the Counterparty database

+++

#### #2 Counterparty Eco System (cont'd)

- Most notable projects built on top of Counterparty:
  - [Age of Chains](https://www.ageofchains.com), [Age of Rust](http://spacepirate.io), [Augmentors](https://www.augmentorsgame.com/), [Authparty](http://authparty.io/), [Bitcorns](https://bitcorns.com/), [Blockfreight™](http://blockfreight.com/), [Blocksafe](http://www.blocksafefoundation.com), [BTCpaymarket.com](http://btcpaymarket.com), [CoinDaddy](http://coindaddy.io), [COVAL](https://coval.readme.io), [FoldingCoin](http://foldingcoin.net/), [FootballCoin](https://www.footballcoin.io/), [GetGems](http://getgems.org/#/), [IndieBoard](https://indiesquare.me/), [LTBCoin - Letstalkbitcoin.com](https://letstalkbitcoin.com/), [Mafia Wars](https://mafiawars.io/), [NVO](https://nvo.io/), [Proof of Visit](https://proofofvisit.com/), [Rarepepe.party](http://rarepepe.party), [SaruTobi Island](http://mandelduck.com/sarutobiisland/), [Spells of Genesis](http://www.spellsofgenesis.com), [Takara](https://mandelduck.com/#portfolio), [The Scarab Experiment](https://www.thescarabexperiment.org/), [Token.FM](https://token.fm/), [Tokenly](http://tokenly.com/), [TopCoin](https://topcoin.com/) and [XCP DEX](https://XCPDEX.COM)

+++

#### #2 Counterparty Eco System (cont'd)

<u>Strengths</u>

- Provides smart contract abilities rooted in Bitcoin block chain


<u>Weaknesses</u>

- Smart contracts and their state updates are executed and maintained off-chain in the federated nodes. If  federated nodes are compromised no evidence of transactions within eco system exists.
- Counterparty is not a Layer 2 scaling solution

<u>Opportunities for Tari</u>

- See '*Scriptless scripts*'

---

#### #3 2-Way Pegged Secondary Block Chains

- 2WP allows "transfer" of BTC from main Bitcoin block chain to secondary block chain and vice-versa at fixed rate, use appropriate security protocol.
- "Transfer" involves BTC be locked on main Bitcoin block chain, made available on secondary block chain
- <u>Sidechain:</u> Security protocol implemented using Simplified Payment Verification (SPV) proofs 
- <u>Drivechain:</u> Custody of BTC to miners, vote when to unlock BTC and where to send them
- <u>Federated Peg/Sidechain:</u> Trusted federation of mutually distrusting functionaries/notaries
- <u>Hybrid Sidechain-Drivechain-Federated Peg:</u> SPV proofs one way and mix of miner Dynamic Membership Multi-party Signature (DMMS) and functionaries/notaries multi-signatures going back

+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)

![RSK_HybridSideDriveChain](https://github.com/tari-labs/tari-university/raw/L2ScalingUpdate/layer2scaling/more-landscape/sources/RSK_HybridSideDriveChain.png)

- Locking of BTC on main block chain with P2SH Tx (BTC to script hash instead of public key hash)
- To unlock, provide a script matching the script hash and data to make script evaluate true

+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)

- 2WP promise concluded when equivalent amount tokens on secondary block chain locked so original bitcoins can be unlocked

<u>Who</u>

- RSK (*formerly Rootstock*) using a hybrid sidechain-drivechain security protocol
- Hivemind (formerly Truthcoin) is implementing a Peer-to-Peer Oracle Protocol 
- Blockstream is implementing a Federated Sidechain called Liquid

![Blockstream-Federated-Sidechain](https://github.com/tari-labs/tari-university/raw/L2ScalingUpdate/layer2scaling/more-landscape/sources/Blockstream-Federated-Sidechain.PNG)



+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)

<u>Strengths</u>

- Permissionless Innovation
- can be used to test or implement new features without risk
- Chains-as-a-Service (CaaS), with data storage 2WP secondary block chains
- Make it easier to implement smart contracts
- Can support larger block sizes and more transactions per second

<u>Weaknesses</u>

- Transferring BTC back into the main Bitcoin block chain is not secure
- Hugely dependent on merged mining, thus 51% attacks are a real threat
- DMMS provided by mining not secure for small systems, while trust of federation/notaries riskier for large systems

<u>Opportunities for Tari</u>

None, if enough functionality will be built into the main Tari block chain

---

#### #4 Lumino

- Lumino Transaction Compression Protocol (LTCP) is a technique for transaction compression
- Lumino network is a lightning-like extension of the RSK platform that uses LTCP
- Difference compression of data from same owner with aggregate signing of previous Txs
- RSK newly launched on main net January 2018, Lumino Network still be launched in test net

![LuminoDataPruning](https://github.com/tari-labs/tari-university/raw/L2ScalingUpdate/layer2scaling/more-landscape/sources/LuminoDataPruning.PNG)

+++

#### #4 Lumino (cont'd)

<u>Strengths</u>

- Promises high efficiency in pruning the RSK block chain

<u>Weaknesses</u>

- Has not been released yet
- Details about payment channels not decisive in the white paper

<u>Opportunities for Tari</u>

LTCP pruning may be beneficial to Tari

+++

#### #4 Lumino (cont'd)



+++

#### #4 Lumino (cont'd)



---

#### #5 Scriptless scripts



+++

#### #5 Scriptless scripts (cont'd)



+++

#### #5 Scriptless scripts (cont'd)



+++

#### #5 Scriptless scripts (cont'd)



---

#### #6 DAG Derivative Protocols



+++

#### #6 DAG Derivative Protocols (cont'd)



+++

#### #6 DAG Derivative Protocols (cont'd)



+++

#### #6 DAG Derivative Protocols (cont'd)



---

## Observations

???
