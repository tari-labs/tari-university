# Layer 2 Scaling Survey
## What is Layer 2 scaling?

In the block chain and cryptocurrency world, transaction processing scaling is a tough problem to solve. This is limited by the average block creation time, the block size limit and number of newer blocks needed to confirm a transaction (confirmation time). These factors make '*over the counter*' type transactions similar to Master Card or Visa nearly impossible if done on the main block chain (on-chain).

**ToDo - need picture**

The Open Systems Interconnection (OSI) model defines 7 layers for communication functions of a computing system. Layer 1 refers to the physical layer and Layer 2 to the data link layer. Layer 1 is never concerned with functions of Layer 2 and up, it just delivers transmission and reception of raw data. In turn Layer 2 only knows about Layer 1 and defines the protocols that deliver node-to-node data transfer. [1]

Analogous to the OSI layers for communication, in block chain technology decentralised Layer 2 protocols, also commonly referred to as Layer 2 scaling, refers to transaction throughput scaling solutions. Decentralised Layer 2 protocols run on top of the main block chain (off-chain), while preserving the attributes of the main block chain. Instead of each transaction only the resultant of a number of transactions are embedded on-chain. [2]

**ToDo - need picture**

## How will this be applicable to Tari?

???


## Layer 2 scaling current initiatives
### #1 Micropayment Channels 
#### What is it?

A Micropayment Channel is a class of techniques designed to allow users to make multiple Bitcoin transations without commiting all of the transactiosn to the Bitcoin block chain. In a typical payment channel, only two transactions are added to the block chain but an unlimited or nearly unlimited number of payments can be made between the participants.[11]

Several channel designs have been proposed or implemented over the years- these include:
- Nakamoto high-frequency transactions 
- Spillman-style payment channels 
- CLTV-style payment channels 
- Poon-Dryja payment channels 
- Decker-Wattenhofer duplex payment channels 
- Decker-Russell-Osuntokun eltoo Channels 
- Hashed Time-Locked Contracts (HTLCs)

With specific focus on Hashed Time-Locked Contracts:
A technique that can allow payments to be secrurely routed across multiple payment channels. HTlCs are integral to the design of more advanced payment channels such as those used by the Lightning Network. 

The Lightning Network is a second layer payment protocol that operates on top of a block chain. It enables instant transactions between participating nodes. It features a peer-to-peer system for making micropayments of digital cryptocurrency through a network of bidirectional payment channels without delegrating custody of funds and minimizing trust of third parties.[12] 

Normal use of the Lightning Network consists of opening a payment channel by commiting a funding transaction to the relevant block chain, followed by making any number of Lightning transactions that update the tentative distribution of the channel's funds wihtout broadcasting to the block chain, optionallu followed by closing the payment channel by broadcasting the final version of the transaction to distribute the channel's funds. 

#### Who does it?
Lightning is spreading across the crytocurrency landscape. It was orginally designed for Bitcoin, however litecoin, zcash, ethereum and ripple and just a few of the many cryptocurrencies planning to implement or test some form of the network.[13]

#### Strengths

- It is one of the leading solutions that has been presented to scale Bitcoin, which does not require a change to the underlying protocol;
- Transactions are processed instantly, the account balances of the nodes are updated and the money is immediately accesible to the new owner;
- Transaction fees are a fraction of the transaction cost.[14] 

#### Weaknesses

- It is not suitable for making bulk payment, asthe intermediate nodes in the multichannel payment network may not be loaded with money to move the funds along; 
- Recipients cannot recieve money unless their node is connected and online at the time of the transaction;
- The Lightning Network implementation written in C, runs into segmentation faults frequently.[14]

#### Opportunities for Tari 

???

#### Threats to Tari 

???

### #2 State Channels

#### What is it?

State channels allow multiple transactions to be made within off-chain agreements with very fast processing and the final settlement on-chain. It keeps the operation mode of block chain protocol but changes the way it is used so as to deal with the challenge of scalability.

#### Who does it?

Ethereum (via Raiden network)

Trinity, a NEO NEP-5 implementation. Trinity is an open-source network protocol based on NEP-5 smart contracts. NEO sees Trinity as their answer to Bitcoin's Lightning Network in order to achieve real-time payments, low transaction fees, scalability, micro transactions, and privacy protection for all NEO (NEP-5) assets. [3]



#### Strengths

???

#### Weaknesses

???

#### Opportunities

???

#### Threats

???

### #3 Trusted, off-chain matching engines

#### What is it?

