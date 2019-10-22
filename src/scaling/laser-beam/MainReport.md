# Laser Beam

## Introduction

Proof-of-Work (PoW) blockchains are slow and have poor scalability properties[[1]]]. A payment channel 
is a class of techniques designed to allow two or more parties to make multiple blockchain transactions, without 
committing all of the transactions to the blockchain. Resulting funds can be committed back to the blockchain. Payment 
channels allow multiple transactions to be made within off-chain agreements. They keep the operation mode of the 
blockchain protocol, but change the way in which it is used in order to deal with the challenge of scalability.

The Lightning Network is a second-layer payment protocol that was originally designed for Bitcoin, and which enables 
instant transactions between participating nodes. It features a peer-to-peer system for making micropayments through a 
network of bidirectional payment channels. The Lightning Network's dispute mechanism requires all users to constantly 
watch the blockchain for fraud. Various mainnet implementations that support Bitcoin exist and, with small tweaks, some 
of them are also able to support Litecoin ([[2]], [[3]], [[4]]).

Laser Beam is an adaptation of the Lightning Network for the Mimblewimble protocol, to be implemented for Beam 
([[5]], [[6]], [[7]]). At the time of writing of this report, the specifications were far advanced, but still work in 
progress. Beam has a working demonstration in its mainnet repository, which at this stage demonstrates off-chain 
transactions in a single channel between two parties [[8]]. According to the Request for Comment (RFC) documents, Beam 
does not plan to support multiparty (more than two) payment channels, but rather to implement routing across different 
payment channels in the Lightning Network style.

## Detail Scheme

Beam's version of a multisignature (multisig) is actually a $2\text{-of-}2$ multiparty UTXO, where each party keeps its 
share of the blinding factor of the Pedersen commitment, $C(v,k_{1}+k_{2})=(vH+(k_{1}+k_{2})G)$, secret. (Refer to 
Appendix A for notations used.) The multiparty commitment is accompanied by a single multiparty Bulletproof range proof, 
not an aggregated Bulletproof range proof, where the individual shares of the blinding factor are used to create the 
combined range proof [[9]].

### Funding Transaction

The parties collaborate to create the multiparty UTXO (i.e. commitment and associated multiparty range proof), combined 
funding transaction (on-chain) and an initial refund transaction for each party (off-chain). All refund transactions 
have a relative time lock in their kernel, referencing the kernel of the original combined funding transaction, which 
has to be confirmed on the blockchain.

