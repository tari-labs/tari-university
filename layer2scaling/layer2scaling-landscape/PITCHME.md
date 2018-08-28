## Layer 2 Scaling Survey

- What is Layer 2 scaling?

- How will this be applicable to Tari?

- Layer 2 scaling current initiatives
  - What is it and who does it?
  - What are its strengths and weaknesses?
  - Opportunities and Threats for Tari?

- Observations

---

## What is Layer 2 scaling?

<u>Block chain problem to solve:</u> 

- Average block creation time
- Block size limit
- Number of newer blocks needed to confirm a transaction 

![waiting](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/waiting.png)

+++

## What is Layer 2 scaling? (cont'd)

Let's postulate block chain and cryptocurrency "takes over the world", ~433.1 billion non-cash transactions per year...

- 13,734 transactions per second (tx/s) on average!

- Segwit enabled Bitcoin 'like' block chains, need ~644 parallel versions, combined growth ~210 GB per day! 
- Ethereum 'like' block chains, need ~541 parallel versions, combined growth  ~120 GB per day!

This is why we need a proper scaling solution

+++

## What is Layer 2 scaling? (cont'd)

<u>Open Systems Interconnection (OSI) model</u> 

This is where the term 'Layer 2' is borrowed from

![OSI_Model](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/OSI_Model.png)

+++

## What is Layer 2 scaling? (cont'd)

<u>Layer 2 scaling</u>

- In block chain, decentralized Layer 2 protocols (i.e. Layer 2 scaling) refers to transaction throughput scaling solutions
- Run on top of the main block chain (off-chain), while preserving the attributes of the main block chain (e.g. crypto economic consensus)

![layer2scaling](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/layer2scaling.png)

---

## How will this be applicable to Tari?

- Tari is a high-throughput protocol that will need to handle real world transaction volumes
- Big Neon, the initial business application to be built on top of the Tari block chain
  - High volume transactions when tickets sales open and when tickets will be redeemed
  - Imagine an 85,000 seat stadium with 72 entrance queues on match days... 
  - Serialized real world scanning: 500 tickets in 4 minutes, or ~2 spectators per second per queue
- This will be impossible to do with parent block chain scaling solutions

---

## Layer 2 scaling current initiatives

#### #1 Micropayment Channels

- Users can make multiple transactions without committing all to the block chain
- Hashed Time-Locked Contracts (HTLC) allow payments to be securely routed across multiple payment channels
- It is a second layer payment protocol that operates on top of a block chain

<u>Example:</u> The Lightning Network

![lightningnetwork](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/bitcoin-lightning-network-basic.png)

+++

#### #1 Micropayment Channels (cont'd)

##### Who ?

- Bitcoin, Litecoin, Zcash, Ripple
- Ethereum also interested

#####  
#####  
##### Strengths

- A leading solution presented to scale Bitcoin and Litecoin
- Speed of confirmed transactions
- Low transaction fees

+++

#### #1 Micropayment Channels (cont'd)

##### Weaknesses

- It is not suitable for making bulk payment
- Must be connected and online at the time of the transaction
- Currently channels are only bilateral

#####  
#####  
##### Opportunities for Tari

Less than expected as Tari's ticketing use case requires many fast transactions with many parties, not many fast transactions with a single party

#####  
#####  
##### Threats to Tari

None

---

#### #2 State Channels

State channels the more general form of micropayment channels  (also used for any arbitrary “state update” on block chain)  like changes inside smart contract.

<u>Consensus</u>: Change of state within channel needs explicit cryptographic consent. Use digitally signed and hash-locked transfers as consensus mechanism (balance proofs) secured by time-out (HTLC).

![Raiden](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/Raiden.PNG)

+++

#### #2 State Channels (cont'd)

##### Who?

- Raiden (*<u>On Ethereum</u>*)
  - Research state channel technology, define protocols and develop reference implementations
  - Works with any ERC20 compatible token

- Counterfactual (*<u>On Ethereum</u>*)

  - Generalised framework for native state channels integration (Ethereum dApps)
  - State deposited once and then used by any set of dApps afterwards
  - Counterfactual instantiation means to instantiate a contract without actually deploying it on-chain, users sign and share commitments to the multisig wallet 
  - All parties act as though it has been deployed, even though it hasn't 

