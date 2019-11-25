<head>
<style>
div.LineHeight20per {
  line-height: 20%;
}
div.LineHeight100per {
  line-height: 100%;
}
div.LineHeight200per {
  line-height: 200%;
}
div.mywrap {
  width: 95%; 
  word-wrap: break-word;
  background: #f1f1f1;
  font-size: 0.7em;
  font-family: "Source Code Pro", Consolas, "Ubuntu Mono", Menlo, "DejaVu Sans Mono", monospace, monospace;
  padding: 0.25em;
  color: #6e6b5e;
}
</style>
</head>


## Laser Beam

- Introduction
- Lightning Network
- Laser Beam Overview
- ???
  - ???
  - ???
- Conclusions

<div class="LineHeight100per"> <br></div>
See full report [*here*](https://tlu.tarilabs.com/scaling/laser-beam/MainReport.html).

---

## Introduction

<div class="LineHeight20per"> <br></div>

@div[text-left]

PoW blockchains:

- Notoriously slow
- Txs need to be a number of blocks in the past to be confirmed
- Have poor scalability properties

<div class="LineHeight20per"> <br></div>
Payment channels:

- 2(+) parties can make multiple blockchain Txs off-chain
- Only some Txs committed to the blockchain
- Final payout committed back to the blockchain

@divend

---

@div[s800px]
![layer2scaling](https://github.com/tari-labs/tari-university/raw/master/src/scaling/layer2scaling-landscape/sources/layer2scaling.png)
@divend

---

## Lightning Network

<div class="LineHeight20per"> <br></div>

@div[text-left]

2nd-layer payment protocol originally designed for Bitcoin

@divend

<div class="LineHeight20per"> <br></div>

@div[s800px]
![lightningnetwork](https://github.com/tari-labs/tari-university/raw/master/src/scaling/layer2scaling-landscape/sources/bitcoin-lightning-network-basic.png)
@divend

<div class="LineHeight20per"> <br></div>
@div[text-left]

Dispute mechanism requires all users to constantly watch the blockchain for fraud

@divend

---

## Laser Beam Overview

<div class="LineHeight20per"> <br></div>

@div[text-left]

Laser Beam (still WIP) is an adaptation of the Lightning Network for [Mimblewimble](../../protocols/mimblewimble-1/MainReport.md) 

<div class="LineHeight20per"> <br></div>

Created by The Beam Team for Beam

<div class="LineHeight20per"> <br></div>

Currently demonstrates off-chain transactions in a single channel between two parties

<div class="LineHeight20per"> <br></div>

Pans to implement routing across different payment channels in the Lightning Network style

@divend

---

@div[s900px]
![lightningnetwork](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/scaling/laser-beam/sources/refund_procedure.png)
@divend


---

## Conclusions

@div[text-left]
???
@divend

- ???
- ???

@div[text-left]
???
@divend

- ???
- ???