The initial funding transaction between Alice and Bob, the $0_{\text{\text{th}}}$ refund procedure, is depicted in 
[\[eq:M0-Tx\]](#eq:M0-Tx){reference-type="eqref" reference="eq:M0-Tx"}. Alice's and Bob's contributions are denoted by 
subscripts $_{a}$ and $_{b}$ respectively. The capitalized use of $R$ and $P$ denotes public nonce and public blinding 
factor respectively; $f$ is the fee; and $\mathcal{X}$ is the excess. The lock height in the signature challenge 
corresponds to the current blockchain height.

$$\begin{aligned}
    \begin{aligned}-\text{Inputs\_}0+\text{MultiSig\_}0+\text{fee}\end{aligned}
                                                                                  & =\text{excess\_}0\label{eq:M0-Tx} \\
    -\left(\left(v_{a}H+k_{a}G\right)+\left(v_{b}H+k_{b}G\right)\right)+\left(v_{0}H+\left(k_{0_{a}}+k_{0_{b}}\right)G\right)+fH 
                                                                              & =\mathcal{X}_{0}\nonumber
\end{aligned}
\mspace{100mu} (5)
$$

Alice and Bob also need to set up their own respective refund transaction so they can be compensated should the channel 
never be used; this is done via a refund procedure. A refund procedure (off-chain) consists of four parts, whereby each 
user creates two transactions: one kept partially secret (discussed below) and the other shared. Each partially secret 
transaction creates a different intermediate multiparty UTXO, which is then used as input in two shared transactions, 
to pay the same set of outputs to each participant.

All consecutive refund procedures work in exactly the same manner. In the equations to follow, double subscripts 
$_{AA}$, $_{AB}$, $_{BA}$ and $_{BB}$ have the following meaning; the 1st letter indicates who controls the transaction 
the term is part of and the 2nd letter whom created the value. Blinding factors denoted by $\hat{k}$ have a special 
purpose, discussed later on, so that $\hat{k}_{N_{a}}\neq k_{N_{a}}$. The $N_{\text{\text{th}}}$ refund procedure is 
as follows:

### Refund Procedure Part 1 - Alice

Alice and Bob set up Alice's intermediate multisig funding transaction, spending the original funding multisig UTXO. The 
lock height $h_{N}$ corresponds to the current blockchain height. They collaborate to create $\text{MultiSig\_}N_{A}$, 
the challenge and Bob's potion of the signature, but Alice does not share the kernel and thus keeps her part of the 
final aggregated signature hidden.

$$\begin{aligned}
    \begin{aligned}\begin{aligned}-\text{MultiSig\_}0+\text{MultiSig\_}N_{A}+\text{fee}\end{aligned}
    \end{aligned}
     & =\text{excess\_}N_{A1}                                                                                                     \\
    \begin{aligned}\begin{aligned}-\left(v_{0}H+\left(k_{0_{a}}+k_{0_{b}}\right)G\right)+\left(v_{0}H+\left(\hat{k}_{N_{a}}+k_{N_{b}}\right)G\right)+fH\end{aligned}
    \end{aligned}
     & =\mathcal{X}_{N_{A1}}\nonumber                                                                                                     \\
    \text{Challenge:}\;\mathcal{H}\left(R_{N_{AA1}}+R_{N_{AB1}}\parallel P_{N_{AA1}}+P_{N_{AB1}}\parallel f\parallel h_{N}\right) \\
    \text{Signature tuple:}\;\left(s_{N_{AA1}}+s_{N_{AB1}},R_{N_{AA1}}+R_{N_{AB1}}\right)                                         \\
    \text{Kernel of this transaction, kept secret:}\quad\mathcal{K}_{N_{AA1}}\end{aligned}$$

### Refund Procedure Part 2 - Alice

Alice and Bob set up a refund transaction, which Alice controls, with a relative time lock $h_{rel}$ to the intermediate 
funding transaction's kernel. She shares the final kernel with Bob.

$$\begin{aligned}
    \begin{aligned}\begin{aligned}-\text{MultiSig.}N_{A}+\text{Outputs\_}N+\text{fee}\end{aligned}
    \end{aligned}
     & \text{=\text{excess\_}\ensuremath{N_{A2}}} \\
    \begin{aligned}\begin{aligned}-\left(v_{0}H+\left(\hat{k}_{N_{a}}+k_{N_{b}}\right)G\right)+\left(\left(v_{a_{N}}H+k_{a_{N}}G\right)+\left(v_{b_{N}}H+k_{b_{N}}G\right)\right)+fH\end{aligned}
    \end{aligned}
     & =\mathcal{X}_{N_{A2}}\nonumber                     \\
    \text{Challenge:}\;\mathcal{H}\left(R_{N_{AA2}}+R_{N_{AB2}}\parallel P_{N_{AA2}}+P_{N_{AB2}}\parallel f\parallel\mathcal{H}\left(\mathcal{K}_{N_{AA1}}\right)\parallel h_{rel}\right)\end{aligned}$$

### Refund Procedure Part 1 - Bob

Alice and Bob set up Bob's intermediate multisig funding transaction, also spending the original funding multisig UTXO. 
The lock height $h_{N}$ again corresponds to the current blockchain height. They collaborate to create 
$\text{MultiSig\_}N_{B}$, the challenge and Bob's potion of the signature, but Bob does not share the kernel and thus 
keeps his part of the final aggregated signature hidden.

$$\begin{aligned}
    \begin{aligned}\begin{aligned}-\text{MultiSig\_}0+\text{MultiSig\_}N_{B}+\text{fee}\end{aligned}
    \end{aligned}
     & =\text{excess\_}N_{B1}                                                                                                     \\
    \begin{aligned}\begin{aligned}-\left(v_{0}H+\left(k_{0_{a}}+k_{0_{b}}\right)G\right)+\left(v_{0}H+\left(k_{N_{a}}+\hat{k}{}_{N_{b}}\right)G\right)+fH\end{aligned}
    \end{aligned}
     & =\mathcal{X}_{N_{B1}}\nonumber                                                                                                     \\
    \text{Challenge:}\;\mathcal{H}\left(R_{N_{BA1}}+R_{N_{BB1}}\parallel P_{N_{BA1}}+P_{N_{BB1}}\parallel f\parallel h_{N}\right) \\
    \text{Signature tuple:}\;\left(s_{N_{BA1}}+s_{N_{BB1}},R_{N_{BA1}}+R_{N_{BB1}}\right)                                         \\
    \text{Kernel of this transaction, kept secret:}\quad\mathcal{K}_{N_{BB1}}\end{aligned}$$

### Refund Procedure Part 2 - Bob

Alice and Bob set up a refund transaction, which Bob controls, with a relative time lock $h_{rel}$ to the intermediate 
funding transaction. He shares the final kernel with Alice.

$$\begin{aligned}
    \begin{aligned}\begin{aligned}-\text{MultiSig\_}N_{B}+\text{Outputs\_}N+\text{fee}\end{aligned}
    \end{aligned}
     & \text{=\text{excess\_}\ensuremath{N_{B2}}} \\
    \begin{aligned}\begin{aligned}-\left(v_{0}H+\left(k_{N_{a}}+\hat{k}_{N_{b}}\right)G\right)+\left(\left(v_{a_{N}}H+k_{a_{N}}G\right)+\left(v_{b_{N}}H+k_{b_{N}}G\right)\right)+fH\end{aligned}
    \end{aligned}
     & =\mathcal{X}_{N_{B2}}\nonumber                     \\
    \text{Challenge:}\;\mathcal{H}\left(R_{N_{BA2}}+R_{N_{BB2}}\parallel P_{N_{BA2}}+P_{N_{BB2}}\parallel f\parallel\mathcal{H}\left(\mathcal{K}_{N_{BB1}}\right)\parallel h_{rel}\right)\end{aligned}$$

### Revoke Previous Refund

Whenever the individual balances in the channel change, a new refund procedure is negotiated, revoking previous 
agreements. Revoking refund transactions involves revealing blinding factor shares for the intermediate multiparty 
UTXOs, thereby nullifying their further use. After the four parts of the refund procedure have been concluded 
successfully, the previous round's blinding factor shares $\hat{k}_{\left(N-1\right)}$ are revealed to each other, in 
order to revoke the previous agreement.

$$\begin{aligned}
    \text{MultiSig.}\left(N-1\right)_{A}:\quad\left(v_{0}H+\left(\hat{k}{}_{\left(N-1\right)_{a}}+k{}_{\left(N-1\right)_{b}}\right)G\right) & \begin{aligned}\quad & \text{\{Alice's commitment\}}\end{aligned}
    \\
    \hat{k}{}_{\left(N-1\right)_{a}}:                                                                                                       & \begin{aligned}\quad & \text{\{Alice shares with Bob\}}\end{aligned}
    \nonumber                                                                                                                                                            \\
    \text{MultiSig.}\left(N-1\right)_{B}:\quad\left(v_{0}H+\left(k_{\left(N-1\right)_{a}}+\hat{k}{}_{\left(N-1\right)_{b}}\right)G\right)   & \begin{aligned}\quad & \text{\{Bob's commitment\}}\end{aligned}
    \\
    \hat{k}{}_{\left(N-1\right)_{b}}:                                                                                                       & \begin{aligned}\quad & \text{\{Bob shares with Alice\}}\end{aligned}
    \nonumber\end{aligned}$$

Although each party will now have its counterparty's blinding factor share in the counterparty's intermediate multiparty 
UTXO, it will still not be able to spend it, because the transaction kernel is still kept secret by the counterparty.

### Punishment Transaction

If a counterparty decides to broadcast a revoked set of refund transactions, and the honest party is actively monitoring 
the blockchain and able to detect the attempted foul play, a punishment transaction can immediately be constructed 
before the relative time lock $h_{rel}$ expires. Whenever the counterparty's $\text{multisig.}\left(N-1\right)$ becomes 
available in the blockchain, the honest party can spend all the funds to its own output, because it knows the total 
blinding factor.

### Channel Closure

Whenever the parties agree to a channel closure, the original on-chain multiparty UTXO is spent to their respective 
outputs in a collaborative transaction. In the event of a single party deciding to close the channel unilaterally for 
whatever reason, its latest refund transaction is broadcast, effectively closing the channel.

Opening a channel requires one collaborative funding transaction on the blockchain. Closing a channel involves each 
party broadcasting its respective portion of the refund transaction to the blockchain, or collaborating to broadcast a 
single settlement transaction. A round-trip open channel, multiple off-chain spending and close channel thus involves, 
at most, three on-chain transactions.

## Appendices

### Appendix A: Notation Used

- Let $p$ be a large prime number.

- Let $\mathbb{Z}_{p}$ denote the ring of integers $modulo\;p$.

- Let $\mathbb{F}_{p}$ be the group of elliptic curve points.

- Let $G\in\mathbb{F}_{p}$ be a random generator point (base point) and let $H\in\mathbb{F}_{p}$ be specially chosen 
  so that the value $x_{H}$ to satisfy $H=x_{H}G$ cannot be found, except if the Elliptic Curve Discrete Logarithm 
  Problem (ECDLP) is solved.

- Let commitment to value $v\in\mathbb{Z}_{p}$ be determined by calculating $C(v,k)=(vH+kG)$, which is called the 
  Elliptic Curve Pedersen Commitment (Pedersen Commitment), with $k\in\mathbb{Z}_{p}$ (the blinding factor) a random 
  value.

- Let scalar multiplication be depicted by $\cdot$, e.g. $e\cdot(vH+kG)=e\cdot vH+e\cdot kG$.

## References

[[1]] J. A. Odendaal, "Layer 2 Scaling Survey - Tari Labs University." \[Online\]. Available: 
<https://tlu.tarilabs.com/scaling/layer2scaling-landscape/layer2scaling-survey.html>. \[Accessed: 06-Oct-2019\].

[[2]] J. Poon, T. Dryja (2016). The Bitcoin Lightning Network: Scalable Off-Chain Instant PaymentsOnline\]. Available: 
<http://lightning.network/lightning-network-paper.pdf>. \[Accessed: 4-Jul-2019\].

[[3]] GitHub: lightningnetwork/lightning-rfc: Lightning Network SpecificationsOnline\]. Available: 
<https://github.com/lightningnetwork/lightning-rfc>. \[Accessed: 4-Jul-2019\].

[[4]] X. Wang, "What is the situation of Litecoin's Lightning Network now?" \[Online\]. Available: 
<https://coinut.com/blog/whats-the-situation-of-litecoins-lightning-network-now/>. \[Accessed: 11-Sep-2019\].

[[5]] The Beam Team, GitHub: Lightning Network - BeamMW/beam WikiOnline\]. Available: 
<https://github.com/BeamMW/beam/wiki/Lightning-Network>. \[Accessed: 5-Jul-2019\].

[[6]] F. Jahr, Beam - Lightning network position paper. (v 1.0)Online\]. Available: 
<https://docs.beam.mw/Beam\_lightning\_network\_position\_paper.pdf>. \[Accessed: 4-Jul-2019\].

[[7]] F. Jahr, GitHub: fjahr/lightning-mw, Lightning Network SpecificationsOnline\]. Available: 
<https://github.com/fjahr/lightning-mw>. \[Accessed: 4-Jul-2019\].

[[8]] The Beam Team, GitHub: beam/node/laser\_beam\_demo at master - BeamMW/beamOnline\]. Available: 
<https://github.com/BeamMW/beam/tree/master/node/laser\_beam\_demo>. \[Accessed: 5-Jul-2019\].

[[9]] The Beam Team, GitHub: beam/ecc\_bulletproof.cpp at mainnet - BeamMW/beamOnline\]. Available: 
<https://github.com/BeamMW/beam/blob/mainnet/core/ecc\_bulletproof.cpp>. \[Accessed: 5-Jul-2019\].

