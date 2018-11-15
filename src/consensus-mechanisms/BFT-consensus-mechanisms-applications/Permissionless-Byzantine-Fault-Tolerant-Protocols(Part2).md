## Contents

  - [HoneyBadgerBFT](#honeybadgerbft)
  - [Stellar Consensus Protocol](#stellar-consensus-protocol)
  - [LinBFT](#linbft)
  - [Algorand](#algorand)
  - [Thunderella](#thunderella)
  - [Avalanche](#avalanche)
  - [PARSEC](#parsec)
  - [Democratic BFT](#democratic-bft)

### HoneyBadgerBFT

HoneyBadgerBFT was released in November 2016 and is seen as the first practical **asynchronous** BFT consensus algorithm. Designed with cryptocurrencies in mind, where bandwidth is considered scarce, but an abundance of CPU power is available. Thus, the protocol implements public-private key encryption to increase the efficiency of the establishment of consensus. The protocol works with a fixed set of servers to run the consensus; however, this leads to centralization and allows an attacker to specifically target these servers. [[9]]

In its threshold encryption scheme, any one party can encrypt a message using a master public key, and it requires f+1 correct nodes to compute and reveal decryption shares for a ciphertext before the plaintext can be recovered.


The work of HoneyBadgerBFT is closely related to [SINTRA](#sintra) , which as mentioned before, is a system implementation based on the asynchronous atomic broadcast protocol from Cachin et al. [[41]] This protocol consists of a reduction from Atomic Broadcast Channel (ABC) to Asynchronous Common Subset (ACS), as well as a reduction from ACS to Multi-Value Validated Agreement (MVBA) 

HoneyBadger offers a novel reductions from ABC to ACS that provides better efficiency (by O(N) factor) through batching, while using threshold encryption to preserve censorship resilience. Better efficiency is also obtained by cherry-picking improved instantiations of sub-components. For example, the expensive MVBA is circumvented by using an alternative ACS along with an effect reliable broadcast (RBC). [[6]]

### Stellar Consensus Protocol

Stellar Consensus Protocol (SCP) is an **asynchronous** protocol proposed by David Mazieres. It considered to be a global consensus protocol consisting of nomination protocol and ballot protocol, and is said to be BFT by bringing with it the concept of quorum slices and defeated byzantine fault tolerance. [[11]]

Each participant forms a quorum of other users, thus creating a trust hierarchy, which requires complex trust decisions. [[9]]

Initially the nomination proctor is run. During this, new values called candidate values are proposed for agreement. Each node receiving these values will vote for a single value among these. Eventually it results in unanimously selected values for that slot. 

After successful execution of nomination protocol, the nodes deploy the ballot protocol. This involves the federated voting to either commit or abort the values resulting from nomination protocol. This results in externalizing the ballot for the current slot. The aborted ballots are now declared irrelevant. However, there can be stuck states where nodes cannot reach a conclusion, whether to abort or commit a value. This situation is avoided by moving it to a higher valued ballot, considering it in a new ballot protocol execution. This aids in case a node believes that this stuck ballot was committed. Thus SCP assures avoidance and management of stuck states and thus provides liveliness. 

The concept of quorum slices in case of SCP provides asymptotic security and flexible trust, making it more acceptable than other earlier consensus algorithms utilizing Federated BFT, like the Ripple consensus protocol. [[14]] Here, the user is provided more independence in deciding whom to trust. [[15]]

SCP protocol claims to be free of blocked states, provides decentralized control, asymptotic security, flexible trust and low latency. But it does not guarantee safety all the time. If the user node chooses an inefficient quorum slice security is not guaranteed. In the event of partition or misbehaving nodes, it halts progress of the network until consensus can be reached.

### LinBFT

LinBFT is a Byzantine fault tolerance protocol for block chain systems that allows for the amortized communication volume per block *O(n)* under reasonable conditions (where *n* is the number of participants) while satisfying deterministic guarantees on safety and liveness. It satisfies liveness in a **partially synchronous** network. 

LinBFT cuts down its *O(n*<sup>4</sup>*)* complexity by implementing changes each by  *O(n)*: linear view change, threshold signatures and verifiable random functions. 

This is clearly optimal, in the sense that disseminating a block already takes *O(n)* transmissions. 

LinBFT is designed to be implemented for permission-less, public block chain systems and takes into account anonymous participants without a public-key infrastructure, PoS, rotating leader and a dynamic participant set. [[16]]

For instance, participants can be anonymous, without a centralized public key infrastructure (PKI) public key among themselves, and participate in a distributed key generation (DKG) protocol required to create threshold signatures, both of which are communication-heavy processes. 

LinBFT is compatible with proof-of-state, which counters Sybil attacks and deters dishonest behavior through slashing. [[16]]

### Algorand 

The Algorand WhitePaper was released in May 2017, and is a **synchronous** BFT consensus mechanism; where the blocks get added at a minimum rate. [[25]] 

Algorand allows participants to privately check whether they are chosen for consensus participation and requires only one message per user, thus limiting possible attacks. [[9]]

Alogrand, scales up to 500 000 users by employing Verifiable Random Functions, which are pseudo-random functions able to provide verifiable proofs that the output of said function is correct.[[9]]

It introduces the concept of a *concrete coin*. Most of these BFT algorithms require some sort of randomness oracle, but all nodes need to see the same value if the oracle is consulted. This had previously been achieved through a _common coin_ idea; the *concrete coin* uses a much simpler approach; but only returns a binary value. [[25]]

### Thunderella

Thunderella, implements an asynchronous strategy, where a **synchronous** strategy is used as a fall back in the event of a malfunction [[26]], thus it achieves both robustness and speed. 

It can be applied in permissionless networks using proof-of-work. Network robustness and "instant confirmations" requires both 75% of the network to be honest, as well as the presence of a leader node.

### Snowflake to Avalanche

This consensus protocol was first seen in the WhitePaper entitled "Snowflake to Avalanche". Outlined in the paper are four protocols which are building blocks forming a protocol family. These leaderless Byzantine fault tolerance protocols are build on a metastable mechanism and are referred to as: Slush; Snowflake; Snowball and Avalanche.

The protocols published by Team Rocket differ from the traditional consensus protocols and the Nakamoto consensus protocols by not requiring an elected leader, but instead the protocol simply guides all the nodes to consensus. 

These four protocols are described as a new family of protocols due to this concept of metastability: a means to establish consensus by guiding all nodes towards an emerging consensus without requiring leaders, while still maintaining the same level of security and inducing a speed that exceeding current protocols. 

This is achieved through the formation of 'sub-quorums', which are small randomized samples from nodes on the network. This allows for greater throughputs and sees parallel consensuses running before they merge to form the overarching consensus: what can be seen as similar in nature to the gossip protocol. 

With regards to safety, throughput (the number of transactions per second) and scalability (the number of people supported by the network) Slush, Snowflake, Snowball and Avalanche seem to be able to achieve all three. They impart a probabilistic safety guarantee in the presence of Byzantine adversaries and achieve a high throughput and scalability due to their concurrent nature. A **synchronous** network is assumed. 

This is the current problem facing the design of BFT protocols, in that a system can be very fast when a small number of nodes are active, since there are less decisions to make, however, when there are many users and an increase in transactions, the system cannot be maintained. 

Unlike the PoW implementation, which requires constant active participation from the miners, Avalanche can function with the even when nodes are dormant. 

While traditional consensus protocols require *O*(n<sup>2</sup>) communication, their communication complexity ranges from *O(kn log n)* to *O(kn)* for some security parameter *k<<n*. In a sense, Team Rocket highlight that the communication complexity of their protocols is less intensive than that of *O*(n<sup>2</sup>) communications, thus making these protocols faster and more scalable. 

To backtrack a bit, Big _O_ notation is used in Computer Science to describe the performance or complexity of an algorithm. It descibes the worst-case scenario and can be used to describe the execution time required by an algorithm. [[49]] In the case of consensus algorithms, _O_ describes a finite expected number of steps or operations. [[50]] For example, _O_(1) describes an algorithm that will always execute in teh same time regardless of the size of the input data set. _O_(n)_ describes an algorithm whose performnace will grow linearly and in direct proportion to the size of the input data set.*O*(n<sup>2</sup>)represents an algorithm whose performance is directly proportional to the sqaure of the size of the input data set.

The reason for this is *O(n<sup>2</sup>)* suggests that the rate of growth of function is determined by *n<sup>2</sup>* where *n* is the number of people on the network. Thus, the addition of a person exponentially increases the time taken to disseminate the information on the network while traditional consensus protocols require everyone to communicate with one another- making it a laborious process. [[18]]

Despite assuming a synchronous network, which is susceptible to the DoS attacks, this new family of protocols "reaches a level of security that is simply good enough while surging forward with other advancements". [[18]] 

### PARSEC

PARSEC is a byzantine fault tolerant consensus algorithm possessing **weak synchrony** assumptions (highly asynchronous, assuming random delays with finite expected value)

Similar to HashGraph, it has no leaders, no round robin, no proof of work and reaches eventual consensus with probability one. It differs from HashGraph, in that it provides high speed in the absence and presence of faults.  Thus, it avoids the structures of delegated PoS (DPoS), which requires a trusted set of leaders, and does not have a round robin (where a permissioned set of miners sign each block)

It is fully open, unlike HashGraph, which is patented and closed sourced. The referenece implementation of PARSEC, written in Rust, was realeased a few weeks after the whitepaper [[1]][[37]]. 

The general problem of reaching Byzantine agreement on any value is reduced to the simple problem of reaching binary Byzantine agreement on the nodes participating in each dicision. This has allowed for PARSEC to reuse the binary Byzantine agreement protocol (Signature-Free Asynchronous Byzantine Consensus) after adapting it to the gossip protocol. [[5]] 

Similar to Honeybadger BFT, this protocol is composed through the additions of interesting ideas presented in literature. 

Like HashGraph and Avalanche, a gossip protocol is used to allow efficient communication between nodes. [[1]]

Finally, the need for a trusted leader or a trusted setup phase implied in Mostefaoui et al.[[2]] is removed by porting the key ideas to an asynchronous setting [[3]]

The network is set to *N of N* instances of the algorithm communicating via randomly synchronous connections. 

Due to random synchrony, all users can reach an agreement on what is going on, there is no guarantee for nodes on the timing that they should be receiving messages and a possibility of up to *t* Byzantine (arbitrary) failures are allowed, were *3t*<N. The instances where no failures have occurred are deemed correct or honest, while the failed instances are termed faulty or malicious. Since a Byzantine failure model allows for malicious behavior, any set of instances containing more than 2/3N of them are referred to as the supermajority. 

When a node receives a gossip request, it creates a new event and sends a response back (in HashGraph, the response was optional). Each gossip event contains [[35]]:

1. The data being transmitted
2. The self-parent (the hash of another gossip event created by the same node)
3. The other-parent (a hash of another gossip event created by a different node)
4. The _Cause_ for creation which can either be a Request for information, a Response to another node’s request, or an 
   _Observation_. An observation is when a node creates a gossip event to record an observation that the node made themselves.
5. Creator ID (public key)
6. Signature – signing the above information.

The self-parent and other-parent prevents tampering because they are signed and related to other gossip events [[35]].

As with HashGraph, it is difficult for adversaries to interfere with the consensus algorithm because all voting is *virtual* and done without sharing details of votes cast; each node figures out what other nodes would have voted based on their
copy of the gossip graph.

PARSEC also uses the concept of a *concrete coin*, from Algorand that is used to break ties; particularly in cases where an adversary is carefully managing communication between nodes in order to maintain a deadlock on votes.

First nodes try and converge on a 'true' result for a set of results. If this is not achieved, they move onto step 2, which is trying to converge on a 'false' result. If consensus still cannot be reached, a coin flip is made and we go back to step 1
in another voting round.

## Democratic BFT

This is a deterministic Byzantine consensus algorithm that relies on a new weak coordinator. This protocol is implemented in the Red Belly Block chain and is said to achieve 30 000 transactions/second on Amazon Cloud Trials [[36]], Through the coupling with an optimized variant of the reduction of multivalve to binary consensus from Ben-Or et al., the Democratic BFT (DBFT) consensus algorithm was generated which terminates in 4 message delays in the good case, when all non-faulty processes propose the same value. [[17]]

The term weak coordinator is used to describe the ability of the algorithm to terminate in the presence of a faulty or slow coordinator unlike previous algorithms that do not have the ability to terminate. The fundamental idea here is to allow processes to complete asynchronous rounds as soon as they receive a threshold of messages, instead of having to wait for a message from a coordinator that may be slow. 

The resulting algorithm assumes partial synchrony, is resilience optimal, time optimal and does not require signatures. 

Moving away from the impossibility of solving consensus in asynchronous message systems, where processes can be faulty or *Byzantine*, the technique of randomization or additional synchrony is adopted. 

Randomized algorithms can use per-process "local" coins or a shared *common coin* to solve consensus probabilistically among *n* processes despite $t<n/3$ Byzantine processes. When based on local coins, the existing algorithms converge *O(n*<sup>2.5</sup>*)* expected time. 

A recent randomized algorithm that does not contain a signature solves consensus in *O*(1) expected time under a fair scheduler, where _O_ is the binary.  

To solve the consensus problem deterministically and prevent the use of the common coin, researchers have assumed partial or eventual synchrony. Here, these solutions require a unique coordinator process, referred to as the leader, in order to remain non-faulty. There are both advantages and disadvantages to this technique: the advantage is if the coordinator is non-faulty and if the messages are delivered in a timely manner in an asynchronous round, then the coordinator broadcasts its proposal to all processes and this value is decided after a contest number of message delays; however a faulty coordinator can dramatically impact the algorithm performance by leveraging the power it has in a round and imposing its value to all. Non-faulty processes thus have no other choices but to decide nothing in this round. 

This protocol sees the use of a weak coordinator; a weak coordinator allows for the introduction of a new deterministic Byzantine consensus algorithm that is time optimal, resilience optimal and does not require the use of signatures. Unlike the classic, strong coordinator, the weak coordinator does not impose its value. It allows non-faulty processes to decide a value quickly, without the need of the coordinator, while helping the algorithm to terminate if non-faulty processes know that they proposed distinct values that might all be decided. In addition, the presence of a weak coordinator allows rounds to be executed optimistically without waiting for a specific message. This is unlike classic BFT algorithms that have to wait for a particular message from their coordinator and occasionally has to recover from a slow network or faulty coordinator. 

With regards to the problem of a slow of Byzantine coordinator, the weak coordinator helps agreement by contributing a value while still allowing termination in a constant number of message delays and thus is unlike the classic coordinator or the eventual leader which cannot be implemented in the Binary Byzantine Consensus Algorithm, BAMP<sub>n,t</sub>[*t<n/3*].   

The validation of protocol was conducted similarly to that of the HoneyBadger block chain, where "Coin", the randomization algorithm from Moustefaoui et al. was used [[38]]. Using the 100 Amazon Virtual Machines located in 5 data centers on different continents, it was shown that the DBFT algorithm outperforms that of "Coin"; which is known to terminate in *O*(1) round in expectation. In addition, since Byzantine behaviors have been seen to severely affect the performance of strong coordinator-based consensus, 4 different Byzantine attacks have been tested in the validation. 

[9]: http://conferences.inf.ed.ac.uk/EuroDW2018/papers/eurodw18-Rusch.pdf
"High-Performance Consensus Mechanisms for Blockchains,
Rusch"

[41]: https://www.shoup.net/papers/ckps.pdf
"Secure and Efficent Asynchronous Broadcast Protocols, 
Cachin et al."

[6]: https://eprint.iacr.org/2016/199.pdf
"The Honey Badger of BFT Protocols WhitePaper,
Miller  et al."

[11]: https://ieeexplore.ieee.org/document/8014672/
"Survey of Consensus Protocols of Blockchain Applications,
4th International Conference on Advanced Computing 
and Communication Systems, Sankar et al."

[14]: https://ripple.com/files/ripple_consensus_whitepaper.pdf
"The Ripple Protocol Consensus 
Algorithm, Schwartz et al."

[15]: https://tendermint.com/static/docs/tendermint.pdf
"Tendermint: Consensus without Mining, Kwon"

[16]: https://arxiv.org/pdf/1807.01829.pdf
"LinBFT: Linear-Communication Byzantine 
Fault Tolerance for Public Blockchains, Yang" 

[25]: https://arxiv.org/pdf/1607.01341.pdf
"Algorand WhitePaper 
Chen and Micali"

[26]: https://eprint.iacr.org/2017/913.pdf
"Thunderella WhitePaper, Pass and Shi"

[49]: https://rob-bell.net/2009/06/a-beginners-guide-to-big-o-notation/
"A beginner's guide to Big O notation"

[50]: http://www.cs.yale.edu/homes/aspnes/papers/jalg90.pdf
"Fast Randomized Consensus using Shared Memory,
Aspnes et al."

[18]: https://flatoutcrypto.com/home/avalancheprotocol
"Protocol Spotlight: Avalanche Part 1"

[1]: http://docs.maidsafe.net/Whitepapers/pdf/PARSEC.pdf
"Protocol for Asynchronous, Reliable, 
Secure and Efficient Consensus (PARSEC) 
WhitePaper, Chevalier et al." 

[37]: https://github.com/maidsafe/parsec
"GitHub repository: Protocol for Asynchronous,
Reliable, Secure and Efficient Consensus"

[5]: https://medium.com/@flatoutcrypto/project-spotlight-maidsafe-and-parsec-part-1-4830cec8d9e3
"Project Spotlight: Maidsafe and PARSEC Part 1" 

[2]: https://hal.inria.fr/hal-00944019v2/document
"Signature-Free Asynchronous Byzantine Consensus 
with t<n/3 and O(n2) Messages,
Mostefaoui et al." 

[3]: https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Distributed%20Computation/BYZANTYNE%20AGREEMENT%20MADE%20TRIVIAL.pdf
"Byzantine Agreement Made Trivial
Micali" 

[35]: https://flatoutcrypto.com/home/maidsafeparsecexplanationpt2
"Project Spotlight: Maidsafe and PARSEC Part 2"

[36]: https://www.ccn.com/tag/red-belly-blockchain/
"Red Belly Blockchain"

[17]: http://gramoli.redbellyblockchain.io/web/doc/pubs/DBFT-preprint.pdf
"DBFT: Efficient Byzantine Consensus 
with a Weak Coordinator and its Application 
to Consortium Blockchains, Crain et al."

[38]: https://hal.inria.fr/hal-00944019v2/document
"Signature-Free Asynchronous Byzantine 
Consensus with $t<n/3$ and 
*O*(n<sup>2</sup>) Messages, 
Mostefaoui et al."