+++

#### #2 State Channels (cont'd)

- - Makes use of global registry, an on-chain contract that maps unique deterministic addresses for any counterfactual contract to actual on-chain deployed addresses 
  - A typical Counterfactual state-channel is composed of counterfactually instantiated objects

![Counterfactual](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/Counterfactual.PNG)

+++

#### #2 State Channels (cont'd)

- Funfair (*<u>On Ethereum</u>*)
  - Decentralized gambling platform, centralized server based random number generation 
  - Investigating threshold cryptography like Boneh–Lynn–Shacham (BLS) signature schemes to enable secure random number generation by a group of participants 

- Trinity  (*<u>On NEO</u>*)
  - Trinity is an open-source network protocol based on NEP-5 smart contracts
  - Trinity for NEO is the same as the Raiden Network for Ethereum
  - Trinity uses the same consensus mechanism as the Raiden network
  - New token (TNC) to fund the Trinity network; NEO, NEP-5 and TNC tokens are supported

+++

#### #2 State Channels (cont'd)

##### Strengths

- Allows payments and changes to smart contracts
- Same as for Micropayment Channels

#####  
#####  
##### Weaknesses, Opportunities for Tari, Threats for Tari

- Same as for Micropayment Channels

---

#### #3 Off-chain matching engines

- Orders are matched off-chain in matching engine and fulfilled on-chain
- Allows complex orders
- Support cross-chain transfers
- Maintains public record of orders and a deterministic specification of behaviour
- Makes use of token representation smart contract, that converts global assets into smart contract tokens and vice versa

+++

#### #3 Trusted, off-chain matching engines (cont'd)

##### Who?

<u>Neon Exchange (NEX)</u> a NEO dApp

- Initially focussed on NEO, GAS and NEP-5 token transactions  -  Exchange on Ethereum and other block chains planned
- Off-chain matching engine will be scalable, distributed, fault-tolerant, function continuously and without downtime
- Consensus is achieved using cryptographically signed requests, public ledgers of transactions 

<u>0x</u> An Ethereum ERC20 based smart contract token (ZRX)

- Open source protocol to exchange ERC20 compliant tokens, matching engines in the form of dApps (*Relayers*), facilitate transactions between *Makers* and *Takers* 
- Consensus are governed with the publically available DEX smart contract 



+++

#### #3 Trusted, off-chain matching engines (cont'd)

![NEX-matching-engine](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/NEX-matching-engine.png)

![0xSequence](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/0xSequence.png)

+++

#### #3 Trusted, off-chain matching engines (cont'd)

#####  Strengths

- Performance {*NEX*, *0x*}
  - Off-chain request/order
  - Off-chain matching
- NEX specific
  - Batched on-chain commits
  - Cross-chain transfers
  - Support of national currencies
  - Public JavaScript Object Notation (JSON) Application Programmers Interface (API) & web extension API for third-party applications to trade tokens
  - Development environment: ***Elixir on top of Erlang*** to enable scalable, distributed, and fault-tolerant matching engine

+++

#### #3 Trusted, off-chain matching engines (cont'd)

- - Cure53 full security audit on web extension, NEX tokens regulated as registered European securities
- 0x specific
  - Open source protocol enable creation of independent off-chain dApp matching engines (*Relayers*)
  - Totally transparent matching of orders with no single point of control
    - Maker's order only enters a Relayer's order book if fee schedule is adhered to
    - Exchange can only happen if a Taker is willing to accept
  - Consensus and settlement governed by the publically available DEX smart contract

#####  
#####  
##### Weaknesses

- NEX and 0x still in development

+++

#### #3 Trusted, off-chain matching engines (cont'd)

- NEX specific
  - Certain level of trust is required, similar to traditional exchange
  - Closed liquidity pool
- 0x specific
  - Trusted Token Registry will be required to verify ERC20 token addresses and exchange rates
  - Front running transactions and transaction collisions possible, more development needed
  - Batch processing ability unknown

#####  
#####  
##### Opportunities for Tari

- Matching engines in general have opportunity for Tari; the specific scheme to be investigated further

#####  
#####  
##### Threats for Tari

