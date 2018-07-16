# Layer 2 Scaling Survey

This report provides a survey of TumbleBit , Counterparty, Rootstock, Drivechains, Scriptless scripts, Braiding and Directed Acyclic Graph (DAG) as layer 2 scaling alternatives.

## What is Layer 2 scaling?

See [Layer 2 Scaling Survey](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md)

## How will this be applicable to Tari?

See [Layer 2 Scaling Survey](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md)

## Layer 2 scaling current initiatives (updated)

### #1 TumbleBit

#### What is it?

The TumbleBit protocol was invented at the Boston University. It is a unidirectional, unlinkable payment hub that is fully compatible with the Bitcoin protocol. TumbleBit allows parties to make fast, anonymous, off-chain payments through an untrusted intermediary called the Tumbler. No-one, not even the Tumbler, can tell which payer paid which payee during a TumbleBit epoch (*i.e. time period of significance*). Two modes of operation are supported; a classic mixing/tumbling/washing mode or as a fully fledged payment hub. TumbleBit consists of two interleaved fair-exchange protocols - *RSA-Puzzle-Solver Protocol* and *Puzzle-Promise Protocol* - that relies on the  Rivest–Shamir–Adleman (RSA) cryptosystem's blinding properties to prevent bad acting from either users or Tumblers and to ensure anonymity. TumbleBit also supports anonymizing through Tor to ensure that the Tumbler server can operate as a hidden service. ([[1]](http://cs-people.bu.edu/heilman/tumblebit/), [[2]](https://eprint.iacr.org/2016/575.pdf), [[8]](https://eprint.iacr.org/2016/056.pdf), [[9]](https://www.youtube.com/watch?v=8BLWUUPfh2Q&feature=youtu.be&t=1h3m10s), [[10]](https://bitcoinmagazine.com/articles/better-bitcoin-privacy-scalability-developers-are-making-tumblebit-reality))

![TumbleBitOverview](C:\Users\pluto\Documents\Code\tari-university\layer2scaling\more-landscape\sources\TumbleBitOverview.PNG)

