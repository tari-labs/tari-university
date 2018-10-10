# <a name="h-Applications of Byzantine Consensus Mechanisms"> </a>Applications of Byzantine Consensus Mechanisms 

## <a name="h-Abstract"> </a>Abstract 

This report investigates the most promising Byzantine Consensus Mechanisms like HoneyBadger, PARSEC, Stellar Consensus Protocol (SCP), etc. to achieve optimal consensus performance and safety against ill-behaved participants.

This paper focuses on analyzing these consensus protocols and their feasibility and efficiency in meeting the characteristics 

## <a name="h-Contents"> </a>Contents
-[Introduction](#h-Introduction)
-[Consensus](#h-Consensus)
-[Byzantine Faults](#h-Byzantine Faults)
- 


## <a name="h-Introduction"> </a>Introduction 

An attractive alternative to Proof-of-Work is Byzantine Fault Tolerance, which involves negligible computations and ensures no forks. There have been several studies regarding this distributed system. 

However, unlike traditional distributed systems, the block chain setting requires alternate application. 

BFT is thought to not be scalable to the number of participants *n*, since most existing solutions insure the transmission of Ω (*n*<sup>2</sup>) messages, even under favorable network conditions. Thus, existing BFT chains do not have many nodes (e.g. 21 in [] ), which can be elected delegates[] , PoW winners [] or a random sample set obtained through cryptographic sortition . 

The concept of fixed delegates does not aid in decentralization, while PoW introduces uncertainty in the participant set due to forks. 

Sortition-based BFT, provides probabilistic guarantees on safety, as there is no fork capability however, its probability of failure is only small enough with a large sample: a minimum of a few hundred nodes. [[16]]

## <a name="h-Consensus"> </a>Consensus

Consensus algorithms are those algorithms that help a distributed or decentralized network to unanimously take a decision whenever necessary. Its features include assuring decentralized governance, quorum structure, authentication, integrity, non-repudiation, byzantine fault tolerance and performance.  [[11]]

One key property of a block chain system is that the nodes do not trust each other, meaning that some may behave in Byzantine manners. The consensus protocol must therefore tolerate Byzantine failures. The research literature on distributed consensus is vast, and there are many variants of previously proposed protocols being devloped for block chains. They can be largely clasified alsong a spectrum. One extrem consists of purely computation based protocols whta use proof of computation to randomly select a node which single-handedly decides the next operation. The other extreme is purely communication based protocols in which nodes have equal votes and go through multiple rounds of communication to reach consensus, PBFT being the prime example.  [[10]]

## <a name="h-Byzantine Faults"> </a>Byzantine Faults 

A Byzantine fault is a fault presenting different symptoms to different observers. A network is Byzantine Fault Tolerant (BFT) when it can provide service and reach a consensus despite faults or failures of the system. The processes use a protocol for consensus or atomic broadcast to agree on a common sequence of operations to execute. 

## <a name="h-PBFT Variants"> </a>PBFT Variants 

PoW suffers from non-finality, that is a block appended to a blockchain is not confirmed until it is extended by many other blocks. Even then, its existence in the blockchain is only probablistic. For example, eclipse attacks on Bitcoin exploit this probabilistic guarantee to allow double spending. In contrast, the original PBFT protocol is deterministic. [[10]]

## <a name="h-Properties of Ordinary Byzantine Consensus"> </a>Properties of Ordinary Byzantine Consensus 

Weak Validity: Suppose all nodes are correct: if all propose v, then a node many only decide v; if a node decides v, then c was proposed by some node

Agreement: No two correct nodes decide differently 

Termination: Every correct node eventually decides [[7]]

## <a name="h-Scalability-performance trade off"> </a>Scalability-performance trade off 

The scalability of BFT protocols considering the number of participants is highly limited and the performance of most protocols deteriorates as the number of involved replicase increases. This effect is especially problematic for BFT deployment in permission less blockchains. [[7]]

The problem of BFT scalability is twofold: a high throughput as well as a large consensus group with good reconfigurability that can tolerate a high number of failures are both desirable properties in BFT protocols, but are often in direct conflict. 

Bitcoin mining, for example supports thousands of participants, offers good reconfigurability, i.e. nodes can join or leave the network at any time, and can tolerate a high number of failures, while only processing a severely limited number of transactions per second. Most BFT protocols achieve a significantly higher throughput, but are limited to small groups of participants of less than 20 nodes and the group reconfiguration is not easily achievable. 

Several approaches have been employed to remedy these problems, e.g. threshold cryptography, creating new consensus groups for every round, or limiting the number of necessary messages to reach consensus. [[9]]

## <a name="h-Deterministic"> </a>Deterministic 

If all processes start from the same initial state, if all operations that modify the state are deterministic, and if all processes execute the same sequences of operations, then the states of the correct processes will remain the same (this is also called active replication) 

Deterministic, bounded Byzantine agreement relies on consensus being finalised for each epoch before moving to the next one ensuring that there is some safety about a consensus reference point prior to continuing. If instead you allow an unbound number of consensus agreement within the same epoch, then there is no overall consensus reference point with which to declare finality and thus safety is compromised. [[8]]

### <a name="h-FLP Theory"> </a>FLP Theory 

According to the paper, 'Impossibility of Distributed Consensus with One Faulty Process' by Fischer, Lynch and Patterson. 

The problem of consensus- that is, getting a distributed network of processors to agree on a common value-was known to be solvable in a synchronous setting, where processes could proceed in simultaneous steps. In particular, the synchronous solution was resilient to faults, where processors crash and take no further part in the computation. Informally, synchronous models allow failures to be detected by waiting one entire step length for a reply from a processor, and presuming that it has created if no reply is received. 

This kind of failure detection is impossible in an asynchronous setting, where there are no bounds on the amount of time a processor might take to complete its work and then respond. The FLP result shows that in an asynchronous setting, where only one processor might crash, there is no distributed algorithm that solves the consensus problem 

### <a name="h-The Gap Between Asynchronous and Weakly Synchronous Network Models"> </a>The Gap Between Asynchronous and Weakly Synchronous Network Models 

Almost all modern BFT protocols reply on timing assumptions (such as partial or weak synchrony) to guarantee liveness. Purely asynchronous BFT protocols have received considerably less attention in recent years. 

The premise has been held that weak synchrony assumptions are unavoidable, since in any network that violates these assumptions, even asynchronous protocols would provide unacceptable performance. 

## <a name="h-Many Forms of Timing Assumptions (Degrees of Synchrony)"> </a>Many Forms of Timing Assumptions (Degrees of Synchrony) 

### <a name="h-Weak Synchrony"> </a>Weak Synchrony 

Most existing Byzantine fault tolerant systems, even those called 'robust', assume some variation of *weak synchrony*, where messages are guaranteed to be delivered after a certain bound △, but △ may be time-varying or unknown to the protocol designer. 

First, the liveness properties of weakly synchronous protocols can fail completely when the expected timing assumptions are violated (e.g., due to a malicious network adversary). To demonstrate this, we explicitly construct an adversarial "intermittently synchronous" network that violates the assumptions, such that exisiting weakly synchronous protocols such as PBFT would grind to a halt. 

Second, even when the weak synchrony assumptions are satisfied in practice, weakly synchronous protocols degrade significantly in throughput when the underlying network is unpredictable. Ideally, we would like a protocol whose through closely tracks the networks's performance even under rapidly changing network conditions. Unfortunately, weakly asynchronous protocols require timeout parameters that are finicky to tune, especially in cryptocurrency application settings; and when the chosen trout values are either too long to too short, throughput can be hampered. As a concrete example, we show that even when the weak synchrony assumptions are satisfied, such protocols are slow to recover from transient network partitions. 

In terms of feasibility, both weak and partially synchronous protocols are equivalent- a protocol that succeeds in one setting can be systematically adapted for another. In terms of concrete performance, however, adjusting for weak synchrony means gradually increasing the timeout parameter over time (e.g. by an exponential back-off policy). This results in delays when recovering from transient network partition. Protocols typically manifest these assumptions in the form of a timeout event. For example, if parties detect that no progress has been made within a certain interval, then they take a corrective action such as electing a new leader. Asynchronous protocols do not rely on timers, and make progress whenever messages are delivered, regardless of actual clock time. 

#### <a name="h-Counting rounds in asynchronous networks"> </a>Counting rounds in asynchronous networks

Although the guarantee of eventual delivery is decoupled from notions of 'real time', it is nonetheless desirable to characterize the running time of asynchronous protocols. The standard approach is for the adverse to assign each message a virtual round number, subject to the condition that every (*r*-1) message between correct nodes must be delivered before any (*r*+1) message is sent. 

### <a name="h-Random Synchrony"> </a>Random Synchrony

Messages are delivered with random delays, such that the average delay is finite. There many be periods of arbitrarily long days (this is a weaker assumption than weak synchrony, and only a bit stronger than full asynchrony, where the only guarantee is that messages are eventually delivered). As in full asynchrony, it is impossible to tell whether an instance has failed by completely stopping or if there is just a delay in message delivery. [[1]]

### <a name="h-Applications of Byzantine Consensus Mechanisms"> </a>Synchrony

Here, the time for nodes to wait and receive information is predefined. If a node has not received an input within the redefined time structure,  there is a problem. [[5]]
A △-synchronous network guarantees that every message sent is delivered after at most a delay of △ (where △ is a measure of real time) [[6]] Synchronous protocols come to a consensus every x seconds. [[5]]

### <a name="h-Partial Synchrony"> </a>Partial Synchrony 

Here, the network retains some form of a predefined timing structure, however it can operate within knowing the assumption of how fast nodes can exchange messages over the network. Instead of pushing out a block every x seconds, in a partially synchronous blockchain would gauge the limit, with messages always being sent and received within the unknown deadline. 

Distributed Ledger Technology that is not synchronous tend to be partially synchronous. Examples include Cosmos. 

Partially synchronous protocols come to a consensus every unknown amount of seconds. [[5]]

#### <a name="h-Unknown-△ Model"> </a>Unknown-△ Model 

The protocol is unable to use the delay bound as a parameter. [[6]] 

#### <a name="h-Eventually Synchronous"> </a>Eventually Synchronous  

The message delay bound △ is only guaranteed to hold after some (unknown instant, called the "Global Stabilization Time". [[6]]

### <a name="h-Asynchrony"> </a>Asynchrony 

In an asynchronous network, the adversary can deliver messages in any order and at any time, however the message must eventually be delivered between correct nodes. Nodes in an asynchronous network effectively have no use for real time clocks, and can only take actions based on the ordering of messages they receive. [[6]]. The speed is determined by the speed at which the network communicates-instead of a fixed limit of x seconds. 

An asynchronous protocol requires a different means to decide when all nodes are able to come to a consensus.  

The well-known FLP result rules out the possibility of the deterministic asynchronous protocols for atomic broadcast and many other tasks. A deterministic protocol mist therefore make some stronger timing assumptions. [[6]]

#### <a name="h-The Problem"> </a>The Problem 

Protocols based on timing assumptions are unsuitable for decentralized, cryptocurrency settings, where network links can be unreliable, network speeds change rapidly, and network delays may even be adversarially induced. [[6]]

The problem with both synchronous and partially synchronous assumptions is that "the protocols based on timing assumptions are unsuitable for decentralized, cryptocurrency settings, where network links can be unreliable, network speeds change rapidly, and network delays may even be adversarially induced."[[6]]

##### <a name="h-Denial of Service Attack"> </a>Denial of Service Attack 

Basing a protocol on timings, exposes the network to Denial of Service attacks. A synchronous protocol will be deemed unsafe if a DoS slows down the network sufficiently. Even though a partially synchronous protocol would be safe, it would be unable to operate, as the messages would be exposed to interference. 

An asynchronous protocol would be able to function under a DoS attack, however it is difficult to reach consensus, as it is impossible to know if the network is under attack, or if a particular message is delayed by the protocol itself. 

## <a name="h-Gossip Protocol"> </a>Gossip Protocol 

A gossip protocol is a procedure or process of computer–computer communication. The manner in which the protocol functions is  based on the way social networks disseminate information or how epidemics spread. Modern distributed systems have implemented gossip protocols to solve problems that might be difficult to solve in other ways, either because the underlying network has an inconvenient structure, is extremely large, or because gossip solutions are the most efficient ones available. [[4]]

## <a name="h-Permission Blockchains"> </a>Permission Blockchains

Byzantine agreement schemes are considered well suited for permission blockchains, where the identify of the participants is known. Examples include Tendermint, Quorum and Chain. Here the Federated Consensus Algorithm is implemented. [[9]] 

### <a name="h-Hyperledger Fabric (HLF)"> </a>Hyperledger Fabric (HLF)

Hyperledger began as a project under the LinX Foundation in early 2016 [[13]], with the aim of creating an open-source cross-industry standard platform for distributed ledgers. Hyperledger Fabric is an implementation of a distributed ledger platform for running smart contracts, leveraging familiar and proven technologies, with a modular architecture allowing pluggable implementations of various functions. The distributed ledger protocol of the fabric is run on the peers. [[11]]

The blockchains hash chain is computed based on the executed transactions and resulting persistent state. The replicated execution of chaincode is used for validating the transactions. They assume that among *n* validating peers, at most *f<n/3* (where *f* is the number of faulty nodes and *n* is the number of nodes present in the network) may behave arbitrarily, while others will execute correctly, thus adapting to concept BFT consensus. Since hyper ledge fabric proposes to follow Practical Byzantine Fault Tolerance, the chaincode transactions must be deterministic in nature, otherwise different peers might have different persistent state. SIEVE protocol is used to filter out the non-deterministic transactions, thus assuring a unique persistent state among peers. [[11]]

A prominent example for permission blockchain platforms is Hyperledger Fabric (HLF). While being redesigned for a v1.0 release, the formats goal was to achieve extensibility. HLF v1.0 allows for multiple of its modules to be exchanged, *viz* membership service, consensus mechanism. Being permission, this consensus mechanism is mainly responsible for receiving the transaction request from the clients and establishing a total execution order. So far, these pluggable consensus modules include a centralized, single orderer for testing purposes and a crash-tolerant ordering service based on Apache Kafka. [[9]]

## <a name="h-Permissionless Blockchains"> </a>Permissionless Blockchains 

BFT protocols face several limitations when utilized in permission less blockchains as they do not scale well with the number of participants resulting in performance deterioration for the targeted network sizes and as they re not well established in this setting, they are prone security issues, e.g. Sybil attacks. Currently, there are approaches that attempt to circumvent or solve this problem, these include HoneyBadgerBFT, Stellar and Algorand. [[9]]

### <a name="h-Paxos"> </a>Paxos 

While Paxos and Raft and many other well-known protocols tolerate crash faults, Byzantine fault tolerant protocols beginning with PBFT, tolerate even arbitrary corrupted nodes. Many subsequent protocols offer improved performance, often through optimistic execution that provides excellent performance when there are no faults, clients do not contend much, and the network is well behaved, and at least some progress otherwise. 

In general, BFT systems are evaluated in deployment scenarios where latency and CPU are the bottelneck, thus the most effective protocols reduce the number of rounds and minimize expensive cryptographic operations.

Clement et al. initiated a recent line of work by advocating improvement of the worst-case performance, providing service quality guarantees even when the system is under attack- even if this comes at the expense of performance in the optimistic case. However, although the "Robust BFT protocols in this vein gracefully tolerate comprised nodes, they still rely on timing assumptions about the underlying network. Thus focus shifted to asynchronous networks. [[6]] 

#### <a name="h-Optimistic Execution"> </a>Optimistic Execution 

### <a name="h-Raft"> </a>Raft

#### <a name="h-The tradeoff between robustness and responsiveness"> </a>The tradeoff between robustness and responsiveness 

## <a name="h-Randomized Agreement"> </a>Randomized Agreement 

Deterministic asynchronous protocols are impossible for most tasks. While the vast majority of practical BFT protocol steer clear of this impossibility result by making timing assumptions, randomness (and, in particular, cryptography) providers an alternative route. Indeed, we know of asynchronous BFT protocols for a variety of tasks such as binary agreement (ABA), reliable broadcast (RBC) and more. [[6]]

### <a name="h-SINTRA"> </a>SINTRA

SINTRA is a system implementation based on the asynchronous atomic broadcast protocol from Cachin et al. This protocol consists of a reduction from atomic broadcast (ABC) to common subset agreement (ACS), as well as reduction from ACS to multi-value validated agreement (MVBA). This protocol can be improved using a more effect ACS construction, [[6]]

### <a name="h-HoneyNadgerBFT"> </a>HoneyBadgerBFT

HoneyBadgerBFT protocol works with a fixed set of servers to run the consensus; however, this leads to centralization and allows an attacker to specifically target these servers. [[9]]

HoneyBadger offers a novel reductions from ABC to ACS that provides better efficiency (by O(N) factor) through batching, while using threshold encryption to preserve censorship resilience. Better efficiency is also obtained by cherry-picking improved instantiations of sub-components. For example, the expensive MVBA is circumvented by using an ternative ACS along with an effect RBC. [[6]]

### <a name="h-Stellar Consensus Protocol"> </a>Stellar Consensus Protocol

Stellar Consensus Protocol (SCP) proposed by David Mazieres is considered to be a global consensus protocol consisting of nomination protocol and ballot protocol, and is said to be Byzantine Fault Tolerant by bringing with it the concept of quorum slices and defeated byzantine fault tolerance. [[11]]

Each participant forms a quorum of other users, thus creating a trust hierarchy, which requires complex trust decisions. [[9]]

A quorum is a set of nodes that act together to attain consensus and a quorum slice is its subset, which helps a node in its cess of agreement. [[11]]

Initially the nomination proctor is run. During this, new values called candidate values are proposed for agreement. Each node receiving these values will vote for a single value among these. Eventually it results in unanimously selected values for that slot. 

After successful execution of nomination protocol, the nodes deploy the ballot protocol. This involves the federated voting to either commit or abort the values resulting from nomination protocol. This results in externalizing the ballot for the current slot. The aborted ballots are now declared irrelevant. However, there can be stuck states where nodes cannot reach a conclusion, whether to abort or commit a value. This situation is avoided by moving it to a higher valued ballot, considering it in a new ballot protocol execution. This aids in case a node believes that this stuck ballot was committed. Thus SCP assures avoidance and management of stuck states and thus provides liveliness. 

SCP protocol claims to be free of blocked states, provides decentralized control, asymptotic security, flexible trust and low latency. But it does not guarantee safety all the time. If the user node chooses an inefficient quorum slice security is not guaranteed.

The key distinction between FBA and prior Byzantine agreement systems is that, in case of FBA, the nodes that involve in the transaction decide quorums. SCP in sprite of a FBA construction achieves optimal safety against ill-behaved participants. [[11]]

The concept of quorum slices in case of SCP provides asymptotic security and flexible trust, making it more acceptable than other earlier consensus algorithms utilizing FBFT, like the ripple consensus protocol. [[14]] Here, the user is provided more independence in deciding whom to trust. Though the concept of asymptotic security was included in the tendermint consensus algorithm [[15]]

### <a name="h-The Ripple Protocol Consensus Algorithm"> </a>The Ripple Protocol Consensus Algorithm 

### <a name="h-Tendermint"> </a>Tendermint

Tendermint Core is a low-level protocol which is composed of two protocols in one: a consensus algorithm and a peer-to-peer networking protocol. Jae Kwon and Ethan Buchman, inspired by the design goal behind Raft and PBFT, specified Tendermint as an easy to understand, developer-friendly algorithm while doing algorithmically complex systems engineering. The new generation of BFT Proof-of-Stake (poS) consensus algorithms draw elements from Tendermint's BFT adaptation to the public blockchain domain. This adaption is referred to as Tendermint BFT, and is more generally callsifed as BFT-bsed Proof-of-Stake (as opposed to chain-based Proof-of-Stake) 

### <a name="h-LinBFT"> </a>LinBFT

LinBFT is another Byzantine fault tolerance protocol for block chain systems that allows for the amortized *O(n)* communication volume per block under reasonable conditions (where *n* is the number of participants) while satisfying deterministic guarantees on safety and liveness. 

LinBFT cuts down its *O(n*<sup>4</sup>*)* complexity by implementing changes each by  *O(n)*:  linear view change, threshold signatures and verifiable random functions. 

This is clearly optimal, in the sense that disseminating a block already takes *Ω(n)* transmissions. 

LinBFT is designed to be implemented for permission-less, public blockchain systems and takes into account anonymous participants without a public-key infrastructure, proof-of-stake, rotating leader ,and a dynamic participant set. [[16]]

For instance, participants can be anonymous, without a centralized public key infrastructure (PKI) public key among themselves, and participate in a distributed key generation (DKG) protocol required to create threshold signatures, both of which are communication-heavy processes. 

LinBFT is compatible with proof-of-state (PoS), which counters Sybil attacks and deters dishonest behavior through slashing. [[16]]

### <a name="h-Cosmos"> </a>Cosmos

### <a name="h-Algorand"> </a>Algorand 

Algorand allows participants to privately check whether they are chosen for consensus participation and requires only one message per user, thus limiting possible attacks. [[9]]

Alogrand, scales up to 500 000 users by employing Verifiable Random Functions, which are pseudo-random functions able to provide verifiable proofs that the output of said function is correct.[[9]]

### <a name="h-Protocol for Asynchronous, Reliable, Secure and Efficient Consensus (PARSEC)"> </a>Protocol for Asynchronous, Reliable, Secure and Efficient Consensus (PARSEC)

PARSEC is a byzantine fault tolerant consensus algorithm possessing weak synchrony assumptions (highly asynchronous, assuming random delays with finite expected value)

Similar to Hashgraph, it has no leaders, no round robin, no proof of work and reaches eventual consensus with probability one. If differs from Hashgraph, in that it provides high speed in the absence and presence of faults.  Thus, it avoids the structures of delegated proof of stake, which requires a trusted set of leaders, and does not have a round robin (where a set of instated miners sign each block)

It is fully open, unlike Hashgraph, which is patented and closed sourced and the GPLv3 written in Rust will soon be available. [[1]][[5]]

"The general problem of reaching Byzantine agreement on any value is reduced to the simple problem of reaching binary Byzantine agreement not eh nodes participating in each agreement protocol"

Similar to Honeybadger BFT, this protocol is composed through the additions of interesting ideas presented in literature. 

Like Hashgraph and Avalanche, a gossip protocol is used to allow efficient communication between nodes. [[1]]

"The general problem of reaching Byzantine agreement on any value is reduced to the simpler problem of reaching binary Byzantine agreement on the nodes participating in each decision. This allows it to reuse the elegant binary Byzantine agreement protocol after adapting it to the gossip protocol". [[2]]

Finally, the need for a trusted leader or a trusted setup phase implied is removed by porting the key ideas to an asynchronous setting [[3]]

The network is set to N of N instances of the algorithm communicating via randomly synchronous connections. 

Due to random synchrony, all users can reach an agreement on what is going on, there is no guarantee for nodes on the timing that they should be receiving messages and a possibility of up to *t* Byzantine (arbitrary) failures are allowed, were *3t*<N. The instances where no failures have occurred are deemed correct or honest, while the failed instances are termed faulty or malicious. Since a Byzantine failure model allows for malicious behavior, any set of instances containing more than 2/3N of them are referred to as the supermajority. 

PARSEC claims it is possible to reach 100% certainty consensus with no reliance on timing, with up to 1/3 of the network being dishonest even if the network is attacked. 

## <a name="h-DBFT: Efficient Byzantine Consensus with a Weak Coordinator and its Application to COnsortium Block chains"> </a>DBFT: Efficient Byzantine Consensus with a Weak Coordinator and its Application to Constortium Block chains 

This is a deterministic Byzantine consensus algorithm that relies on a new weak coordinator (what is that?)
The term weak coordinator is used to describe the ability of the algorithm to terminate in the presence of a faulty or slow coordinator unlike previous algorithms that do not have the abiliity to terminate. 

## <a name="h-DAGS"> </a>DAGs

Fully asynchronous BFT consensus systems require the limiting structure of a DAG or other equivalent reference point in order to achieve 100% probability of finality (within the liveness and safety BFT thresholds). David Mazieres' SCIP consensus system algorithm designed for Stellar apparently employee the structure of trust reputation as a reference point to achieve 100% probability of finality (within the liveness and safety BFT thresholds). [[8]]

## <a name="h-Conclusion"> </a>Conclusion

## <a name="h-References"> </a>References 

[[1]] Protocol for Asynchronous, Reliable, Secure and Efficient Consensus (PARSEC) WhitePaper, Chevalier et al., http://docs.maidsafe.net/Whitepapers/pdf/PARSEC.pdf, Date accessed: 2018-08-30

[1]: http://docs.maidsafe.net/Whitepapers/pdf/PARSEC.pdf
"Protocol for Asynchronous, Reliable, 
Secure and Efficient Consensus (PARSEC) 
WhitePaper, Chevalier et al." 

[[2]] Signature-Free Asynchronous Byzantine Consensus with t<n/3 and O(n2) Messages, Mostefaoui et al., https://hal.inria.fr/hal-00944019v2/document, Date accessed: 2018-08-30

[2]: https://hal.inria.fr/hal-00944019v2/document
"Signature-Free Asynchronous Byzantine Consensus 
with t<n/3 and O(n2) Messages,
Mostefaoui et al." 

[[3]] Byzantine Agreement Made Trivial, Micali.,https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Distributed%20Computation/BYZANTYNE%20AGREEMENT%20MADE%20TRIVIAL.pdf, Date accessed: 2018-08-30

[3]: https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Distributed%20Computation/BYZANTYNE%20AGREEMENT%20MADE%20TRIVIAL.pdf
"Byzantine Agreement Made Trivial
Micali" 

[[4]] Gossip Protocol. Wikipedia https://en.wikipedia.org/wiki/Gossip_protocol, Date accessed: 2018-09-07

[4]: https://en.wikipedia.org/wiki/Gossip_protocol
"Gossip Protocol, Wikipedia"

[[5]] Project Spotlight: Maidsafe and PARSEC Part 1, https://medium.com/@flatoutcrypto/project-spotlight-maidsafe-and-parsec-part-1-4830cec8d9e3, Date accessed: 2018-08-30

[5]: https://medium.com/@flatoutcrypto/project-spotlight-maidsafe-and-parsec-part-1-4830cec8d9e3
"Project Spotlight: Maidsafe and PARSEC Part 1" 

[[6]] The Honey Badger of BFT Protocols WhitePaper, Miller et al.  https://eprint.iacr.org/2016/199.pdf, Date accessed: 2018-08-30

[6]: https://eprint.iacr.org/2016/199.pdf
"The Honey Badger of BFT Protocols WhitePaper,
Miller  et al."

[[7]] Blockchain, cryptography and consensus, Cachin,  https://cachin.com/cc/talks/20170622-blockchain-ice.pdf, Date accessed: 2018-09-04

[7]:  https://cachin.com/cc/talks/20170622-blockchain-ice.pdf
"Blockchain, crytography and consensus 2017, Cachin"

[[8]] Comments from Medium: I don't see how it's plausible for parallel forks of the hash chain to be finalised concurrently, https://medium.com/@shelby_78386/i-dont-see-how-it-s-plausible-for-parallel-forks-of-the-hash-chain-to-be-finalized-concurrently-cb57afe9dd0a, Date accessed: 2018-09-14 

[8]: https://medium.com/@shelby_78386/i-dont-see-how-it-s-plausible-for-parallel-forks-of-the-hash-chain-to-be-finalized-concurrently-cb57afe9dd0a
"Comments from Medium: 
I don't see hope it's plausible
for parallel forks of the hash chain
to be finalised concurrently"

[[9]] High-Performance Consensus Mechanisms for Blockchains, Rusch,  http://conferences.inf.ed.ac.uk/EuroDW2018/papers/eurodw18-Rusch.pdf, Date accessed: 2018-08-30

[9]: http://conferences.inf.ed.ac.uk/EuroDW2018/papers/eurodw18-Rusch.pdf
"High-Performance Consensus Mechanisms for Blockchains,
Rusch"
 
[[10]] Untangling Blockchain: A Data Processing View of Blockchain Systems, Dinh et al., https://arxiv.org/pdf/1708.05665.pdf, Date accessed: 2018-08-30

[10]: https://arxiv.org/pdf/1708.05665.pdf
"Untangling Blockchain: A Datat Processing View of Blockchain Systems"

[[11]] Survey of Consensus Protocols of Blockchain Applications, Sankar et al., https://ieeexplore.ieee.org/document/8014672/, Date accessed: 2018-08-30

[11]: https://ieeexplore.ieee.org/document/8014672/ 
"Survey of Consensus Protocols of Blockchain Applications,
4th International Conference on Advanced Computing 
and Communication Systems, Sankar et al."

[[12]] The Stellar Consensus Protocol: A Federated Model for Internet-level Consensus, Mazières, https://www.stellar.org/papers/stellar-consensus-protocol.pdf, Date accessed: 2018-08-30

[12]: https://www.stellar.org/papers/stellar-consensus-protocol.pdf
"The Stellar Consensus Procotol: 
A Federated Model for Internet- level
Consensus, Mazières"

[[13]] Architecture of the Hyperledger Blockchain Fabric, Cachin, https://www.zurich.ibm.com/dccl/papers/cachin_dccl.pdf, Date accessed: 2018-09-16

[13]: https://www.zurich.ibm.com/dccl/papers/cachin_dccl.pdf
"Architecture of the Hyperledger 
Blockchain Fabric, Cachin"

[[14]] The Ripple Protocol Consensus Algorithm, Schwartz et al.,  https://ripple.com/files/ripple_consensus_whitepaper.pdf, Date accessed: 2018-09-13

[14]: https://ripple.com/files/ripple_consensus_whitepaper.pdf
"The Ripple Protocol Consensus 
Algorithm, Schwartz et al."

[[15]] Tendermint: Consenus without Mining, Kwon,  https://tendermint.com/static/docs/tendermint.pdf, Date accessed: 2018-09-20

[15]: https://tendermint.com/static/docs/tendermint.pdf
"Tendermint: Consenus without Mining, Kwon"

[[16]] LinBFT: Liner-Communication Byzantine Fault Tolerance for Public Blockchains https://arxiv.org/pdf/1807.01829.pdf