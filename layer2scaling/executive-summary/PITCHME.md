## Scaling Executive Summary

- Scaling Landscape

- How will this be applicable to Tari?

- Scaling Context for Tari

- Scaling Alternatives for Tari

- Observations



*See Layer 2 Scaling Survey [part 1](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md) and [part 2](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/more-landscape/landscape-update.md) for the full reports.*

---

## Scaling Landscape

- Layer 2 Scaling can be roughly grouped into these groups:
  - Off-chain matching engines
  - Off-chain processing nodes
  - Off-chain payment channels
  - Two Way Pegged (2WP) secondary chains
  - Tiered block chains



![L2ScalingLandscape](https://raw.githubusercontent.com/tari-labs/tari-university/L2ScalingUpdate/layer2scaling/executive-summary/sources/L2ScalingLandscape.png)

+++

- Other scaling technologies investigated, mainly directly on the primary block chain
  - DAG derivative protocols
  - Transaction data compression
  - Scriptless scripts

![Non_L2S](https://raw.githubusercontent.com/tari-labs/tari-university/L2ScalingUpdate/layer2scaling/executive-summary/sources/Non_L2S.png)

---

## How will this be Applicable to Tari?

- Tari is a high-throughput protocol -> tens of thousands of transactions per second imagined
- This will be impossible to do with primary block chain scaling solutions alone
- Example use cases:
  - EventCorp, the stadium owner
    - Selling: Do not let the servers crash!
    - Redeeming: 
      - 85,000 seats, 72 queues tickets on match days
      - scanning 500 tickets in 4 minutes, that is ~2 spectators allowed access per second per queue
  - Steve, the collectable card game entrepreneur
    - Steve created his own digital collectible
    - Steve developed a website with the Tari API to display his digital cards, update their status in real time, and facilitate trading and interacting

---

## Scaling Context for Tari

- Scaling technologies worth investigating further 
  - *TumbleBit,* as an off-chain matching engine (L2S)
  - *Federated Nodes/Masternodes* as off-chain processing nodes (L2S)
  - *Scriptless Scripts & Schnorr Signature Aggregation*, as Layer 1 scaling
  - *SPECTRE, PHANTOM*, as DAG derivative protocol alternative to a traditional block chain, also Layer 1 scaling

![L2ContextTari](https://raw.githubusercontent.com/tari-labs/tari-university/L2ScalingUpdate/layer2scaling/executive-summary/sources/L2ContextTari.png)

---

#### #1 Why TumbleBit?

![TumbleBitOverview](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/TumbleBitOverview.PNG)

+++

#### #1 Why TumbleBit? (cont'd)

- TumbleBit has many excellent properties as trustless matching engine
- TumbleBit can perform off-chain payments in batch mode
- Commercial implementation NTumbleBit far advanced, backed by Boston University that provided proof-of-concept and reference implementation alongside white paper
- Anonymity & bad acting prevention provided by *RSA-Puzzle-Solver Protocol* & *Puzzle-Promise Protocol*, making use of RSA crypto blinding properties
- TumbleBit also supports anonymizing through Tor

---

#### #2 Why Federated Nodes/Masternodes?

![CounterpartyStack](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/CounterpartyStack.png)

+++

#### #2 Why Federated Nodes/Masternodes?

- A federated node is a special case of a masternode, with emphasis on the federated trust model.
- Federated Nodes provides a protocol and network of nodes for creating smart contract applications using a customized virtual machine or other mechanism and linked to the primary block chain.
- All smart contracts and their state updates are executed and maintained off-chain in the federated nodes.
- The Federated Node software stack model lends itself for high volume processing.
- Federated Nodes does not have to use embedded consensus (*although Counterparty does*); improved **consensus models like Federated Byzantine Agreement (FBA) can be implemented**.

---

#### #3 Why Scriptless Scripts & Schnorr Sig. Aggregation?

![Mimblewimble](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/Mimblewimble.PNG)

+++

#### #3 Why Scriptless Scripts & Schnorr Sig. Aggregation? (cont'd)

- <u>Data savings:</u> Signature aggregation using an appropriate Schnorr-based multi-signature scheme (*e.g. MuSig*) provides data compression on the block chain
- <u>Privacy:</u> Nothing about the *Scriptless Script* smart contract, other than the settlement transaction,  is ever recorded on the block chain. No one will ever know that an underlying smart contract was executed.
- <u>Multiplicity:</u> Multiple digital assets can be transferred between two parties in a single settlement transaction.
- <u>Implicit scalability:</u> Scalability on the block chain is achieved by virtue of compressing multiple transactions into a single settlement transaction. Transactions are only broadcasted to the block chain once all preconditions are met.

+++

#### #3 Why Scriptless Scripts & Schnorr Sig. Aggregation? (cont'd)

- Signature aggregation properties sought here are:
  - Must be provably secure in the *plain public-key model*;
  - Must satisfy the normal Schnorr equation, whereby the resulting signature can be written as a function of a combination of the public keys; 
  - Must allow for Interactive Aggregate Signatures (IAS) where the signers are required to cooperate;
  - Must allow for Non-interactive Aggregate Signatures (NAS) where the aggregation can be done by anyone;
  - Must allow each signer to sign the same message;
  - Must allow each signer to sign their own message.

Note: 

(1) Plain public-key model: allows for signature aggregation, mathematically combining several signatures into a single signature, without having to prove Knowledge of Secret Keys (KOSK). 

(2) KOSK requires that users prove knowledge (or possession) of the secret key during public key registration with a certification authority.

+++

#### #3 Why Scriptless Scripts & Schnorr Sig. Aggregation? (cont'd)

- Possible software stack:
  - The Mimblewimble *Scriptless Script*s logic could be implemented by Federated Nodes with FBA on layer 2
  - The MuSig Schnorr-based multi-signature scheme with key aggregation can be used
  - Secrets revealed by virtue of the MuSig Schnorr signatures can instantiate normal smart contracts inside the Federated Nodes, with intermediate state updates confirmed by FBA

+++

#### #3 Why Scriptless Scripts & Schnorr Sig. Aggregation? (cont'd)

- - Challenges: 
  - No space allowed to embed data other than Tx inputs and outputs in the form of coin amounts
  - Smart contract state updates can't be written back to the block chain after the event
  - Immediate cut-through (& pruning) may delete transactions intended to be persistent
  - HTLC not supported
  - Standard Mimblewimble Tx does not support signaling a Federated Node

---

#### #4 Why SPECTRE, PHANTOM?

![SPECTRE](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/SPECTRE.PNG)

+++

#### #4 Why SPECTRE, PHANTOM? (cont'd)

- DAG derivative protocols SPECTRE and PHANTOM offer an alternative to a block chain, i.e. block DAG
- Strengths:
  - <u>Layer 1 scaling:</u> Increased transaction throughput on the main block chain

  - <u>Fairness:</u> Better payoff for weak miners
  - <u>Decentralization mitigation:</u> Weaker miners also get profits
  - <u>Transaction confirmation times:</u> Confirmation times of several seconds (SPECTRE)
  - <u>Smart contracts:</u> Support smart contracts (PHANTOM)

+++

#### #4 Why SPECTRE, PHANTOM? (cont'd)

SPECTRE:
- High throughput and fast confirmation times [***GOOD***]
- Weak liveness for conflicting transactions [***BAD***]
- DAG structure represents an abstract vote regarding the order between each pair of blocks [***BAD***]

PHANTOM:
- Confirmation times are mush slower than those in SPECTRE [***BAD***]
- Strong liveness for conflicting transactions [***GOOD***]
- Linear ordering over the blocks of the DAG and can support consensus regarding any general computation (smart contracts) [***GOOD***]

#### 

#### 

SPECTRE and PHANTOM can be combined.

Note:

(1) Strong liveness: all conflicts decided in finite time

(2) Weak liveness: no guarantee a resolution can be reached for conflicting transactions published soon one after the other

---

## Observations

Not all protocols presented here have production or even reference implementations 

Careful consideration about each aspect of these technologies and their applicability to the Taro protocol are required 