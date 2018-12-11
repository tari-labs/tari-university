# Confidential Assets

## Introduction

Confidential assets in the context of blockchain technology and blockchain-based cryptocurrencies can have different meanings to different audiences, and can also be something totally different or unique depending on the use case. It is a special type of digital asset and inherits all its properties except that it is also confidential. Confidential assets therefor has value, can be owned but has no physical presence.  The confidentiality aspect implies that the amount of assets owned as well as the asset type can be confidential. A further classification can be made with regards to whether it is fungible (interchangeable) or non-fungible (unique, not interchangeable). Confidential assets can only exist in the form of a cryptographic token or derivative thereof that is also cryptographically secure, at least under the Discrete Logarithmic Problem<sup>[def][dlp~]</sup>  (DLP) assumption. 

The basis of confidential assets are confidential transactions as proposed by Maxwell [[4]] and Poelstra et al. [[5]], where the amounts transferred are kept visible only to participants in the transaction (and those they designate). Confidential transactions succeed in making the transaction amounts private, while still preserving the ability of the public blockchain network to verify that the ledger entries and Unspent Transaction Output (UTXO) set still add up. All amounts in the UTXO set are blinded, while preserving public verifiability. Poelstra et al. [[5]] showed how the asset types can also be blinded in conjunction with the output amounts. Multiple asset types can be accommodated within single transactions on the same blockchain.

This report investigates confidential assets as a natural progression of confidential transactions in the context of a [Mimblewimble](../../protocols/mimblewimble-1/sources/PITCHME.link.md) ([[9]], [[10]]), a privacy preserving blockchain. 

Bulletproofs could be extended to allow assets that are functions of other assets, i.e. crypto derivatives ???



## Contents

