# Confidential Assets

## Introduction

Confidential assets in the context of blockchain technology and blockchain-based cryptocurrencies can have different meanings to different audiences, and can also be something totally different or unique depending on the use case. It is a special type of digital asset and inherits all its properties except that it is also confidential. Confidential assets therefore have value, can be owned but has no physical presence. The confidentiality aspect implies that the amount of assets owned as well as the asset type that was transacted in can be confidential. A further classification can be made with regards to whether it is fungible (interchangeable) or non-fungible (unique, not interchangeable). Confidential assets can only exist in the form of a cryptographic token or derivative thereof that is also cryptographically secure, at least under the Discrete Logarithmic Problem<sup>[def][dlp~]</sup>  (DLP) assumption. 

The basis of confidential assets are confidential transactions as proposed by Maxwell [[4]] and Poelstra et al. [[5]], where the amounts transferred are kept visible only to participants in the transaction (and those they designate). Confidential transactions succeed in making the transaction amounts private, while still preserving the ability of the public blockchain network to verify that the ledger entries and Unspent Transaction Output (UTXO) set still add up. All amounts in the UTXO set are blinded, while preserving public verifiability. Poelstra et al. [[5]] showed how the asset types can also be blinded in conjunction with the output amounts. Multiple asset types can be accommodated within single transactions on the same blockchain.

This report investigates confidential assets as a natural progression of confidential transactions. 



## Contents

