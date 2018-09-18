# Bulletproofs and Mimblewimble

## Introduction

Bulletproofs is a non-interactive zero-knowledge proof protocol for general arithmetic circuits with very short proofs (arguments of knowledge[^*]) and without requiring a trusted setup. The name 'Bulletproofs' originated from a non-technical summary of the scheme's properties: "short like a bullet with bulletproof security assumption". [[12]][12]

Bulletproofs form part of the family of distinct zero-knowledge proof systems, like Zero-Knowledge Succinct Non-Interactive ARguments of Knowledge (zk-SNARK), Succinct Transparent ARgument of Knowledge (STARK) and Zero Knowledge Prover and Verifier for Boolean Circuits (ZKBoo).  Zero-knowledge proofs are designed so that a prover is able to indirectly verify that a statement is true without having to provide any information beyond the verification of the statement, for example to prove that a number is found that solves a cryptographic puzzle and fits the hash value without having to reveal the nonce. ([[9]][9], [[11]][11])

In essence they are inner product arguments that provide general ways to prove knowledge about multiplying things to some other things. [[9]][9]



Bulletproofs have wide application [[7]][7] and can be used for :

- Rangeproofs
  - ???
- Merkle proofs
  - ???
- Proof of solvency
  - ???
- Multisig with deterministic nonces
  - ???
- Scriptless Scripts (with ECDSA in some cases)
  - ???
- Assets / smart contracts / crypto-derivatives
  - ???



## Contents

[TOC]





## Interesting Grin-Bulletproof Snippets

### Wallet Reconstruction [[5]][5]

"{**yeastplume** } Single commit bullet proofs appear to be working, which is all we need. The only think I think we're missing here from being able to use this implementation is the ability to store an amount within the rangeproof (for wallet reconstruction). From conversations with @apoelstra earlier, I believe it's possible to store 64 bytes worth of 'message' (not nearly as much as the current range proofs). We also need to be aware that we can't rely as much on the message hiding properties of range proofs when switching to bullet proofs."

"{**yeastplume** } @apoelstra the amount, and quite possibly the switch commitment hash as well (or just a hash of the entire output) as per #207..."

"{**apoelstra**} Ok, I can get you 64 bytes without much trouble (xoring them into `tau_1` and `alpha` which are easy to extract from `tau_x` and `mu` if you know the original seed used to produce the randomness). I think it's possible to get another 32 bytes into `t` but that's way more involved since `t` is a big inner product." 

???

## Negatives

- A discrete-log attacker (e.g. a quantum computer) would be able to exploit Bulletproofs to silently inflate any currency that used them [[10]][10]

## Conclusions, Observations, Recommendations

- ???

## Definition of Terms

[^*]: See 'Definition of Terms'

- <u>Argument of knowledge system</u>: Proof systems with computational soundness like Bulletproofs are sometimes called argument systems [[12]][12]
- ???

## References

[[1]][1] Bulletproofs, https://github.com/adjoint-io/bulletproofs, Date accessed: 2018-09-10.

[1]: https://github.com/adjoint-io/bulletproofs, "Bulletproofs"

[[2]][2] Elliptic Curve Cryptography: a gentle introduction, http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/, Date accessed: 2018-09-10.

[2]: http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction "Elliptic Curve Cryptography: a gentle introduction"

[[3]][3] **?????** Range Proofs - A Primer, https://github.com/ignopeverell/grin/blob/master/doc/rangeproofs.md, Date accessed: 2018-09-10. **?????**

[3]: https://github.com/ignopeverell/grin/blob/master/doc/rangeproofs.md	"Range Proofs - A Primer"

[[4]] Mimblewimble Explained, https://www.weusecoins.com/mimble-wimble-andrew-poelstra/, Date accessed: 2018-09-10.

[4]: https://www.weusecoins.com/mimble-wimble-andrew-poelstra	"Mimblewimble Explained"

[[5]] Bulletproofs #273, https://github.com/mimblewimble/grin/issues/273, Date  accessed: 2018-09-10.

[5]: https://github.com/mimblewimble/grin/issues/273	"Bulletproofs #273"

[[6]] Message hiding in Bulletproofs #721, https://github.com/mimblewimble/grin/issues/721, Date accessed: 2018-09-10.

[6]: https://github.com/mimblewimble/grin/issues/721	"Message hiding in Bulletproofs #721"

[[7]] Bullet Proofs (Slides), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra, https://download.wpsoftware.net/bitcoin/2018-02-bp-slides/slides.pdf, Date accessed: 2018-09-10.

[7]: https://download.wpsoftware.net/bitcoin/2018-02-bp-slides/slides.pdf "Bullet Proofs (Slides), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra"

[[8]] Bullet Proofs (Video), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra, https://www.pscp.tv/w/1mnxerNaNkLKX, Date accessed: 2018-09-10.

[8]: https://www.pscp.tv/w/1mnxerNaNkLKX	"Bullet Proofs (Video), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra"

[[9]] Bullet Proofs (Transcript), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra, https://diyhpl.us/wiki/transcripts/2018-02-02-andrew-poelstra-bulletproofs, Date accessed: 2018-09-10.

[9]: https://diyhpl.us/wiki/transcripts/2018-02-02-andrew-poelstra-bulletproofs "Bullet Proofs (Transcript), Bitcoin Milan Meetup 2018-02-02"

[[10]] Bulletproofs presentation at Feb 2 Milan Meetup (Andrew Poelstra), https://www.reddit.com/r/Bitcoin/comments/7w72pq/bulletproofs_presentation_at_feb_2_milan_meetup, Date accessed: 2018-09-10.

[10]: https://www.reddit.com/r/Bitcoin/comments/7w72pq/bulletproofs_presentation_at_feb_2_milan_meetup	"Bulletproofs presentation at Feb 2 Milan Meetup (Andrew Poelstra)"

[[11]] Decoding zk-SNARKs, https://medium.com/wolverineblockchain/decoding-zk-snarks-85e73886a040, Date accessed: 2018-09-17.

[11]: https://medium.com/wolverineblockchain/decoding-zk-snarks-85e73886a040	"Decoding zk-SNARKs"

[[12]] Bulletproofs: Short Proofs for Confidential Transactions and More, Bünz B. et al., http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf, Date accessed: 2018-09-??.

[12]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf	"Bulletproofs: Short Proofs for Confidential Transactions and More, Bünz B. et al"

[[13]] , , Date accessed: 2018-09-??.

[13]: 

[[14]] , , Date accessed: 2018-09-??.

[14]: 

[[15]] , , Date accessed: 2018-09-??.

[15]: 

[[16]] , , Date accessed: 2018-09-??.

[16]: 

[[17]] , , Date accessed: 2018-09-??.

[17]: 

[[18]] , , Date accessed: 2018-09-??.

[18]: 

[[19]] , , Date accessed: 2018-09-??.

[19]: 

[[20]] , , Date accessed: 2018-09-??.

[20]: 

[[21]] , , Date accessed: 2018-09-??.

[21]: 

[[22]] , , Date accessed: 2018-09-??.

[22]: 

[[23]] , , Date accessed: 2018-09-??.

[23]: 

[[24]] , , Date accessed: 2018-09-??.

[24]: 

[[25]] , , Date accessed: 2018-09-??.

[25]: 

[[26]] , , Date accessed: 2018-09-??.

[26]: 

[[27]] , , Date accessed: 2018-09-??.

[27]: 

[[28]] , , Date accessed: 2018-09-??.

[28]: 

[[29]] , , Date accessed: 2018-09-??.

[29]: 

## Contributors

- [https://github.com/???](https://github.com/??)
- 