Orders are matched off-chain in matching engine and fulfilled on-chain, allows complex orders, support cross-chain transfers, maintains public record of orders and a deterministic specification of behaviour. Makes use of token representation smart contract, that converts global assets into smart contract tokens and vice versa. [5]

![NEX-matching-engine](./sources/NEX-matching-engine.png)

#### Who does it?

Neon Exchange (NEX), a NEO decentralized application (dApp). NEX will first run on NEO, before later expanding to support exchange on Ethereum and other blockchains. Initially focussed on NEO, GAS and NEP-5 token transactions. [5]

#### Strengths

- Flexibility:
  - Cross-chain transfers;
  - Support of national currencies;
  - Smart contracts with reward to mitigate unfair exchange;
  - Public JavaScript Object Notation (JSON) Application Programmers Interface (API) & web extension API for third-party applications to trade tokens.
- Performance:
  - Off-chain matching;
  - Batched on-chain commits.
- Development environment: ***Elixir on top of Erlang*** to enable scalable, distributed, and fault-tolerant matching engine;
- Cure53 full security audit on web extension;
- NEX tokens will be regulated as registered European securities.

#### Weaknesses

- A certain level of trust is required, similar to a traditional exchange.

- Still in development.


#### Opportunities

- Has alignment with Tari's base requirements.

#### Threats

- None.

### #4 Masternodes
#### What is it?

A masternode is a server on a dencentralised network.[7] It is utilized to complete unique functions in ways ordinary nodes cannot. It can be used for features like direct send/instant transactions or private transactions.[8] 

Because of their increased capabilities, masternodes typically require a sizable investment in order to run. However, masternode operators are incentivised, and are rewarded by earning portions of block rewards in the cryptocurrency they are facilitating.[8] 

Masternodes are a type of full node that offers various services to a network and are compensated by the network for these functions. Like all full nodes, masternodes host an entire copy of the block chain. However, masternodes differ from ordinary full nodes in a few important ways.[9]

Masternodes will get the standard return on their stakes. But will also be entitled to a portion of the transaction fees. Allowing for a greater ROI.[10] 

#### Who does it?[9]

- Block 
- Bata
- Crown 
- Chaincoin 
- Dash 
- Diamond 
- ION 
- Monetary Unit
- Neutron 
- PIVX
- Vcash 
- XtraBytes 

#### Strengths

- Sustain and care of the ecosystem;  
- Masternodes perform specialised services: 
  * InstantSend (instant transactions- in contrast, Bitcoin takes about 10 minutes to confirm a payment);  
  * PrivateSend (anonymous transactions- in contrast, Bitcoin transactions are totally public and traceable. Only the identity of the addresses is anonymous);
  * Decentralised Governance (masternodes govern, while the block chain funds development- in contrast, Bitcoin is controlled by few big miners and funded by 3rd party centralised institutions with self interests).[7]
- Masternodes are given voting rights on proposals. Each masternode has a 1 vote and this vote can be used on budget proposals or important decisions;
- Masternodes offer payouts in a way similar to staking- whether in a bear or bull market there is still a payout.[7]

#### Weaknesses

