

# Grin design choice criticisms - Truth or Fiction

Grin is a blockchain and cryptocurrency implemented in Rust that make use of MimbleWimble transactions and the Cuckatoo algorithm to perform Proof-of-Work calculations. The main goals of the Grin project are: privacy, transaction scaling and design simplicity to promote long term maintenance of the Grin source code [[1]](https://medium.com/novamining/grin-testnet-is-live-98b0f8cd135d).

During the development of the Grin project, the developers have received criticisms from the community on many of the design and implementation decisions that they have made. We will have a look at some of these criticism and determine if their is some truth to these concerns or if the concerns are unwarranted or invalid. Some suggestions will be made on how these problem could be improved or addressed.

![sample](sources/intro.PNG)



We will investigate their selected emission scheme, Proof-of-Work (PoW) algorithm, their choice of cryptographic curve used for signatures and the database technology used for their wallets. Each of these topics will be discussed in detail, starting with their selected emission scheme.

## Monetary policy due to static emission scheme

Bitcoin has a limited and finite supply of coins. It makes use of 10 minute block times, where the initial reward for solving the first block was 50 BTC. This reward is reduced every 4 years, by halving it, until a maximum of 21 million coins are in circulation [[2]](https://bitcoin.org/bitcoin.pdf). During this process, the transaction fees and newly minted coins are paid to miners and is used as an incentive for miners to maintain the blockchain. Once all 21 million Bitcoins are released, only transaction fees will be paid to miners. Many fear that paying miners only transaction fees in the future will not be sufficient to maintain a large network of miners and it will result in the centralisation of the network. As only large mining farms will be able to perform the mining task in a profitable manner. Others believe that in time mining fees will increase and hardware costs for miners will decrease making the act of mining and maintaining the bitcoin blockchain remain lucrative and profitable [[3]](https://www.investopedia.com/tech/what-happens-bitcoin-after-21-million-mined/).

Grin has decided on a different approach, where they will have an unlimited and infinite supply of coins. It will make use of a static emission rate, where a constant 60 Grin is released as a reward for solving every block. Their algorithm makes use of a block goal of 60 seconds. This will result in roughly 1 coin being created every second, for as long as they blockchain is being maintained [[4]](https://www.grin-forum.org/t/emmission-rate-of-grin/171).

Their primary motivations for selecting a static emission rate is: 

- their will be no upper limit on the amount of coins that can be created
- the percentage of newly created coins compared to the total coins in circulation will tend to zero
- it will mitigate the effect of orphaned and lost coins
- It will encourage spending rather than holding of coins

Unfortunately, the selected emission rate will result in Grin becoming an inflationary currency with more than 10% inflation for the first 10 years, that is higher than most competing cryptocurrencies or fiat systems. In comparison to other cryptocurrencies such as Monero, that will have less than 1% inflation after the first 8 years in circulation, and have a decreasing 0.87% inflation with the start of their tail emissions [[5]](https://www.reddit.com/r/Monero/comments/512kwh/useful_for_learning_about_monero_coin_emission/d78tpgi). Monero will have a better potential of being used as a Store of Value (SoV) in the long run.

The fixed emission rate of Grin on the other hand will limit its use as a SoV, as it will experience constant price pressure, which might make it difficult for Grin to maintain a high value initially, while the inflation rate remains high. This high inflation rate might encourage Grin to rather be used as a Medium of Exchange (MoE), [[6]](https://www.reddit.com/r/grincoin/comments/91g1nx/on_grin_mimblewimble_and_monetary_policy/) as it will take approximately 50 years for the inflation to drop below 2%. The Grin team believes that the inflation rate is not that high as many coins are lost and become unusable on a blockchain. These lost coins should be subtracted from the inflation rate. The Grin team believes that by selecting a high inflation rate it will improve the distribution of coins as holding of coins will be discouraged. They also hope that a high inflation rate will produce natural pricing and limit price manipulation by large coin holders [[7]](https://github.com/mimblewimble/docs/wiki/Monetary-Policy). Only time will tell if this is the case.

##From ASIC Resistant to ASIC Friendly

Initially, the Grin team proposed using two ASIC resistant algorithms: Cuckoo cycles and a high memory requirement Equihash algorithm called Equigrin. These algorithms were selected to encourage mining decentralisation. ASIC resistance was obtained by having high memory requirements for the PoW algorithms, limiting its calculation to CPUs and High-range GPUs [[8]](https://www.grin-forum.org/t/proof-of-work-update/713). The plan was to adjust the parameters of these PoW algorithms every 6 months to deter stealth ASIC mining and move over to using only Cuckoo cycles as the primary PoW algorithm.

Recently, the Grin team proposed to switch to a new dual PoW system, where one PoW algorithm is ASIC friendly and the other PoW algorithm is not. Grin will now make use of the new Cuckatoo Cycle. The Cuckatoo PoW algorithm is a variation of Cuckoo that aims to be more ASIC friendly [[9]](https://www.grin-forum.org/t/meeting-notes-governance-sep-25-2018/874).  This is achieved by using plain bits for ternary counters and requiring large amounts of SRAM to speedup the memory latency bound access of random node bits. SRAM tends to be limited on GPU and CPU processors, but increasing SRAM on ASIC processors are much easier to implement [[10]](https://github.com/tromp/cuckoo).

![sample](sources/attack51.JPG)

ASIC miners tend to be specialised hardware that is very efficient at calculating and solving specific PoW algorithms. Encouraging ASIC miners on your network, might not seem like a bad idea as your mining network will have a higher hash rate making it more difficult to hack and it will use less electrical power compared to using primarily CPU and GPU based miners. Unfortunately, a negative side effect of running a PoW algorithm that is ASIC friendly is that your network of miners will become more centralised. General consumers do not have access to this type of hardware and tend to be primarily reserved for large corporations establishing mining farms. This will result in your blockchain becoming more vulnerable to 51% attacks [[11]](https://www.investopedia.com/terms/1/51-attack.asp), especially when specific ASIC manufacturers recommend or enforce their hardware to make use of specific mining pools.

Using general purpose and multi use hardware such as CPUs and GPUs that are primarily used for gaming, ensures that your network of miners are more widely distributed and not controlled by a single potential bad player. This will make it more difficult for a single entity to control more than 50% of the networks total computational power, limiting the potential of double spends. Selecting to be ASIC resistant or ASIC friendly is an important decision that can affect the security of your blockchain.

## Choice of Cryptographic Elliptic-curve - secp256k1

Elliptic curve cryptography is used for generating Private and Public key pairs that can be used for digital signatures, authorisation and authenticating individuals and transactions. It is much more secure and require smaller keys for similar security compared to other Public-key cryptography techniques such as RSA [[12]](https://hackernoon.com/what-is-the-math-behind-elliptic-curve-cryptography-f61b25253da3).

![sample](sources/publickey.PNG)

Secp256k1 is an elliptic curve defined in the Standards for Efficient Cryptography [[13]](http://www.secg.org/) and is used for digital signatures in a number of cryptocurrencies such as Bitcoin, Ethereum, EOS, Litecoin, etc [[14]](https://en.bitcoin.it/wiki/Secp256k1). Grin also makes use of this same elliptic curve [[15]](https://www.grin-forum.org/t/schnorr-signatures-in-grin-information/730). Some security experts recommend not using the secp256k1 curve as some issues have been uncovered, but not necessarily exploited. One of these problems are that the complex-multiplication field discriminant is not high enough to be secure, this could result in potential exploits and it will make this curve easier to break [[16]](http://safecurves.cr.yp.to/disc.html).

A number of alternative curves exist that could be used to improve security such as ED25519. The Ed25519 signature scheme makes use of Edwards-curve Digital Signature Algorithm (EdDSA) and uses SHA-512 and Curve25519 [[17]](https://cr.yp.to/ecdh/curve25519-20060209.pdf) to build a fast signature scheme without sacrificing security. Other potential cryptographic curves can also be investigated using SafeCurves [[18]](http://safecurves.cr.yp.to/).

## Storing wallet state using LMDB

Grin uses the Lightning Memory-Mapped Database(LMDB) to maintain the state of Grin Wallets [[19]](https://github.com/mimblewimble/grin/blob/master/doc/wallet/usage.md). LMDB is used by a number of cryptocurrencies, but this does not seem like the optimal choice as a number of alternatives with better characteristics exist such as LevelDB[[20]](http://leveldb.org/), RocksDB[[21]](https://rocksdb.org/) and HyperLevelDB[[22]](http://hyperdex.org/). Most of these alternatives have better performance such as producing small database sizes and performing faster queries [[23]](https://www.influxdata.com/blog/benchmarking-leveldb-vs-rocksdb-vs-hyperleveldb-vs-lmdb-performance-for-influxdb/). RocksDB or LevelDB seem like better alternatives to LMDB as they produced the fastest reads and deletes and produce some of the smallest databases compared to the other database libraries [[24]](https://github.com/lmdbjava/benchmarks/blob/master/results/20160630/README.md). Using LMDB might not be the right choice and it might be worth investigating some of these alternatives.


## Conclusions, Observations, Recommendations

- Selecting the correct emmision rate to create a sustainable monetary policy is an important decision and care should be taken to ensure that the right balance is found between being a SoV and/or a MoE.
- Weighting the benefits and potential issues of being ASIC friendly compared to ASIC resistant need to be carefully evaluated.
- Tools such as SafeCurves can be used to select a secure elliptic curve for your application. Cryptographic curves with even potential security vulnerabilities should rather be ignored.
-  Benchmarks can be used to make better decisions when selecting external libraries for your project. From these benchmarks LevelDB and RocksDB seem like better alternatives to LMDB.

## References

[1] Mattia Franzoni, Grin: a lightweight implementation of the MimbleWimble protocol, https://medium.com/novamining/grin-testnet-is-live-98b0f8cd135d, Date accessed: 2018-10-05.

[2] Satoshi Nakamoto, Bitcoin: A Peer-to-Peer Electronic Cash System, https://bitcoin.org/bitcoin.pdf, Date accessed: 2018-10-05.

[3] Nathan Reiff, What Happens to Bitcoin After All 21 Million are Mined?, https://www.investopedia.com/tech/what-happens-bitcoin-after-21-million-mined/, Date accessed: 2018-10-07.

[4] Emission rate of Grin, https://www.grin-forum.org/t/emmission-rate-of-grin/171, Date accessed: 2018-10-15.

[5] Coin Emission and Block Reward Schedules: Bitcoin vs. Monero, https://www.reddit.com/r/Monero/comments/512kwh/useful_for_learning_about_monero_coin_emission/d78tpgi, Date accessed: 2018-10-15.

[6] On Grin, MimbleWimble, and Monetary Policy,  https://www.reddit.com/r/grincoin/comments/91g1nx/on_grin_mimblewimble_and_monetary_policy/, Date accessed: 2018-10-07.

[7] Grin - Monetary Policy, https://github.com/mimblewimble/docs/wiki/Monetary-Policy, Date accessed: 2018-10-08.

[8] Grin - Proof of word update, https://www.grin-forum.org/t/proof-of-work-update/713, Date accessed: 2018-10-15.

[9] Grin - Meeting Notes: Governance, Sep 25 2018, https://www.grin-forum.org/t/meeting-notes-governance-sep-25-2018/874, Date accessed: 2018-10-15.

[10] Cuck(at)oo Cycle, https://github.com/tromp/cuckoo, Date accessed: 2018-10-15.

[11] 51% Attack, https://www.investopedia.com/terms/1/51-attack.asp, Date accessed: 2018-10-11.

[12] Hans Knutson, What is the math behind elliptic curve cryptography?, https://hackernoon.com/what-is-the-math-behind-elliptic-curve-cryptography-f61b25253da3, Date accessed: 2018-10-14.

[13] Standards for Efficient Cryptography Group, http://www.secg.org/, Date accessed: 2018-10-11.

[14] Secp256k1, https://en.bitcoin.it/wiki/Secp256k1, Date accessed: 2018-10-15.

[15] Grin - Schnorr signatures in Grin & information, https://www.grin-forum.org/t/schnorr-signatures-in-grin-information/730, Date accessed: 2018-10-08.

[16] SafeCurves - CM field discriminants, http://safecurves.cr.yp.to/disc.html, Date accessed: 2018-10-15.

[17] Daniel J. Bernstein, Curve25519: New Diffie-Hellman Speed Records, https://cr.yp.to/ecdh/curve25519-20060209.pdf, Date accessed: 2018-10-15.

[18] SafeCurves - choosing safe curves for elliptic-curve cryptography, http://safecurves.cr.yp.to/, Date accessed: 2018-10-10.

[19] Grin - Basic Wallet, https://github.com/mimblewimble/grin/blob/master/doc/wallet/usage.md, Date accessed: 2018-10-15.

[20] LevelDB, http://leveldb.org/, Date accessed: 2018-10-15.

[21] RocksDB, https://rocksdb.org/, Date accessed: 2018-10-10.

[22] HyperLevelDB, http://hyperdex.org/, Date accessed: 2018-10-15.

[23] Paul Dix, Benchmarking LevelDB vs. RocksDB vs. HyperLevelDB vs. LMDB Performance for InfluxDB, https://www.influxdata.com/blog/benchmarking-leveldb-vs-rocksdb-vs-hyperleveldb-vs-lmdb-performance-for-influxdb/, Date accessed: 2018-10-15.

[24] Ben Alex, Lmdbjave - benchmarkshttps://github.com/lmdbjava/benchmarks/blob/master/results/20160630/README.md, Date accessed: 2018-10-14.

## Contributors

- https://github.com/neonknight64
