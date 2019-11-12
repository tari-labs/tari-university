# Tari Labs Research Overview - Q2 of 2018 till Q3 of 2019 

During Q2 of 2018, the first team Tari Labs employed, was the research team. The research team initially consisted of 
only two team members, but has recently (Q3 of 2019) been supplemented by a third team member. Blockchain technology was 
totally foreign to all three team members when they joined. The task was given to learn as much as possible about 
Blockchain technology, and to perform pertinent research that would support general knowledge and possible application 
to the Tari Blockchain eco system. 

## TLU

Towards supporting the Tari Blockchain eco system, [Tari Labs University](https://tlu.tarilabs.com/) [TLU] was 
established as an open source knowledge project on [GitHub]( https://github.com/tari-labs/tari-university). Apart form 
the the research team, all new Tari Lab employees were consequently motivated to contribute to the content of the TLU. 
Due to TLU being open source and managed on GitHub, contributions from all community members are welcomed. Contributions 
can range from reviewing and submitting pull requests on existing publications, performing peer reviews and creating new 
content. Making use of such a wide range of contributors has the effect that the TLU content caters for an audience with 
different levels of expertise.

The primary platform to publish research deliverables is the TLU, and it is seen as a valuable and public resource. To 
that end guiding principles, a [style guide](https://tlu.tarilabs.com/preface/style-guide.html) and a submission process 
have been established to facilitate creating professional content. As the TLU is also used as a platform to get 
contributors that is new to Blockchain technology up to speed, a Learning Paths module has also been introduced.

## Research

Current research themes to support the Tari Blockchain eco system involves: fast payment channels/side chains; 
Bulletproofs rank 1 constraint systems; and Tari eco system modelling. Some of the reports published by the research 
team, to name just a few, are:

- [Layer 2 Scaling Survey](https://tlu.tarilabs.com/scaling/layer2scaling-landscape/layer2scaling-survey.html)

  In the blockchain and cryptocurrency world, transaction processing scaling is a tough problem to solve. This is 
  limited by the average block creation time, the block size limit, and the number of newer blocks needed to confirm a 
  transaction (confirmation time). This report explains what layer 2 scaling is, why it is important to the Tari 
  Blockchain eco system and gives an overview of current layer 2 scaling initiatives.

- [Merged Mining Introduction](https://tlu.tarilabs.com/merged-mining/merged-mining-scene/MergedMiningIntroduction.html#merged-mining-introduction)

  Merged mining is the act of using work done on another blockchain (the Parent) on one or more than one Auxiliary 
  blockchain and to accept it as valid on its own chain, using Auxiliary Proof-of-Work (AuxPoW), which is the 
  relationship between two blockchains for one to trust the other's work as their own. This report explains what merged 
  mining is on a high level, present some interesting facts and case studies and end of discussing known attack vectors 
  for blockchains being merged mined.

- [Introduction to Applications of Byzantine Consensus Mechanisms](https://tlu.tarilabs.com/consensus-mechanisms/BFT-consensus-mechanisms-applications/MainReport.html)

  When considering the concept of consensus in cryptocurrency and cryptographic protocols, the Byzantine Generals 
  Problem is often referenced, which stems from an analogy, as a means to understand the problem of distributed 
  consensus. This report examines proposals considering BFT consensus mechanisms, and considers their feasibility and 
  efficiency in meeting the characteristics of scalability, decentralization and security. In each instance, the 
  following are assessed:
    protocol assumptions;
    reference implementations; and
    discernment regarding whether the protocol may be used for Tari as a means to maintain the distributed asset state.

- [Probability of a Byzantine Takeover of the Digital Assets Network](https://tlu.tarilabs.com/network-analysis/probabilistic-attack/byzantine_takeover_of_the_DAN.html)

  This investigation aims to provide answers to questions posed about the workings of the Tari Digital Assets Network 
  (DAN) environment. It covers probabilistic attack vector with regard to the total nodes, compromised nodes, committee 
  size and BFT threshold.  The investigation attempts to answer the following question: *What is the percentage chance 
  of controlling the majority of nodes in a random sample with varying quantities of the total number of nodes, 
  committee size, bad nodes and BFT threshold?*

- [Introduction to Scriptless Scripts](https://tlu.tarilabs.com/cryptography/scriptless-scripts/introduction-to-scriptless-scripts.html)

  Scriptless Scripts are a means to execute smart contracts off-chain, through the use of Schnorr signatures. The 
  concept of Scriptless Scripts was born from Mimblewimble, which is a blockchain design that with the exception of 
  kernels and their signatures, does not store permanent data. This report lists and discusses the main scriptless 
  scripts and its benefits.

- [Mimblewimble Multiparty Bulletproof UTXO](https://tlu.tarilabs.com/protocols/mimblewimble-mp-bp-utxo/MainReport.html)

  In Mimblewimble, the concept of a Bitcoin-type multi-signature (multisig) applied to an Unspent Transaction Output 
  (UTXO) does not really exist. Unlike Bitcoin, Mimblewimble transactions do not involve payment addresses, as all 
  transactions are confidential. Another fundamental difference is that for any Mimblewimble transaction, all parties, 
  i.e. all senders and all receivers, must interact to conclude the transaction. This report demonstrates the concept 
  of an $n-\text{of}-n$ and  $m-\text{of}-n$ Mimblewimble multiparty Bulletproof UTXO.

## Clacks

A paper called "The Clacks. A Fast and Secure Payment Channel for Mimblewimble." has recently jointly been submitted to 
the [Cryptoeconomic Systems Conference 2020](https://cryptoeconomicsystems.pubpub.org/) and the [Stanford Blockchain 
Conference 2020](https://cbr.stanford.edu/sbc20). After this report has been peer reviewed one or more reports about the 
content will be published on TLU.