- In order to get a masternode,you have to invest 1000 Dash into it- and not touch that money (if balance drops below 1000 Dash, one can lose voting rights and effectively be blocked from the Dash masternode network; 
- The maintaining of masternodes can be a long and arduous task- malfunctions are common; 
- ROI is not guaranteed and inconsistant (Your masternode only gets rewarded if it mines a block- most masternodes quote a 90% chance of getting paid- the system randomly decids if your node gets paid).[7]
- Masternodes are put forth with very little to no utility (only the promise of a Whitepaper);
- By putting up a masternode, your IP address becomes publicised, and so the location of your masternode is known- node can be stolen.[7]

#### Opportunities for Tari

A new token, such as a Tari token, by itself might not be worth very much except for speculative interest on its value proposition: what project it proposes to underlie. So it may not start with very much demand. However, if a token has MNs in its codebase, it proposes to offer additional benefits to holders of a fixed amount of tokens who run a MN. These benefits start with an extra share of all new tokens created in the regularly released blocks on the blockchain. The promise of this passive income increases demand, which increases price.

#### Threats to Tari

None

### #5 Plasma
#### What is it?

Plasma is a framework for incentivised and enforced execution of smart contracts, scalable to a significant amount of state updates per second, enabling the root block chain to be able to represent a significant amount of dApps, each employing its own block chain in a tree format. [4]

Plasma relies on two key parts, namely reframing all block chain computations into a set of MapReduce functions, and an optional method to do Proof-of-Stake token bonding on top of existing block chains where the Nakamoto Consensus incentives discourage block withholding or other Byzantine behavior. [4]

![Plasma example](./sources/Plasma-example.png)

#### Who does it?

Loom Network, using Delegated Proof of Stake (DPoS) consensus and validation, enabling scalable Application Specific Side Chains (DAppChains), running on top of Ethereum. [16]

OMG Network (OmiseGO), using Proof of Stake (PoS) consensus and validation, a Plasma block chain scaling solution for finance running on top of Ethereum. ([6], [15])

#### Strengths

- Not all participants need to be online to update state;
- Participants do not need a record of entry on the parent block chain to enable their participation in a Plasma block chain;
- Minimal data needed on the parent block chain to confirm transactions when constructing Plasma block chains in a tree format;
- Private block chain networks can be constructed, enforced by the root block chain. Transactions may occur on a local private block chain and have financial activity bonded by a public parent block chain.

#### Weaknesses

Must still be proven on other networks apart from Ethereum.

#### Opportunities for Tari

Has alignment with Tari's base requirements. Possibility to create a Tari ticketing Plasma dAppChain running of Monero?

#### Threats for Tari

The Loom Network's Software Development Kit (SDK) makes it extremely easy for anyone to create a new Plasma block chain. In less than a year a number of successful and diverse dAppChains have launched. The next one can easily be for ticket sales.

### #? What else?

#### What is it?

???

#### Who does it?

???

#### Strengths

???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #? What else?

#### What is it?

???

#### Who does it?

???

#### Strengths

???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

## Observations

???

## References

[1] OSI mode, https://en.wikipedia.org/wiki/OSI_model, Date accessed: 2018-06-07.

[2] Decentralized Digital Identities and Blockchain – The Future as We See It, https://cloudblogs.microsoft.com/enterprisemobility/2018/02/12/decentralized-digital-identities-and-blockchain-the-future-as-we-see-it, Date accessed: 2018-06-07.

[3] Trinity Protocol: The Scaling Solution of the Future?, https://www.investinblockchain.com/trinity-protocol, Date accessed: 2018-06-08.

[4] Plasma: Scalable Autonomous Smart Contracts, Poon J and Buterin V, http://plasma.io/plasma.pdf, Date accessed: 2018-06-14.

[5] NEX: A High Performance Decentralized Trade and Payment Platform, https://neonexchange.org/pdfs/whitepaper_v2.pdf, Date accessed: 2018-06-11.

[6] OmiseGO: Decentralized Exchange and Payments Platform, Poon J et. al., https://cdn.omise.co/omg/whitepaper.pdf, Date accessed: 2018-06-14.

[7] Masternodes- Risk vs. Reward, https://medium.com/@averagejoecrypto/masternodes-risk-vs-reward-6ca41eccfb08 , Date accessed: 2018-06-13.

[8] The Rise of Masternodes Might Soon be Followed by the Creation of Servicenodes, https://cointelegraph.com/news/the-rise-of-masternodes-might-soon-be-followed-by-the-creation-of-servicenodes, Date accessed: 2018-06-13.

[9] What are Masternodes?- Beginner's Guide, https://blockonomi.com/masternode-guide/, Date accessed: 2018-06-14.

[10] What the Heck is a DASH Masternode and How Do I get One, https://medium.com/dash-for-newbies/what-the-heck-is-a-dash-masternode-and-how-do-i-get-one-56e24121417e, Date accessed: 2018-06-14.

[11] Payment channels, https://en.bitcoin.it/wiki/Payment_channels , Date accessed: 2018-06-14.

[12] Lightning Network ,https://en.wikipedia.org/wiki/Lightning_Network, Date accessed: 2018-06-14.

[13] Bitcoin Isn't the Only Crypto Adding Lightning Tech Now, https://www.coindesk.com/bitcoin-isnt-crypto-adding-lightning-tech-now/, Date accessed: 2018-06-14.

[14] What is Bitcoin Lighning Network? And How Does it Work?, https://cryptoverze.com/what-is-bitcoin-lightning-network/ , Date accessed: 2018-06-14.

[15] OmiseGO, https://omisego.network/, Date accessed: 2018-06-14.

[16] Everything You Need to Know About Loom Network, All In One Place (Updated Regularly), https://medium.com/loom-network/everything-you-need-to-know-about-loom-network-all-in-one-place-updated-regularly-64742bd839fe, Date accessed: 2018-06-14.

[17] , , Date accessed: 2018-06-??.

[18] , , Date accessed: 2018-06-??.

[19] , , Date accessed: 2018-06-??.

[20] , , Date accessed: 2018-06-??.

[21] , , Date accessed: 2018-06-??.

[22] , , Date accessed: 2018-06-??.