- None

---

#### #4 Masternodes

- A masternode is a server on a decentralized network, features like direct send/instant transactions or private transactions
- Masternode operators rewarded by earning portions of block rewards, standard return on their stakes & portion of the transaction fees - allowing for a greater ROI

#####  
#####  
- **Dash Example**

  - 2nd tier network masternodes exists alongside a 1st tier network miners to achieve distributed consensus on the block chain
  - Special deterministic algorithm used to create pseudo-random ordering of Masternodes 
  - N pseudo random Masternodes perform the same task, act as an oracle 
  - Proof of service algorithm: Masternodes check rest of network to ensure they remain active, ~1% checked each block, entire network checked ~6 times per day (trustless, randomly via the Quorum system)

+++

#### #4 Masternodes (cont'd)

##### Who?

Block, Bata, Crown, Chaincoin, Dash, Diamond, ION, Monetary Unit, Neutron, PIVX, Vcash, XtraBytes 

#####  
#####  
##### Strengths

- Can sustain and take care of the ecosystem, protect block chains from network attacks
- Can perform decentralized governance of miners by having the power to reject or orphan blocks
- Can support decentralized exchanges
- Can be used to facilitate smart contracts 
- Can facilitate a decentralized marketplace
- Can compensate PoW's limitations; avoids mining centralization and consumes less energy
- Masternodes promise enhanced stability and network loyalty 

+++

#### #4 Masternodes (cont'd)

##### Weaknesses

- Maintaining of masternodes can be long and arduous
- ROI is not guaranteed and inconsistent
- IP address is publicized and thus open to attacks

#####  
#####  
##### Opportunities for Tari

- Masternodes can facilitate smart contracts off-chain & enhance security
- Increases incentives

#####  
#####  
##### Threats to Tari

None

---

#### #5 Plasma

##### What is it?

- Plasma is a framework for incentivized and enforced execution of smart contracts, scalable to a significant amount of state updates per second, enabling the root block chain to be able to represent a significant amount of dApps, each employing its own block chain in a tree format

![Plasma-example-01](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/Plasma-example-01.png)

- Plasma relies on two key parts: MapReduce functions, and an optional method to do Proof-of-Stake token bonding on top of existing block chains

+++

#### #5 Plasma (cont'd)

- Nakamoto Consensus incentives discourage block withholding or other Byzantine behaviour

- MapReduce: commitments on block chain computations as input in map phase, merkleized proof of state transition in reduce step when returning the result

![Plasma-example](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/layer2scaling-landscape/sources/Plasma-example.png)

+++

#### #5 Plasma (cont'd)

##### Who?

- *Loom Network*, using Delegated Proof of Stake (DPoS) consensus and validation, enabling scalable Application Specific Side Chains (DAppChains), running on top of Ethereum

- *OMG Network (OmiseGO)*, using Proof of Stake (PoS) consensus and validation, a Plasma block chain scaling solution for finance running on top of Ethereum

+++

#### #5 Plasma (cont'd)

##### Strengths

- Not all participants need to be online to update state
- Participants do not need record of entry on parent block chain to enable their participation in a Plasma block chain
- Minimal data needed on parent block chain to confirm transactions when constructing Plasma block chains in tree format
- Private block chain networks can be constructed, enforced by the root block chain (transactions may occur on local private block chain and have financial activity bonded by a public parent block chain)

#####  
#####  
##### Weaknesses

Must still be proven on other networks apart from Ethereum

+++

#### #5 Plasma (cont'd)

##### Opportunities for Tari

- Has opportunities for Tari as a L2 scaling solution
- *Possibility to create a Tari ticketing Plasma dAppChain running of Monero without creating a Tari specific root block chain?* [**Note:** This will make the Tari block chain dependent on another block chain.]

#####  
#####  
##### Threats for Tari

The Loom Network's SDK makes it extremely easy for anyone to create a new Plasma block chain. In less than a year a number of successful and diverse dAppChains have launched. *The next one can easily be for ticket sales...*

---

## Observations

Further investigation into the more promising layer 2 scaling solutions and technologies is required to verify alignment, applicability and use-ability

An overview of Counterparty, Rootstock, Drivechains and Scriptless scripts must still be added