- [Confidential Assets](#confidential-assets)
  - [Introduction](#introduction)
  - [Contents](#contents)
  - [Preliminaries](#preliminaries)
  - [The Basis of Confidential Assets](#the-basis-of-confidential-assets)
    - [Confidential Transactions](#confidential-transactions)
    - [Asset Commitments and Surjection Proofs](#asset-commitments-and-surjection-proofs)
  - [The Confidential Asset Scheme](#the-confidential-asset-scheme)
    - [Asset Transactions](#asset-transactions)
    - [Asset Issuance](#asset-issuance)
    - [Asset Reissuance](#asset-reissuance)
    - [Flexibility](#flexibility)
  - [Confidential Asset Implementations](#confidential-asset-implementations)
    - [Elements Project](#elements-project)
    - [Chain Core Confidential Assets](#chain-core-confidential-assets)
    - [Cloak](#cloak)
  - [Conclusions, Observations, Recommendations](#conclusions-observations-recommendations)
  - [References](#references)
  - [Appendices](#appendices)
    - [Appendix A: Definition of Terms](#appendix-a-definition-of-terms)
    - [Appendix B: Ricardian Contracts](#appendix-b-ricardian-contracts)
  - [Contributors](#contributors)



## Preliminaries

The general notation of mathematical expressions when specifically referenced are listed here. These notations are important pre-knowledge for the remainder of the report.

- Let $ p $ be a large prime number.
- Let $ \mathbb G $ denote a cyclic group of prime order $ p $. 
- let $ \mathbb Z_p $ denote the ring of integers $ modulo \mspace{4mu} p $.
- Let $ \mathbb F_p $ be a group of elliptic curve points over a finite (prime) field.
- All references to Pedersen Commitment will imply Elliptic Curve Pedersen Commitment.



## The Basis of Confidential Assets

Confidential transactions, asset commitments and Asset Surjection Proofs (ASP) are really the basis of confidential assets. 

### Confidential Transactions

Confidential transactions are made confidential by replacing each explicit UTXO with a homomorphic commitment, like a [Pedersen Commitment](../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments), and made robust against overflow and inflation attacks by using efficient zero-knowledge range proofs, like a [Bulletproof](../../cryptography/bulletproofs-and-mimblewimble/MainReport.md#how-do-bulletproofs-work). [[1]]

Range proofs are proofs that a secret value, which has been encrypted or committed to, lies in a certain interval. It prevents any numbers coming near the magnitude of a large prime, say $ 2^{256} $, that can cause wrap around when adding a small number, e.g. proof that a number $ x \in [0,2^{64} - 1] $.

Pedersen Commitments are perfectly hiding (an attacker with infinite computing power cannot tell what amount has been committed to) and computationally binding (no efficient algorithm running in a practical amount of time can produce fake commitments except with small probability). The Elliptic Curve Pedersen Commitment for value $ x \in \mathbb Z_p $ has the following form
$$
C(x,r) = xH + rG
$$
where $ r \in  \mathbb Z_p $ is a random blinding factor, $ G \in  \mathbb F_p $ is a random generator point and $ H \in  \mathbb F_p $ is specially chosen so that the value $ x_H $ satisfying $ H = x_H G $ cannot be found except if the Elliptic Curve DLP<sup>[def][dlp~]</sup> (ECDLP) is solved. The number $ H $ is what is known as a Nothing Up My Sleeve (NUMS) number. With secp256k1 the value of $ H $ is the SHA256 hash of a simple encoding of the  $ x $-coordinate of the generator point $ G $. The Pedersen Commitment scheme is implemented with three algorithms: <code>Setup()</code> to set up the commitment parameters $ G $ and $ H $; <code>Commit()</code> to commit to the message $ x $ using the commitment parameters $ r $, $ H $ and $ G $ and <code>Open()</code> to open and verify the commitment. ([[5]], [[6]], [[7]], [[8]])

[Mimblewimble](../../protocols/mimblewimble-1/sources/PITCHME.link.md) ([[9]], [[10]]) is based on and achieves confidentiality using these confidential transaction primitives. If confidentiality is not sought, inputs may be given as explicit amounts, in which case the homomorphic commitment to the given amount will have a blinding factor $ r = 0 $.



### Asset Commitments and Surjection Proofs

The different assets need to be identified and transacted with in a confidential manner and proven to not be inflationary, and this is made possible by using asset commitments and Asset Surjection Proofs (ASP). An ASP in this context is a cryptographic proof. In mathematics a surjection function simply means that for every element $ y $ in the codomain $ Y $ of function $ f $ there is at least one element $ x $ in the domain $ X $ of function $ f $ such that $ f(x) = y$. Given some unique asset description $ A $, the associated asset tag $ H_A \in \mathbb G $ is calculated using the Pedersen Commitment function <code>Setup()</code> using $ A $ as auxiliary input. (*Selection of $ A $ is discussed in [Asset Issuance](#asset-issuance).*) The asset commitment to asset tag $ H_A $ is then defined as the point
$$
H_0 = H_A + rG
$$
Blinding of the the asset tag is necessary to make transactions in the asset, i.e. which asset was transacted in, confidential. $ H_0 $ will then be used in place of the generator $ H $ in the Pedersen Commitments. Such Pedersen Commitments thus commit to the committed amount as well as to the underlying asset tag. [[1]]

An ASP scheme provides a proof $ \pi $ for a set of input asset commitments $ [ H_i  ] ^n_{i=1} $, an output commitment $ H = H_{\hat i} + rG $ for $  \hat i = 1 \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} n  $ and blinding factor $ r $, that every output asset type is the same as some input asset type while blinding which outputs correspond to which inputs. Such a proof $ \pi $ is secure if it is a zero-knowledge proof of knowledge for the blinding factor $ r $. [[1]]

The ASP is based on the *Back-Maxwell* range proof (see Definition 9 of [[1]]), which uses a variation of the Borromean ring signatures [[18]]. The Borromean ring signatures in turn is a variant of the Abe-Ohkubo-Suzuki (AOS) ring signatures [[19]]. An AOS ASP computes a ring signature  that is equal to the proof $ \pi $ as follows:

- Calculate $ n $ differences $ H - H_{\hat i } $ for $  \hat i = 1 \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} n  $, one of which will be equal to the blinding factor $ r $;
- Calculate a ring signature $ S $ of an empty message using the $ n $ differences. 

The resulting ring signature $ S $ is equal to the proof $ \pi $, and the ASP consist of this ring signature $ S $. [[1]]



## The Confidential Asset Scheme

Using the building blocks discussed in [The Basis of Confidential Assets](#the-basis-of-confidential-assets), asset issuance, asset transactions and asset re-issuance can be performed in a confidential manner.

### Asset Transactions

Confidential assets propose a scheme where multiple non-interchangeable asset types can be supported within a single transaction. This all happens within one blockchain and can theoretically improve the value of the blockchain by offering a service to more users and can also enable extended functionality like base layer atomic asset trades. The latter implies Alice can offer Bob $ 100 $ of asset type $ A $ for $ 50 $ of asset type $ B $ in a single transaction, both participants using a single wallet. In this case no relationship between output asset types can be established or inferred because all asset tags are blinded. Privacy can be increased as the blinded asset types brings another dimension that needs to be unraveled in order to obtain user identity and transaction data by not having multiple single-asset transactions. Such a confidential asset scheme simplifies verification and complexity and reduces on-chain data. It also prohibits censorship of transactions involving specific asset types, and blinds assets with low transaction volume where users could be identified very easily. [[1]]

Assets originate in asset-issuance inputs, which take the place of coinbase transactions in confidential transactions. The asset type to pay fees must be revealed in each transaction, but in practice all fees could be paid in only one asset type, thus preserving privacy. Payment authorization is achieved by means of the input signatures. A confidential asset transaction consists of the following data:

- A list of inputs, each of which can have one of the following forms:
  - A reference to an output of another transaction, with a signature using that output's verification key, or;
  - An asset issuance input, which has an explicit amount and asset tag.
- A list of outputs that contains:
  - A verification key;
  - An asset commitment $ H_0 $ with an ASP from all input asset commitments to $ H_0 $;
  - Pedersen commitment to an amount using generator $ H_0 $ in place of  $ H $, with the associated range proof.
- A fee, listed explicitly as $ \{ (f_i , H_i) \}_{i=1}^n $, where $ f_i $ is a non-negative scalar amount denominated in the asset with tag $ H_i $. 

Every output has a range proof and ASP associated with it, which are proofs of knowledge of the Pedersen commitment opening information and asset commitment blinding factor. Every range proof can be considered as being with respect to the underlying asset tag $ H_A $, rather than the asset commitment $ H_0 $. The confidential transaction is restricted to only inputs and outputs with asset tag $ H_A $, except that output commitments minus input commitments minus fee sum to a commitment to $ 0 $ instead of to the point $ 0 $ itself. [[1]]

However, confidential assets come at an additional data cost. For a transaction with $ m $ outputs and $ n $ inputs, in relation to the units of space used for confidential transactions, the asset commitment has size $ 1$, the ASP has size $ n + 1 $ and the entire transaction therefor has size $ m(n + 2) $. [[1]]



### Asset Issuance

It is important to ensure that any auxiliary input $ A $ used to create asset tag $ H_A \in \mathbb G $ only be used once to prevent inflation by means of many independent issuances. Associating a maximum of one issuance with the spending of a specific UTXO can ensure this uniqueness property. Poelstra et al. [[5]] suggest the use of a Ricardian contract [[11]] to be hashed together with the reference to the UTXO being spent. This hash can then be used to generate the auxiliary input $ A $ as follows. Let $I $ be the input being spent (an unambiguous reference to a specific UTXO used to create the asset), let $ \widehat {RC} $ be the issuer-specified Ricardian contract, then the asset entropy $ E $ is defined as 
$$
E = \mathrm {Hash} ( \mathrm {Hash} (I) \parallel \mathrm {Hash} (\widehat {RC}))
$$
The auxiliary input $ A $ is then defined as 
$$
A = \mathrm {Hash} ( E \parallel 0)
$$
Note that a Ricardian contract $ \widehat {RC} $ is not crucial for entropy $ E $ generation as some other unique data set could have been used in its stead, but suggested. Ricardian contracts warrant a bit more explanation and is discussed in [Appendix&nbsp;B](#appendix-b-ricardian-contracts).


Every non-coinbase transaction input can have up to one new asset issuance associated with it. An asset issuance input then consists of the UTXO being spent, the Ricardian contract, either an initial issuance explicit value or a Pedersen commitment, a range proof and a Boolean field indicating whether reissuance is allowed. [[1]]



### Asset Reissuance

The confidential asset scheme allows the asset owner to later increase or decrease the amount of the asset in circulation, given that an asset reissuance token is generated together with the initial asset issuance. Given an asset entropy $ E $, the asset reissuance capability is the element (asset tag) $ H_{\hat A} \in \mathbb G $ obtained using an alternate auxiliary input $ \hat A $ defined as
$$
\hat A = \mathrm {Hash} ( E \parallel 1)
$$
The resulting asset tag $ H_{\hat A} \in \mathbb G $ is linked to its reissuance capability, and the asset owner can assert their reissuance right by revealing the blinding factor $ r $ for the reissuance capability along with the original asset entropy $ E $. An asset reissuance input then consists of the spend of a UTXO containing an asset reissuance capability, the original asset entropy, the blinding factor for the asset commitment of the UTXO being spent, either an explicit reissuance amount or Pedersen commitment and a range proof. The same mechanism can be used to manage capabilities for other restricted operations for example to decrease issuance, or to make the commitment generator the hash of a script that validates the spending transaction. [[1]]



### Flexibility

ASPs prove that the asset commitments associated with outputs commit to legitimately issued asset tags. This feature allows compatible blockchains to support indefinitely many asset types, which may be added after the chain has been defined. There is room to adapt this scheme for optimal tradeoff between ASP data size and privacy by introducing a global dynamic list of assets, whereby each transaction selects a subset of asset tags for the corresponding ASPs. [[1]]

If all the asset tags are defined at the instantiation of the blockchain it will be compatible with the [Mimblewimble](../../protocols/mimblewimble-1/sources/PITCHME.link.md) protocol. The range proofs used for the development of this scheme were based on the Back-Maxwell range proof scheme (see Definition 9 of [[1]]). Poelstra et al. [[1]] suggests more efficient range proofs, ASPs and use of aggregate range proofs. It is thus an open question if Bulletproofs could fulfill this requirement.



## Confidential Asset Implementations

Three independent implementations of confidential assets are summarized here. The first two implementations closely resembles the theoretical description as in [[1]], with the last implementation adding Bulletproofs functionality to confidential assets.

### Elements Project

[Elements](https://elementsproject.org) is an open source, sidechain-capable blockchain platform, providing access to advanced features, such as Confidential Transactions and Issued Assets (`Github: ElementsProject/elements` [[16]]). It allows digitizable collectables, reward points and attested assets (for example gold coins) to be realized on a blockchain. The main idea behind Elements is to serve as research platform and testbed for changes to the Bitcoin protocol. Their implementation of confidential assets is called Issued Assets ([[13]], [[14]], [[15]]) and is based on their formal publication in [[1]]. 

The Elements project hosts a working demonstration ([Figure&nbsp;2](#fig_eca)) of confidential asset transfers (`Github: ElementsProject/confidential-assets-demo` [[17]]) involving 5 parties. The demonstration depicts a scenario where a coffee shop owner *Dave* charges a customer *Alice* for coffee in an asset called MELON. *Alice* does not hold enough MELON and needs to convert some AIRSKY into MELON making use of an exchange operated by *Charlie*. The coffee shop owner *Dave* has a competitor *Bob* who is trying to gather information about *Dave's* sales. Due to the blockchain's confidential transactions and assets features, he will not be able to see anything useful by processing transactions on the blockchain. *Fred* is a miner and does not care about the detail of the transactions, but he makes blocks on the blockchain when transactions enter his miner mempool. The demonstration also includes generating the different types of assets.

<p align="center"><a name="fig_eca"> </a><img src="sources/elements-tx-example.png" width="650" /></p>

<p align="center"><b>Figure&nbsp;2: Elements Confidential Assets Transfer Demonstration [<a href="https://github.com/ElementsProject/confidential-assets-demo" title="ElementsProject/confidential-assets-demo">17</a>]</b></p>



### Chain Core Confidential Assets

Chain Core [[20]] is a shared, multi-asset, cryptographic ledger, designed for enterprise financial infrastructure. It supports the coexistence and interoperability of multiple types of assets on the same network in their Confidential Assets framework. Chain Core is based on [[1]], and available as an open source project in `Github: chain/chain` [[21]], which have been archived. It has been succeeded by Sequence, a ledger-as-a-service project, that enables secure tracking and transfer of balances in a token format ([[22]], [[23]]). They offer a free plan for up to 1,000,000 transactions per month.

Chain Core implements all native features as defined in [[1]]. They were also working towards implementing ElGamal commitments into Chain Core to make their Confidential Assets framework quantum secure, but it is unclear if this effort was concluded at the time the project was archived. ([[24]], [[25]])



### Cloak

Chain/Interstellar [[26]] introduced Cloak [[29]], a redesign of Chain Core's Confidential Assets framework to make use of Bulletproof range proofs [[27]]. It is available as an open source project in `Github: interstellar/spacesuit` [[28]]. Cloak is all about confidential asset transactions, called cloaked transactions, which exchange values of different asset types, called flavors. The protocol ensures that values are not transmuted to any other asset types, that quantities do not overflow and that both quantities and asset types are kept secret. 

A traditional Bulletproofs implementation convert an arithmetic circuit into a Rank-1 Constraint System (R1CS); Cloak bypasses arithmetic circuits and provide an Application Programmers Interface (API) for building a [constraint system](../../cryptography/bulletproofs-protocols/MainReport.md#evolving-bulletproof-protocols) directly. The R1CS API consists of a hierarchy of task-specific “gadgets” and is used by the *Prover* and *Verifier* alike to allocate variables and define constraints. Cloak uses a collection of gadgets like “shuffle”, “merge”, “split” and “range proof” to build a constraint system for cloaked transactions. All transactions of the same size are indistinguishable because the layout of all the gadgets is only determined by the number of inputs and outputs.

The Cloak development is still ongoing.



## Conclusions, Observations, Recommendations

- The idea to embed a Ricardian contract in the asset tag creation as suggested by Poelstra et al. [[1]] warrants more investigation for a new confidential blockchain protocol like Tari; Ricardian contracts could be used in asset generation in the probable 2nd layer.
- Asset commitments and ASPs are important cryptographic primitives for confidential asset transactions.
- The Elements project implemented a range of useful confidential asset framework features and should be critically assessed for usability in a probable Tari 2nd layer.
- Cloak has the potential to take confidential assets implementation to the next level in efficiency and should be closely monitored. They are in the process to fully implement and extend Bulletproofs for use in confidential assets.
- If confidential assets are to be implemented in the Mimblewimble blockchain, all asset tags must be defined at its instantiation, otherwise it will not be compatible. 



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

[[11]] The Ricardian Contract, First IEEE International Workshop on Electronic Contracting. IEEE (2004), Grigg I., http://iang.org/papers/ricardian_contract.html, Date accessed: 2018-12-13.

[11]: http://iang.org/papers/ricardian_contract.html
"The Ricardian Contract, 
First IEEE International Workshop on 
Electronic Contracting.
IEEE (2004), 
Grigg I."

[[12]] Smart vs. Ricardian Contracts: What’s the Difference?, Koteshov D., February 2018, https://www.elinext.com/industries/financial/trends/smart-vs-ricardian-contracts/, Date accessed: 2018-12-13.

[12]: https://www.elinext.com/industries/financial/trends/smart-vs-ricardian-contracts/
"Smart vs. Ricardian Contracts: 
What’s the Difference?, 
Koteshov D., 
February 201"

[[13]] Issued Assets - You can issue your own Confidential Assets on Elements, Elements by Blockstream, https://elementsproject.org/features/issued-assets, Date accessed: 2018-12-14.

[13]: https://elementsproject.org/features/issued-assets
"Issued Assets - You can issue your 
own Confidential Assets on Elements, 
Elements by Blockstream"

[[14]] Issued Assets - Investigation, Principal Investigator: Andrew Poelstra, Elements by Blockstream, https://elementsproject.org/features/issued-assets/investigation, Date accessed: 2018-12-14.

[14]: https://elementsproject.org/features/issued-assets/investigation
"Issued Assets - Investigation, 
Principal Investigator: Andrew Poelstra, 
Elements by Blockstream"

[[15]] Elements code tutorial - Issuing your own assets, Elements by Blockstream, elementsproject.org, https://elementsproject.org/elements-code-tutorial/issuing-assets, Date accessed: 2018-12-14.

[15]: https://elementsproject.org/elements-code-tutorial/issuing-assets
"Elements code tutorial - Issuing your own assets, 
Elements by Blockstream, 
elementsproject.org"

[[16]] Github: ElementsProject/elements, https://github.com/ElementsProject/elements, Date accessed: 2018-12-18.

[16]: https://github.com/ElementsProject/elements
"Github: ElementsProject/elements"

[[17]] Github: ElementsProject/confidential-assets-demo, https://github.com/ElementsProject/confidential-assets-demo, Date accessed: 2018-12-18.

[17]: https://github.com/ElementsProject/confidential-assets-demo
"ElementsProject/confidential-assets-demo"

[[18]] Borromean ring signatures (2015), Maxwell G. and Poelstra A., http://diyhpl.us/
~bryan/papers2/bitcoin/Borromean%20ring%20signatures.pdf, Date accessed: 2018-12-18.

[18]: http://diyhpl.us/~bryan/papers2/bitcoin/Borromean%20ring%20signatures.pdf
"Borromean ring signature (2015), 
Maxwell G. and Poelstra A."

[[19]] 1-out-of-n Signatures from a Variety of Keys, Abe M., Ohkubo M. and Suzuki K., https://www.iacr.org/cryptodb/archive/2002/ASIACRYPT/50/50.pdf, Date accessed: 2018-12-18.

[19]: http://???
"1-out-of-n Signatures from a Variety of Keys, 
Abe M., Ohkubo M. and Suzuki K"

[[20]] Chain Core, https://chain.com/docs/1.2/core/get-started/introduction, Date accessed: 2018-12-18.

[20]: https://chain.com/docs/1.2/core/get-started/introduction
"Chain Core"

[[21]] Github: chain/chain, https://github.com/chain/chain, Date accessed: 2018-12-18.

[21]: https://github.com/chain/chain
"Github: chain/chain"

[[22]] Chain: Sequence, https://chain.com/sequence, Date accessed: 2018-12-18.

[22]: https://chain.com/sequence
"Chain: Sequence"

[[23]] Sequence Documentation, https://dashboard.seq.com/docs, Date accessed: 2018-12-18.

[23]: https://dashboard.seq.com/docs
"Sequence Documentation"

[[24]] Hidden in Plain Sight: Transacting Privately on a Blockchain - Introducing Confidential Assets in the Chain Protocol, https://blog.chain.com/hidden-in-plain-sight-transacting-privately-on-a-blockchain-835ab75c01cb, Date accessed: 2018-12-??.

[24]: https://blog.chain.com/hidden-in-plain-sight-transacting-privately-on-a-blockchain-835ab75c01cb
"Hidden in Plain Sight: 
Transacting Privately on a Blockchain - 
Introducing Confidential Assets in the Chain Protocol"

[[25]] Blockchains in a Quantum Future - Protecting Against Post-Quantum Attacks on Cryptography, https://blog.chain.com/preparing-for-a-quantum-future-45535b316314, Date accessed: 2018-12-??.

[25]: https://blog.chain.com/preparing-for-a-quantum-future-45535b316314
"Blockchains in a Quantum Future - 
Protecting Against Post-Quantum 
Attacks on Cryptography"

[[26]] Inter/stellar website, https://interstellar.com, Date accessed: 2018-11-22.

[26]: https://interstellar.com
"Inter/stellar Website"

[[27]] Programmable Constraint Systems for Bulletproofs, https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7, Date accessed: 2018-11-22.

[27]: https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7
"Programmable Constraint Systems for Bulletproofs,
Interstellar,
Cathie Yun"

[[28]] Github: interstellar/spacesuit, https://github.com/interstellar/spacesuit/blob/master/spec.md, Date accessed: 2018-12-??.

[28]: https://github.com/interstellar/spacesuit/blob/master/spec.md
"Github: interstellar/spacesuit"

[[29]] Github: interstellar/spacesuit/spec.md, https://github.com/interstellar/spacesuit/blob/master/spec.md, Date accessed: 2018-12-18.

[29]: https://github.com/interstellar/spacesuit/blob/master/spec.md
"Github: interstellar/spacesuit/spec.md"


## Appendices

### Appendix A: Definition of Terms

Definitions of terms presented here are high level and general in nature. Full mathematical definitions are available in the cited references. 

- <i><u>Discrete Logarithm/Discrete Logarithm Problem (DLP)</u></i>:<a name="dlp"> </a>In the mathematics of real numbers, the logarithm $ \log_b^a $ is a number $ x $ such that $ b^x=a $, for given numbers $ a $ and $ b $. Analogously, in any group $ G $ , powers $ b^k $ can be defined for all integers $ k $, and the discrete logarithm $ \log_ba $ is an integer $ k $ such that $ b^k=a $. Algorithms in public-key cryptography base their security on the assumption that the discrete logarithm problem over carefully chosen cyclic finite groups and cyclic subgroups of elliptic curves over finite fields has no efficient solution. ([[2]], [[3]])

[dlp~]: #dlp
"In the mathematics of real 
numbers, the logarithm log_b(a) 
is a number x such that ..."



### Appendix B: Ricardian Contracts

A Ricardian contract is “*a digital contract that deﬁnes the terms and conditions of an interaction, between two or more peers, that is cryptographically signed and veriﬁed, being both human and machine readable and digitally signed*” [[12]]. The main properties of a Ricardian contract is listed below (also see [Figure&nbsp;1](#fig_rc)):

- Human readable;
- Document is printable;
- Program parsable;
- All forms (displayed, printed, parsed) are manifestly equivalent;
- Signed by issuer;
- Can be identified securely, where security means that any attempts to change the linkage between a reference and the contract are impractical.

<p align="center"><a name="fig_rc"> </a><img src="sources/ricardian_contract.png" width="690" /></p>

<p align="center"><b>Figure&nbsp;1: Ricardian Contract [<a href="https://www.elinext.com/industries/financial/trends/smart-vs-ricardian-contracts" title="Smart vs. Ricardian Contracts: 
What’s the Difference?, 
Koteshov D., 
February 2018">12</a>]</b></p>



## Contributors

- [https://github.com/hansieodendaal](https://github.com/hansieodendaal)
- ?