TumbleBit combines off-chain cryptographic computations with standard on-chain Bitcoin scripting functionalities to realize smart contracts ([[11]](https://en.bitcoin.it/wiki/Contract)) that is not dependent on Segwit. The most important Bitcoin functionality used here are hashing conditions, signing conditions, conditional execution, 2-of-2 multi signatures and timelocking. [[2]](https://eprint.iacr.org/2016/575.pdf)

#### Who does it?

The Boston University provided a proof-of-concept and reference implementation alongside the white paper [[4]](https://github.com/BUSEC/TumbleBit). NTumbleBit [[5]](https://github.com/nTumbleBit/nTumbleBit) is being developed as a C# production implementation of the TumbleBit protocol that is currently being deployed by Stratis with their Breeze implementation ([[6]](https://stratisplatform.com/2017/07/17/breeze-tumblebit-server-experimental-release), [[7]](https://stratisplatform.com/2017/09/20/breeze-wallet-with-breeze-privacy-protocol-dev-update/)), currently at alpha/experimental release level in testnet.

*"NTumbleBit will be a cross-platform framework, server and client for the TumbleBit payment scheme. TumbleBit is separated into two modes, tumbler mode and payment hub mode. The tumbler mode improves transaction fungibility and offers risk free unlinkable transactions. Payment hub mode is a way of making off-chain payments possible without requiring implementations like Segwit or the Lightning Network."* [[3]](https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e)

#### Strengths

<u>Anonymity properties</u> - TumbleBit provides unlinkability without the need to trust the Tumbler service (i.e. untrusted intermediary). [[2]](https://eprint.iacr.org/2016/575.pdf)

<u>Denial of Service (DoS) and Sybil protection</u> - "*TumbleBit uses transaction fees to resist DoS and Sybil attacks.*" [[2]](https://eprint.iacr.org/2016/575.pdf)

<u>Balance</u> - "*The system should not be exploited to print new money or steal money, even when parties collude.*"  [[2]](https://eprint.iacr.org/2016/575.pdf)

<u>As a classic tumbler</u> - TumbleBit can also be used as a classic Bitcoin tumbler. [[2]](https://eprint.iacr.org/2016/575.pdf)

<u>Bitcoin compatibility</u> - TumbleBit is fully compatible with the Bitcoin protocol. [[2]](https://eprint.iacr.org/2016/575.pdf)

<u>Scale ability</u> - Each TumbleBit user only need to interact with the Tumbler and the corresponding transaction party; this lack of coordination between all TumbleBit users makes scale ability possible for the tumbler mode. [[2]](https://eprint.iacr.org/2016/575.pdf)

<u>Batch processing</u> - TumbleBit  supports one-to-one, many-to-one, one-to-many and many-to-many transactions in payment hub mode. [[2]](https://eprint.iacr.org/2016/575.pdf)

<u>Masternode compatibility</u> - The TumbleBit protocol can be fully implemented as a service in a Masternode. "*The Breeze Wallet is now fully capable of providing enhanced privacy to bitcoin transactions through a secure connection. Utilizing Breeze Servers that are pre registered on the network using a secure, trustless registration mechanism that is resistant to manipulation and censorship.*" ([[6]](https://stratisplatform.com/2017/07/17/breeze-tumblebit-server-experimental-release), [[7]](https://stratisplatform.com/2017/09/20/breeze-wallet-with-breeze-privacy-protocol-dev-update), [[12]](https://stratisplatform.com/2017/08/10/bitcoin-privacy-tumblebit-integrated-into-breeze))

<u>Nearly production ready</u> - The NTumbleBit and Breeze implementations have gained testnet status.

#### Weaknesses

<u>Privacy not 100% proven</u> - Payees have better privacy than the payers, and theoretically collusion involving payees and the Tumbler can exist to discover the identity of the payer. [[13]](https://www.bitcoinmarketinsider.com/tumblebit-wallet-reaches-one-step-forward)

<u>Tumbler service not distributed</u> - More work needs to be done to ensure persistent transaction state in case a Tumbler server goes down.

<u>Equal denominations required</u> - The TumbleBit protocol can only support a common denominator unit value. [[2]](https://eprint.iacr.org/2016/575.pdf)

#### Opportunities for Tari

Has alignment with Tari's base requirements as a trustless Masternode matching/batch processing engine with strong privacy features.

#### Threats for Tari

None

### #2 Counterparty

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #3 RSK/Rootstock and Lumino

#### What is it?

RSK (*formerly Rootstock*)  is a "2-way pegged" Bitcoin sidechain onto which the Ethereum Virtual Machine has been cloned. The platform supports Executable Distributed Code Contracts (EDCC), also known as smart contracts. RSK is scale-able up to 100 transactions per second (Tx/s) and provides a second layer scaling solution for Bitcoin as it can relieve on-chain Bitcoin transactions. ([[14]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1),  [[15]](https://lunyr.com/article/Second-Layer_Scaling), [[16]](https://www.rsk.co))

Lumino Transaction Compression Protocol (LTCP) is a technique for transaction compression that allows processing a higher volume of transactions but storing much less information. The Lumino network is a lightning-like extension of the RSK platform that uses LTCP. Delta (difference) compression of selected fields of transactions from the same owner are done by using aggregate signing of previous transactions so previous signatures can be disposed. [[17]](https://uploads.strikinglycdn.com/files/ec5278f8-218c-407a-af3c-ab71a910246d/LuminoTransactionCompressionProtocolLTCP.pdf)

Each transaction contains a set of persistent fields called the Persistent Transaction Information (PTI), and a compound record of user transaction data called the SigRec. A Lumino block stores two Merkle trees - one containing all PTIs and the other all transaction IDs (hash of the signed SigRec). This second Merkle  tree is conceptually similar to the Segwit witness tree, thus forming the witness part. Docking is the process where SicRec and signature data can be pruned from the block chain if valid linked PTI information exist.  [[17]](https://uploads.strikinglycdn.com/files/ec5278f8-218c-407a-af3c-ab71a910246d/LuminoTransactionCompressionProtocolLTCP.pdf)

![LuminoDataPruning](C:\Users\pluto\Documents\Code\tari-university\layer2scaling\more-landscape\sources\LuminoDataPruning.PNG)



#### Who does it?

RSK, which has been newly launched on main net in January 2018. The Lumino Network must still be launched in test net. ([[18]](https://cryptonewsmonitor.com/2017/11/11/bitcoin-based-ethereum-rival-rsk-set-to-launch-next-month), [[19]]([18] ))

#### Strengths

- The Lumino Network promises high efficiency in pruning the RSK block chain

#### Weaknesses

- The Lumino Network has not been released yet
- Details about how the Lumino Network will handle payment channels were not decisive in the white paper [[17]](https://uploads.strikinglycdn.com/files/ec5278f8-218c-407a-af3c-ab71a910246d/LuminoTransactionCompressionProtocolLTCP.pdf)

#### Opportunities for Tari

LTCP pruning may be beneficial to Tari

#### Threats for Tari

None

### #4 Drivechains

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #5 Scriptless scripts

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #6 Braiding

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #7 Directed Acyclic Graph (DAG)

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

## Observations

- ???

## References

[1] TumbleBit: An Untrusted Bitcoin-Compatible Anonymous Payment Hub, http://cs-people.bu.edu/heilman/tumblebit, Date accessed: 2018-07-12.

[2] TumbleBit: An Untrusted Bitcoin-Compatible Anonymous Payment Hub, Heilman E. et. el., https://eprint.iacr.org/2016/575.pdf, Date accessed: 2018-07-08.

[3] Anonymous Transactions Coming to Stratis, [https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e](https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e), Date accessed: 2018-07-08.

[4] TumbleBit Proof of Concept GitHub Repository, https://github.com/BUSEC/TumbleBit, Date accessed: 2018-07-08.

[5] NTumbleBit GitHub Repository, https://github.com/nTumbleBit/nTumbleBit, Date accessed: 2018-07-12.

[6] Breeze Tumblebit Server Experimental Release, https://stratisplatform.com/2017/07/17/breeze-tumblebit-server-experimental-release, Date accessed: 2018-07-12.

[7] Breeze Wallet with Breeze Privacy Protocol (Dev. Update), https://stratisplatform.com/2017/09/20/breeze-wallet-with-breeze-privacy-protocol-dev-update, Date accessed: 2018-07-12.

[8] Blindly Signed Contracts - Anonymous On-chain and Off-chain Bitcoin Transactions, Heilman E. et. el., https://eprint.iacr.org/2016/056.pdf, Date accessed: 2018-07-12.

[9] Conference: Scaling Bitcoin 2016 Milan, TumbleBit: An Untrusted Bitcoin-Compatible Anonymous Payment Hub - Ethan Heilman & Leen AlShenibr - 08 October 2016, https://www.youtube.com/watch?v=8BLWUUPfh2Q&feature=youtu.be&t=1h3m10s, Date accessed: 2018-07-13.

[10] Better Bitcoin Privacy, Scalability: Developers Making TumbleBit a Reality, https://bitcoinmagazine.com/articles/better-bitcoin-privacy-scalability-developers-are-making-tumblebit-reality, Date accessed: 2018-07-13.

[11] Bitcoinwiki: Contract, https://en.bitcoin.it/wiki/Contract, Date accessed: 2018-07-13.

[12] Bitcoin Privacy is a Breeze: TumbleBit Successfully Integrated Into Breeze, https://stratisplatform.com/2017/08/10/bitcoin-privacy-tumblebit-integrated-into-breeze, Date accessed: 2018-07-13.

[13] TumbleBit Wallet Reaches One Step Forward, https://www.bitcoinmarketinsider.com/tumblebit-wallet-reaches-one-step-forward, Date accessed: 2018-07-13.

[14] A Survey Of Second Layer Solutions For Blockchain Scaling  Part 1, https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1, https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1, Date accessed: 2018-07-16.

[15] Second-Layer Scaling, https://lunyr.com/article/Second-Layer_Scaling, Date accessed: 2018-07-16.

[16] RSK website, https://www.rsk.co, Date accessed: 2018-07-16.

[17] Lumino Transaction Compression Protocol (LTCP), Lerner S. D., https://uploads.strikinglycdn.com/files/ec5278f8-218c-407a-af3c-ab71a910246d/LuminoTransactionCompressionProtocolLTCP.pdf, Date accessed: 2018-07-16.

[18] Bitcoin-Based Ethereum Rival RSK Set to Launch Next Month, https://cryptonewsmonitor.com/2017/11/11/bitcoin-based-ethereum-rival-rsk-set-to-launch-next-month, Date accessed: 2018-07-16.

[19] RSK Blog website, https://media.rsk.co/, Date accessed: 2018-07-16.



## Contributors

- [https://github.com/hansieodendaal](https://github.com/hansieodendaal)
- [https://github.com/ksloven](https://github.com/ksloven)
- ???