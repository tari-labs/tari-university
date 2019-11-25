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
- Multiparty UTXO, Pedersen Commitment Trick
- Refund Procedure
- Revoke Previous Refund
- Punishment!

<div class="LineHeight100per"> <br></div>
See full report [*here*](https://tlu.tarilabs.com/scaling/laser-beam/MainReport.html).

---

## Introduction

<div class="LineHeight20per"> <br></div>
@div[text-left]

PoW blockchains:

@divend

- Notoriously slow
- Txs need to be a number of blocks in the past to be confirmed
- Have poor scalability properties

<div class="LineHeight20per"> <br></div>
@div[text-left]

Payment channels:<br>

@divend

- 2(+) parties can make multiple blockchain Txs off-chain
- Only some Txs committed to the blockchain
- Final payout committed back to the blockchain

+++

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

- Laser Beam (still WIP) is an adaptation of the Lightning Network for [Mimblewimble](../../protocols/mimblewimble-1/MainReport.md) 

- Created by The Beam Team for Beam

- Currently demonstrates off-chain transactions in a single channel between two parties

- Plans to implement routing across different payment channels in the Lightning Network style

@divend

+++

@div[s900px]
![lightningnetwork](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/scaling/laser-beam/sources/refund_procedure.png)
@divend

---

## Multiparty UTXO, Pedersen Commitment Trick

<div class="LineHeight20per"> <br></div>

@div[text-left]
Remember...
@divend

<div class="mywrap">08c15e94ddea81e6a0a31ed558ef5e0574e5369c4fcba92808fe992fbff68884cc</div>

<div class="LineHeight20per"> <br></div>
<div class="LineHeight20per"> <br></div>

@div[text-left]

Normal Pedersen commitment: 

@divend

`
$$
C(v,k)=\Big(vH+kG\Big)
$$
`

@div[text-left]

MultiSig Pedersen commitment between 2 parties:

@divend

`
$$
C(v,k_{1}+k_{2})=\Big(vH+(k_{1}+k_{2})G\Big)
$$
`

@div[text-left]

Initial funding Tx:

@divend

`
$$
\begin{aligned}
-\text{Inputs}(0)+\text{MultiSig}(0)&+\text{fee} = \text{Excess}(0) \\\\
-\Big((v_{a}H+k_{a}G)+(v_{b}H+k_{b}G)\Big)&+\Big(v_{0}H+
    (k_{0_{a}}+k_{0_{b}})G\Big)+fH = \mathcal{X}_{0}
\end{aligned}
$$
`

---

## Refund Procedure

<div class="LineHeight20per"> <br></div>

@div[text-left]

**Alice Part 1:**

@divend

`
$$
\begin{aligned}
-\text{MultiSig}(0) &+ \text{MultiSig}(N)_{A} + \text{fee} = \text{Excess}(N)_{A1} \\\\
-\Big(v_{0}H+(k_{0_{a}}+k_{0_{b}})G\Big) &+ \Big((v_{0}-f)H + (\hat{k}_{N_{a}}+k_{N_{b}})G\Big) + fH 
  = \mathcal{X}_{N_{A1}}
\end{aligned}
$$
`

-> _Alice gets Bob's part of the signature, but keeps her part secret!_

<div class="LineHeight20per"> <br></div>

@div[text-left]

**Bob Part 1:**

@divend

`
$$
\begin{aligned}
-\text{MultiSig}(0) &+ \text{MultiSig}(N)_{B} + \text{fee} = \text{Excess}(N)_{B1} \\\\
-\Big(v_{0}H+(k_{0_{a}}+k_{0_{b}})G\Big) &+ \Big((v_{0}-f)H+(k_{N_{a}}+\hat{k}_{N_{b}})G\Big) + fH 
  = \mathcal{X}_{N_{B1}}
\end{aligned}
$$
`

-> _Bob gets Alice's part of the signature, but keeps his part secret!_

+++

@div[text-left]

**Alice Part 2:**

@divend

`
$$
\begin{aligned}
-\text{MultiSig}(N)_{A}&+\text{Outputs}(N)+\text{fee} =\text{Excess}(N)_{A2} \\\\
-\Big((v_{0}-f)H+(\hat{k}_{N_{a}}+k_{N_{b}})G\Big)&+\Big((v_{N_{a}}^{\prime}H+k_{N_{a}}^{\prime}G)+(v_{N_{b}}^
  {\prime}H+k_{N_{b}}^{\prime}G)\Big)+fH =\mathcal{X}_{N_{A2}}
\end{aligned}
$$
`

-> _Has relative time lock, Alice shares her part of the signature with Bob_

<div class="LineHeight20per"> <br></div>

@div[text-left]

**Bob Part 2:**

@divend

`
$$
\begin{aligned}
-\text{MultiSig}(N)_{B}&+\text{Outputs}(N)+\text{fee} =\text{Excess}(N)_{B2} \\\\
-\Big((v_{0}-f)H+(k_{N_{a}}+\hat{k}_{N_{b}})G\Big)&+\Big((v_{N_{a}}^{\prime}H+k_{N_{a}}^{\prime}G)+(v_{N_{b}}^
  {\prime}H+k_{N_{b}}^{\prime}G)\Big)+fH =\mathcal{X}_{N_{B2}}
\end{aligned}
$$
`

-> _Has relative time lock, Bob shares his part of the signature with Alice_


---

## Revoke Previous Refund

<div class="LineHeight20per"> <br></div>

@div[text-left]

**Alice:**

@divend

`
$$
\begin{aligned}
\text{MultiSig}(N-1)_{A}:\quad\Big((v_{0}-f)H+(\hat{k}_{(N-1)_{a}}+k_{(N-1)_{b}})G\Big) \quad &\lbrace\text{Alice's commitment}\rbrace \\
\hat{k}_{(N-1)_{a}} \quad &\lbrace\text{Alice shares with Bob}\rbrace \\
\Big((v_{0}-f)H+(\hat{k}_{(N-1)_{a}}+k_{(N-1)_{b}})G\Big) \overset{?}{=} C(v_{0}-f,\ \hat{k}_{(N-1)_{a}}+k_{(N-1)_{b}}) \quad &\lbrace\text{Bob verifies}\rbrace 
\end{aligned}
$$
`

@div[text-left]

**Bob:**

@divend

`
$$
\begin{aligned}
\text{MultiSig}(N-1)_{B}:\quad\Big((v_{0}-f)H+(k_{(N-1)_{a}}+\hat{k}_{(N-1)_{b}})G\Big) \quad &\lbrace\text{Bob's commitment}\rbrace \\
\hat{k}_{(N-1)_{b}} \quad &\lbrace\text{Bob shares with Alice}\rbrace \\
\Big((v_{0}-f)H+(k_{(N-1)_{a}}+\hat{k}_{(N-1)_{b}})G\Big) \overset{?}{=} C(v_{0}-f,\ k_{(N-1)_{a}}+\hat{k}_{(N-1)_{b}}) \quad &\lbrace\text{Alice verifies}\rbrace 
\end{aligned}
$$
`

---

## Punishment!

<div class="LineHeight20per"> <br></div>

@div[text-left]
- Must monitor to detect foul play
@divend

@div[text-left]
- Race against time!
@divend

@div[text-left]
- Alice tries her luck and create:
@divend

`
$$
\begin{aligned}
-\Big(v_{0}H+(k_{0_{a}}+k_{0_{b}})G\Big) &+ \Big((v_{0}-f)H + (\hat{k}_{(N-1)_{a}}+k_{(N-1)_{b}})G\Big) + fH 
  = \mathcal{X}_{(N-1)_{A}}
\end{aligned}
$$
`

@div[text-left]
- Bob jumps and claim all the funds!
@divend

