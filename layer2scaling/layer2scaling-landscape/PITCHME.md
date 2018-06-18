## Layer 2 Scaling Survey

#### What is Layer 2 scaling?
#### How will this be applicable to Tari?
#### Layer 2 scaling current initiatives
- What is it and who does it?
- What are its strengths and weaknesses?
- Opportunities and Threats for Tari?

#### Observations
---
## What is Layer 2 scaling?

<u>Problem to solve:</u> 

- average block creation time
- block size limit
- number of newer blocks needed to confirm a transaction 


![waiting](https://github.com/tari-labs/tari-university/raw/layer2scaling/layer2scaling/layer2scaling-landscape/sources/waiting.png)
---

<u>Open Systems Interconnection (OSI) model</u> 

This is where the term 'Layer 2' is borrowed from

![OSI_Model](https://github.com/tari-labs/tari-university/raw/layer2scaling/layer2scaling/layer2scaling-landscape/sources/OSI_Model.png)

---

- In block chain, decentralised Layer 2 protocols (i.e. referred to as Layer 2 scaling) refers to transaction throughput scaling solutions
- Run on top of the main block chain (off-chain), while preserving the attributes of the main block chain (e.g. crypto economic consensus)

![layer2scaling](https://github.com/tari-labs/tari-university/raw/layer2scaling/layer2scaling/layer2scaling-landscape/sources/layer2scaling.png)

---

## How will this be applicable to Tari?

The initial business application to be built on top of the Tari block chain, Big Neon, requires high volume transactions in a short time, especially when tickets sales open and when tickets will be redeemed at an event.

This will be impossible to do with parent block chain scaling solutions.

---

## Layer 2 scaling current initiatives

#### #1 Micropayment Channels

- Users can make multiple transactions without committing all to the block chain
- Hashed Time-Locked Contracts (HTLC) allow payments to be securely routed across multiple payment channels

<u>Example:</u> The Lightning Network

![lightningnetwork](https://github.com/tari-labs/tari-university/raw/layer2scaling/layer2scaling/layer2scaling-landscape/sources/bitcoin-lightning-network-basic.png)

---

##### Who ?

- Bitcoin, Litecoin, Zcash, Ripple. 
- Ethereum also interested. 

##### Strengths

- It is one of the leading solutions that has been presented to scale Bitcoin
- Speed
- Low transaction fees

---

##### Weaknesses

- It is not suitable for making bulk payment
- Must be connected and online at the time of the transaction
- Lightning Network implementation written in C, runs into segmentation faults frequently

##### Opportunities for Tari

Less than expected as Tari's ticketing use case requires many fast transactions with many parties, not many fast transactions with a single party.

##### Threats to The ari

None

---

#### #2 State Channels

State channels are the more general form of micropayment channels — they can be used not only for payments, but for any arbitrary “state update” on a block chain — like changes inside a smart contract.

##### Who?

On Ethereum:

- Raiden
  - Research state channel technology, define protocols and develop reference implementations;
  - Works with any ERC20 compatible token.

---

- Counterfactual

  - A generalised framework for native state channels integration in Ethereum-based dApps;
  - A generalised state channel generalised framework is one where state is deposited once and then be used by any application or set of applications afterwards.

- Funfair
  - Decentralized gambling platform

- SpankChain 
  - Adult performers

- Horizon Blockchain Games
  - Ethereum-based game

---

On NEO:

- Trinity
  - Trinity is an open-source network protocol based on NEP-5 smart contracts. NEO sees Trinity as their answer to achieve real-time payments, low transaction fees, scalability, micro transactions, and privacy protection for all NEO (NEP-5) assets.

##### Strengths

- Allows payments and changes to smart contracts
- As for Micropayment Channels

##### Weaknesses, Opportunities for Tari, Threats for Tari

- As for Micropayment Channels

---

#### #3 Trusted, off-chain matching engines

Orders are matched off-chain in matching engine and fulfilled on-chain, allows complex orders, support cross-chain transfers, maintains public record of orders and a deterministic specification of behaviour. Makes use of token representation smart contract, that converts global assets into smart contract tokens and vice versa.

![NEX-matching-engine](https://github.com/tari-labs/tari-university/raw/layer2scaling/layer2scaling/layer2scaling-landscape/sources/NEX-matching-engine.png)

---

##### Who?

Neon Exchange (NEX), a NEO dApp. Initially focussed on NEO, GAS and NEP-5 token transactions. Exchange on Ethereum and other blockchains planned.

##### Strengths

- Flexibility:
  - Cross-chain transfers;
  - Support of national currencies;
  - Smart contracts with reward to mitigate unfair exchange;
  - Public JSON API & web extension API for third-party applications to trade tokens.

- Performance:
  - Off-chain matching;

  - Batched on-chain commits.

---

- Development environment: ***Elixir on top of Erlang*** for scalable, distributed, fault-tolerant matching engine;

- Cure53 full security audit on web extension;

- NEX tokens will be regulated as registered European securities.

##### Weaknesses

- A certain level of trust is required, similar to a traditional exchange.
- Still in development.

##### Opportunities for Tari

- Has alignment with Tari's base requirements.

##### Threats for Tari

- None

---

#### #4 Masternodes

A masternode is a server on a decentralised network, can be used for features like direct send/instant transactions or private transactions.

Masternode operators are rewarded by earning portions of block rewards, standard return on their stakes, portion of the transaction fees. Allowing for a greater ROI.

##### **Dash Example**

Dash - masternodes for proof of service and miners for proof of work, achieve distributed consensus on the blockchain. Masternodes share an equal block rewards with miners. 

##### Who?

- Block, Bata, Crown, Chaincoin, Dash, Diamond, ION, Monetary Unit, Neutron, PIVX, Vcash, XtraBytes 

##### Strengths

- InstantSend (Dash)
- PrivateSend (Dash)
- Decentralised Governance 
- It compensates for proof of work’s limitations and behaves almost like a buffed-up version proof of stake systems.

##### Weaknesses

- In order to be a masternode, you have to invest first;
- The maintaining of masternodes can be a long and arduous task- malfunctions are common; 
- ROI is not guaranteed and inconsistent;
- Location of your masternode is known, node can be stolen.

##### Opportunities for Tari

Increases incentives

##### Threats to Tari

None

---

#### #5 Plasma

##### What is it?

Plasma is a framework for incentivised and enforced execution of smart contracts, scalable to a significant amount of state updates per second, enabling the root block chain to be able to represent a significant amount of dApps, each employing its own block chain in a tree format.

Plasma relies on two key parts, namely reframing all block chain computations into a set of MapReduce functions, and an optional method to do Proof-of-Stake token bonding on top of existing block chains where the Nakamoto Consensus incentives discourage block withholding or other Byzantine behaviour.

![Plasma example](https://github.com/tari-labs/tari-university/raw/layer2scaling/layer2scaling/layer2scaling-landscape/sources/Plasma-example.png)

##### Who?

Loom Network, using Delegated Proof of Stake (DPoS) consensus and validation, enabling scalable Application Specific Side Chains (DAppChains), running on top of Ethereum. [16]

OMG Network (OmiseGO), using Proof of Stake (PoS) consensus and validation, a Plasma block chain scaling solution for finance running on top of Ethereum. ([6], [15])

---

##### Strengths

- Not all participants need to be online to update state;
- Participants do not need a record of entry on the parent block chain to enable their participation in a Plasma block chain;
- Minimal data needed on the parent block chain to confirm transactions when constructing Plasma block chains in a tree format;
- Private block chain networks can be constructed, enforced by the root block chain. Transactions may occur on a local private block chain and have financial activity bonded by a public parent block chain.

##### Weaknesses

Must still be proven on other networks apart from Ethereum.

##### Opportunities for Tari

- Has alignment with Tari's base requirements.
- *Possibility to create a Tari ticketing Plasma dAppChain running of Monero?*

##### Threats for Tari

The Loom Network's Software Development Kit (SDK) makes it extremely easy for anyone to create a new Plasma block chain. In less than a year a number of successful and diverse dAppChains have launched. *The next one can easily be for ticket sales...

## Observations

Further investigation into the more promising layer 2 scaling solutions and technologies is required to verify alignment, applicability and use-ability.