- [Confidential Assets](#confidential-assets)
  - [Introduction](#introduction)
  - [Contents](#contents)
  - [Preliminaries](#preliminaries)
  - [The Basis of Confidential Assets](#the-basis-of-confidential-assets)
  - [?](#?)
  - [Conclusions, Observations, Recommendations](#conclusions-observations-recommendations)
  - [References](#references)
  - [Appendices](#appendices)
    - [Appendix A: Definition of Terms](#appendix-a-definition-of-terms)
  - [Contributors](#contributors)



## Preliminaries

The general notation of mathematical expressions when specifically referenced are listed here. These notations are important pre-knowledge for the remainder of the report.

- Let $ p $ be a large prime number.
- Let $ \mathbb G $ denote a cyclic group of prime order $ p $. 
- let $ \mathbb Z_p $ denote the ring of integers $ modulo \mspace{4mu} p $.
- Let $ \mathbb F_p $ be a group of elliptic curve points over a finite (prime) field.
- All references to Pedersen Commitment will imply (Elliptic Curve) Pedersen Commitment.



## The Basis of Confidential Assets

### Confidential Transactions

Confidential transactions are made confidential by replacing each explicit UTXO with a homomorphic commitment, like an (Elliptic Curve) [Pedersen Commitment](../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments), and made robust against overflow and inflation attacks by using efficient zero-knowledge range proofs, like a [Bulletproof](../../cryptography/bulletproofs-and-mimblewimble/MainReport.md#how-do-bulletproofs-work).

Range proofs are proofs that a secret value, which has been encrypted or committed to, lies in a certain interval. It prevents any numbers coming near the magnitude of a large prime, say $ 2^{256} $, that can cause wrap around when adding a small number, e.g. proof that a number $ x \in [0,2^{64} - 1] $.

Pedersen Commitments are perfectly hiding (an attacker with infinite computing power cannot tell what amount has been committed to) and computationally binding (no efficient algorithm running in a practical amount of time can produce fake commitments except with small probability). The (Elliptic Curve) Pedersen Commitment for value $ x \in \mathbb Z_p $ has the following form
$$
C(x,r) = xH + rG
$$
where $ r \in  \mathbb Z_p $ is a random blinding factor, $ G \in  \mathbb F_p $ is a random generator point and $ H \in  \mathbb F_p $ is specially chosen so that the value $ x_H $ to satisfy $ H = x_H G $ cannot be found except if the Elliptic Curve DLP<sup>[def][dlp~]</sup> (ECDLP) is solved. The Pedersen Commitment scheme is implemented with three algorithms: <code>Setup()</code> to set up the commitment parameters $ G $ and $ H $; <code>Commit()</code> to commit to the message $ x $ using the commitment parameters $ r $, $ H $ and $ G $ and <code>Open()</code> to open and verify the commitment.

[Mimblewimble](../../protocols/mimblewimble-1/sources/PITCHME.link.md) is based on and achieves confidentiality using these primitives, thus Mimblewimble confidential transactions. If confidentiality is not sought inputs may be given as explicit amounts, in which case the homomorphic commitment to the given amount will have blinding factor $ r = 0 $.



### Asset Commitments and Surjection Proofs

The different assets needs to be identified and transacted with in a confidential manner and proven to not be inflationary, thus asset commitments and Asset Surjection Proofs (ASP) are defined. Given some asset description $ A $, the associated asset tag $ H_A \in \mathbb G $  is calculated using the Pedersen Commitment function <code>Setup()</code> using $ A $ as auxiliary input. The asset commitment to asset tag $ H_A $ is then defined as the point $ H = H_A + rG $, which will be used in place of the generator $ H $ in the Pedersen Commitments. An ASP scheme provides a proof for a set of asset commitments. Such a proof is secure if it is a zero-knowledge proof of knowledge for the blinding factor $ r $.



### The Confidential Asset Scheme





## Conclusions, Observations, Recommendations

- ?
- 



## References

[[1]] Confidential Assets, Poelstra A., Back A., Friedenbach M., Maxwell G. and Wuille P., Blockstream, https://blockstream.com/bitcoin17-final41.pdf, Date accessed: 2018-09-25.

[1]: https://blockstream.com/bitcoin17-final41.pdf
"Confidential Assets,
Poelstra A. et al.,
Blockstream"

[[2]] Wikipedia: Discrete logarithm, https://en.wikipedia.org/wiki/Discrete_logarithm, Date accessed: 2018-09-20.

[2]: https://en.wikipedia.org/wiki/Discrete_logarithm
"Wikipedia: Discrete logarithm"

[[3]] Assumptions Related to Discrete Logarithms: Why Subtleties Make a Real Difference, Sadeghi A. and Steiner M., http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf, Date accessed: 2018-09-24.

[3]: http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf
"Assumptions Related to Discrete Logarithms: 
Why Subtleties Make a Real Difference, 
Sadeghi A et al." 

[[4]] Confidential Transactions write up, G. Maxwell, https://people.xiph.org/~greg/confidential_values.txt, Date accessed: 2018-12-10.

[4]: https://people.xiph.org/~greg/confidential_values.txt
"Confidential Transactions write up, 
G. Maxwell"

[[5]] An investigation into Confidential Transactions, Gibson A., July 2018, https://github.com/AdamISZ/ConfidentialTransactionsDoc/blob/master/essayonCT.pdf, Date accessed: 2018-11-22.

[5]: https://github.com/AdamISZ/ConfidentialTransactionsDoc/blob/master/essayonCT.pdf
"An investigation into Confidential Transactions, 
Gibson A., 
July 2018"

[[6]] pedersen-commitment: An implementation of Pedersen Commitment schemes, https://hackage.haskell.org/package/pedersen-commitment, Date accessed: 2018-09-25.

[6]: https://hackage.haskell.org/package/pedersen-commitment
"Pedersen-commitment: An implementation
of Pedersen Commitment schemes"

[[7]] Homomorphic Mini-blockchain Scheme, Franca B., April 2015, http://cryptonite.info/files/HMBC.pdf, Date accessed: 2018-11-22.

[7]: http://cryptonite.info/files/HMBC.pdf
"Homomorphic Mini-blockchain Scheme, 
Franca B., 
April 2015"

[[8]] Efficient Implementation of Pedersen Commitments Using Twisted Edwards Curves, Franck C. and Großschädl J., University of Luxembourg, http://orbilu.uni.lu/bitstream/10993/33705/1/MSPN2017.pdf, Date accessed: 2018-11-22.

[8]: http://orbilu.uni.lu/bitstream/10993/33705/1/MSPN2017.pdf
"Efficient Implementation of Pedersen 
Commitments Using Twisted Edwards Curves, 
Franck C. and Großschädl J., 
University of Luxembourg"

[[9]] Mimblewimble, Poelstra A., October 2016, http://diyhpl.us/~bryan/papers2/bitcoin/mimblewimble-andytoshi-draft-2016-10-20.pdf, Date accessed: 2018-12-??.

[9]: http://diyhpl.us/~bryan/papers2/bitcoin/mimblewimble-andytoshi-draft-2016-10-20.pdf
"Mimblewimble, 
Poelstra A., 
October 2016"

[[10]] Mimblewimble Explained, Poelstra A., November 2016, https://www.weusecoins.com/mimble-wimble-andrew-poelstra/, Date accessed: 2018-09-10.

[10]: https://www.weusecoins.com/mimble-wimble-andrew-poelstra
"Mimblewimble Explained,
Poelstra A., 
November 2016"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

[[?]] ?, ?, ?, Date accessed: 2018-12-??.

[?]: http://???
"?"

## Appendices

### Appendix A: Definition of Terms

Definitions of terms presented here are high level and general in nature. Full mathematical definitions are available in the cited references. 

- <i><u>Discrete Logarithm/Discrete Logarithm Problem (DLP)</u></i>:<a name="dlp"> </a>In the mathematics of real numbers, the logarithm $ \log_b^a $ is a number $ x $ such that $ b^x=a $, for given numbers $ a $ and $ b $. Analogously, in any group $ G $ , powers $ b^k $ can be defined for all integers $ k $, and the discrete logarithm $ \log_ba $ is an integer $ k $ such that $ b^k=a $. Algorithms in public-key cryptography base their security on the assumption that the discrete logarithm problem over carefully chosen cyclic finite groups and cyclic subgroups of elliptic curves over finite fields has no efficient solution. ([[2]], [[3]])

[dlp~]: #dlp
"In the mathematics of real 
numbers, the logarithm log_b(a) 
is a number x such that ..."



## Contributors

- [https://github.com/hansieodendaal](https://github.com/hansieodendaal)
- ?
