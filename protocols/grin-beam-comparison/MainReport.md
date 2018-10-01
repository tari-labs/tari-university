# Grin vs. BEAM, a Comparison

## Introduction
Grin and BEAM are two open-source cryptocurrency projects based on the Mimblewimble protocol. The Mimblewimble protocol was first proposed by a anonymous user using the pseudonym Tom Elvis Jedusor (the french translation of Voldemort's name from the Harry Potter series of books). This user logged onto a bitcoin research IRC channel and posted a link to a text article hosted on a Tor hidden service [1]. This article provided the basis for a new way to construct blockchain-style transactions that provided inherent privacy and the ability to dramatically compress the transaction history of the chain. This initial article presented the main ideas of the protocol but it left out a number of critical elements required for a practical implementation and even contained a mistake in the cryptographic formulation. Andrew Poelstra published a follow-up paper that addresses many of these issues and refines the core concepts of Mimblewimble [2] which have been applied to the practical implementations of this protocol in both the Grin [3] and BEAM [4] projects.

The Mimblewimble protocol describes how transacting parties will interactively work to build a valid transaction while keeping their private keys, used to prove ownership of transaction outputs, secret and keeping the value of their transaction known only to them. The protocol also performs a process called cut-through which aggregates transactions by eliminating intermediary transactions. This improves privacy and compresses the amount of data that is maintained on the blockchain [3]. This cut-through process precludes general purpose scripting systems like those found in Bitcoin. However, Andrew Poelstra proposed the concept of Scriptless scripts, which make use of Schnorr signatures, to build adaptor signatures that allow for encoding of many of the functions that scripts are used for in a Mimblewimble chain. Scriptless scripts enable functionality like Atomic swaps and Lightning network like payment channels [5].

Grin and BEAM both implement the Mimblewimble protocol but each has been built from scratch. Grin is written in RUST and BEAM in C++. The remainder of this report will focus on describing some of the aspects of each project that sets them apart. Both project are very young and many of these details are changing on a daily basis. Furthermore, the BEAM project documentation is still mostly in russian so as of the writing of this report not all the technical details are available for english readers. As such the discussion in this report will likely become out of date as the respective project evolve. 

The remainder of this report will be structured as follows: Firstly, some implementation details and unique features of the project will be discussed. Secondly, we will examine the difference in the proof-of-work algorithms employed and finally we will discuss the different governance models the projects are using.

## Unique features of Grin vs BEAM
The two projects are being independently built from scratch by different teams in different languages (Rust and C++), so there will be many differences in the raw implementations. From the perspective of features the two projects exhibit all the features inherent to Mimblewimble. The most significant difference between the projects is their proof-of-work algorithms and governance models which will be discussed in subsequent sections. Beyond the features afforded by Mimblewimble each project does include some unique features of interest.

The Grin project has built a version of the Dandelion relay protocol that supports transaction aggregation. One of the major outstanding challenges for privacy that cryptocurrencies face is that it is possible to track transactions as they are added to the mempool and propagate across the network and to link those transactions to their originating IP addresses. This information can be used to deanonymize users even on networks with strong transaction privacy. To improve privacy during the propagation of transactions to the network the Dandelion network propagation scheme was proposed [6]. In this scheme transactions are propagated in two phases, the Anonymity phase (or "stem" phase) and the Spreading phase (or "fluff" phase) as illustrated in the Figure 1. In the stem phase a transaction is propagated to only a single single randomly selected peer from the current nodes peer list. After a random number of hope along the network, each hop propagating to only a single random peer, the propagation process enters the second phase. During the fluff phase the transaction is then propagated using a full flood/diffusion method as found in most networks. This approach means that the transaction has first propagated to a random point in the network before flooding the network so it becomes much more difficult to track its origin.

![fig1](sources/dandelion-stem-fluff.png)
Figure 1: Two phases of Dandelion P2P transaction propagation [6].

Grin has adapted this approach to work with Mimblewimble transactions [7]. Grin's implementation allows for transaction aggregation and cut-through in the stem phase of propagation which provides even greater anonymity to the transactions before they spread during the fluff phase.

A unique feature of BEAM that stands out is an implementation of an auditable wallet. For a business to operate in a given regulatory environment it will need to demonstrate its compliance to the relevant authorities. BEAM has designed a wallet designed for compliant businesses which generates additional public/private key pairs specifically for audit purposes. These signatures are used to tag transactions so that only auditing authority who are given the public key can identify these transactions on the blockchain but cannot create transactions with this tag themselves. This allows a business to provide visibility to their transactions to a given authority without compromising their privacy to the public [8]

## Proof of work (PoW) mining algorithm
BEAM has opted to employ the Equihash PoW mining algorithm. Equihash was proposed in 2016 as a memory-hard PoW algorithm which relied heavily on memory-usage to achieve ASIC-resistance [9]. The goal was to produce an algorithm that would be more efficient to run on consumer GPUs as opposed to the growing field of ASIC miners, mainly produced by Bitmain at the time. It was hoped this would aid in decentralising the mining power for cryptocurrencies that used this algorithm. The idea behind Equihash's ASIC resistance was that at the time implementing memory in an ASIC was expensive and so GPUs were more efficient at calculating the Equihash PoW. This ASIC resistance did last for a while but in early 2018 Bitmain released an ASIC for Equihash which were significantly more efficient than GPUs for the Equihash configurations used by Zcash, Bitcoin Gold, Zencash to name a few. It is possible to tweak the parameters of the Equihash algorithm to make it more memory intensive and thus mke obsolete current ASICs and the older GPU mining farms but it remains to be seen if BEAM will do this. No block time has been published as of the writing of this report.

Grin initially opted to use the new Cuckoo Cycle PoW algorithm, also purported to be ASIC resistant due to being memory latency bound [10]. This means that the algorithm is bound by memory bandwidth rather than raw processor speed with the hope that it will make mining possible on commodity hardware. The Cuckoo Cycle algorithm is based on finding cycles of a certain length of edges in a bipartite graph of N nodes and M edges. The graph is bipartite because it consists of two separate groups of nodes and edges connect nodes from one set to the other. As an example lets consider nodes with even indices to be in one group and nodes with odd indices in a second group. Figure 2 shows 8 nodes with 4 randomly placed edges, N = 8 and M = 4. So if we are looking for cycles of length 4 we can easily confirm that none exist in Figure 2. By adjusting the number of edges present in the graph vs the number of nodes we can control the probability that a cycle of a certain length exists in the graph. When looking for cycles of length 4 the difficulty illustrated in Figure 2 a 4/8 (M/N) graph would mean that the 4 edges would need to be randomly chosen in an exact cycle for one to exist [11]. 

![fig2](sources/cuckoo_base_numbered_few_edges.png)<br>
Figure 2: 8 Nodes with 4 Edges, no solution [11]

If we increase the number of edges in the graph relative to the number of nodes we adjust the probability of a cycle occuring in the randomly chosen set of edges. Figure 3 shows an example of M = 7 and N = 8 case and it can be seen that a 4 edge cycle appeared. Thus, we can control the probability of a cycle of a certain length occuring by adjusting the ratio of M/N [11]. 

![fig3](sources/cuckoo_base_numbered_few_edges_cycle.png)<br>
Figure 3: Cycle Found from 0-5-4-1-0 [11]

Detecting that a cycle of a certain length has occurred in a graph with randomly selected edges becomes significantly more difficult as the number as the graphs get larger. Figure 4 shows a 22 node graph with 14 random edges in it. Can you determine if a cycle of 8 edges is present? [11]

![fig4](sources/cuckoo_base_numbered_many_edges.png)<br>
Figure 4: 22 Nodes with 14 edges, can you find a cycle 8 edges long? [11]

The Cuckoo cycle PoW algorithm is built to solve this problem. The bipartite graph that is analyzed is called a "Cuckoo Hashtable" where a key is inserted into two arrays, each with their own hash function, into a location based on the hash of the key. Each key inserted in this way produces an edge between the locations generated by the two hashing functions. Nonces are enumerated for the hashing functions until a cycle is detected of the desired length. This algorithm has two main parameters that control its difficult which is the M/N ratio and the number of nodes in the graph. There are a number of variants of this algorithm that make speed/memory tradeoffs [10]. In Grin a third difficulty parameter was introduced to more finely tune the difficulty of the PoW algorithm to ensure a 1 minute block time in the face of changing network hash rates. This was to take a Blake2b hash of the set of nonces and ensure that the result is above a difficulty threshold [11].

In August 2018 the Grin team made an announcement that they have become aware that it was likely that an ASIC would be available for the Cuckoo cycle algorithm at launch of their mainnet [12]. While they acknowledge that ASIC mining is inevitable they are concerned that the current ASIC market is very centralized (i.e. Bitmain) and that they want to foster a grassroots GPU mining community in the early days of Grin. Grin wants to aim to foster this community for 2 years by which time they hope that ASICs have become more of a commodity and thus decentralized.

To address this it was proposed to use two PoW algorithms initially. One that is ASIC Friendly (AF) and one that is ASIC Resistant (AR) and then select which PoW is used per block to balance the mining rewards over a 24h period between the two algorithms. The Governance committee resolved on 25 September to go ahead with this approach using a modified version of the Cuckoo cycle algorithm called Cuckatoo Cycle. The AF algorithm at launch will be Cuckatoo32+ which will gradually increase its memory requirements to make older single-chip ASICs obsolete over time. The AR algorithm is still not defined [13].

## Governance models and monetary policy 
Both the Grin and BEAM projects are open-source and available on Github [14, 15]. The Grin project has 75 contributors of which 8 have contributed the vast majority of the code. BEAM has 8 contributors of which 3 have contributed the vast majority of the code (at the time of writing). The two projects have opted for different models of governance. BEAM has opted to setup a foundation to manage the project and which the core developers are members of. This is the route taken by the majority of cryptocurrency projects in the space. The Grin community has decided against setting up a central foundation. The Grin community has compiled an interesting discussion of the pro's and con's of a centralized foundation [16]. This document contains a very in depth discussion weighing up the various governance functions that a foundation might serve and evaluating each use-case in depth. The Grin community came to the conclusion that while foundations are useful that they do not represent the only solution to governance problems and have opted to remain a completely decentralized community driven project. Currently decisions are made by periodic govenance meetings that are convened on Gitter with community members where an agenda is discussed and decisions are ratified. These meeting agendas and minutes can be found in the Grin Forums governance section and an example of the outcomes of such a meeting can be seen in [13].

Neither project will engage in an ICO or pre-mine but the two project also have different funding models. BEAM has attracted investors to its foundation for its initial round of funding and for sustainability will put 20% of each mining block rewards into a treasury to be used to fund further development and promotion of BEAM [17]. Grin will not levy a dev tax on the mining rewards and will rely on community participation and community funding. The Grin project does accept financial support but these funding campaigns are conducted according to their "Community Funding Principles" [18] which will be conducted on a "need-by-need" basis. A campaign will specify a specific need it is aimed at fulfilling (e.g. "Hosting fees for X for the next year") and the funding will be received by the community member who ran the campaign. This will provide 100% visibility on who is responsible for the received funds. An example of a funding campaign is the Developer Funding Campaign run by Yeastplume to fund his full-time involvement in the project from Oct 2018 to February 2019 can be seen in [19].

In terms of the monetary policy of the two projects BEAM has stated that they will be using a deflationary model with periodic halving of their mining reward and a maximum supply of BEAM of 210 million. They have not stated the details of the period of the halving. Grin has opted for an inflationary model where the block reward will remain constant, they make their arguments for this approach in [20]. This approach will asymptotically tend towards a zero percent dilution as the supply increases instead of enforcing a set supply [21]. BEAM has not specified their mining reward or fees structure as yet but based on their current documentation Grin is planning on a 60 Grin per block reward. Neither project has made a final decision of how to structure fees but the Grin project has started to explore how to set a fee baseline by using a metric of "fees per reward per minute" [21].

## Conclusions, Observations, Recommendations
In summary, Grin and BEAM are two open-source projects that are implementing the Mimblewimble blockchain scheme. Both projects are building from scratch. Grin is using Rust and BEAM is using C++ and as such there are many technical differences in their design and implementations. However, from a functional perspective both projects will support all the core Mimblewimble functionality. Each project does contain some unique functionality which was discussed. The main differences between the projects are the chosen PoW algorithms, monetary policy and their Governance models. 

These projects are still very young and many of their core design choices have not been made or tested yet. It will be interesting to keep an eye on these projects to see how their various decisions play out both technically and in terms of their monetary policy and governance models.

## References

[1] T.E. Jedusor, "MIMBLEWIMBLE", https://download.wpsoftware.net/bitcoin/wizardry/mimblewimble.txt, Date access: 2018-09-30

[2] A. Poelstra, "Mimblewimble", https://download.wpsoftware.net/bitcoin/wizardry/mimblewimble.pdf, Date accessed: 2018-09-30

[3] Introduction to Mimblewimble and Grin, https://github.com/mimblewimble/grin/blob/master/doc/intro.md, Date accessed: 2018-09-30.

[4] BEAM: The Scalable Confidential Cryptocurrency, https://docs.wixstatic.com/ugd/87affd_3b032677d12b43ceb53fa38d5948cb08.pdf, Date accessed: 2018-09-28

[5] A. Gibson, "Flipping the scriptless script on Schnorr", https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/, Date accessed: 2018-09-30

[6] Shaileshh Bojja Venkatakrishnan, Giulia Fanti, and Pramod Viswanath, "Dandelion: Redesigning the Bitcoin Network for Anonymity.", Proc. ACM Meas. Anal. Comput. Syst. 1, 1, 2017

[7] Dandelion in Grin: Privacy-Preserving Transaction Aggregation and Propogation, https://github.com/mimblewimble/grin/blob/master/doc/dandelion/dandelion.md, Date accessed: 2018-09-30

[8] Wallet Audit, https://github.com/beam-mw/beam/wiki/Wallet-audit, Date accessed: 2018-09-30

[9] Alex Biryukov, Dmitry Khovratovich, "Equihash: asymmetric proof-of-work based on the Generalized Birthday problem", Proceedings of NDSS, 2016

[10] Cuckoo Cycle, https://github.com/tromp/cuckoo, Date accessed: 2018-09-30

[11] Grin's Proof-of-Work, https://github.com/mimblewimble/grin/blob/master/doc/pow/pow.md, Date accessed 2018-09-30

[12] I. Peverell, "Proof of work update", https://www.grin-forum.org/t/proof-of-work-update/713

[13] Meeting Notes: Governance, Sep 25 2018, https://www.grin-forum.org/t/meeting-notes-governance-sep-25-2018/874, Date accessed: 2018-09-30

[14] Grin Github Repository, https://github.com/mimblewimble/grin, Date accessed: 2018-09-30

[15] BEAM Github Repository, https://github.com/beam-mw/beam, Date accessed: 2018-09-30

[16] Regarding Foundation, https://github.com/mimblewimble/docs/wiki/Regarding-Foundations, Date accessed: 2018-09-30

[17] BEAM Features, https://www.beam-mw.com/features, Date accessed: 2018-09-30

[18] Grin's Community Funding Principles, https://grin-tech.org/funding.html, Date accessed: 2018-09-28

[19] Oct 2018 - Feb 2019 Developer Funding - Yeastplume, https://grin-tech.org/yeastplume.html, Date accessed: 2018-09-30

[20] Monetary Policy, https://github.com/mimblewimble/docs/wiki/Monetary-Policy, Date accessed: 2018-09-30

[21] Economic Policy: Fees and Mining Reward, https://github.com/mimblewimble/grin/wiki/fees-mining, Date accessed: 2018-09-30
