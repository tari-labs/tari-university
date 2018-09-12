
Applications of Byzantine Consensus Mechanisms 

Abstract 

This report investigates the most promising Byzantine Consensus Mechanisms like HoneyBadger, PARSEC, Stellar Consensus Protocol (SCP), etc. to achieve optimal consensus performance and safety against ill-behaved participants.

Introduction 



Consensus Algorithms 



Byzantine Faults 

A Byzantine fault is a fault presenting different symptoms to different observers. A network is Byzantine Fault Tolerant (BFT) when it can provide service and reach a consensus despite faults or failures of the system. It is through consensus algorithms that it is possible for distributed networks to solve the Byzantine Generals Problem. 

Protocol for Asynchronous, Reliable, Secure and Efficient Consensus (PARSEC)

PARSEC is a byzantine fault tolerant consensus algorithm possessing weak synchrony assumptions (highly asynchronous, assuming random delays with finite expected value)

Similar to Hashgraph, it has no leaders, no round robin, no proof of work and reaches eventual consensus with probability one. If differs from Hashgraph, in that it provides high speed in the absence and presence of faults.  Thus, it avoids the structures of delegated proof of stake, which requires a trusted set of leaders, and does not have a round robin (where a set of instated miners sign each block)

It is fully open, unlike Hashgraph, which is patented and closed sourced and the GPLv3 written in Rust will soon be available. [1][5]

Similar to Honeybadger BFT, this protocol is composed through the additions of interesting ideas presented in literature. 

Like Hashgraph and Avalanche, a gossip protocol is used to allow efficient communication between nodes. [1]

"The general problem of reaching Byzantine agreement on any value is reduced to the simpler problem of reaching binary Byzantine agreement on the nodes participating in each decision. This allows it to reuse the elegant binary Byzantine agreement protocol after adapting it to the gossip protocol". [2]

Finally, the need for a trusted leader or a trusted setup phase implied is removed by porting the key ideas to an asynchronous setting [3]

The network is set to N of N instances of the algorithm communicating via randomly synchronous connections. 

Due to random synchrony, all users can reach an agreement on what is going on, there is no guarantee for nodes on the timing that they should be receiving messages and a possibility of up to t Byzantine (arbitrary) failures are allowed, were 3t<N. The instances where no failures have occurred are deemed correct or honest, while the failed instances are termed faulty or malicious. Since a Byzantine failure model allows for malicious behavior, any set of instances containing more than 2/3N of them are referred to as the supermajority. 

Gossip Protocol 

A gossip protocol is a procedure or process of computer–computer communication. The manner in which the protocol functions is  based on the way social networks disseminate information or how epidemics spread. Modern distributed systems have implemented gossip protocols to solve problems that might be difficult to solve in other ways, either because the underlying network has an inconvenient structure, is extremely large, or because gossip solutions are the most efficient ones available.[4]

Many Forms of Timing Assumptions (Degrees of Synchrony) 

Random Synchrony

Messages are delivered with random delays, such that the average delay is finite. There many be periods of arbitrarily long days (this is a weaker assumption than weak synchrony, and only a bit stronger than full asynchrony, where the only guarantee is that messages are eventually delivered). As in full asynchrony, it is impossible to tell whether an instance has failed by completely stopping or if there is just a delay in message delivery. [1]

Synchrony

Here, the time for nodes to wait and receive information is predefined. If a node has not received an input within the redefined time structure,  there is a problem. [5] A △-synchronous network guarantees that every message sent is delivered after at most a delay of △ (where △ is a measure of real time) [6] Synchronous protocols come to a consensus every x seconds. [5]

Partial Synchrony 

Here, the network retains some form of a predefined timing structure, however it can operate within knowing the assumption of how fast nodes can exchange messages over the network. Instead of pushing out a block every x seconds, in a partially synchronous blockchain would gauge the limit, with messages always being sent and received within the unknown deadline. 

Distributed Ledger Technology that is not synchronous tend to be partially synchronous. Examples include Cosmos. 

Partially synchronous protocols come to a consensus every unknown amount of seconds. [5]

Asynchrony 

In an asynchronous network, the adversary can deliver messages in any order and at any time, however the message must eventually be delivered between correct nodes. Nodes in an asynchronous network effectively have no use for real time clocks, and can only take actions based on the ordering of messages they receive. [6]  The speed is determined by the speed at which the network communicates-instead of a fixed limit of x seconds. 

An asynchronous protocol requires a different means to decide when all nodes are able to come to a consensus. 

The Problem 

The problem with both synchronous and partially synchronous assumptions is that "the protocols based on timing assumptions are unsuitable for decentralized, cryptocurrency settings, where network links can be unreliable, network speeds change rapidly, and network delays may even be adversarially induced."[6]

Denial of Service Attack 

Basing a protocol on timings, exposes the network to Denial of Service attacks. A synchronous protocol will be deemed unsafe if a DoS slows down the network sufficiently. Even though a partially synchronous protocol would be safe, it would be unable to operate, as the messages would be exposed to interference. 

An asynchronous protocol would be able to function under a DoS attack, however it is difficult to reach consensus, as it is impossible to know if the network is under attack, or if a particular message is delayed by the protocol itself. 

The Solution 

This problem is what PARSEC aims to solve. PARSEC claims it is possible to reach 100% certainty consensus with no reliance on timing, with up to 1/3 of the network being dishonest even if the network is attacked. 

CommonCoin 



Summary

Byzantine agreement schemes are considered especially well suited for permissioned blockchains in which all participants are known.Well-known examples include Tendermint , Quorum, and Chain [10], which utilizes the Federated Consensus algorithm. 

BFT protocols, however, face several limitations when employed in permissionless blockchains: 

(i) they do not scale well with the number of participants and their performance degrades drastically

for the targeted network sizes, and 

(ii) they are not yet widely established in this setting due to additional security issues, e. g. Sybil attacks where an attacker participates using several identities. This way, an attacker might partake in the BFT protocol as multiple participants, thereby manipulating and corrupting the process in her advantage. 

Current approaches try to solve these issues in multiple ways. HoneyBadgerBFT works with a fixed set of servers to run the consensus; however, this leads to centralization and allows an attacker to specifically target these servers. In Stellar, each participant forms a quorum of other users, thus creating a trust hierarchy, which requires complex trust decisions. Algorand allows participants to privately check whether they are chosen for consensus participation, and requires only one message per user, thus limiting possible attacks. 

Algorand [8], for example, scales up to 500, 000 users by employing Verifiable Random Functions, which are pseudorandom functions able to provide verifiable proofs that the output of said function is correct.

References 

1. Protocol for Asynchronous, Reliable, Secure and Efficient Consensus (PARSEC) WhitePaper 20 June 2018. http://docs.maidsafe.net/Whitepapers/pdf/PARSEC.pdf
2. Mostefaoui A. et al. Signature-Free Asynchronous Byzantine Consensus with t<n/3 and O(n2) Messages. 2014. ACM PODC. https://hal.inria.fr/hal-00944019v2/document
3. Micali s. Byzantine Agreement, Made Trivial 2018. https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Distributed%20Computation/BYZANTYNE%20AGREEMENT%20MADE%20TRIVIAL.pdf
4. Gossip Protocol. Wikipedia https://en.wikipedia.org/wiki/Gossip_protocol
5. Project Spotlight: Maidsafe and PARSEC Part 1 https://medium.com/@flatoutcrypto/project-spotlight-maidsafe-and-parsec-part-1-4830cec8d9e3
6. The Honey Badger of BFT Protocols https://eprint.iacr.org/2016/199.pdf

