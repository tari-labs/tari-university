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

@div[text-left]

**General notation**

@divend

- $ p ​$ will be a large prime number
- $ \mathbb G $ will denote a cyclic group of prime order $ p $ 
- $ \mathbb Z_p $ will denote the ring of integers $ modulo \mspace{4mu} p $ 
- $ \mathbb F_p $ will be a group of elliptic curve points over a finite (prime) field
- Lower case letters -> ordinary numbers (integers), upper case letters -> curve points

@div[text-left]

<u>Discrete Logarithm Problem (DLP):</u> The discrete logarithm $ \log_ba = k $ such that $ b^k=a $ for any integer $ k $ where $ a,b \in \mathbb G $ is hard to guess (has no efficient solution) for carefully chosen $  \mathbb F_p $. 

@divend

---

## Confidential Transactions Overview

EC Pedersen Commitment to value $ x \in \mathbb Z_p $ has the following form $ C(x,r) = xH + rG $

Confidential transactions -> replace each UTXO with a homomorphic commitment (i.e. Pedersen Commitment), and made robust against overflow and inflation attacks by using efficient ZK range proofs (i.e. Bulletproofs)

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

<p align="center"><a name="fig_rc"> </a><img src="https://raw.githubusercontent.com/tari-labs/tari-university/master/src/digital-assets/confidential-assets/sources/ricardian_contract.png" width="1050" /></p>


+++

@div[left-50 text-left]

<u>Ricardian Contract:</u>

- Human readable;
- Document is printable;
- Program parsable;
- All forms (displayed, printed, parsed) are manifestly equivalent;
- Signed by issuer;
- Can be identified securely, where security means that any attempts to change the linkage between a reference and the contract are impractical.

@divend

@div[right-50 text-left]

<u>Smart Contract</u>

- Self-executing (of course, it means that they don’t run unless someone initiates them)
- Immutable
- Self-verifying
- Auto-enforcing
- Cost saving
- Removes third parties or escrow agents

@divend

---

## Conclusions

???
