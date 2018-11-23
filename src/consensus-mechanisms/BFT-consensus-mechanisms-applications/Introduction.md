# Introduction to Applications of Byzantine Consensus Mechanisms 

When considering the how Tari will potentially build its second layer, an analysis of the most promising Byzantine Consensus Mechanisms and their applications was sought. 

Important to consider is the 'scalability trilemma'; a phrase referred to by Vitalik Buterin, which takes into account the potential trade-offs regarding decentralization, security and scalability. [[19]]

**Decentralization** : a core principle on which majority of the systems are build, taking into account censorship-resistance and ensuring that everyone, without prejudice, is permitted to partake in the decentralized system. 

**Scalability** : encompasses the ability of the network to process transactions. Thus, if a public block chain is deemed to be efficient, effective and usable, it should be designed to handle millions of users on the network. 

**Security** : refers to the immutability of the ledger and takes into account threats of 51% attacks, Sybil attacks and DDoS attacks etc. 

Through the recent development of this ecosystem, most block chains have focused on two of the three factors, namely decentralization and security; at the expense of scalability. The primary reason for this is that nodes must reach consensus before transactions can be processed. [[19]]

This report sees the examination of proposals considering Byzantine Fault Tolerant (BFT) consensus mechanisms and considers their feasibility and efficiency in meeting the characteristics of scalability, decentralization and security. In each instance the protocol assumptions, reference implementations and discernment on whether the protocol may be used for Tari as a means to maintain the distributed asset state will be assessed. 

This report discusses several terms and concepts related to consensus mechanisms; these include definitions of [Consensus](./Appendix.md#consensus), [Binary Consensus](./Appendix.md#binary-consensus), [Byzantine Fault Tolerance](./Appendix.md#byzantine-fault-tolerance), [Practical Byzantine Fault Tolerant Variants](./Appendix.md#practical-byzantine-fault-tolerant-variants), [Deterministic and Non-Deterministic Protocols](./Appendix.md#deterministic-and-non-deterministic-protocols) and [Scalability-performance trade off](./Appendix.md#scalability-performance-trade-off). An important characteristic of consensus mechanisms are degrees of synchrony which range from [Synchrony](./Appendix.md#synchrony), [Partial Synchrony](./Appendix.md#partial-synchrony), [Weak Synchrony](./Appendix.md#weak-synchrony), [Random Synchrony](./Appendix.md#random-synchrony) and [Asynchrony](./Appendix.md#asynchrony), as well as [The Problem with Timing Assumptions](./Appendix.md#the-problem-with-timing-assumptions). Definitions on [Denial of Service Attack](./Appendix.md#denial-of-service-attack), [The FLP Impossibility](./Appendix.md#the-flp-impossibility) and [Randomized Agreement](./Appendix.md#randomized-agreement) are also provided.

## A brief survey of Byzantine Fault Tolerant Consensus Mechanisms

Many peer-to-peer online Real-time strategy games use a modified Lockstep protocol as a consensus protocol in order to manage game state between players in a game. Each game action results in a game state delta broadcast to all other  players in the game along with a hash of the total game state. Each player validates the change by applying the delta to their own game state and comparing the game state hashes. If the hashes do not agree then a vote is cast, and those players whose game state is in the minority are disconnected and removed from the game (known as a desync.) [[21]]

[19]: https://bitcoinist.com/breaking-down-the-scalability-trilemma/
"Breaking down the Blockchain 
Scalability Trilemma, Asolo,"

[21]: https://en.wikipedia.org/wiki/Consensus_(computer_science)
"Consensus Mechanisms, Wikipedia"
