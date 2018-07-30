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



+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)



+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)



+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)



---

#### #4 Lumino



+++

#### #4 Lumino (cont'd)



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
