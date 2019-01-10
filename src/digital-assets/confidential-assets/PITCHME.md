## Confidential Assets

<br>

@div[text-left]

*Confidential assets have value, can be owned (right to use), has no physical presence (intangible), stored in digital form/online account. The confidentiality aspect implies that the amount of assets owned & the asset type transacted in can be confidential.*

@divend

<br>

- Preliminaries
- Confidential Transactions Overview
- Asset Commitments and Surjection Proofs
- Asset Transactions
- Asset Issuance, Reissuance
- Implementations
- Ricardian Contracts vs. Smart Contracts
- Conclusions

---

## Preliminaries

<br>

@div[text-left]

**General notation**

- Let $ p $ be a large prime number
- Let $ \mathbb G $ denote a cyclic group of prime order $ p $ 
- Let $ \mathbb Z_p $ denote the ring of integers $ modulo \mspace{4mu} p $ 
- Let $ \mathbb F_p $ be a group of elliptic curve points over a finite (prime) field
- If not otherwise specified, lower case $ x,r,y $ etc. are ordinary numbers (integers), upper case $ H,G $ are curve points

<br>

<u>Discrete Logarithm Problem (DLP):</u> The discrete logarithm $ \log_ba = k $ such that $ b^k=a $ for any integer $ k $ where $ a,b \in \mathbb G $ is hard to guess (has no efficient solution) for carefully chosen $  \mathbb F_p $. 

@divend

---

## Confidential Transactions Overview

@div[text-left]

<u>Confidential transactions</u> made confidential by replacing each UTXO with a homomorphic commitment (e.g. Pedersen Commitment), and made robust against overflow and inflation attacks by using efficient ZK range proofs (e.g. Bulletproofs).

<br>

<u>Range proofs</u> provide proof that secret committed value lies in certain interval, prevents numbers coming near magnitude of large prime, say $ 2^{256} $, that can cause wrap around when adding a small number, e.g. proof that a number $ x \in [0,2^{64} - 1] $.

<br>

<u>Pedersen Commitments</u> are perfectly hiding (an attacker with infinite computing power cannot tell what amount has been committed to) and computationally binding (no efficient algorithm running in a practical amount of time can produce fake commitments except with small probability).

<br>

<u>Elliptic Curve (EC) Pedersen Commitment</u> to value $ x \in \mathbb Z_p $ is $ C(x,r) = xH + rG $ where $ r \in  \mathbb Z_p $ is a random blinding factor, $ G \in  \mathbb F_p $ is a random generator point and $ H \in  \mathbb F_p $ specially chosen so that $ x_H $ satisfying $ H = x_H G $ cannot be found except if the EC DLP is solved. In secp256k1 $ H $ is the SHA256 hash of simple encoded $ x $-coordinate of generator point $ G $.

@divend



---

## Asset Commitments and Surjection Proofs

???

---

## Asset Transactions

???

---

## Asset Issuance, Reissuance

???

---

## Flexibility

???

---

## Implementations

  - Elements Project
  - Chain Core Confidential Assets
  - Cloak

+++

### Elements Project

???

+++


### Chain Core Confidential Assets

???

+++


### Cloak

???

---


## Ricardian Contracts vs. Smart Contracts

@div[text-left]

“*<u>Ricardian Contract:</u> A digital contract that deﬁnes the terms and conditions of an interaction, between two or more peers, that is cryptographically signed and veriﬁed, being both human and machine readable and digitally signed.*”

<br>
<br>

Ricardian contracts are robust (due to identification by cryptographic hash functions), transparent (due to readable text for legal prose) and efficient (due to computer markup language to extract essential information).

<br>
<br>

“*<u>Smart Contract</u>: A computerized transaction protocol that executes the terms of a contract. The general objectives are to satisfy common contractual conditions.*”

<br>
<br>

With smart contracts, digital assets can be exchanged in a transparent and non-conflicting way; it provides trust. 

@divend

+++

@div[s750px]

![Ricardian Contract](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/digital-assets/confidential-assets/sources/ricardian_contract.png)

@divend

+++

@div[left-50 text-left]

<u>Ricardian Contract:</u>

<ul>
<li>Human readable;
<li>Document is printable;
<li>Program parsable;
<li>All forms (displayed, printed, parsed) are manifestly equivalent;
<li>Signed by issuer;
<li>Can be identified securely, where security means that any attempts to change the linkage between a reference and the contract are impractical.
</ul>

@divend

@div[right-50 text-left]

<u>Smart Contract</u>

<ul>
<li>Self-executing (of course, it means that they don’t run unless someone initiates them)
<li>Immutable
<li>Self-verifying
<li>Auto-enforcing
<li>Cost saving
<li>Removes third parties or escrow agents
</ul>

@divend

---

## Conclusions

???
