## Confidential Assets

<br>

@div[text-left]

*Confidential assets have value, can be owned (with the right to use) but has no physical presence (intangible). It is stored in digital form or in an online account. The confidentiality aspect implies that the amount of assets owned as well as the asset type that was transacted in can be confidential.*

@divend

<br>

- Preliminaries
- Confidential Transactions Overview
- Asset Commitments and Surjection Proofs
- Asset Transactions
- Asset Issuance, Reissuance
- Implementations
  - Elements Project
  - Chain Core Confidential Assets
  - Cloak
- Conclusions

---

## Preliminaries

Mathematics ????

General notation

- $ p $ will be a large prime number
- $ \mathbb G $ will denote a cyclic group of prime order $ p $ 
- $ \mathbb Z_p $ will denote the ring of integers $ modulo \mspace{4mu} p $ 
- $ \mathbb F_p $ will be a group of elliptic curve points over a finite (prime) field
- Elliptic curve arithmetic: lower case letters -> ordinary numbers (integers), upper case letters -> curve points

Ricardian contract ???

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

## Elements Project

???

+++


## Chain Core Confidential Assets

???

+++


## Cloak

???

---

## Conclusions

???
