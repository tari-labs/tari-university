## Layer 2 Scaling Executive Summary

- Layer 2 Scaling Landscape

- How will this be applicable to Tari?

- Layer 2 Scaling Context for Tari

- Layer 2 Scaling Alternatives for Tari

- Observations



*See Layer 2 Scaling Survey [part 1](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md) and [part 2](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/more-landscape/landscape-update.md) for the full reports.*

---

## Layer 2 Scaling Landscape

- L2S can be roughly grouped into these groups:
  - Off-chain matching engines
  - Off-chain processing nodes
  - Off-chain payment channels
  - Two Way Pegged (2WP) secondary chains
  - Tiered block chains



![L2ScalingLandscape](https://raw.githubusercontent.com/tari-labs/tari-university/L2ScalingUpdate/layer2scaling/executive-summary/sources/L2ScalingLandscape.png)



- Other technologies investigated, mainly directly on the primary block chain
  - DAG derivative protocols
  - Transaction data compression
  - Scriptless scripts

![Non_L2S](https://raw.githubusercontent.com/tari-labs/tari-university/L2ScalingUpdate/layer2scaling/executive-summary/sources/Non_L2S.png)

---

## How will this be Applicable to Tari?

- Tari is a high-throughput protocol -> tens of thousands of transactions per second imagined
- This will be impossible to do with primary block chain scaling solutions alone
- Use cases:
  - EventCorp, the stadium owner
    - Selling: Do not let the servers crash!
    - Redeeming: 
      - 85,000 seats, 72 queues tickets on match days
      - scanning 500 tickets in 4 minutes, that is ~2 spectators allowed access per second per queue
  - Steve, the collectable card game entrepreneur
    - Steve created his own digital collectible
    - Steve developed a website with the Tari API to display his digital cards, update their status in real time, and facilitate trading and interacting

---

## Layer 2 Scaling Context for Tari

- Proposal going forward
  - <u>TumbleBit,</u> as an off-chain matching engines
  - <u>Federated Nodes/Masternodes,</u> as off-chain processing nodes 
  - <u>Scriptless Scripts & Schnorr Signature Aggregation</u>, as Layer 1 scaling
  - <u>SPECTRE, PHANTOM</u>, as DAG derivative protocol alternative to a traditional block chain, also Layer 1 scaling

![L2ContextTari](https://raw.githubusercontent.com/tari-labs/tari-university/L2ScalingUpdate/layer2scaling/executive-summary/sources/L2ContextTari.png)

---

## Layer 2 Scaling Context for Tari (cont'd)

#### #1 Why TumbleBit?

- TumbleBit has many excellent properties as trustless matching engine
- TumbleBit can perform off-chain payments in batch mode
- Commercial implementation NTumbleBit far advanced, backed by Boston University that provided proof-of-concept and reference implementation alongside white paper
- Anonymity & bad acting prevention provided by *RSA-Puzzle-Solver Protocol* & *Puzzle-Promise Protocol*, making use of RSA crypto blinding properties
- TumbleBit also supports anonymizing through Tor

+++

#### #2 Why Federated Nodes/Masternodes?

- ???

#####  
#####  

+++

#### #2 Why Federated Nodes/Masternodes? (cont'd)

#####  

---

#### #3 Why Scriptless Scripts & Schnorr Signature Aggregation?

- ???

#####  

+++

#### #3 Why Scriptless Scripts & Schnorr Signature Aggregation? (cont'd)

#####  
#####  
---

#### #4 Why SPECTRE, PHANTOM?

- ???

#####  
#####  
+++

#### #4 Why SPECTRE, PHANTOM? (cont'd)

#####  
#####  
---

## Observations

???

???