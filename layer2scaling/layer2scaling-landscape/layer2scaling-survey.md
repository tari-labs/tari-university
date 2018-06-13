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
### #1 ???? (What technology? The Lightning Network)
#### What is it?

Lightning is a decentralised network of bilateral bitcoin payment channels off the bitcoin blockchain. Lightning transactions are typically small, and most are not broadcast to the blockchain. The Lightning Network will enable transactions to be  be faster and cheaper than on-chain bitcoin transactions. 

#### Who does it?

Bitcoin (via The Lightning Network)

???

#### Strengths

???

#### Weaknesses

???

#### Opportunities

???

#### Threats

???


### #2 Proof of Stake
#### What is it?

???

#### Who does it?

???

#### Strengths

???

#### Weaknesses

???

#### Opportunities

???

#### Threats

???

### #3 State Channels

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

### #4 Trusted, off-chain matching engines

#### What is it?

Orders are matched off-chain in matching engine and fulfilled on-chain, allows complex orders, support cross-chain transfers (starting with NEO and Ethereum), maintains public record of orders and a deterministic specification of behaviour. Initially focussed on NEO, GAS and NEP-5 token transactions. Makes use of token representation smart contract, that converts global assets into smart contract tokens and vice versa. [5]

![NEX-matching-engine](./sources/NEX-matching-engine.png)

#### Who does it?

Neon Exchange (NEX), a NEO decentralized application (dApp). NEX will first run on NEO, before later expanding to support exchange on Ethereum and other blockchains. [5]

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

### #3 Masternodes
#### What is it?

A masternode is a server on a dencentralised network. It is utilized to complete unique functions in ways ordinary nodes cannot. It can be used for features like direct send/instant

Masternodes are a type of full node that offers various services to a network and are compensated by the network for these functions. Like all full nodes, masternodes host an entore copy of the Blockchain. However, masternodes differ from ordinary full nodes in a few important ways.

Masternodes perform specialied transactions like InstantSend and PrivateSend

In Dash, the masternodes represent the layer that is responsible for caring, cultivating and expanding the Dash digital currency as a whole. It is also the layer that makes the governance and tresury decisions. 

Masternodes will get the standard return on their stakes. But will also be entitled to a portion of the transaction fees. Allowing for a greater ROI. 

#### Who does it?

- Dash 
- PIVX
- Crown 
- ExclusiveCoin 
- Helium 

#### Strengths

- Sustain and care of the ecosystem 
- 


#### Weaknesses

- In order to get a masternode,you have to invest 1000 Dash into it- and not touch that money (if balance drops below 1000 Dash, one can lose voting rights and effectively be blocked from the Dash masternode network. 




#### Opportunities

???

#### Threats

???

### #4 Loom Network
#### What is it?

???

#### Who does it?

???

#### Strengths

???

#### Weaknesses

???

#### Opportunities

???

#### Threats

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

#### Opportunities

???

#### Threats

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

#### Opportunities

???

#### Threats

???

## Observations

???

## References

[1] OSI mode, https://en.wikipedia.org/wiki/OSI_model, Date accessed: 2018-06-07.

[2] Decentralized Digital Identities and Blockchain – The Future as We See It, https://cloudblogs.microsoft.com/enterprisemobility/2018/02/12/decentralized-digital-identities-and-blockchain-the-future-as-we-see-it, Date accessed: 2018-06-07.

[3] Trinity Protocol: The Scaling Solution of the Future?, https://www.investinblockchain.com/trinity-protocol, Date accessed: 2018-06-08.

[4] , , Date accessed: 2018-06-??.

[5] NEX: A High Performance Decentralized Trade and Payment Platform, https://neonexchange.org/pdfs/whitepaper_v2.pdf, Date accessed: 2018-06-11.

[6] , , Date accessed: 2018-06-??.

[7] , , Date accessed: 2018-06-??.

[8] , , Date accessed: 2018-06-??.

[9] , , Date accessed: 2018-06-??.

[10] , , Date accessed: 2018-06-??.

[11] , , Date accessed: 2018-06-??.

[12] , , Date accessed: 2018-06-??.

[13] , , Date accessed: 2018-06-??.

[14] , , Date accessed: 2018-06-??.

[15] , , Date accessed: 2018-06-??.

[16] , , Date accessed: 2018-06-??.

[17] , , Date accessed: 2018-06-??.

[18] , , Date accessed: 2018-06-??.

[19] , , Date accessed: 2018-06-??.

[20] , , Date accessed: 2018-06-??.

[21] , , Date accessed: 2018-06-??.

[22] , , Date accessed: 2018-06-??.


